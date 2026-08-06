//! Host-owned generated policy loading and in-memory active-turn transactions.

use serde::Deserialize;

use animus_runtime::active_turn_recovery::{
    ActiveTurnAcceptancePlan, ActiveTurnAcceptanceStore, ActiveTurnCheckpointStore,
    ActiveTurnRecoveryIssue, ActiveTurnRecoveryPolicy, ActiveTurnRecoveryPolicyInput,
    ActiveTurnReplacementPlan, ActiveTurnSteeringPlan, ActiveTurnSteeringStore,
    ActiveTurnTransactionObservation, ActiveTurnTransactionStatus,
};

/// Stable JSON failure category for generated recovery policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveTurnRecoveryJsonErrorKind {
    /// JSON syntax is malformed.
    Syntax,
    /// JSON values do not match the exact generated shape.
    Data,
    /// JSON ends before one complete value is available.
    EndOfInput,
}

/// Safe host-shell policy error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveTurnRecoveryConfigError {
    /// The explicit policy path cannot be read.
    Read(std::io::ErrorKind),
    /// JSON shape or syntax is invalid.
    Json(ActiveTurnRecoveryJsonErrorKind),
    /// Runtime revalidation rejects policy content.
    Validation(ActiveTurnRecoveryIssue),
}

/// Parses generated recovery-policy JSON without filesystem access.
///
/// # Errors
///
/// Returns a bounded JSON or runtime-validation category.
pub fn parse_active_turn_recovery_policy_json(
    input: &str,
) -> Result<ActiveTurnRecoveryPolicy, ActiveTurnRecoveryConfigError> {
    let generated: GeneratedPolicy =
        serde_json::from_str(input).map_err(|error| classify_json_error(&error))?;
    animus_runtime::active_turn_recovery::validate_active_turn_recovery_policy(
        &generated.into_runtime_input(),
    )
    .map_err(|error| ActiveTurnRecoveryConfigError::Validation(error.issue()))
}

/// Loads generated recovery-policy JSON from an explicit host path.
///
/// # Errors
///
/// Returns a bounded read, JSON, or runtime-validation category.
pub fn load_active_turn_recovery_policy(
    path: impl AsRef<std::path::Path>,
) -> Result<ActiveTurnRecoveryPolicy, ActiveTurnRecoveryConfigError> {
    let input = std::fs::read_to_string(path.as_ref())
        .map_err(|error| ActiveTurnRecoveryConfigError::Read(error.kind()))?;
    parse_active_turn_recovery_policy_json(input.as_str())
}

/// Injected failure used by the deterministic fixture host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActiveTurnFixtureFailure {
    /// The next transaction exposes partial evidence without mutation.
    Partial,
    /// The next transaction reports a store failure without mutation.
    Store,
}

/// Non-default in-memory host that models compare-and-swap transaction evidence.
///
/// This fixture does not claim power-loss durability or store correctness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActiveTurnMemoryHost {
    generation: u64,
    turn_hash: Option<String>,
    checkpoint_hash: Option<String>,
    steering_hashes: Vec<String>,
    next_failure: Option<ActiveTurnFixtureFailure>,
}

impl ActiveTurnMemoryHost {
    /// Creates a host at an explicit initial generation.
    #[must_use]
    pub const fn new(generation: u64) -> Self {
        Self {
            generation,
            turn_hash: None,
            checkpoint_hash: None,
            steering_hashes: Vec::new(),
            next_failure: None,
        }
    }

    /// Selects one failure for the next transaction.
    pub fn fail_next(&mut self, failure: ActiveTurnFixtureFailure) {
        self.next_failure = Some(failure);
    }

    /// Returns the current generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the current active-turn identity.
    #[must_use]
    pub fn turn_hash(&self) -> Option<&str> {
        self.turn_hash.as_deref()
    }

    /// Returns the current checkpoint identity.
    #[must_use]
    pub fn checkpoint_hash(&self) -> Option<&str> {
        self.checkpoint_hash.as_deref()
    }

    /// Returns the number of durable steering identities.
    #[must_use]
    pub fn steering_count(&self) -> usize {
        self.steering_hashes.len()
    }

