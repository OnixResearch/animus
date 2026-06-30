//! Data-only policy/profile descriptors for prompt guidance composition.

use alloc::{string::String, vec::Vec};

const KIND_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("policy_kind");
const SCOPE_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("policy_scope");
const TRUST_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("policy_trust");
const GOAL_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("policy_goal");
const SUCCESS_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_success_criteria");
const CONSTRAINT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_constraint");
const VALIDATION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_validation_rule");
const STOP_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_stop_rule");
const PROVENANCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_provenance");
const SNIPPET_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("policy_prompt_snippet");
const MIN_POLICY_PRIORITY: u16 = 1;
const MAX_POLICY_PRIORITY: u16 = 1_000;
const MAX_POLICY_ITEMS: usize = 12;
const MAX_POLICY_PROMPT_CHARS: usize = 2_048;
const GLOBAL_SCOPE_RANK: u8 = 0;
const SESSION_SCOPE_RANK: u8 = 1;
const PROJECT_SCOPE_RANK: u8 = 2;
const TOOL_SCOPE_RANK: u8 = 3;
const PROCESS_HEAVY_MARKER: &str = "step-by-step script";
const UNBOUNDED_MARKER: &str = "forever";
const STALE_MODEL_MARKER: &str = "gpt-4 only";
const REQUIRE_APPROVAL_CONSTRAINT: &str = "require-approval-for-destructive-actions";
const ALLOW_DESTRUCTIVE_CONSTRAINT: &str = "allow-destructive-without-approval";
const READ_ONLY_CONSTRAINT: &str = "read-only-session";
const PROMPT_SECTION_SEPARATOR: &str = "\n";

const _: () = assert!(MIN_POLICY_PRIORITY > 0);
const _: () = assert!(MAX_POLICY_PRIORITY >= MIN_POLICY_PRIORITY);
const _: () = assert!(MAX_POLICY_ITEMS > 0);
const _: () = assert!(MAX_POLICY_PROMPT_CHARS > 0);

/// Borrowed host-supplied policy/profile descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProfileDraft<'a> {
    /// Policy kind label.
    pub kind: Option<&'a str>,
    /// Source scope label.
    pub scope: Option<&'a str>,
    /// Trust status label.
    pub trust: Option<&'a str>,
    /// Deterministic priority within a scope.
    pub priority: u16,
    /// Goal-first policy text.
    pub goal: Option<&'a str>,
    /// Observable success criteria.
    pub success_criteria: &'a [&'a str],
    /// Safety or side-effect constraints.
    pub constraints: &'a [&'a str],
    /// Required validation rules.
    pub validation_rules: &'a [&'a str],
    /// Conditions that require stopping or asking the host/user.
    pub stop_rules: &'a [&'a str],
    /// Safe policy source/provenance label.
    pub provenance: Option<&'a str>,
    /// Optional attributed prompt snippets.
    pub prompt_snippets: &'a [&'a str],
}

/// Validated policy/profile descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile {
    kind: PolicyKind,
    scope: SourceScope,
    trust: TrustStatus,
    priority: u16,
    goal: String,
    success_criteria: Vec<String>,
    constraints: Vec<String>,
    validation_rules: Vec<String>,
    stop_rules: Vec<String>,
    provenance: String,
    prompt_snippets: Vec<String>,
}

