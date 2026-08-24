use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::provider::{ProviderStreamError, SharedProviderState};
use animus_std::coding_session_host as coding_host;

const QUEUE_POLICY_LABEL: &str = "live-steel-agent-queue";
const QUEUE_SOURCE: &str = "coding-session-host";
pub(crate) const RUN_ID_PREFIX: &str = "run.live-steel-agent.";
pub(crate) const ENTRY_ID_PREFIX: &str = "entry.live-steel-agent.user.";
const SUMMARY: &str = "live Steel agent summary omitted";
const LIVE_STEEL_TOOL_EXECUTOR_BINDING: &str = "executor.live-steel-agent.steel-repl";
const LIVE_STEEL_TOOL_LABEL: &str = "Steel REPL";
const LIVE_STEEL_TOOL_DESCRIPTION: &str = "Evaluates one bounded Steel expression.";
const LIVE_STEEL_TOOL_PROMPT: &str = "Use steel_repl for one bounded Steel expression.";
const LIVE_STEEL_TOOL_GUIDELINE: &str = "Supply one code argument.";
const LIVE_STEEL_TOOL_PROVENANCE: &str = "maintained-live-composition";
const LIVE_STEEL_TOOL_RESULT_SHAPE: &str = "bounded-text.v1";
const LIVE_PROVIDER_TRANSPORT_ID: &str = "openai-responses-sse-v1";
const LIVE_MODEL_DIALECT_ID: &str = "openai-responses-tools-v1";
const MAX_UTF8_BYTES_PER_CHAR: u64 = 4;

/// Coding-session persistence with explicit lease release.
pub trait SessionPersistence: coding_host::CodingSessionPersistenceService {
    /// Releases the session persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] when release is uncertain.
    fn close(self) -> Result<(), LiveSteelAgentError>
    where
        Self: Sized;
}

impl<Synchronizer> SessionPersistence
    for coding_host::DurableCodingSessionPersistence<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn close(self) -> Result<(), LiveSteelAgentError> {
        self.close().map_err(|_error| LiveSteelAgentError::Session)
    }
}

/// Model-request snapshot persistence with explicit lease release.
pub trait SnapshotPersistence:
    animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort
{
    /// Releases the snapshot persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] when release is uncertain.
    fn close(self) -> Result<(), LiveSteelAgentError>
    where
        Self: Sized;
}

impl<Synchronizer> SnapshotPersistence
    for animus_std::model_request_snapshot_host::DurableModelRequestSnapshotStore<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn close(self) -> Result<(), LiveSteelAgentError> {
        self.close().map_err(|_error| LiveSteelAgentError::Session)
    }
}

/// Complete explicit inputs for one application composition.
pub struct LiveSteelAgentCompositionInput<
    Provider,
    Executor,
    SessionPersistence,
    SnapshotPersistence,
    KernelPersistence,
> {
    /// Validated deterministic application configuration.
    pub config: crate::ValidatedLiveSteelAgentConfig,
    /// Explicit provider stream service.
    pub provider: Provider,
    /// Durable coding-session persistence.
    pub session_persistence: SessionPersistence,
    /// Durable private model-request snapshot persistence.
    pub snapshot_persistence: SnapshotPersistence,
    /// Fresh or effect-disabled restored Steel kernel.
    pub kernel: animus_steel::SteelRlmKernel,
    /// Exact host-effect executor, usually `RlmHostExecutor`.
    pub host_executor: Executor,
    /// Private committed kernel safe-point persistence.
    pub kernel_persistence: KernelPersistence,
}

/// Typed application construction or run failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSteelAgentError {
    /// Provider request, stream, or continuation failed.
    Provider,
    /// Steel tool, host effect, or safe-point persistence failed.
    Tool,
    /// Coding-session state, service, reducer, or ordering failed.
    Session,
    /// Authority or queue policy construction failed.
    Authority,
    /// Selected Steel skills could not load atomically.
    Skill,
    /// Final receipt or kernel safe-point projection failed.
    Receipt,
}

impl core::fmt::Display for LiveSteelAgentError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live Steel agent failed: {self:?}")
    }
}

impl std::error::Error for LiveSteelAgentError {}

/// Completed bounded application output.
pub struct LiveSteelAgentRunOutput {
    pub(crate) final_text: String,
    pub(crate) runtime_events: Vec<animus_runtime::RuntimeEvent>,
    pub(crate) receipt: crate::LiveSteelAgentRunReceipt,
}

