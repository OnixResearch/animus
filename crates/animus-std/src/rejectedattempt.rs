//! Injected host services for rejected-attempt context preparation.
//!
//! Concrete products retain receipt loading, VCS observation, diff reading,
//! classification, redaction, prompt assembly, and all mutation authority.

use animus_runtime::rejected_attempt::{
    AttemptIdentity, ContentIdentity, RejectedAttemptContextPlan, RejectedAttemptIssue,
    RejectedAttemptPolicy, RejectedAttemptRequest, RejectionContextFacts, ReviewHandoffFacts,
    RevisionIdentity, RunIdentity, SummaryIdentity, TaskIdentity, VcsAttemptFacts, VcsProvider,
    WorkspaceSnapshotIdentity, plan_rejected_attempt_context,
};

/// Host-side service failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectedAttemptHostIssue {
    /// A required injected service is unavailable.
    ServiceUnavailable,
    /// The VCS fixture was used with a non-Jujutsu provider.
    WrongVcsProvider,
}

/// Host request containing expected nominal identities only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedAttemptHostRequest {
    /// Expected task identity.
    pub task: TaskIdentity,
    /// Expected run identity.
    pub run: RunIdentity,
    /// Expected attempt identity.
    pub attempt: AttemptIdentity,
    /// Expected revision identity.
    pub revision: RevisionIdentity,
    /// Expected workspace snapshot identity.
    pub workspace_snapshot: WorkspaceSnapshotIdentity,
    /// Expected content identity.
    pub content: ContentIdentity,
    /// Expected summary identity.
    pub summary: SummaryIdentity,
}

/// Injected review-evidence loader.
pub trait ReviewEvidenceService {
    /// Loads one bounded review handoff.
    ///
    /// # Errors
    ///
    /// Returns a service issue when evidence is unavailable.
    fn load_review_handoff(
        &self,
        request: &RejectedAttemptHostRequest,
    ) -> Result<ReviewHandoffFacts, RejectedAttemptHostIssue>;
}

/// Injected current VCS observation service.
pub trait VcsAttemptObservationService {
    /// Observes one current attempt without granting mutation authority.
    ///
    /// # Errors
    ///
    /// Returns a service issue when observation is unavailable.
    fn observe_attempt(
        &self,
        request: &RejectedAttemptHostRequest,
    ) -> Result<VcsAttemptFacts, RejectedAttemptHostIssue>;
}

/// Injected redaction and classification service.
pub trait RejectionContextService {
    /// Loads bounded, classified context candidates.
    ///
    /// # Errors
    ///
    /// Returns a service issue when context preparation is unavailable.
    fn load_context(
        &self,
        request: &RejectedAttemptHostRequest,
    ) -> Result<RejectionContextFacts, RejectedAttemptHostIssue>;
}

/// Named injected service bundle for the thin host shell.
pub struct RejectedAttemptHostServices<'a> {
    /// Review service.
    pub review: &'a dyn ReviewEvidenceService,
    /// VCS service.
    pub vcs: &'a dyn VcsAttemptObservationService,
    /// Redaction and context service.
    pub context: &'a dyn RejectionContextService,
}

/// Host execution failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RejectedAttemptHostError {
    /// An injected service failed.
    Host(RejectedAttemptHostIssue),
    /// The pure planner denied supplied facts.
    Planning(Vec<RejectedAttemptIssue>),
}

/// Loads host-owned facts and delegates all decisions to the pure planner.
///
/// # Errors
///
/// Returns a host-service error or all pure planning issues.
pub fn prepare_rejected_attempt_context(
    request: &RejectedAttemptHostRequest,
    policy: &RejectedAttemptPolicy,
    services: &RejectedAttemptHostServices<'_>,
) -> Result<RejectedAttemptContextPlan, RejectedAttemptHostError> {
    let review = services
        .review
        .load_review_handoff(request)
        .map_err(RejectedAttemptHostError::Host)?;
    let vcs = services
        .vcs
        .observe_attempt(request)
        .map_err(RejectedAttemptHostError::Host)?;
    let context = services
        .context
        .load_context(request)
        .map_err(RejectedAttemptHostError::Host)?;
    let core_request = RejectedAttemptRequest {
        task_id: request.task.clone(),
        run_id: request.run.clone(),
        attempt_id: request.attempt.clone(),
        revision_id: request.revision.clone(),
        workspace_snapshot_id: request.workspace_snapshot.clone(),
        content_id: request.content.clone(),
        review,
        vcs,
        context,
    };
    plan_rejected_attempt_context(&core_request, policy).map_err(RejectedAttemptHostError::Planning)
}

/// Optional bounded Jujutsu host fixture over already-observed facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JujutsuAttemptObservationFixture {
    facts: VcsAttemptFacts,
}

impl JujutsuAttemptObservationFixture {
    /// Creates a fixture from supplied facts.
    ///
    /// # Errors
    ///
    /// Returns an error if the facts do not name Jujutsu.
    pub fn new(facts: VcsAttemptFacts) -> Result<Self, RejectedAttemptHostIssue> {
        if facts.provider != VcsProvider::Jujutsu {
            return Err(RejectedAttemptHostIssue::WrongVcsProvider);
        }
        Ok(Self { facts })
    }
}