impl Profile {
    /// Validates a host-supplied policy/profile descriptor as data.
    ///
    /// # Errors
    ///
    /// Returns [`ProfileError`] when required fields are missing, labels are
    /// unsupported, priority or prompt budgets are out of bounds, or prompt
    /// hygiene checks fail.
    pub fn validate(draft: &ProfileDraft<'_>) -> Result<Self, ProfileError> {
        let kind_text = required_text(KIND_FIELD, draft.kind)?;
        let scope_text = required_text(SCOPE_FIELD, draft.scope)?;
        let trust_text = required_text(TRUST_FIELD, draft.trust)?;
        let goal = required_text(GOAL_FIELD, draft.goal)?;
        let provenance = required_text(PROVENANCE_FIELD, draft.provenance)?;
        let kind =
            parse_kind(kind_text.as_str()).ok_or_else(|| ProfileError::UnsupportedValue {
                field: KIND_FIELD.as_str(),
                value: kind_text,
            })?;
        let scope =
            parse_scope(scope_text.as_str()).ok_or_else(|| ProfileError::UnsupportedValue {
                field: SCOPE_FIELD.as_str(),
                value: scope_text,
            })?;
        let trust =
            parse_trust(trust_text.as_str()).ok_or_else(|| ProfileError::UnsupportedValue {
                field: TRUST_FIELD.as_str(),
                value: trust_text,
            })?;
        validate_priority(draft.priority)?;
        let success_criteria = required_collection(SUCCESS_FIELD, draft.success_criteria)?;
        let constraints = optional_collection(CONSTRAINT_FIELD, draft.constraints)?;
        let validation_rules = required_collection(VALIDATION_FIELD, draft.validation_rules)?;
        let stop_rules = required_collection(STOP_FIELD, draft.stop_rules)?;
        let prompt_snippets = optional_collection(SNIPPET_FIELD, draft.prompt_snippets)?;
        let profile = Self {
            kind,
            scope,
            trust,
            priority: draft.priority,
            goal,
            success_criteria,
            constraints,
            validation_rules,
            stop_rules,
            provenance,
            prompt_snippets,
        };
        profile.validate_prompt_hygiene()?;
        Ok(profile)
    }

    /// Returns the policy kind.
    #[must_use]
    pub const fn kind(&self) -> PolicyKind {
        self.kind
    }

    /// Returns the policy source scope.
    #[must_use]
    pub const fn scope(&self) -> SourceScope {
        self.scope
    }

    /// Returns the trust status.
    #[must_use]
    pub const fn trust(&self) -> TrustStatus {
        self.trust
    }

    /// Returns deterministic priority within the source scope.
    #[must_use]
    pub const fn priority(&self) -> u16 {
        self.priority
    }

    /// Returns the goal text.
    #[must_use]
    pub fn goal(&self) -> &str {
        self.goal.as_str()
    }

    /// Returns success criteria.
    #[must_use]
    pub fn success_criteria(&self) -> &[String] {
        self.success_criteria.as_slice()
    }

    /// Returns constraints.
    #[must_use]
    pub fn constraints(&self) -> &[String] {
        self.constraints.as_slice()
    }

    /// Returns validation rules.
    #[must_use]
    pub fn validation_rules(&self) -> &[String] {
        self.validation_rules.as_slice()
    }

    /// Returns stop rules.
    #[must_use]
    pub fn stop_rules(&self) -> &[String] {
        self.stop_rules.as_slice()
    }

    /// Returns safe provenance label.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }

    /// Returns optional prompt snippets.
    #[must_use]
    pub fn prompt_snippets(&self) -> &[String] {
        self.prompt_snippets.as_slice()
    }

    /// Returns the policy/enforcement boundary marker.
    #[must_use]
    pub const fn enforcement_boundary(&self) -> EnforcementBoundary {
        match self.scope {
            SourceScope::Global
            | SourceScope::Session
            | SourceScope::Project
            | SourceScope::Tool => EnforcementBoundary::GuidanceOnly,
        }
    }

    fn precedence_rank(&self) -> u8 {
        match self.scope {
            SourceScope::Global => GLOBAL_SCOPE_RANK,
            SourceScope::Session => SESSION_SCOPE_RANK,
            SourceScope::Project => PROJECT_SCOPE_RANK,
            SourceScope::Tool => TOOL_SCOPE_RANK,
        }
    }

    fn validates_as_read_only_guidance(&self) -> bool {
        self.constraints
            .iter()
            .any(|constraint| constraint == READ_ONLY_CONSTRAINT)
    }

    fn validate_prompt_hygiene(&self) -> Result<(), ProfileError> {
        let mut prompt_chars = self.goal.chars().count();
        for collection in [
            self.success_criteria.as_slice(),
            self.constraints.as_slice(),
            self.validation_rules.as_slice(),
            self.stop_rules.as_slice(),
            self.prompt_snippets.as_slice(),
        ] {
            for value in collection {
                reject_hygiene_markers(value)?;
                prompt_chars = prompt_chars.saturating_add(value.chars().count());
            }
        }
        reject_hygiene_markers(&self.goal)?;
        if prompt_chars > MAX_POLICY_PROMPT_CHARS {
            return Err(ProfileError::PromptHygiene {
                issue: PromptHygieneIssue::PromptTooLong,
            });
        }
        Ok(())
    }
}

