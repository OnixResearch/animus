use std::cell::RefCell;
use std::rc::Rc;

use crate::provider::{ProviderStreamError, SharedProviderState};
use animus_std::coding_session_host as coding_host;

const QUEUE_POLICY_LABEL: &str = "live-steel-agent-queue";
const QUEUE_SOURCE: &str = "coding-session-host";
const RUN_ID_PREFIX: &str = "run.live-steel-agent.";
const ENTRY_ID_PREFIX: &str = "entry.live-steel-agent.user.";
const SUMMARY: &str = "live Steel agent summary omitted";

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
    final_text: String,
    receipt: crate::LiveSteelAgentRunReceipt,
}

impl LiveSteelAgentRunOutput {
    /// Returns the final assistant text. This private result is separate from evidence.
    #[must_use]
    pub fn final_text(&self) -> &str {
        self.final_text.as_str()
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

    /// Runs one prompt through `CodingSessionHost` until a terminal result.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentError`] for provider, authority, persistence,
    /// tool, cancellation, bounds, ordering, or receipt failures.
    #[allow(
        clippy::too_many_lines,
        reason = "keep provider, snapshot, session, and kernel lease order visible in one shell"
    )]
    pub fn run_prompt(
        mut self,
        prompt: &str,
    ) -> Result<LiveSteelAgentRunOutput, LiveSteelAgentError> {
        // r[impl agent_harness_runtime.live_steel_agent_shell.tool_loop]
        // r[impl agent_harness_runtime.live_steel_agent_shell.session]
        let shared = Rc::new(RefCell::new(SharedProviderState::default()));
        let services = coding_host::CodingSessionHostServices {
            provider: ProviderAdapter {
                provider: &mut self.provider,
                snapshot_store: &mut self.snapshot_persistence,
                config: &self.config,
                shared: Rc::clone(&shared),
                dispatch_count: 0,
            },
            tool: ToolAdapter {
                tool: &mut self.tool,
                shared: Rc::clone(&shared),
                dispatch_count: 0,
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
        let state = new_state(&self.config)?;
        let mut host = coding_host::CodingSessionHost::new(state, services)
            .map_err(|_error| LiveSteelAgentError::Session)?;
        if self.config.session_resume_required() {
            host.resume()
                .map_err(|_error| LiveSteelAgentError::Session)?;
        }
        let prompt_identity = blake3::hash(prompt.as_bytes()).to_hex().to_string();
        let run_id = format!("{RUN_ID_PREFIX}{prompt_identity}");
        let entry_id = format!("{ENTRY_ID_PREFIX}{prompt_identity}");
        host.prompt(
            run_id.as_str(),
            entry_id.as_str(),
            animus_runtime::coding_session_host::CodingSessionInput {
                body: String::from(prompt),
                posture: animus_runtime::coding_session_host::CodingSessionInputPosture::ExpandedAndAdmitted,
            },
        )
        .map_err(|_error| LiveSteelAgentError::Session)?;
        let final_text = host
            .message_snapshot()
            .iter()
            .rev()
            .find(|message| {
                message.kind()
                    == animus_runtime::coding_session_host::CodingSessionMessageKind::Assistant
            })
            .map(|message| String::from(message.body()))
            .ok_or(LiveSteelAgentError::Session)?;
        let committed_message_count = u32::try_from(host.message_snapshot().len())
            .map_err(|_error| LiveSteelAgentError::Receipt)?;
        let event_count = u32::try_from(host.event_snapshot().len())
            .map_err(|_error| LiveSteelAgentError::Receipt)?;
        let coding_host::CodingSessionHostServices {
            provider,
            tool,
            persistence,
            policy: _,
            authority: _,
            cancellation: _,
            timer: _,
            summarizer: _,
            events: _,
        } = host.into_services();
        let provider_dispatch_count = provider.dispatch_count;
        let tool_dispatch_count = tool.dispatch_count;
        drop(provider);
        drop(tool);
        persistence.close()?;
        self.snapshot_persistence.close()?;
        let kernel_snapshot = self
            .tool
            .kernel()
            .private_journal_snapshot(
                self.config.session_identity(),
                self.config.authority_generation(),
            )
            .map_err(|_error| LiveSteelAgentError::Receipt)?;
        let host_observation_count = self.tool.host_observation_count();
        let receipt = crate::LiveSteelAgentRunReceipt::completed(
            &crate::receipt::LiveSteelAgentReceiptDraft {
                config_identity: self.config.config_identity(),
                session_identity: self.config.session_identity(),
                committed_message_count,
                provider_dispatch_count,
                tool_dispatch_count,
                host_observation_count,
                event_count,
                kernel_head_identity: kernel_snapshot.committed_head_identity(),
            },
        );
        self.tool
            .close()
            .map_err(|_error| LiveSteelAgentError::Tool)?;
        Ok(LiveSteelAgentRunOutput {
            final_text,
            receipt,
        })
    }
}

struct ProviderAdapter<'a, Provider, SnapshotStore> {
    provider: &'a mut Provider,
    snapshot_store: &'a mut SnapshotStore,
    config: &'a crate::ValidatedLiveSteelAgentConfig,
    shared: Rc<RefCell<SharedProviderState>>,
    dispatch_count: u32,
}

impl<Provider, SnapshotStore> coding_host::CodingSessionProviderService
    for ProviderAdapter<'_, Provider, SnapshotStore>
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
        let planned = crate::provider::plan_request(
            self.config,
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
            crate::provider::initial_dispatch(self.config.model_id(), model_call_id.as_str())
                .map_err(service_issue)?
        };
        let prepared = prepare_live_model_request_snapshot(
            self.config,
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
            self.snapshot_store,
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
                crate::provider::pending_from_call(self.config.model_id(), call)
                    .map_err(service_issue)?,
            );
            return Ok(
                animus_runtime::coding_session_host::CodingSessionProviderReply::ToolCall {
                    assistant_response: assistant,
                    tool_request,
                },
            );
        }
        let text = crate::provider::final_text(&outcome).map_err(service_issue)?;
        let response = animus_adapters::ModelResponse::new(text.as_str()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        Ok(animus_runtime::coding_session_host::CodingSessionProviderReply::Final(response))
    }
}

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
        provider_plan.model_id(),
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
                model_id: provider_plan.model_id(),
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

struct ToolAdapter<'a, Executor, Persistence> {
    tool: &'a mut crate::SteelToolService<Executor, Persistence>,
    shared: Rc<RefCell<SharedProviderState>>,
    dispatch_count: u32,
}

impl<Executor, Persistence> coding_host::CodingSessionToolService
    for ToolAdapter<'_, Executor, Persistence>
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
        let response = self.tool.execute(request.request()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
        })?;
        self.shared
            .borrow_mut()
            .pending_mut()
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?
            .bind_result(response.output())
            .map_err(service_issue)?;
        Ok(response)
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SteelArguments {
    code: String,
}

#[derive(Default)]
struct MetadataEventSink {
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