    fn failure_observation(
        &mut self,
        transaction_hash: &str,
    ) -> Option<ActiveTurnTransactionObservation> {
        let failure = self.next_failure.take()?;
        let status = match failure {
            ActiveTurnFixtureFailure::Partial => ActiveTurnTransactionStatus::Partial,
            ActiveTurnFixtureFailure::Store => ActiveTurnTransactionStatus::StoreFailure,
        };
        Some(ActiveTurnTransactionObservation {
            transaction_hash: transaction_hash.to_string(),
            observed_prior_generation: self.generation,
            observed_generation: self.generation,
            status,
            complete_manifest: false,
        })
    }

    fn stale_observation(&self, transaction_hash: &str) -> ActiveTurnTransactionObservation {
        ActiveTurnTransactionObservation {
            transaction_hash: transaction_hash.to_string(),
            observed_prior_generation: self.generation,
            observed_generation: self.generation,
            status: ActiveTurnTransactionStatus::Stale,
            complete_manifest: false,
        }
    }
}

impl ActiveTurnAcceptanceStore for ActiveTurnMemoryHost {
    fn commit_acceptance(
        &mut self,
        plan: &ActiveTurnAcceptancePlan,
    ) -> ActiveTurnTransactionObservation {
        if let Some(observation) = self.failure_observation(plan.transaction_hash()) {
            return observation;
        }
        if self.generation != plan.expected_generation() {
            return self.stale_observation(plan.transaction_hash());
        }
        let prior = self.generation;
        self.generation = plan.next_generation();
        self.turn_hash = Some(plan.turn_hash().to_string());
        ActiveTurnTransactionObservation {
            transaction_hash: plan.transaction_hash().to_string(),
            observed_prior_generation: prior,
            observed_generation: self.generation,
            status: ActiveTurnTransactionStatus::Committed,
            complete_manifest: true,
        }
    }
}

impl ActiveTurnCheckpointStore for ActiveTurnMemoryHost {
    fn replace_checkpoint(
        &mut self,
        plan: &ActiveTurnReplacementPlan,
    ) -> ActiveTurnTransactionObservation {
        if let Some(observation) = self.failure_observation(plan.transaction_hash()) {
            return observation;
        }
        let exact_turn = self.turn_hash.as_deref() == Some(plan.turn_hash());
        if self.generation != plan.expected_generation() || !exact_turn {
            return self.stale_observation(plan.transaction_hash());
        }
        let prior = self.generation;
        self.generation = plan.next_generation();
        self.checkpoint_hash = Some(plan.checkpoint_hash().to_string());
        ActiveTurnTransactionObservation {
            transaction_hash: plan.transaction_hash().to_string(),
            observed_prior_generation: prior,
            observed_generation: self.generation,
            status: ActiveTurnTransactionStatus::Committed,
            complete_manifest: true,
        }
    }
}