/// Policy descriptor kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyKind {
    /// Global operating posture.
    GlobalPosture,
    /// Project-local overlay.
    ProjectOverlay,
    /// Tool- or capability-scoped guidance.
    ToolGuidance,
}

/// Source scope for a policy descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceScope {
    /// Global host or harness policy.
    Global,
    /// Session-scoped policy.
    Session,
    /// Project-local policy.
    Project,
    /// Tool-specific policy.
    Tool,
}

/// Trust status for a policy descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrustStatus {
    /// Host has marked the profile trusted.
    Trusted,
    /// Host has not admitted the profile.
    Untrusted,
}

/// Host trust decision for project-local profiles.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProjectTrustMode {
    /// Project profiles are omitted from prompt composition.
    #[default]
    DenyProjectPolicy,
    /// Trusted project profiles may contribute to prompt composition.
    AdmitTrustedProjectPolicy,
}

/// Policy descriptor boundary relative to runtime enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnforcementBoundary {
    /// Descriptor is prompt/configuration guidance only.
    GuidanceOnly,
}

/// Runtime guard decision required for capability enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeGuardRequirement {
    /// A typed runtime capability guard must still enforce the request.
    Required,
}

/// Prompt segment attributed to a policy profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptSegment {
    provenance: String,
    text: String,
}

impl PromptSegment {
    /// Returns safe policy provenance/source attribution.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }

    /// Returns bounded prompt text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

/// Diagnostic emitted while composing policy profiles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionDiagnostic {
    kind: DiagnosticKind,
    provenance: String,
}

impl CompositionDiagnostic {
    /// Returns the diagnostic kind.
    #[must_use]
    pub const fn kind(&self) -> DiagnosticKind {
        self.kind
    }

    /// Returns safe policy provenance/source attribution.
    #[must_use]
    pub fn provenance(&self) -> &str {
        self.provenance.as_str()
    }
}

/// Diagnostic class emitted while composing policy profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticKind {
    /// Project-local profile was omitted because host trust did not admit it.
    ProjectPolicyOmitted,
    /// Project-local side-effect rule was defeated by a global safety rule.
    GlobalSafetyPrecedence,
}

/// Result of deterministic policy profile composition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionReport {
    segments: Vec<PromptSegment>,
    diagnostics: Vec<CompositionDiagnostic>,
}

impl CompositionReport {
    /// Returns prompt segments in deterministic precedence order.
    #[must_use]
    pub fn segments(&self) -> &[PromptSegment] {
        self.segments.as_slice()
    }

    /// Returns safe composition diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[CompositionDiagnostic] {
        self.diagnostics.as_slice()
    }
}

/// Errors raised while validating policy profiles.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileError {
    /// Generic adapter field contract error.
    Contract(crate::AdapterContractError),
    /// Descriptor value was not one of the supported labels.
    UnsupportedValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: String,
    },
    /// Priority was outside the supported range.
    PriorityOutOfRange {
        /// Minimum supported priority.
        min: u16,
        /// Maximum supported priority.
        max: u16,
        /// Actual priority.
        actual: u16,
    },
    /// Collection exceeded its bounded item ceiling.
    TooManyItems {
        /// Boundary collection field name.
        field: &'static str,
        /// Maximum supported item count.
        max_items: usize,
        /// Actual item count.
        actual_items: usize,
    },
    /// Prompt hygiene validation failed.
    PromptHygiene {
        /// Specific prompt hygiene issue.
        issue: PromptHygieneIssue,
    },
}

impl From<crate::AdapterContractError> for ProfileError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Prompt hygiene issue class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PromptHygieneIssue {
    /// Success criteria were absent.
    MissingSuccessCriteria,
    /// Validation rules were absent.
    MissingValidationRule,
    /// Stop rules were absent.
    MissingStopRule,
    /// Process-heavy script text was detected.
    ProcessHeavy,
    /// Unbounded prompt text was detected.
    UnboundedInstruction,
    /// Stale model-specific guidance was detected.
    StaleModelGuidance,
    /// Prompt text exceeded the bounded budget.
    PromptTooLong,
}