impl LiveSteelAgentRunOutput {
    /// Returns the final assistant text. This private result is separate from evidence.
    #[must_use]
    pub fn final_text(&self) -> &str {
        self.final_text.as_str()
    }

    /// Returns the ordered private runtime events for a worker-owned delivery path.
    ///
    /// These events are not evidence and never enter the metadata-only receipt.
    // r[impl agent_harness_runtime.live_terminal_agent_host.private_events.source]
    #[must_use]
    pub fn runtime_events(&self) -> &[animus_runtime::RuntimeEvent] {
        self.runtime_events.as_slice()
    }

    /// Returns the metadata-only run receipt.
    #[must_use]
    pub const fn receipt(&self) -> &crate::LiveSteelAgentRunReceipt {
        &self.receipt
    }
}

/// One visible product composition root.
pub struct LiveSteelAgentComposition<
    Provider,
    Executor,
    SessionPersistence,
    SnapshotPersistence,
    KernelPersistence,
> {
    config: crate::ValidatedLiveSteelAgentConfig,
    provider: Provider,
    session_persistence: SessionPersistence,
    snapshot_persistence: SnapshotPersistence,
    tool: crate::SteelToolService<Executor, KernelPersistence>,
}

impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    LiveSteelAgentComposition<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
where
    Provider: crate::ProviderStreamService,
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: SessionPersistence,
    SnapshotStore: SnapshotPersistence,
    KernelPersistence: crate::tool::KernelSafePointPersistence,
{
    /// Selects and binds the provider, Steel kernel, bridge, host executor,
    /// skills, control, events, receipt, and coding-session adapters.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] before a provider or host effect when
    /// skill, kernel, or composition admission fails.
    pub fn new(
        input: LiveSteelAgentCompositionInput<
            Provider,
            Executor,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
    ) -> Result<Self, LiveSteelAgentError> {
        // r[impl agent_harness_runtime.live_steel_agent_shell.composition]
        let capabilities = input
            .config
            .admitted_operations()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let authority = animus_adapter_rlm_kernel::SteelSkillModuleAuthority {
            session: capabilities.as_slice(),
            profile: capabilities.as_slice(),
            skill: capabilities.as_slice(),
            host: capabilities.as_slice(),
        };
        let plan = animus_adapter_rlm_kernel::resolve_steel_skill_modules(
            input.config.skill_modules(),
            authority,
        )
        .map_err(|_error| LiveSteelAgentError::Skill)?;
        let mut kernel = input.kernel;
        match kernel.skill_module_set_identity() {
            Some(identity) if identity != plan.module_set_identity() => {
                return Err(LiveSteelAgentError::Skill);
            }
            Some(_identity) => {}
            None => {
                kernel
                    .load_skill_modules(&plan)
                    .map_err(|_error| LiveSteelAgentError::Skill)?;
            }
        }
        let tool = crate::SteelToolService::new(
            kernel,
            input.host_executor,
            input.kernel_persistence,
            crate::SteelToolBinding {
                session_identity: String::from(input.config.session_identity()),
                generation: input.config.authority_generation(),
            },
        );
        Ok(Self {
            config: input.config,
            provider: input.provider,
            session_persistence: input.session_persistence,
            snapshot_persistence: input.snapshot_persistence,
            tool,
        })
    }

    /// Opens one persistent session that keeps provider, tool, and persistence
    /// authority inside the live shell while a worker interleaves commands.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] when resume or host construction fails.
    pub fn open_session(
        self,
    ) -> Result<
        crate::PersistentLiveHost<
            Provider,
            Executor,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
        LiveSteelAgentError,
    > {
        // r[impl agent_harness_runtime.live_steel_agent_shell.persistent_session]
        let shared = Rc::new(RefCell::new(SharedProviderState::default()));
        let config = Rc::new(self.config);
        let services = coding_host::CodingSessionHostServices {
            provider: ProviderAdapter {
                provider: self.provider,
                snapshot_store: self.snapshot_persistence,
                config: Rc::clone(&config),
                shared: Rc::clone(&shared),
                dispatch_count: 0,
            },
            tool: ToolAdapter {
                tool: self.tool,
                config: Rc::clone(&config),
                shared,
                dispatch_count: 0,
                pending_stage: None,
            },
            persistence: self.session_persistence,
            policy: coding_host::AllowCodingSessionPolicy,
            authority: coding_host::AdmittedCodingSessionAuthority::default(),
            cancellation: coding_host::ObservedCodingSessionCancellation,
            timer: coding_host::AvailableCodingSessionTimer,
            summarizer: coding_host::FakeCodingSessionSummarizer {
                summary: String::from(SUMMARY),
            },
            events: MetadataEventSink::default(),
        };
        let state = new_state(config.as_ref())?;
        let mut host = coding_host::CodingSessionHost::new(state, services)
            .map_err(|_error| LiveSteelAgentError::Session)?;
        if config.session_resume_required() {
            host.resume()
                .map_err(|_error| LiveSteelAgentError::Session)?;
        }
        Ok(crate::PersistentLiveHost::new(config, host))
    }

    /// Runs one prompt through `CodingSessionHost` until a terminal result.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] for provider, authority, persistence,
    /// tool, cancellation, bounds, ordering, or receipt failures.
    pub fn run_prompt(self, prompt: &str) -> Result<LiveSteelAgentRunOutput, LiveSteelAgentError> {
        // r[impl agent_harness_runtime.live_steel_agent_shell.tool_loop]
        // r[impl agent_harness_runtime.live_steel_agent_shell.session]
        let prompt_identity = blake3::hash(prompt.as_bytes()).to_hex().to_string();
        let mut session = self.open_session()?;
        session.begin_prompt(crate::LiveSteelPrompt {
            correlation_id: prompt_identity.as_str(),
            text: prompt,
        })?;
        session.drive_pending()?;
        session.finish()
    }
}

