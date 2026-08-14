//! Imperative model-request snapshot commit and provider-dispatch shell.

use animus_runtime::model_request_snapshot as snapshot;

#[path = "modelrequestsnapshot/durable.rs"]
mod durable;
pub use durable::DurableModelRequestSnapshotStore;

/// Explicit current facts checked after private commit and before provider contact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentModelRequestSnapshotBindings {
    /// Current active session leaf.
    pub active_leaf_id: String,
    /// Current turn or authority generation.
    pub turn_generation: u64,
    /// Current request sequence.
    pub request_sequence: u64,
    /// Current model-invocation identity.
    pub model_invocation_identity: String,
    /// Current request-context plan identity.
    pub request_context_plan_identity: String,
    /// Current tool-schema-set identity.
    pub tool_schema_set_identity: String,
    /// Current output-control identity.
    pub output_control_identity: String,
    /// Current budget identity.
    pub budget_identity: String,
    /// Current runtime-authority seal identity.
    pub authority_seal_identity: String,
    /// Current provider-adapter cohort identity.
    pub adapter_cohort_identity: String,
}

impl CurrentModelRequestSnapshotBindings {
    /// Captures exact current facts from a validated snapshot.
    #[must_use]
    pub fn exact(snapshot: &snapshot::ModelRequestSnapshot) -> Self {
        Self {
            active_leaf_id: String::from(snapshot.active_leaf_id()),
            turn_generation: snapshot.turn_generation(),
            request_sequence: snapshot.request_sequence(),
            model_invocation_identity: String::from(snapshot.model_invocation_identity()),
            request_context_plan_identity: String::from(snapshot.request_context_plan_identity()),
            tool_schema_set_identity: String::from(snapshot.tool_schema_set_identity()),
            output_control_identity: String::from(snapshot.output_control_identity()),
            budget_identity: String::from(snapshot.budget_identity()),
            authority_seal_identity: String::from(snapshot.authority_seal_identity()),
            adapter_cohort_identity: String::from(snapshot.adapter_cohort_identity()),
        }
    }

    fn as_facts(&self) -> snapshot::ModelRequestDispatchFacts<'_> {
        snapshot::ModelRequestDispatchFacts {
            active_leaf_id: self.active_leaf_id.as_str(),
            turn_generation: self.turn_generation,
            request_sequence: self.request_sequence,
            model_invocation_identity: self.model_invocation_identity.as_str(),
            request_context_plan_identity: self.request_context_plan_identity.as_str(),
            tool_schema_set_identity: self.tool_schema_set_identity.as_str(),
            output_control_identity: self.output_control_identity.as_str(),
            budget_identity: self.budget_identity.as_str(),
            authority_seal_identity: self.authority_seal_identity.as_str(),
            adapter_cohort_identity: self.adapter_cohort_identity.as_str(),
        }
    }
}

/// Complete provider-bound snapshot input prepared by a pure composition owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedModelRequestSnapshot {
    /// Validated complete private snapshot.
    pub snapshot: snapshot::ModelRequestSnapshot,
    /// Exact prior durable session head.
    pub prior_head_hash: String,
    /// Current facts rechecked before provider contact.
    pub current: CurrentModelRequestSnapshotBindings,
}