/// Composes trusted policy profiles into attributed prompt segments.
#[must_use]
pub fn compose_policy_context(
    profiles: &[Profile],
    project_trust_mode: ProjectTrustMode,
) -> CompositionReport {
    let mut ordered: Vec<&Profile> = profiles.iter().collect();
    ordered.sort_by(|left, right| {
        left.precedence_rank()
            .cmp(&right.precedence_rank())
            .then_with(|| right.priority.cmp(&left.priority))
            .then_with(|| left.provenance.cmp(&right.provenance))
    });

    let has_global_destructive_guard = ordered.iter().any(|profile| {
        profile.scope == SourceScope::Global && has_constraint(profile, REQUIRE_APPROVAL_CONSTRAINT)
    });
    let mut segments = Vec::new();
    let mut diagnostics = Vec::new();
    for profile in ordered {
        if profile.scope == SourceScope::Project
            && (profile.trust == TrustStatus::Untrusted
                || project_trust_mode == ProjectTrustMode::DenyProjectPolicy)
        {
            diagnostics.push(diagnostic(
                DiagnosticKind::ProjectPolicyOmitted,
                profile.provenance(),
            ));
            continue;
        }
        if has_global_destructive_guard
            && profile.scope == SourceScope::Project
            && has_constraint(profile, ALLOW_DESTRUCTIVE_CONSTRAINT)
        {
            diagnostics.push(diagnostic(
                DiagnosticKind::GlobalSafetyPrecedence,
                profile.provenance(),
            ));
            continue;
        }
        segments.push(prompt_segment(profile));
    }
    CompositionReport {
        segments,
        diagnostics,
    }
}

/// Returns the runtime guard requirement for a policy-described mutation request.
#[must_use]
pub fn mutation_requires_runtime_guard(profile: &Profile) -> RuntimeGuardRequirement {
    if profile.validates_as_read_only_guidance() {
        return RuntimeGuardRequirement::Required;
    }
    RuntimeGuardRequirement::Required
}

fn prompt_segment(profile: &Profile) -> PromptSegment {
    let mut text = String::new();
    push_section(&mut text, "Goal", profile.goal());
    push_items(&mut text, "Success", profile.success_criteria());
    push_items(&mut text, "Constraints", profile.constraints());
    push_items(&mut text, "Validation", profile.validation_rules());
    push_items(&mut text, "Stop", profile.stop_rules());
    push_items(&mut text, "Snippets", profile.prompt_snippets());
    PromptSegment {
        provenance: profile.provenance.clone(),
        text,
    }
}

fn push_section(text: &mut String, label: &str, value: &str) {
    if !text.is_empty() {
        text.push_str(PROMPT_SECTION_SEPARATOR);
    }
    text.push_str(label);
    text.push_str(": ");
    text.push_str(value);
}

fn push_items(text: &mut String, label: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    push_section(text, label, values[0].as_str());
    for value in &values[1..] {
        text.push_str("; ");
        text.push_str(value);
    }
}

fn diagnostic(kind: DiagnosticKind, provenance: &str) -> CompositionDiagnostic {
    CompositionDiagnostic {
        kind,
        provenance: String::from(provenance),
    }
}

fn has_constraint(profile: &Profile, target: &str) -> bool {
    profile
        .constraints
        .iter()
        .any(|constraint| constraint == target)
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<String, ProfileError> {
    match value {
        Some(text) => Ok(crate::validation::normalized_adapter_field(
            field_name, text,
        )?),
        None => Err(crate::AdapterContractError::EmptyField {
            field: field_name.as_str(),
        }
        .into()),
    }
}

fn required_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
) -> Result<Vec<String>, ProfileError> {
    if values.is_empty() {
        let issue = if field_name.as_str() == SUCCESS_FIELD.as_str() {
            PromptHygieneIssue::MissingSuccessCriteria
        } else if field_name.as_str() == VALIDATION_FIELD.as_str() {
            PromptHygieneIssue::MissingValidationRule
        } else {
            PromptHygieneIssue::MissingStopRule
        };
        return Err(ProfileError::PromptHygiene { issue });
    }
    optional_collection(field_name, values)
}