pub(crate) struct ProviderAdapter<Provider, SnapshotStore> {
    provider: Provider,
    pub(crate) snapshot_store: SnapshotStore,
    config: Rc<crate::ValidatedLiveSteelAgentConfig>,
    shared: Rc<RefCell<SharedProviderState>>,
    pub(crate) dispatch_count: u32,
}

impl<Provider, SnapshotStore> coding_host::CodingSessionProviderService
    for ProviderAdapter<Provider, SnapshotStore>
where
    Provider: crate::ProviderStreamService,
    SnapshotStore: animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort,
{
    fn dispatch(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionProviderReply,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        let (model_call_id, continuation, correlation_id) = {
            let mut shared = self.shared.borrow_mut();
            let model_call_id = shared.next_model_call_id();
            let correlation_id = shared
                .pending()
                .map(|pending| String::from(pending.correlation_id()));
            (model_call_id, shared.pending().is_some(), correlation_id)
        };
        let planned = crate::runtime::provider::plan_request(
            self.config.as_ref(),
            request,
            continuation,
            correlation_id.as_deref(),
        )
        .map_err(service_issue)?;
        let dispatch = if continuation {
            self.shared
                .borrow_mut()
                .take()
                .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?
                .into_dispatch()
                .map_err(service_issue)?
        } else {
            crate::runtime::provider::initial_dispatch(
                self.config.model_id(),
                model_call_id.as_str(),
            )
            .map_err(service_issue)?
        };
        let prepared = prepare_live_model_request_snapshot(
            self.config.as_ref(),
            request,
            &planned.canonical,
            &planned.plan,
        )?;
        let mut pending_dispatch = Some(dispatch);
        let mut provider_effect =
            |_snapshot_plan: &animus_runtime::model_request_snapshot::ModelRequestDispatchPlan,
             _canonical: &animus_adapters::CanonicalModelRequest| {
                let dispatch = pending_dispatch.take().ok_or(
                animus_runtime::model_request_snapshot::ModelRequestProviderTerminalStatus::Failed,
            )?;
                self.provider
                .stream(&planned.plan, dispatch)
                .map_err(|_error| {
                    animus_runtime::model_request_snapshot::ModelRequestProviderTerminalStatus::Failed
                })
            };
        let outcome = animus_std::model_request_snapshot_host::dispatch_initial_model_request(
            &mut self.snapshot_store,
            &mut provider_effect,
            prepared,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?
        .output;
        if outcome.tool_calls().len() > 1 {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed);
        }
        if let Some(call) = outcome.tool_calls().first() {
            if call.tool_name() != animus_std_openai::STEEL_REPL_TOOL_NAME {
                return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied);
            }
            let arguments: SteelArguments =
                serde_json::from_str(call.arguments_json()).map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?;
            let tool_request = steel_tool_request(
                call.continuation_binding().tool_call_id(),
                arguments.code.as_str(),
                self.config.max_tool_result_chars(),
            )?;
            let assistant =
                animus_adapters::ModelResponse::new("steel_repl requested").map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?;
            self.shared.borrow_mut().install(
                crate::runtime::provider::pending_from_call(self.config.model_id(), call)
                    .map_err(service_issue)?,
            );
            return Ok(
                animus_runtime::coding_session_host::CodingSessionProviderReply::ToolCall {
                    assistant_response: assistant,
                    tool_request,
                },
            );
        }
        let text = crate::runtime::provider::final_text(&outcome).map_err(service_issue)?;
        let response = animus_adapters::ModelResponse::new(text.as_str()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        Ok(animus_runtime::coding_session_host::CodingSessionProviderReply::Final(response))
    }
}