/// Private durable snapshot and terminal-observation port.
pub trait ModelRequestSnapshotStorePort {
    /// Commits one complete private request snapshot.
    ///
    /// # Errors
    ///
    /// Returns a safe issue when the append is unavailable, stale, partial, or failed.
    fn commit_snapshot(
        &mut self,
        plan: &snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<snapshot::ModelRequestSnapshotCommitObservation, ModelRequestSnapshotStoreIssue>;

    /// Commits one correlated metadata-only terminal provider observation.
    ///
    /// # Errors
    ///
    /// Returns a safe issue when the terminal append is unavailable, stale, or failed.
    fn commit_terminal(
        &mut self,
        observation: &snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), ModelRequestSnapshotStoreIssue>;

    /// Loads one exact committed private snapshot by identity.
    ///
    /// # Errors
    ///
    /// Returns a safe issue when recovery is unavailable, stale, or malformed.
    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<Option<snapshot::CommittedModelRequestSnapshot>, ModelRequestSnapshotStoreIssue>;

    /// Loads the latest correlated terminal observation for one snapshot.
    ///
    /// # Errors
    ///
    /// Returns a safe issue when recovery is unavailable or malformed.
    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<snapshot::ModelRequestProviderTerminalObservation>,
        ModelRequestSnapshotStoreIssue,
    >;
}

/// Provider effect port that receives only a committed and revalidated request.
pub trait ModelRequestSnapshotProviderPort<Output> {
    /// Dispatches the exact provider-neutral request cited by the dispatch plan.
    ///
    /// # Errors
    ///
    /// Returns a terminal provider status without private provider payloads or raw errors.
    fn dispatch(
        &mut self,
        plan: &snapshot::ModelRequestDispatchPlan,
        request: &animus_adapters::CanonicalModelRequest,
    ) -> Result<Output, snapshot::ModelRequestProviderTerminalStatus>;
}

impl<Output, Dispatch> ModelRequestSnapshotProviderPort<Output> for Dispatch
where
    Dispatch: FnMut(
        &snapshot::ModelRequestDispatchPlan,
        &animus_adapters::CanonicalModelRequest,
    ) -> Result<Output, snapshot::ModelRequestProviderTerminalStatus>,
{
    fn dispatch(
        &mut self,
        plan: &snapshot::ModelRequestDispatchPlan,
        request: &animus_adapters::CanonicalModelRequest,
    ) -> Result<Output, snapshot::ModelRequestProviderTerminalStatus> {
        self(plan, request)
    }
}

/// Safe private-store issue class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelRequestSnapshotStoreIssue {
    /// The store or lease is unavailable.
    Unavailable,
    /// The session lineage or durable head is stale.
    Stale,
    /// The append was incomplete or unsynchronized.
    Partial,
    /// The append failed without safe detail.
    Failed,
}

/// Safe host failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelRequestSnapshotHostIssue {
    /// Pure validation, commit admission, dispatch binding, or terminal correlation failed.
    CoreRejected,
    /// The private snapshot append failed.
    SnapshotCommitFailed,
    /// Provider dispatch returned a terminal failure or cancellation.
    ProviderFailed,
    /// The correlated terminal provider observation could not commit.
    TerminalCommitFailed,
    /// Private committed recovery state could not load or was absent.
    RecoveryLoadFailed,
}

/// Safe host failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRequestSnapshotHostError {
    issue: ModelRequestSnapshotHostIssue,
    core_issue: Option<snapshot::ModelRequestSnapshotIssue>,
    store_issue: Option<ModelRequestSnapshotStoreIssue>,
}

impl ModelRequestSnapshotHostError {
    #[must_use]
    const fn core(error: &snapshot::ModelRequestSnapshotError) -> Self {
        Self {
            issue: ModelRequestSnapshotHostIssue::CoreRejected,
            core_issue: Some(error.issue()),
            store_issue: None,
        }
    }

    #[must_use]
    const fn store(
        issue: ModelRequestSnapshotHostIssue,
        store_issue: ModelRequestSnapshotStoreIssue,
    ) -> Self {
        Self {
            issue,
            core_issue: None,
            store_issue: Some(store_issue),
        }
    }

    #[must_use]
    const fn provider() -> Self {
        Self {
            issue: ModelRequestSnapshotHostIssue::ProviderFailed,
            core_issue: None,
            store_issue: None,
        }
    }

    /// Returns the safe host issue.
    #[must_use]
    pub const fn issue(&self) -> ModelRequestSnapshotHostIssue {
        self.issue
    }

    /// Returns the optional pure-core issue.
    #[must_use]
    pub const fn core_issue(&self) -> Option<snapshot::ModelRequestSnapshotIssue> {
        self.core_issue
    }

    /// Returns the optional private-store issue.
    #[must_use]
    pub const fn store_issue(&self) -> Option<ModelRequestSnapshotStoreIssue> {
        self.store_issue
    }
}