fn optional_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
) -> Result<Vec<String>, ProfileError> {
    let actual_items = values.len();
    if actual_items > MAX_POLICY_ITEMS {
        return Err(ProfileError::TooManyItems {
            field: field_name.as_str(),
            max_items: MAX_POLICY_ITEMS,
            actual_items,
        });
    }
    let mut normalized = Vec::with_capacity(actual_items);
    for value in values {
        normalized.push(crate::validation::normalized_adapter_field(
            field_name, value,
        )?);
    }
    Ok(normalized)
}

fn validate_priority(priority: u16) -> Result<(), ProfileError> {
    if !(MIN_POLICY_PRIORITY..=MAX_POLICY_PRIORITY).contains(&priority) {
        return Err(ProfileError::PriorityOutOfRange {
            min: MIN_POLICY_PRIORITY,
            max: MAX_POLICY_PRIORITY,
            actual: priority,
        });
    }
    Ok(())
}

fn reject_hygiene_markers(value: &str) -> Result<(), ProfileError> {
    let lowered = value.to_ascii_lowercase();
    if lowered.contains(PROCESS_HEAVY_MARKER) {
        return Err(ProfileError::PromptHygiene {
            issue: PromptHygieneIssue::ProcessHeavy,
        });
    }
    if lowered.contains(UNBOUNDED_MARKER) {
        return Err(ProfileError::PromptHygiene {
            issue: PromptHygieneIssue::UnboundedInstruction,
        });
    }
    if lowered.contains(STALE_MODEL_MARKER) {
        return Err(ProfileError::PromptHygiene {
            issue: PromptHygieneIssue::StaleModelGuidance,
        });
    }
    Ok(())
}

fn parse_kind(value: &str) -> Option<PolicyKind> {
    match value {
        "global-posture" => Some(PolicyKind::GlobalPosture),
        "project-overlay" => Some(PolicyKind::ProjectOverlay),
        "tool-guidance" => Some(PolicyKind::ToolGuidance),
        _unsupported => None,
    }
}

fn parse_scope(value: &str) -> Option<SourceScope> {
    match value {
        "global" => Some(SourceScope::Global),
        "session" => Some(SourceScope::Session),
        "project" => Some(SourceScope::Project),
        "tool" => Some(SourceScope::Tool),
        _unsupported => None,
    }
}