// r[impl animus.dialect.ownership.provider]
fn prepare_live_model_request_snapshot(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    canonical: &animus_adapters::CanonicalModelRequest,
    provider_plan: &animus_std::ProviderDispatchPlan,
) -> Result<
    animus_std::model_request_snapshot_host::PreparedModelRequestSnapshot,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    use animus_runtime::model_request_snapshot as snapshot;

    let route_identity = snapshot::model_request_route_identity(
        provider_plan.provider_id(),
        LIVE_PROVIDER_TRANSPORT_ID,
        provider_plan.model_id(),
        LIVE_MODEL_DIALECT_ID,
    );
    let tool_identity = snapshot::model_request_tool_schema_set_identity(canonical);
    let output_identity = snapshot::model_request_output_control_identity(canonical);
    let budget_identity = snapshot::model_request_budget_identity(canonical);
    let authority = crate::build_expected_authority(config)
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)?;
    let model_invocation_identity =
        blake3::hash(provider_plan.request_projection_identity().as_bytes())
            .to_hex()
            .to_string();
    let adapter_cohort_identity = blake3::hash(config.endpoint_identity().as_bytes())
        .to_hex()
        .to_string();
    let validated =
        snapshot::validate_model_request_snapshot(&snapshot::ModelRequestSnapshotDraft {
            schema: snapshot::MODEL_REQUEST_SNAPSHOT_SCHEMA,
            session_id: request.session_id(),
            branch_id: request.branch_id(),
            active_leaf_id: request.active_leaf_id(),
            turn_generation: config.authority_generation(),
            request_sequence: request.request_sequence(),
            canonical_request: canonical,
            route: snapshot::ModelRequestRouteDraft {
                provider_id: provider_plan.provider_id(),
                provider_transport_id: LIVE_PROVIDER_TRANSPORT_ID,
                model_id: provider_plan.model_id(),
                model_dialect_id: LIVE_MODEL_DIALECT_ID,
                route_identity: route_identity.as_str(),
            },
            model_invocation_identity: model_invocation_identity.as_str(),
            request_context_plan_identity: config.config_identity(),
            selected_context_contribution_identities: &[],
            tool_schema_set_identity: tool_identity.as_str(),
            output_control_identity: output_identity.as_str(),
            budget_identity: budget_identity.as_str(),
            authority_seal_identity: authority.unit.unit_identity(),
            adapter_cohort_identity: adapter_cohort_identity.as_str(),
            unsupported: snapshot::ModelRequestSnapshotUnsupportedDraft::default(),
        })
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
    Ok(
        animus_std::model_request_snapshot_host::PreparedModelRequestSnapshot {
            prior_head_hash: String::from(request.durable_head_hash()),
            current:
                animus_std::model_request_snapshot_host::CurrentModelRequestSnapshotBindings::exact(
                    &validated,
                ),
            snapshot: validated,
        },
    )
}

pub(crate) struct ToolAdapter<Executor, Persistence> {
    pub(crate) tool: crate::SteelToolService<Executor, Persistence>,
    config: Rc<crate::ValidatedLiveSteelAgentConfig>,
    shared: Rc<RefCell<SharedProviderState>>,
    pub(crate) dispatch_count: u32,
    pending_stage: Option<animus_std::tool_execution_stage_host::PendingStagedToolExecution>,
}

impl<Executor, Persistence> ToolAdapter<Executor, Persistence> {
    pub(crate) const fn host_executor_mut(&mut self) -> &mut Executor {
        self.tool.host_executor_mut()
    }
}