impl core::fmt::Display for ModelRequestSnapshotHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "model request snapshot host failed: {:?}",
            self.issue
        )
    }
}

impl std::error::Error for ModelRequestSnapshotHostError {}

/// Successful provider effect with exact private commit and terminal evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelRequestSnapshotHostOutput<Output> {
    /// Provider output.
    pub output: Output,
    /// Exact committed snapshot.
    pub committed: snapshot::CommittedModelRequestSnapshot,
    /// Exact provider dispatch plan.
    pub dispatch: snapshot::ModelRequestDispatchPlan,
    /// Correlated terminal provider observation.
    pub terminal: snapshot::ModelRequestProviderTerminalObservation,
}

// r[impl agent_harness_runtime.model_request_snapshot.durability]
/// Commits one private snapshot without contacting a provider.
///
/// This split supports restart classification after a process exit at the
/// commit/provider boundary.
///
/// # Errors
///
/// Returns a safe host issue when planning, storage, or exact commit admission fails.
pub fn commit_model_request_snapshot<Store>(
    store: &mut Store,
    prepared: PreparedModelRequestSnapshot,
) -> Result<snapshot::CommittedModelRequestSnapshot, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
{
    let plan = snapshot::plan_model_request_snapshot_commit(
        prepared.snapshot,
        prepared.prior_head_hash.as_str(),
    )
    .map_err(|error| ModelRequestSnapshotHostError::core(&error))?;
    let observation = store.commit_snapshot(&plan).map_err(|issue| {
        ModelRequestSnapshotHostError::store(
            ModelRequestSnapshotHostIssue::SnapshotCommitFailed,
            issue,
        )
    })?;
    snapshot::observe_model_request_snapshot_commit(plan, &observation)
        .map_err(|error| ModelRequestSnapshotHostError::core(&error))
}

// r[impl agent_harness_runtime.model_request_snapshot.dispatch]
/// Commits, revalidates, dispatches, and commits terminal evidence in exact order.
///
/// # Errors
///
/// Returns a safe host issue for private commit, composition drift, provider,
/// or terminal-evidence failure. Provider contact never occurs before commit.
pub fn dispatch_initial_model_request<Store, Provider, Output>(
    store: &mut Store,
    provider: &mut Provider,
    prepared: PreparedModelRequestSnapshot,
) -> Result<ModelRequestSnapshotHostOutput<Output>, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
    Provider: ModelRequestSnapshotProviderPort<Output> + ?Sized,
{
    let current = prepared.current.clone();
    let committed = commit_model_request_snapshot(store, prepared)?;
    let dispatch = snapshot::plan_initial_model_request_dispatch(&committed, current.as_facts())
        .map_err(|error| ModelRequestSnapshotHostError::core(&error))?;
    dispatch_committed_request(store, provider, committed, dispatch)
}

struct SameModelRequestRetryInput<'input> {
    committed: snapshot::CommittedModelRequestSnapshot,
    current: &'input CurrentModelRequestSnapshotBindings,
    owner_decision: snapshot::ModelRequestRetryOwnerDecision,
    attempt_number: u32,
}

/// Concrete existing provider-retry decision supplied to the snapshot shell.
pub struct ProviderOwnerRetryInput<'input> {
    /// Original committed snapshot.
    pub committed: snapshot::CommittedModelRequestSnapshot,
    /// Current composition and authority facts.
    pub current: &'input CurrentModelRequestSnapshotBindings,
    /// Concrete provider-retry owner decision.
    pub owner_decision: &'input animus_runtime::provider_retry::ProviderRetryDecision,
}

