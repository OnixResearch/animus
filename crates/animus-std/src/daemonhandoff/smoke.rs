//! Local fake-backed daemon-handoff smoke.

type FixtureMode = super::test_double::FixtureMode;
type FixtureServices = super::test_double::FixtureServices;
type HandoffHost<S> = super::orchestrator::HandoffHost<S>;
type HandoffHostError = super::orchestrator::HandoffHostError;
type HandoffHostOutcome = super::orchestrator::HandoffHostOutcome;
type HandoffRunRequest = super::orchestrator::HandoffRunRequest;

const LEASE_GENERATION: u64 = 7;
const QUEUE_GENERATION: u64 = 3;
const ACTIVE_TURN_GENERATION: u64 = 9;
const VAULT_GENERATION: u64 = 11;

/// Metadata-only packaged smoke receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffSmokeReceipt {
    /// Final state label.
    pub state: String,
    /// Handoff identity.
    pub handoff: String,
    /// Final transition identity.
    pub transition: String,
    /// Metadata-only core receipt identity.
    pub receipt_identity: String,
    /// Number of injected host calls.
    pub host_call_count: usize,
    /// Required redaction marker.
    pub redaction: String,
    /// Required non-claims.
    pub non_claims: Vec<String>,
}

/// Runs one local success path through the production host entrypoint.
///
/// # Errors
///
/// Returns a bounded host or receipt-validation issue without remote transport.
// r[impl agent_harness_runtime.daemon_handoff.verification]
pub fn run_fake_handoff_smoke() -> Result<HandoffSmokeReceipt, HandoffHostError> {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::Success));
    let outcome = host.run(&fixture_request())?;
    let HandoffHostOutcome::Complete(transaction) = outcome else {
        return Err(HandoffHostError::Core(
            super::core::Issue::InvalidTransition,
        ));
    };
    let receipt =
        super::core::project_handoff_receipt(&transaction, super::core::SensitiveEvidence::None)?;
    super::core::validate_handoff_receipt(&transaction, &receipt)?;
    debug_assert!(!receipt.receipt_identity.is_empty());
    debug_assert!(!host.services().calls.is_empty());
    Ok(HandoffSmokeReceipt {
        state: receipt.state.as_str().to_string(),
        handoff: receipt.handoff.as_str().to_string(),
        transition: receipt.transition.as_str().to_string(),
        receipt_identity: receipt.receipt_identity,
        host_call_count: host.services().calls.len(),
        redaction: receipt.redaction,
        non_claims: receipt.non_claims,
    })
}

pub(super) fn fixture_request() -> HandoffRunRequest {
    HandoffRunRequest {
        idle: super::core::StrictIdleSnapshot {
            worker: super::core::WorkerIdentity::new("worker.old.fixture"),
            session: super::core::SessionIdentity::new("session.fixture"),
            branch: super::core::BranchIdentity::new("branch.main"),
            lease_generation: super::core::LeaseGeneration::new(LEASE_GENERATION),
            primary_queue_generation: super::core::QueueGeneration::new(QUEUE_GENERATION),
            child_queue_generation: super::core::QueueGeneration::new(QUEUE_GENERATION),
            steering_generation: super::core::QueueGeneration::new(QUEUE_GENERATION),
            active_turn_generation: super::core::ActiveTurnGeneration::new(ACTIVE_TURN_GENERATION),
            authority_profile: super::core::AuthorityProfileIdentity::new("authority.fixture"),
            provider: super::core::ActivityState::Idle,
            tool: super::core::ActivityState::Idle,
            repair: super::core::ActivityState::Idle,
            confirmation: super::core::ActivityState::Idle,
            cancellation: super::core::ActivityState::Idle,
            durable_commit: super::core::ActivityState::Idle,
            terminal_transition: super::core::ActivityState::Idle,
            primary_queue_depth: 0,
            child_queue_depth: 0,
            steering_depth: 0,
        },
        session_head: super::core::SessionHeadIdentity::new("session.head.fixture"),
        composer_draft: super::core::ComposerDraftIdentity::new("composer.draft.fixture"),
        vault_generation: super::core::VaultGeneration::new(VAULT_GENERATION),
        policy: super::core::PolicyIdentity::new("policy.handoff.fixture"),
        deadline_class: "local.bounded.fixture".to_string(),
    }
}