impl<Executor, Persistence> coding_host::CodingSessionToolService
    for ToolAdapter<Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::tool::KernelSafePointPersistence,
{
    fn dispatch(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionToolRequest,
    ) -> Result<
        animus_adapters::ToolResponse,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        if self.pending_stage.is_some() {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale);
        }
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        let correlation = self
            .shared
            .borrow()
            .pending()
            .map(|pending| String::from(pending.correlation_id()))
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?;
        if request.request().correlation_id() != correlation {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale);
        }
        let catalog = live_steel_tool_catalog(request.request(), self.config.as_ref())?;
        let capabilities = live_steel_tool_capabilities()?;
        let executor_bindings = [animus_runtime::tool_executor_kit::ToolExecutorBinding::new(
            LIVE_STEEL_TOOL_EXECUTOR_BINDING,
            animus_runtime::tool_executor_kit::ToolExecutorReadiness::Ready,
        )
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?];
        let authority =
            crate::build_expected_authority(self.config.as_ref()).map_err(|_error| {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied
            })?;
        let commit_owner =
            animus_runtime::tool_execution_stages::coding_session_tool_commit_owner_identity();
        let stage_input = animus_std::tool_execution_stage_host::StagedToolExecutionInput {
            tool: animus_runtime::tool_executor_kit::ToolExecutionPlanRequest {
                request: request.request(),
                catalog: &catalog,
                capabilities: &capabilities,
                executors: &executor_bindings,
                admission: animus_runtime::tool_executor_kit::ToolExecutorAdmission::admitted(),
            },
            authority_seal_identity: authority.unit.unit_identity(),
            commit_owner_identity: commit_owner.as_str(),
            handlers: &[],
        };
        let mut stage_services = live_noop_stage_services();
        let mut executor = LiveSteelToolKitExecutor {
            tool: &mut self.tool,
        };
        let pending = animus_std::tool_execution_stage_host::execute_staged_tool(
            &mut executor,
            &mut stage_services,
            &stage_input,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        executor.persist_safe_point()?;
        let response = pending.outcome().response().clone();
        self.pending_stage = Some(pending);
        Ok(response)
    }

    fn observe_committed(
        &mut self,
        observation: &coding_host::CodingSessionToolCommitObservation,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        let pending = self
            .pending_stage
            .take()
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?;
        let stage_observation =
            animus_runtime::tool_execution_stages::ToolExecutionStageCommitObservation {
                stage_plan_identity: String::from(pending.plan().plan_identity()),
                result_identity: observation.result_identity.clone(),
                commit_owner_identity: observation.commit_owner_identity.clone(),
                durable_head_identity: observation.durable_head_identity.clone(),
                complete: true,
                synchronized: observation.synchronized,
            };
        let mut post_commit = animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers;
        let completed = animus_std::tool_execution_stage_host::complete_staged_tool_commit(
            &mut post_commit,
            pending,
            &stage_observation,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        self.shared
            .borrow_mut()
            .pending_mut()
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?
            .bind_result(completed.outcome.response().output())
            .map_err(service_issue)
    }
}

struct LiveSteelToolKitExecutor<'tool, Executor, Persistence> {
    tool: &'tool mut crate::SteelToolService<Executor, Persistence>,
}

impl<Executor, Persistence> LiveSteelToolKitExecutor<'_, Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::tool::KernelSafePointPersistence,
{
    fn persist_safe_point(
        &mut self,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        self.tool.persist_staged_safe_point().map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
        })
    }
}