impl ActiveTurnSteeringStore for ActiveTurnMemoryHost {
    fn append_steering(
        &mut self,
        plan: &ActiveTurnSteeringPlan,
    ) -> ActiveTurnTransactionObservation {
        if let Some(observation) = self.failure_observation(plan.append_hash()) {
            return observation;
        }
        let exact_turn = self.turn_hash.as_deref() == Some(plan.turn_hash());
        if self.generation != plan.expected_generation() || !exact_turn {
            return self.stale_observation(plan.append_hash());
        }
        let prior = self.generation;
        self.steering_hashes.push(plan.steering_hash().to_string());
        ActiveTurnTransactionObservation {
            transaction_hash: plan.append_hash().to_string(),
            observed_prior_generation: prior,
            observed_generation: self.generation,
            status: ActiveTurnTransactionStatus::Committed,
            complete_manifest: true,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPolicy {
    schema: String,
    policy_id: String,
    max_entries: u32,
    max_entry_bytes: u32,
    max_aggregate_bytes: u64,
    max_steering_entries: u32,
    corruption_posture: String,
    replacement_posture: String,
    transaction_evidence_required: bool,
    retention_class: String,
    receipt_fields: Vec<String>,
    non_claims: Vec<String>,
    policy_hash: String,
}

impl GeneratedPolicy {
    fn into_runtime_input(self) -> ActiveTurnRecoveryPolicyInput {
        ActiveTurnRecoveryPolicyInput {
            schema: self.schema,
            policy_id: self.policy_id,
            max_entries: self.max_entries,
            max_entry_bytes: self.max_entry_bytes,
            max_aggregate_bytes: self.max_aggregate_bytes,
            max_steering_entries: self.max_steering_entries,
            corruption_posture: self.corruption_posture,
            replacement_posture: self.replacement_posture,
            transaction_evidence_required: self.transaction_evidence_required,
            retention_class: self.retention_class,
            receipt_fields: self.receipt_fields,
            non_claims: self.non_claims,
            policy_hash: self.policy_hash,
        }
    }
}

fn classify_json_error(error: &serde_json::Error) -> ActiveTurnRecoveryConfigError {
    let kind = match error.classify() {
        serde_json::error::Category::Syntax => ActiveTurnRecoveryJsonErrorKind::Syntax,
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            ActiveTurnRecoveryJsonErrorKind::Data
        }
        serde_json::error::Category::Eof => ActiveTurnRecoveryJsonErrorKind::EndOfInput,
    };
    ActiveTurnRecoveryConfigError::Json(kind)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::active_turn_recovery::{
        ActiveTurnAcceptanceDraft, active_turn_provider_dispatch_allowed,
        evaluate_active_turn_acceptance, plan_active_turn_acceptance,
    };

    const PRIOR_GENERATION: u64 = 2;
    const NEXT_GENERATION: u64 = 3;
    const TURN_SEQUENCE: u64 = 9;

    #[test]
    fn generated_policy_parses_and_acceptance_commits_atomically() {
        // r[verify agent_harness_runtime.active_turn_recovery.verification]
        let policy =
            parse_active_turn_recovery_policy_json(valid_policy_json()).expect("generated policy");
        let plan = plan_active_turn_acceptance(
            &policy,
            &ActiveTurnAcceptanceDraft {
                session_id: "session.one".to_string(),
                branch_id: "branch.one".to_string(),
                turn_sequence: TURN_SEQUENCE,
                accepted_message_hash: hash("accepted"),
                expected_generation: PRIOR_GENERATION,
            },
        )
        .expect("acceptance plan");
        let mut host = ActiveTurnMemoryHost::new(PRIOR_GENERATION);
        let observation = host.commit_acceptance(&plan);
        let outcome = evaluate_active_turn_acceptance(&plan, &observation).expect("outcome");

        assert_eq!(host.generation(), NEXT_GENERATION);
        assert_eq!(host.turn_hash(), Some(plan.turn_hash()));
        assert!(active_turn_provider_dispatch_allowed(&plan, Some(&outcome)));
    }

    #[test]
    fn stale_and_failed_transactions_do_not_mutate_host_state() {
        let policy =
            parse_active_turn_recovery_policy_json(valid_policy_json()).expect("generated policy");
        let plan = plan_active_turn_acceptance(
            &policy,
            &ActiveTurnAcceptanceDraft {
                session_id: "session.one".to_string(),
                branch_id: "branch.one".to_string(),
                turn_sequence: TURN_SEQUENCE,
                accepted_message_hash: hash("accepted"),
                expected_generation: PRIOR_GENERATION,
            },
        )
        .expect("acceptance plan");
        let mut stale = ActiveTurnMemoryHost::new(NEXT_GENERATION);
        let stale_observation = stale.commit_acceptance(&plan);
        assert_eq!(stale_observation.status, ActiveTurnTransactionStatus::Stale);
        assert!(stale.turn_hash().is_none());

        let mut failed = ActiveTurnMemoryHost::new(PRIOR_GENERATION);
        failed.fail_next(ActiveTurnFixtureFailure::Partial);
        let failed_observation = failed.commit_acceptance(&plan);
        assert_eq!(
            failed_observation.status,
            ActiveTurnTransactionStatus::Partial
        );
        assert_eq!(failed.generation(), PRIOR_GENERATION);
        assert!(failed.turn_hash().is_none());
    }

    #[test]
    fn malformed_and_unsafe_generated_policy_fails_closed() {
        assert_eq!(
            parse_active_turn_recovery_policy_json("{").expect_err("malformed json"),
            ActiveTurnRecoveryConfigError::Json(ActiveTurnRecoveryJsonErrorKind::EndOfInput)
        );
        let unsafe_policy = valid_policy_json().replace("fail_closed", "skip_rows");
        assert_eq!(
            parse_active_turn_recovery_policy_json(unsafe_policy.as_str())
                .expect_err("unsafe policy"),
            ActiveTurnRecoveryConfigError::Validation(ActiveTurnRecoveryIssue::InvalidPolicy)
        );
    }

    fn hash(value: &str) -> String {
        blake3::hash(value.as_bytes()).to_hex().to_string()
    }

    fn valid_policy_json() -> &'static str {
        include_str!("../../../config/generated/active-turn-recovery-policy.json")
    }
}