impl VcsAttemptObservationService for JujutsuAttemptObservationFixture {
    fn observe_attempt(
        &self,
        _request: &RejectedAttemptHostRequest,
    ) -> Result<VcsAttemptFacts, RejectedAttemptHostIssue> {
        Ok(self.facts.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::rejected_attempt::{
        RawDiffDisposition, ReviewDisposition, ReviewEvidenceProfile, ReviewHandoffIdentity,
    };

    const SUMMARY_BOUND: u32 = 128;
    const DIFF_BOUND: u32 = 256;
    const SINGLE_MATCH: u32 = 1;

    #[test]
    fn injected_services_prepare_current_jujutsu_context() {
        // r[agent_harness_runtime.rejected_attempt.host]
        // r[agent_harness_runtime.rejected_attempt.verification]
        let request = host_request();
        let review = FixedReview(review(&request));
        let context = FixedContext(context(&request));
        let vcs = JujutsuAttemptObservationFixture::new(vcs(&request)).expect("jj facts");
        let services = RejectedAttemptHostServices {
            review: &review,
            vcs: &vcs,
            context: &context,
        };
        let plan = prepare_rejected_attempt_context(&request, &policy(), &services)
            .expect("injected services");
        assert_eq!(plan.attempt_id, request.attempt);
        assert_eq!(plan.raw_diff_disposition, RawDiffDisposition::NotSupplied);
    }

    #[test]
    fn missing_review_service_fails_before_vcs_or_context_authority_is_inferred() {
        // r[agent_harness_runtime.rejected_attempt.host]
        let request = host_request();
        let unavailable = UnavailableReview;
        let context = FixedContext(context(&request));
        let vcs = JujutsuAttemptObservationFixture::new(vcs(&request)).expect("jj facts");
        let services = RejectedAttemptHostServices {
            review: &unavailable,
            vcs: &vcs,
            context: &context,
        };
        let error = prepare_rejected_attempt_context(&request, &policy(), &services)
            .expect_err("missing review must deny");
        assert_eq!(
            error,
            RejectedAttemptHostError::Host(RejectedAttemptHostIssue::ServiceUnavailable)
        );
    }

    struct FixedReview(ReviewHandoffFacts);

    impl ReviewEvidenceService for FixedReview {
        fn load_review_handoff(
            &self,
            _request: &RejectedAttemptHostRequest,
        ) -> Result<ReviewHandoffFacts, RejectedAttemptHostIssue> {
            Ok(self.0.clone())
        }
    }

    struct UnavailableReview;

    impl ReviewEvidenceService for UnavailableReview {
        fn load_review_handoff(
            &self,
            _request: &RejectedAttemptHostRequest,
        ) -> Result<ReviewHandoffFacts, RejectedAttemptHostIssue> {
            Err(RejectedAttemptHostIssue::ServiceUnavailable)
        }
    }

    struct FixedContext(RejectionContextFacts);

    impl RejectionContextService for FixedContext {
        fn load_context(
            &self,
            _request: &RejectedAttemptHostRequest,
        ) -> Result<RejectionContextFacts, RejectedAttemptHostIssue> {
            Ok(self.0.clone())
        }
    }

    fn host_request() -> RejectedAttemptHostRequest {
        RejectedAttemptHostRequest {
            task: TaskIdentity::new("task.one"),
            run: RunIdentity::new("run.one"),
            attempt: AttemptIdentity::new("attempt.one"),
            revision: RevisionIdentity::new("revision.one"),
            workspace_snapshot: WorkspaceSnapshotIdentity::new("workspace.one"),
            content: ContentIdentity::new("content.one"),
            summary: SummaryIdentity::new("summary.one"),
        }
    }

    fn review(request: &RejectedAttemptHostRequest) -> ReviewHandoffFacts {
        ReviewHandoffFacts {
            handoff_id: ReviewHandoffIdentity::new("handoff.one"),
            profile: ReviewEvidenceProfile::CairnRevisionHandoff,
            task_id: request.task.clone(),
            run_id: request.run.clone(),
            attempt_id: request.attempt.clone(),
            revision_id: request.revision.clone(),
            disposition: ReviewDisposition::Rejected,
            matching_attempts: SINGLE_MATCH,
            validated: true,
        }
    }

    fn vcs(request: &RejectedAttemptHostRequest) -> VcsAttemptFacts {
        VcsAttemptFacts {
            provider: VcsProvider::Jujutsu,
            attempt_id: request.attempt.clone(),
            revision_id: request.revision.clone(),
            workspace_snapshot_id: request.workspace_snapshot.clone(),
            content_id: request.content.clone(),
            current: true,
        }
    }

    fn context(request: &RejectedAttemptHostRequest) -> RejectionContextFacts {
        RejectionContextFacts {
            summary_id: request.summary.clone(),
            redacted_summary: "check failed: stale projection".to_string(),
            summary_classification_safe: true,
            raw_diff: None,
            raw_diff_classification_safe: false,
            raw_diff_capability_admitted: false,
        }
    }

    fn policy() -> RejectedAttemptPolicy {
        RejectedAttemptPolicy {
            policy_id: "rejected-attempt.default.v1".to_string(),
            accepted_review_profiles: vec![ReviewEvidenceProfile::CairnRevisionHandoff],
            accepted_vcs_providers: vec![VcsProvider::Jujutsu],
            max_summary_bytes: SUMMARY_BOUND,
            max_raw_diff_bytes: DIFF_BOUND,
            allow_raw_diff: false,
            required_non_claims: vec!["does-not-authorize-retry".to_string()],
        }
    }
}