impl<Executor, Persistence> animus_runtime::tool_executor_kit::ToolExecutorService
    for LiveSteelToolKitExecutor<'_, Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::tool::KernelSafePointPersistence,
{
    fn binding_label(&self) -> &str {
        LIVE_STEEL_TOOL_EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &animus_runtime::tool_executor_kit::ToolExecutionPlan,
        request: &animus_adapters::ToolRequest,
    ) -> Result<
        animus_runtime::tool_executor_kit::ToolExecutorOutput,
        animus_runtime::tool_executor_kit::ToolExecutorServiceFailure,
    > {
        let value = self
            .tool
            .execute_stage_value(request)
            .map_err(|error| match error {
                crate::SteelToolError::Cancelled => {
                    animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::Cancelled
                }
                crate::SteelToolError::Request
                | crate::SteelToolError::Evaluation
                | crate::SteelToolError::Host
                | crate::SteelToolError::Output
                | crate::SteelToolError::Persistence => {
                    animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::ExecutionFailed
                }
            })?;
        Ok(animus_runtime::tool_executor_kit::ToolExecutorOutput::new(
            value.as_str(),
            None,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

fn live_noop_stage_services() -> animus_std::tool_execution_stage_host::ToolExecutionStageServices<
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
> {
    animus_std::tool_execution_stage_host::ToolExecutionStageServices {
        pre_admission: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        pre_execution: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        output: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        post_commit: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    }
}

fn live_steel_tool_catalog(
    request: &animus_adapters::ToolRequest,
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<
    animus_runtime::tool_executor_kit::ToolCatalog,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
            label: Some(LIVE_STEEL_TOOL_LABEL),
            description: Some(LIVE_STEEL_TOOL_DESCRIPTION),
            parameter_schema_identity: Some(request.schema_identity()),
            prompt_snippet: Some(LIVE_STEEL_TOOL_PROMPT),
            prompt_guidelines: &[LIVE_STEEL_TOOL_GUIDELINE],
            source_identity: Some(config.config_identity()),
            provenance: Some(LIVE_STEEL_TOOL_PROVENANCE),
            capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
            result_metadata_shape: Some(LIVE_STEEL_TOOL_RESULT_SHAPE),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
    let max_output_bytes = u64::from(config.max_tool_result_chars())
        .checked_mul(MAX_UTF8_BYTES_PER_CHAR)
        .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    let output_policy = animus_runtime::tool_executor_kit::ToolExecutorOutputPolicy::new(
        config.max_tool_result_chars(),
        max_output_bytes,
        false,
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    let definition = animus_runtime::tool_executor_kit::ToolDefinition::from_draft(
        animus_runtime::tool_executor_kit::ToolDefinitionDraft {
            metadata,
            executor_binding: LIVE_STEEL_TOOL_EXECUTOR_BINDING,
            argument_names: &["code"],
            mutation_posture: animus_runtime::tool_executor_kit::ToolMutationPosture::ReadOnly,
            output_policy,
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
        },
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    animus_runtime::tool_executor_kit::ToolCatalog::new(vec![definition])
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)
}

fn live_steel_tool_capabilities() -> Result<
    animus_runtime::RuntimeCapabilityRegistry,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    animus_runtime::RuntimeCapabilityRegistry::from_drafts(&[
        animus_runtime::RuntimeCapabilityDescriptorDraft {
            label: animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY,
            service_kind: "tool",
            scope: None,
        },
    ])
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SteelArguments {
    code: String,
}

#[derive(Default)]
pub(crate) struct MetadataEventSink {
    count: u32,
}

impl coding_host::CodingSessionEventSink for MetadataEventSink {
    fn observe(&mut self, _event: &animus_runtime::RuntimeEvent) {
        self.count = self.count.saturating_add(1);
    }
}

fn new_state(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<animus_runtime::coding_session_host::CodingSessionState, LiveSteelAgentError> {
    let components = animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL
        .iter()
        .map(
            |class| animus_runtime::session_authority_profile::SessionAuthorityComponentDraft {
                class: *class,
                identity: config.authority_component_identity(),
            },
        )
        .collect::<Vec<_>>();
    let authority_profile =
        animus_runtime::session_authority_profile::build_session_authority_profile(
            animus_runtime::session_authority_profile::SessionAuthorityProfileDraft {
                components: components.as_slice(),
            },
        )
        .map_err(|_error| LiveSteelAgentError::Authority)?;
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: config.max_queue_items(),
            allowed_sources: &[QUEUE_SOURCE],
            policy_label: QUEUE_POLICY_LABEL,
            policy_hash: config.config_identity(),
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .map_err(|_error| LiveSteelAgentError::Authority)?;
    animus_runtime::coding_session_host::new_coding_session_state(
        animus_runtime::coding_session_host::CodingSessionConfig {
            session_id: String::from(config.application_id()),
            branch_id: String::from(config.branch_id()),
            queue_profile:
                animus_runtime::coding_session_host::CodingSessionQueueProfile::Sequential,
            services: animus_runtime::coding_session_host::CodingSessionServiceSet::all_available(),
            authority_profile,
            authority_authorization_identity: String::from(
                config.authority_authorization_identity(),
            ),
            resume_required: config.session_resume_required(),
        },
        queue_policy,
    )
    .map_err(|_error| LiveSteelAgentError::Session)
}

fn steel_tool_request(
    correlation_id: &str,
    code: &str,
    visible_char_budget: u32,
) -> Result<
    animus_adapters::ToolRequest,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            allowed_field_names: &["code"],
            compatibility_rules: &[],
            fields: &[animus_adapters::tool::ArgumentFieldDraft {
                name: Some("code"),
                value: Some(code),
            }],
        },
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(correlation_id),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget,
        },
        mutation_targets: &[],
        required_capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)
}

fn service_issue(
    _error: ProviderStreamError,
) -> animus_runtime::coding_session_host::CodingSessionServiceIssue {
    animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
}