// r[impl agent_harness_runtime.model_request_snapshot.dispatch]
/// Dispatches one retry only after the existing retry owner admits the exact request.
///
/// # Errors
///
/// Returns a safe host issue for retry denial, drift, provider failure, or
/// terminal-evidence failure.
fn dispatch_same_model_request_retry<Store, Provider, Output>(
    store: &mut Store,
    provider: &mut Provider,
    input: SameModelRequestRetryInput<'_>,
) -> Result<ModelRequestSnapshotHostOutput<Output>, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
    Provider: ModelRequestSnapshotProviderPort<Output> + ?Sized,
{
    let dispatch = snapshot::plan_same_model_request_retry(
        &input.committed,
        input.current.as_facts(),
        input.owner_decision,
        input.attempt_number,
    )
    .map_err(|error| ModelRequestSnapshotHostError::core(&error))?;
    dispatch_committed_request(store, provider, input.committed, dispatch)
}

/// Dispatches one retry only when the concrete provider-retry owner admits it.
///
/// # Errors
///
/// Returns a safe host issue when the retry decision is not exact or dispatch fails.
pub fn dispatch_same_model_request_retry_from_owner<Store, Provider, Output>(
    store: &mut Store,
    provider: &mut Provider,
    input: ProviderOwnerRetryInput<'_>,
) -> Result<ModelRequestSnapshotHostOutput<Output>, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
    Provider: ModelRequestSnapshotProviderPort<Output> + ?Sized,
{
    let admitted = input.owner_decision.status()
        == animus_runtime::provider_retry::ProviderRetryDecisionStatus::Retry
        && input.owner_decision.exact_plan_hash()
            == input.committed.snapshot().model_invocation_identity();
    let attempt_number = input.owner_decision.next_attempt_count().unwrap_or(0);
    dispatch_same_model_request_retry(
        store,
        provider,
        SameModelRequestRetryInput {
            committed: input.committed,
            current: input.current,
            owner_decision: if admitted {
                snapshot::ModelRequestRetryOwnerDecision::Admitted
            } else {
                snapshot::ModelRequestRetryOwnerDecision::Denied
            },
            attempt_number,
        },
    )
}

// r[impl agent_harness_runtime.model_request_snapshot.recovery]
/// Loads durable request state and classifies restart uncertainty.
///
/// # Errors
///
/// Returns a safe host issue when the committed snapshot is absent, recovery
/// loading fails, or terminal correlation is stale.
pub fn classify_committed_recovery<Store>(
    store: &mut Store,
    snapshot_identity: &str,
) -> Result<snapshot::ModelRequestRecoveryDecision, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
{
    let committed = store
        .load_committed_snapshot(snapshot_identity)
        .map_err(|issue| {
            ModelRequestSnapshotHostError::store(
                ModelRequestSnapshotHostIssue::RecoveryLoadFailed,
                issue,
            )
        })?
        .ok_or_else(|| {
            ModelRequestSnapshotHostError::store(
                ModelRequestSnapshotHostIssue::RecoveryLoadFailed,
                ModelRequestSnapshotStoreIssue::Stale,
            )
        })?;
    let terminal = store
        .load_terminal_observation(snapshot_identity)
        .map_err(|issue| {
            ModelRequestSnapshotHostError::store(
                ModelRequestSnapshotHostIssue::RecoveryLoadFailed,
                issue,
            )
        })?;
    snapshot::classify_model_request_snapshot_recovery(&committed, terminal.as_ref())
        .map_err(|error| ModelRequestSnapshotHostError::core(&error))
}

fn dispatch_committed_request<Store, Provider, Output>(
    store: &mut Store,
    provider: &mut Provider,
    committed: snapshot::CommittedModelRequestSnapshot,
    dispatch: snapshot::ModelRequestDispatchPlan,
) -> Result<ModelRequestSnapshotHostOutput<Output>, ModelRequestSnapshotHostError>
where
    Store: ModelRequestSnapshotStorePort + ?Sized,
    Provider: ModelRequestSnapshotProviderPort<Output> + ?Sized,
{
    let provider_result = provider.dispatch(&dispatch, committed.snapshot().canonical_request());
    let terminal_status = match &provider_result {
        Ok(_output) => snapshot::ModelRequestProviderTerminalStatus::Succeeded,
        Err(status) => *status,
    };
    let terminal = snapshot::observe_model_request_provider_terminal(
        &dispatch,
        snapshot::ModelRequestProviderTerminalDraft {
            snapshot_identity: dispatch.snapshot_identity(),
            adapter_cohort_identity: dispatch.adapter_cohort_identity(),
            attempt_number: dispatch.attempt_number(),
            status: terminal_status,
        },
    )
    .map_err(|error| ModelRequestSnapshotHostError::core(&error))?;
    store.commit_terminal(&terminal).map_err(|issue| {
        ModelRequestSnapshotHostError::store(
            ModelRequestSnapshotHostIssue::TerminalCommitFailed,
            issue,
        )
    })?;
    let output = provider_result.map_err(|_status| ModelRequestSnapshotHostError::provider())?;
    Ok(ModelRequestSnapshotHostOutput {
        output,
        committed,
        dispatch,
        terminal,
    })
}