fn parse_trust(value: &str) -> Option<TrustStatus> {
    match value {
        "trusted" => Some(TrustStatus::Trusted),
        "untrusted" => Some(TrustStatus::Untrusted),
        _unsupported => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GLOBAL_PROVENANCE: &str = "global-hermes-posture";
    const PROJECT_PROVENANCE: &str = "project-policy";
    const VALIDATION_RULE: &str = "run-smallest-relevant-check";
    const STOP_RULE: &str = "stop-on-destructive-uncertainty";
    const SUCCESS_RULE: &str = "observable-evidence-recorded";

    #[test]
    fn trusted_global_profile_composes_attributed_prompt_segment() {
        let profile = valid_profile(global_draft()).expect("global profile should validate");
        let report = compose_policy_context(
            alloc::slice::from_ref(&profile),
            ProjectTrustMode::DenyProjectPolicy,
        );

        assert_eq!(profile.kind(), PolicyKind::GlobalPosture);
        assert_eq!(profile.scope(), SourceScope::Global);
        assert_eq!(profile.trust(), TrustStatus::Trusted);
        assert_eq!(report.segments().len(), 1);
        assert_eq!(report.segments()[0].provenance(), GLOBAL_PROVENANCE);
        assert!(report.segments()[0].text().starts_with("Goal:"));
        assert!(report.segments()[0].text().contains("Validation:"));
        assert!(report.diagnostics().is_empty());
    }

    #[test]
    fn global_safety_constraint_defeats_project_override() {
        let global = valid_profile(ProfileDraft {
            constraints: &[REQUIRE_APPROVAL_CONSTRAINT],
            ..global_draft()
        })
        .expect("global profile should validate");
        let project = valid_profile(ProfileDraft {
            kind: Some("project-overlay"),
            scope: Some("project"),
            constraints: &[ALLOW_DESTRUCTIVE_CONSTRAINT],
            provenance: Some(PROJECT_PROVENANCE),
            ..global_draft()
        })
        .expect("project profile should validate");
        let report = compose_policy_context(
            &[global, project],
            ProjectTrustMode::AdmitTrustedProjectPolicy,
        );

        assert_eq!(report.segments().len(), 1);
        assert_eq!(report.segments()[0].provenance(), GLOBAL_PROVENANCE);
        assert_eq!(report.diagnostics().len(), 1);
        assert_eq!(
            report.diagnostics()[0].kind(),
            DiagnosticKind::GlobalSafetyPrecedence
        );
        assert_eq!(report.diagnostics()[0].provenance(), PROJECT_PROVENANCE);
        assert!(
            !alloc::format!("{:?}", report.diagnostics()).contains(ALLOW_DESTRUCTIVE_CONSTRAINT)
        );
    }

    #[test]
    fn prompt_hygiene_rejects_bad_policy_text() {
        let cases = [
            (
                ProfileDraft {
                    prompt_snippets: &["long step-by-step script for every action"],
                    ..global_draft()
                },
                PromptHygieneIssue::ProcessHeavy,
            ),
            (
                ProfileDraft {
                    stop_rules: &["continue forever without stopping"],
                    ..global_draft()
                },
                PromptHygieneIssue::UnboundedInstruction,
            ),
            (
                ProfileDraft {
                    constraints: &["use gpt-4 only for all tasks"],
                    ..global_draft()
                },
                PromptHygieneIssue::StaleModelGuidance,
            ),
            (
                ProfileDraft {
                    validation_rules: &[],
                    ..global_draft()
                },
                PromptHygieneIssue::MissingValidationRule,
            ),
        ];

        for (draft, expected_issue) in cases {
            let error = valid_profile(draft).expect_err("bad policy should fail hygiene");
            assert_eq!(
                error,
                ProfileError::PromptHygiene {
                    issue: expected_issue
                }
            );
        }
    }

    #[test]
    fn untrusted_project_policy_is_omitted_from_prompt_context() {
        let global = valid_profile(global_draft()).expect("global profile should validate");
        let project = valid_profile(ProfileDraft {
            kind: Some("project-overlay"),
            scope: Some("project"),
            trust: Some("untrusted"),
            provenance: Some(PROJECT_PROVENANCE),
            ..global_draft()
        })
        .expect("untrusted project profile should still parse as data");
        let report =
            compose_policy_context(&[project, global], ProjectTrustMode::DenyProjectPolicy);

        assert_eq!(report.segments().len(), 1);
        assert_eq!(report.segments()[0].provenance(), GLOBAL_PROVENANCE);
        assert_eq!(report.diagnostics().len(), 1);
        assert_eq!(
            report.diagnostics()[0].kind(),
            DiagnosticKind::ProjectPolicyOmitted
        );
    }

    #[test]
    fn read_only_policy_does_not_replace_runtime_enforcement() {
        let profile = valid_profile(ProfileDraft {
            constraints: &[READ_ONLY_CONSTRAINT],
            ..global_draft()
        })
        .expect("read-only guidance should validate");

        assert_eq!(
            profile.enforcement_boundary(),
            EnforcementBoundary::GuidanceOnly
        );
        assert_eq!(
            mutation_requires_runtime_guard(&profile),
            RuntimeGuardRequirement::Required
        );
    }

    fn valid_profile(draft: ProfileDraft<'_>) -> Result<Profile, ProfileError> {
        Profile::validate(&draft)
    }

    fn global_draft() -> ProfileDraft<'static> {
        ProfileDraft {
            kind: Some("global-posture"),
            scope: Some("global"),
            trust: Some("trusted"),
            priority: MIN_POLICY_PRIORITY,
            goal: Some("Complete the requested outcome with concise evidence"),
            success_criteria: &[SUCCESS_RULE],
            constraints: &["preserve-user-changes"],
            validation_rules: &[VALIDATION_RULE],
            stop_rules: &[STOP_RULE],
            provenance: Some(GLOBAL_PROVENANCE),
            prompt_snippets: &["Run the smallest relevant check"],
        }
    }
}