/// Pure composition owner used by the coding-session provider wrapper.
pub trait CodingSessionModelRequestSnapshotComposer {
    /// Composes the complete snapshot for one coding-session provider effect.
    ///
    /// # Errors
    ///
    /// Returns a safe coding-session service issue for missing or stale composition inputs.
    fn compose(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<
        PreparedModelRequestSnapshot,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    >;
}

/// Coding-session provider adapter that enforces private snapshot commit ordering.
pub struct SnapshotBoundCodingSessionProvider<Composer, Store, Provider> {
    composer: Composer,
    store: Store,
    provider: Provider,
}

impl<Composer, Store, Provider> SnapshotBoundCodingSessionProvider<Composer, Store, Provider> {
    /// Binds explicit composition, private storage, and provider ports.
    #[must_use]
    pub const fn new(composer: Composer, store: Store, provider: Provider) -> Self {
        Self {
            composer,
            store,
            provider,
        }
    }

    /// Returns the explicit owned ports.
    #[must_use]
    pub fn into_parts(self) -> (Composer, Store, Provider) {
        (self.composer, self.store, self.provider)
    }
}

impl<Composer, Store, Provider> super::coding_session::CodingSessionProviderService
    for SnapshotBoundCodingSessionProvider<Composer, Store, Provider>
where
    Composer: CodingSessionModelRequestSnapshotComposer,
    Store: ModelRequestSnapshotStorePort,
    Provider: super::coding_session::CodingSessionProviderService,
{
    fn dispatch(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionProviderReply,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        let prepared = self.composer.compose(request)?;
        let provider = &mut self.provider;
        let mut dispatch =
            |_plan: &snapshot::ModelRequestDispatchPlan,
             _request: &animus_adapters::CanonicalModelRequest| {
                provider.dispatch(request).map_err(|issue| match issue {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Cancelled => {
                        snapshot::ModelRequestProviderTerminalStatus::Cancelled
                    }
                    _other => snapshot::ModelRequestProviderTerminalStatus::Failed,
                })
            };
        dispatch_initial_model_request(&mut self.store, &mut dispatch, prepared)
            .map(|output| output.output)
            .map_err(|error| map_host_issue(&error))
    }
}

fn map_host_issue(
    error: &ModelRequestSnapshotHostError,
) -> animus_runtime::coding_session_host::CodingSessionServiceIssue {
    match error.issue() {
        ModelRequestSnapshotHostIssue::CoreRejected => {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        }
        ModelRequestSnapshotHostIssue::SnapshotCommitFailed
        | ModelRequestSnapshotHostIssue::TerminalCommitFailed
        | ModelRequestSnapshotHostIssue::RecoveryLoadFailed => match error.store_issue() {
            Some(ModelRequestSnapshotStoreIssue::Unavailable) => {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Unavailable
            }
            Some(ModelRequestSnapshotStoreIssue::Stale) => {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale
            }
            Some(ModelRequestSnapshotStoreIssue::Partial) => {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
            }
            Some(ModelRequestSnapshotStoreIssue::Failed) | None => {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
            }
        },
        ModelRequestSnapshotHostIssue::ProviderFailed => {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
        }
    }
}

#[cfg(test)]
#[path = "modelrequestsnapshot/tests.rs"]
mod tests;
