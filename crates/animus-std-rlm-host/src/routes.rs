use std::ffi::OsString;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use crate::{
    BindingIdentity, DispatchFailure, DispatchFailureClass, ExecutorLimits, RlmHostResult,
    RouteBinding, RouteClass,
};

const READ_SENTINEL_BYTES: u64 = 1;
const FILE_RESULT_DOMAIN: &str = "animus.rlm-host.file-read.v1";
const PROCESS_OWNER_DOMAIN: &str = "animus.rlm-host.process.v1";
const CONTEXT_OWNER_DOMAIN: &str = "animus.rlm-host.context.v1";
const CHILD_OWNER_DOMAIN: &str = "animus.rlm-host.child.v1";
const DEFAULT_POLL_INTERVAL_MS: u64 = 10;
const DEFAULT_TEARDOWN_TIMEOUT_MS: u64 = 5_000;
const DEFAULT_STDIN_MAX_BYTES: usize = 1;
const DEFAULT_ACCEPTED_EXIT_CODE: i32 = 0;

/// Capability-relative no-follow bounded file reader.
pub struct FileReadPack {
    root: cap_std::fs::Dir,
    binding: RouteBinding,
}

impl FileReadPack {
    /// Binds one already-opened directory capability.
    ///
    /// # Errors
    ///
    /// Returns an error when the binding belongs to another route.
    pub fn new(
        root: cap_std::fs::Dir,
        binding: RouteBinding,
    ) -> Result<Self, crate::ConstructionError> {
        require_route(&binding, RouteClass::FileRead)?;
        Ok(Self { root, binding })
    }

    /// Returns the exact route binding.
    #[must_use]
    pub const fn binding(&self) -> &RouteBinding {
        &self.binding
    }

    pub(crate) fn execute(
        &self,
        target: &str,
        limits: ExecutorLimits,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::FileRead;
        let admitted = cap_root_core::RootRelativePath::new(
            target,
            cap_root_core::AdmissionMode::NonEmptyRelative,
        )
        .map_err(|_error| failure(route, DispatchFailureClass::Denied, "path"))?;
        let before = self
            .root
            .symlink_metadata(admitted.as_path())
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "metadata"))?;
        if before.file_type().is_symlink() || !before.is_file() {
            return Err(failure(route, DispatchFailureClass::Denied, "file_kind"));
        }
        let mut options = cap_std::fs::OpenOptions::new();
        options.read(true);
        #[cfg(unix)]
        {
            use cap_std::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NOFOLLOW);
        }
        let file = self
            .root
            .open_with(admitted.as_path(), &options)
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "open"))?;
        let opened_before = file
            .metadata()
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "metadata"))?;
        if !opened_before.is_file() || opened_before.len() != before.len() {
            return Err(failure(route, DispatchFailureClass::Stale, "path_race"));
        }
        let read_limit = limits
            .max_file_read_bytes
            .checked_add(READ_SENTINEL_BYTES)
            .ok_or_else(|| failure(route, DispatchFailureClass::BoundExceeded, "read_limit"))?;
        let capacity = usize::try_from(limits.max_file_read_bytes)
            .map_err(|_error| failure(route, DispatchFailureClass::BoundExceeded, "file_bytes"))?;
        let mut bytes = Vec::with_capacity(capacity);
        (&file)
            .take(read_limit)
            .read_to_end(&mut bytes)
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "read"))?;
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > limits.max_file_read_bytes {
            return Err(failure(
                route,
                DispatchFailureClass::BoundExceeded,
                "file_bytes",
            ));
        }
        let opened_after = file
            .metadata()
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "metadata"))?;
        let after = self
            .root
            .symlink_metadata(admitted.as_path())
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "metadata"))?;
        if after.file_type().is_symlink()
            || !after.is_file()
            || opened_before.len() != opened_after.len()
            || opened_after.len() != after.len()
            || file_metadata_identity(&before) != file_metadata_identity(&opened_before)
            || file_metadata_identity(&opened_before) != file_metadata_identity(&opened_after)
            || file_metadata_identity(&opened_after) != file_metadata_identity(&after)
        {
            return Err(failure(route, DispatchFailureClass::Stale, "path_race"));
        }
        let value = String::from_utf8(bytes)
            .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "utf8"))?;
        let owner_identity = hash_identity(route, FILE_RESULT_DOMAIN, &[value.as_bytes()])?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding: &self.binding,
                value: value.as_str(),
                owner_identity,
                max_result_chars: limits.max_result_chars,
            },
        )
    }
}

/// Exact revision-gated edit request supplied by the owning adapter.
pub struct WorkspaceEditBinding {
    /// Exact typed host request. It includes snapshot, revision, visibility, and gates.
    pub request: animus_std_workspace_edit::HostRequest<'static>,
}

/// Binds an opaque RLM edit to exact pre-admitted workspace facts.
pub trait WorkspaceEditRequestBinder {
    /// Selects one exact revision, target, visibility set, gates, and operation list.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure when no exact binding matches.
    fn bind_workspace_edit(
        &mut self,
        target: &str,
        replacement: &str,
    ) -> Result<WorkspaceEditBinding, DispatchFailure>;
}

/// Concrete route over the existing revision-gated workspace edit host.
pub struct RevisionGatedEditRoute<'a, Services, Names, Binder> {
    /// Caller-opened capability root.
    pub root: &'a cap_std::fs::Dir,
    /// Existing workspace host services.
    pub services: &'a mut Services,
    /// Existing finite private-stage name owner.
    pub names: &'a mut Names,
    /// Exact request binder.
    pub binder: Binder,
}

impl<Services, Names, Binder> EditRoute for RevisionGatedEditRoute<'_, Services, Names, Binder>
where
    Services: animus_std_workspace_edit::HostServices,
    Names: animus_std_workspace_edit::StageNameService,
    Binder: WorkspaceEditRequestBinder,
{
    fn execute_edit(
        &mut self,
        target: &str,
        replacement: &str,
    ) -> Result<BindingIdentity, DispatchFailure> {
        let route = RouteClass::FileEdit;
        let binding = self.binder.bind_workspace_edit(target, replacement)?;
        let mut host =
            animus_std_workspace_edit::WorkspaceEditHost::new(self.root, self.services, self.names);
        let receipt = host
            .apply(&binding.request)
            .map_err(|error| map_workspace_failure(&error))?;
        BindingIdentity::new(receipt.identity())
            .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "edit_receipt"))
    }
}

/// Application owner that binds and applies opaque RLM edit requests.
pub trait EditRoute {
    /// Calls the existing revision-gated workspace edit host with exact bound facts.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure that preserves conflict meaning.
    fn execute_edit(
        &mut self,
        target: &str,
        replacement: &str,
    ) -> Result<BindingIdentity, DispatchFailure>;
}

/// Binds an opaque RLM tool payload to the existing strict tool request.
pub trait ToolRequestBinding {
    /// Builds one exact current-schema tool request without tool discovery.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure when the payload does not match the named schema.
    fn bind_tool_request(
        &mut self,
        name: &str,
        payload: &str,
        max_result_chars: u32,
    ) -> Result<animus_adapters::ToolRequest, DispatchFailure>;
}

/// Concrete adapter over the existing tool-executor kit.
pub struct ToolKitRoute<'a, Service, Binder> {
    /// Existing metadata-only tool catalog.
    pub catalog: &'a animus_runtime::tool_executor_kit::ToolCatalog,
    /// Exact runtime capability registry.
    pub capabilities: &'a animus_runtime::RuntimeCapabilityRegistry,
    /// Exact current executor bindings.
    pub executors: &'a [animus_runtime::tool_executor_kit::ToolExecutorBinding],
    /// Current tool-specific admission facts.
    pub admission: animus_runtime::tool_executor_kit::ToolExecutorAdmission,
    /// Exact executor service.
    pub service: Service,
    /// Strict request binder.
    pub binder: Binder,
}

impl<Service, Binder> ToolRoute for ToolKitRoute<'_, Service, Binder>
where
    Service: animus_runtime::tool_executor_kit::ToolExecutorService,
    Binder: ToolRequestBinding,
{
    fn execute_tool(
        &mut self,
        name: &str,
        payload: &str,
        max_result_chars: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        let route = RouteClass::Tool;
        let request = self
            .binder
            .bind_tool_request(name, payload, max_result_chars)?;
        let plan = animus_runtime::tool_executor_kit::plan_tool_execution(
            animus_runtime::tool_executor_kit::ToolExecutionPlanRequest {
                request: &request,
                catalog: self.catalog,
                capabilities: self.capabilities,
                executors: self.executors,
                admission: self.admission,
            },
        );
        if !plan.is_admitted() {
            return Err(failure(route, DispatchFailureClass::Denied, "tool_plan"));
        }
        let outcome = animus_runtime::tool_executor_kit::execute_admitted_tool(
            &mut self.service,
            &plan,
            &request,
        )
        .map_err(map_tool_failure)?;
        let identity = BindingIdentity::new(outcome.receipt().receipt_hash())
            .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "tool_receipt"))?;
        Ok((outcome.response().model_visible_text().to_owned(), identity))
    }
}

/// Existing tool-executor-kit owner for one exact route.
pub trait ToolRoute {
    /// Executes through tool catalog admission and one exact executor binding.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure for admission, service, output, or receipt failure.
    fn execute_tool(
        &mut self,
        name: &str,
        payload: &str,
        max_result_chars: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure>;
}

/// Existing application context owner.
pub trait ContextRoute {
    /// Resolves and budgets one explicit context key.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure before excess content becomes visible.
    fn resolve_context(
        &mut self,
        key: &str,
        max_bytes: u32,
        max_lines: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure>;
}

/// Existing child-session service owner.
pub trait ChildRoute {
    /// Executes one explicitly staged RLM child through the existing service.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure for admission, service, timeout, cancellation, or result failure.
    fn execute_child(
        &mut self,
        request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
        max_status_checks: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure>;
}

/// Explicit Steel kernel owner.
pub trait KernelClearRoute {
    /// Clears the bound kernel without service discovery.
    ///
    /// # Errors
    ///
    /// Returns a safe typed failure when clear planning or publication fails.
    fn clear_kernel(&mut self) -> Result<BindingIdentity, DispatchFailure>;
}

/// Exact loaded process profiles and policy.
#[derive(Clone, Debug)]
pub struct ProcessProfileSet {
    /// Already-loaded profile sources.
    pub sources: Vec<animus_runtime::ProcessJobProfileSource>,
    /// Pure profile policy.
    pub policy: animus_runtime::ProcessJobProfilePolicy,
    /// Safe owner label for resolved requests.
    pub owner_label: String,
}

/// Exact bounded process pack.
pub struct ProcessPack {
    binding: RouteBinding,
    profiles: ProcessProfileSet,
    cancellation: AtomicBool,
}

impl ProcessPack {
    /// Binds loaded profiles and an explicit cancellation flag.
    ///
    /// # Errors
    ///
    /// Returns an error when the binding belongs to another route.
    pub fn new(
        binding: RouteBinding,
        profiles: ProcessProfileSet,
        cancellation: AtomicBool,
    ) -> Result<Self, crate::ConstructionError> {
        require_route(&binding, RouteClass::Shell)?;
        Ok(Self {
            binding,
            profiles,
            cancellation,
        })
    }

    /// Returns the exact route binding.
    #[must_use]
    pub const fn binding(&self) -> &RouteBinding {
        &self.binding
    }

    /// Returns the explicit cancellation flag owned by this pack.
    #[must_use]
    pub const fn cancellation(&self) -> &AtomicBool {
        &self.cancellation
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn execute(
        &self,
        profile_name: &str,
        limits: ExecutorLimits,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::Shell;
        let resolved = animus_runtime::resolve_process_job_profile(
            animus_runtime::ProcessJobProfileResolveRequest {
                sources: self.profiles.sources.as_slice(),
                profile_name,
                owner_label: self.profiles.owner_label.as_str(),
                policy: &self.profiles.policy,
            },
        )
        .map_err(|_error| failure(route, DispatchFailureClass::Denied, "unknown_profile"))?;
        if resolved.request.backend != animus_runtime::ProcessJobBackend::Native
            || resolved.request.command.is_some()
        {
            return Err(failure(
                route,
                DispatchFailureClass::Denied,
                "shell_fallback",
            ));
        }
        let program = resolved
            .request
            .program
            .as_deref()
            .ok_or_else(|| failure(route, DispatchFailureClass::Malformed, "program"))?;
        let current_dir = resolved
            .request
            .cwd
            .as_deref()
            .ok_or_else(|| failure(route, DispatchFailureClass::Malformed, "cwd"))?;
        if resolved.request.resources.memory_bytes.is_some()
            || resolved.request.resources.cpu_millis.is_some()
        {
            return Err(failure(
                route,
                DispatchFailureClass::Denied,
                "unsupported_resource_limits",
            ));
        }
        let timeout_seconds = resolved
            .request
            .resources
            .timeout_seconds
            .ok_or_else(|| failure(route, DispatchFailureClass::Malformed, "timeout"))?;
        let timeout_ms = u64::from(timeout_seconds)
            .checked_mul(1_000)
            .ok_or_else(|| failure(route, DispatchFailureClass::BoundExceeded, "timeout"))?;
        let log_bytes = resolved
            .request
            .resources
            .log_bytes
            .ok_or_else(|| failure(route, DispatchFailureClass::Malformed, "log_bytes"))?;
        let capture_bytes = usize::try_from(log_bytes)
            .map_err(|_error| failure(route, DispatchFailureClass::BoundExceeded, "log_bytes"))?;
        let environment = resolved
            .request
            .env
            .iter()
            .map(|entry| (OsString::from(&entry.key), OsString::from(&entry.value)))
            .collect::<Vec<_>>();
        let outcome_policy =
            bounded_exec::OutcomePolicy::new(vec![DEFAULT_ACCEPTED_EXIT_CODE], true, true)
                .map_err(|_error| {
                    failure(route, DispatchFailureClass::Malformed, "outcome_policy")
                })?;
        let request = bounded_exec::RunRequest {
            command: bounded_exec::CommandSpec {
                program: PathBuf::from(program),
                args: resolved.request.args.iter().map(OsString::from).collect(),
                current_dir: PathBuf::from(current_dir),
                environment_mode: bounded_exec::EnvironmentMode::Clear,
                environment,
                input: bounded_exec::Input::Null,
            },
            limits: bounded_exec::ExecutionLimits {
                timeout_ms,
                stdin_max_bytes: DEFAULT_STDIN_MAX_BYTES,
                stdout_max_bytes: capture_bytes,
                stderr_max_bytes: capture_bytes,
                poll_interval_ms: DEFAULT_POLL_INTERVAL_MS.min(timeout_ms),
                teardown_timeout_ms: DEFAULT_TEARDOWN_TIMEOUT_MS,
            },
            termination_scope: bounded_exec::TerminationScope::ProcessGroup,
            outcome_policy,
        };
        let output = bounded_exec::run_with_cancellation(request, &self.cancellation)
            .map_err(|error| map_bounded_exec_failure(&error))?;
        match output.disposition {
            bounded_exec::Disposition::Succeeded => {}
            bounded_exec::Disposition::TimedOut => {
                return Err(failure(route, DispatchFailureClass::TimedOut, "process"));
            }
            bounded_exec::Disposition::Cancelled => {
                return Err(failure(route, DispatchFailureClass::Cancelled, "process"));
            }
            bounded_exec::Disposition::OutputLimitExceeded(_) => {
                return Err(failure(
                    route,
                    DispatchFailureClass::BoundExceeded,
                    "process_output",
                ));
            }
            bounded_exec::Disposition::ExitFailed => {
                return Err(failure(
                    route,
                    DispatchFailureClass::OwnerFailure,
                    "process_exit",
                ));
            }
        }
        let value = String::from_utf8(output.stdout.bytes)
            .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "stdout_utf8"))?;
        let owner_identity = hash_identity(
            route,
            PROCESS_OWNER_DOMAIN,
            &[
                resolved.identity.request_hash().as_bytes(),
                value.as_bytes(),
            ],
        )?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding: &self.binding,
                value: value.as_str(),
                owner_identity,
                max_result_chars: limits.max_result_chars,
            },
        )
    }
}

/// Context route over existing resolver contracts.
pub struct ResolverContextRoute<'a> {
    /// Existing explicit resolver set.
    pub resolvers: animus_adapters::ContextResolverSet<'a>,
    /// Explicit context-reference policy.
    pub policy: animus_adapters::ContextReferencePolicy,
}

impl ContextRoute for ResolverContextRoute<'_> {
    fn resolve_context(
        &mut self,
        key: &str,
        max_bytes: u32,
        max_lines: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        let route = RouteClass::ContextRead;
        let reference = animus_adapters::parse_context_reference(key)
            .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "reference"))?;
        let candidate =
            animus_adapters::expand_context_reference(&reference, self.policy, &self.resolvers)
                .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "resolver"))?;
        let budget = animus_adapters::ContextReferenceBudget::new(
            max_bytes,
            max_lines,
            animus_adapters::ContextOversizedPolicy::Reject,
        )
        .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "context_budget"))?;
        let fragment = animus_adapters::apply_context_budget(candidate, budget)
            .map_err(|_error| failure(route, DispatchFailureClass::OwnerFailure, "context"))?;
        let value = fragment
            .visible_text()
            .ok_or_else(|| failure(route, DispatchFailureClass::BoundExceeded, "context_output"))?;
        let identity = hash_identity(
            route,
            CONTEXT_OWNER_DOMAIN,
            &[fragment.content_hash().as_str().as_bytes()],
        )?;
        Ok((value.to_owned(), identity))
    }
}

/// Kernel route over one explicitly owned Steel kernel.
pub struct SteelKernelClearRoute<'a> {
    /// Exact kernel owner.
    pub kernel: &'a mut animus_steel::SteelRlmKernel,
}

impl KernelClearRoute for SteelKernelClearRoute<'_> {
    fn clear_kernel(&mut self) -> Result<BindingIdentity, DispatchFailure> {
        let receipt = self.kernel.clear_explicit().map_err(|_error| {
            failure(
                RouteClass::KernelClear,
                DispatchFailureClass::OwnerFailure,
                "steel_kernel",
            )
        })?;
        BindingIdentity::new(receipt.identity()).map_err(|_error| {
            failure(
                RouteClass::KernelClear,
                DispatchFailureClass::Malformed,
                "clear_receipt",
            )
        })
    }
}

/// Child route over one concrete child-session owner.
pub struct ConcreteChildRoute<'a, Factory, Store, Authority, Timer, Events> {
    /// Existing concrete service.
    pub service: &'a mut animus_std::child_session_host::ConcreteChildSessionService<
        Factory,
        Store,
        Authority,
        Timer,
        Events,
    >,
    /// Resolves an RLM request to a pre-admitted staged child session.
    pub admission: &'a mut dyn FnMut(
        &animus_adapter_rlm_kernel::ChildSpawnRequest,
    ) -> Result<
        animus_runtime::child_session_service::ConcreteChildSessionAdmission,
        DispatchFailure,
    >,
}

impl<Factory, Store, Authority, Timer, Events> ChildRoute
    for ConcreteChildRoute<'_, Factory, Store, Authority, Timer, Events>
where
    Factory: animus_std::child_session_host::ChildSessionFactory,
    Store: animus_std::child_session_host::ChildSessionLifecycleStore,
    Authority: animus_std::child_session_host::ChildSessionAuthorityService,
    Timer: animus_std::child_session_host::ChildSessionTimer,
    Events: animus_std::child_session_host::ChildSessionEventSink,
{
    fn execute_child(
        &mut self,
        request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
        max_status_checks: u32,
    ) -> Result<(String, BindingIdentity), DispatchFailure> {
        use animus_runtime::subagents::SubagentService;

        let route = RouteClass::Subagent;
        let admission = (self.admission)(request)?;
        if admission.profile_id() != request.profile() {
            return Err(failure(route, DispatchFailureClass::Denied, "profile"));
        }
        self.service
            .stage(admission.clone())
            .map_err(|error| map_child_failure(error.class, "stage"))?;
        let handle = self
            .service
            .enqueue(admission.subagent())
            .map_err(|error| map_subagent_failure(error.kind, "enqueue"))?;
        let mut checks = 0;
        loop {
            if checks >= max_status_checks {
                return Err(failure(
                    route,
                    DispatchFailureClass::BoundExceeded,
                    "status_checks",
                ));
            }
            checks = checks.saturating_add(1);
            match self
                .service
                .status(&handle)
                .map_err(|error| map_subagent_failure(error.kind, "status"))?
            {
                animus_runtime::subagents::SubagentRunStatus::Completed
                | animus_runtime::subagents::SubagentRunStatus::AwaitingParent => break,
                animus_runtime::subagents::SubagentRunStatus::Failed
                | animus_runtime::subagents::SubagentRunStatus::Blocked => {
                    return Err(failure(route, DispatchFailureClass::OwnerFailure, "child"));
                }
                animus_runtime::subagents::SubagentRunStatus::TimedOut => {
                    return Err(failure(route, DispatchFailureClass::TimedOut, "child"));
                }
                animus_runtime::subagents::SubagentRunStatus::Cancelled => {
                    return Err(failure(route, DispatchFailureClass::Cancelled, "child"));
                }
                animus_runtime::subagents::SubagentRunStatus::Admitted
                | animus_runtime::subagents::SubagentRunStatus::Queued
                | animus_runtime::subagents::SubagentRunStatus::Running
                | animus_runtime::subagents::SubagentRunStatus::Cancelling
                | animus_runtime::subagents::SubagentRunStatus::Collected => {}
            }
        }
        let result = self
            .service
            .collect_result(&handle)
            .map_err(|error| map_child_failure(error.class, "collect"))?;
        let projection = animus_std::child_session_host::project_rlm_child_session_result(
            &result,
            request,
            request.profile(),
        )
        .map_err(|error| map_child_failure(error.class, "projection"))?;
        let value = projection
            .result
            .map(|child| child.value().to_owned())
            .ok_or_else(|| failure(route, DispatchFailureClass::OwnerFailure, "child_result"))?;
        let identity = hash_identity(
            route,
            CHILD_OWNER_DOMAIN,
            &[projection.content_identity.as_bytes()],
        )?;
        Ok((value, identity))
    }
}

fn require_route(
    binding: &RouteBinding,
    expected: RouteClass,
) -> Result<(), crate::ConstructionError> {
    if binding.route() == expected {
        Ok(())
    } else {
        Err(crate::ConstructionError::PackBindingMismatch(expected))
    }
}

fn map_bounded_exec_failure(error: &bounded_exec::RunError) -> DispatchFailure {
    use bounded_exec::RunError;
    let class = match error {
        RunError::InvalidLimits(_)
        | RunError::InputLimitExceeded
        | RunError::CaptureCountOverflow(_)
        | RunError::ProcessIdentifierOverflow => DispatchFailureClass::BoundExceeded,
        RunError::EmptyProgram
        | RunError::ProgramNotAbsolute
        | RunError::EmptyCurrentDirectory
        | RunError::CurrentDirectoryNotAbsolute
        | RunError::DuplicateEnvironmentName => DispatchFailureClass::Malformed,
        RunError::WorkerTeardownTimedOut(_) | RunError::ChildTeardownTimedOut => {
            DispatchFailureClass::TimedOut
        }
        RunError::MissingPipe(_)
        | RunError::Io { .. }
        | RunError::WorkerDisconnected(_)
        | RunError::WorkerPanicked(_) => DispatchFailureClass::OwnerFailure,
    };
    failure(RouteClass::Shell, class, "bounded_exec")
}

fn map_workspace_failure(error: &animus_std_workspace_edit::HostError) -> DispatchFailure {
    use animus_std_workspace_edit::{HostError, HostIssue, HostServiceIssue};
    let class = match error {
        HostError::Service(HostServiceIssue::Unavailable) => DispatchFailureClass::Unavailable,
        HostError::Service(HostServiceIssue::Denied)
        | HostError::PathDenied
        | HostError::PermissionDenied
        | HostError::OwnershipDenied => DispatchFailureClass::Denied,
        HostError::Service(HostServiceIssue::Stale) | HostError::Core(HostIssue::Stale(_)) => {
            DispatchFailureClass::Stale
        }
        HostError::Service(HostServiceIssue::Cancelled) => DispatchFailureClass::Cancelled,
        HostError::Service(HostServiceIssue::TimedOut) => DispatchFailureClass::TimedOut,
        HostError::Core(
            HostIssue::SourceTooLarge
            | HostIssue::InvalidOperationCount
            | HostIssue::InvalidReplacement,
        ) => DispatchFailureClass::BoundExceeded,
        HostError::Core(HostIssue::OverlappingOperation)
        | HostError::PublicationFailed
        | HostError::UncertainAfterPublication => DispatchFailureClass::Conflict,
        HostError::Core(_)
        | HostError::ReadFailed
        | HostError::UnsupportedFileKind
        | HostError::Service(HostServiceIssue::ReleaseFailed)
        | HostError::Receipt(_)
        | HostError::UnsupportedPlatform => DispatchFailureClass::OwnerFailure,
    };
    failure(RouteClass::FileEdit, class, "workspace_edit")
}

fn map_tool_failure(
    error: animus_runtime::tool_executor_kit::ToolExecutorKitError,
) -> DispatchFailure {
    use animus_runtime::tool_executor_kit::{
        ToolExecutionFailureClass, ToolExecutorKitError, ToolExecutorServiceFailure,
    };
    let class = match error {
        ToolExecutorKitError::PlanDenied { failure } => match failure {
            ToolExecutionFailureClass::MissingExecutor
            | ToolExecutionFailureClass::UnavailableExecutor => DispatchFailureClass::Unavailable,
            ToolExecutionFailureClass::Cancelled => DispatchFailureClass::Cancelled,
            ToolExecutionFailureClass::TimedOut => DispatchFailureClass::TimedOut,
            ToolExecutionFailureClass::OutputPolicyDenied
            | ToolExecutionFailureClass::UnboundedOutput => DispatchFailureClass::BoundExceeded,
            ToolExecutionFailureClass::MutationConflict => DispatchFailureClass::Conflict,
            ToolExecutionFailureClass::MalformedArguments
            | ToolExecutionFailureClass::SchemaMismatch
            | ToolExecutionFailureClass::UnsafeOutput
            | ToolExecutionFailureClass::ReceiptLeak => DispatchFailureClass::Malformed,
            ToolExecutionFailureClass::UnknownTool
            | ToolExecutionFailureClass::InactiveDefinition
            | ToolExecutionFailureClass::UntrustedDefinition
            | ToolExecutionFailureClass::DeniedCapability
            | ToolExecutionFailureClass::ApprovalDenied
            | ToolExecutionFailureClass::FallbackDenied
            | ToolExecutionFailureClass::ExecutorBindingMismatch => DispatchFailureClass::Denied,
            ToolExecutionFailureClass::ExecutorFailed => DispatchFailureClass::OwnerFailure,
        },
        ToolExecutorKitError::ServiceFailed { failure } => match failure {
            ToolExecutorServiceFailure::Cancelled => DispatchFailureClass::Cancelled,
            ToolExecutorServiceFailure::TimedOut => DispatchFailureClass::TimedOut,
            ToolExecutorServiceFailure::ExecutionFailed => DispatchFailureClass::OwnerFailure,
        },
        ToolExecutorKitError::OutputTooLarge { .. } => DispatchFailureClass::BoundExceeded,
        ToolExecutorKitError::BindingMismatch | ToolExecutorKitError::RequestBindingMismatch => {
            DispatchFailureClass::Stale
        }
        ToolExecutorKitError::CatalogTooLarge { .. }
        | ToolExecutorKitError::DuplicateTool
        | ToolExecutorKitError::InvalidArgumentNames
        | ToolExecutorKitError::InvalidOutputPolicy
        | ToolExecutorKitError::UnsafeMetadata
        | ToolExecutorKitError::InvalidOutput
        | ToolExecutorKitError::UnsafeReceipt => DispatchFailureClass::Malformed,
    };
    failure(RouteClass::Tool, class, "tool_kit")
}

fn map_child_failure(
    class: animus_runtime::child_session_service::ChildSessionFailureClass,
    source: &str,
) -> DispatchFailure {
    use animus_runtime::child_session_service::ChildSessionFailureClass;
    let class = match class {
        ChildSessionFailureClass::MissingService => DispatchFailureClass::Unavailable,
        ChildSessionFailureClass::Denied => DispatchFailureClass::Denied,
        ChildSessionFailureClass::AuthorityDrift
        | ChildSessionFailureClass::StaleHandle
        | ChildSessionFailureClass::ForeignParent
        | ChildSessionFailureClass::RestoreDrift => DispatchFailureClass::Stale,
        ChildSessionFailureClass::Cancelled => DispatchFailureClass::Cancelled,
        ChildSessionFailureClass::TimedOut => DispatchFailureClass::TimedOut,
        ChildSessionFailureClass::Malformed => DispatchFailureClass::Malformed,
        ChildSessionFailureClass::BoundExceeded => DispatchFailureClass::BoundExceeded,
        ChildSessionFailureClass::DuplicateChild => DispatchFailureClass::Conflict,
        ChildSessionFailureClass::Factory
        | ChildSessionFailureClass::Persistence
        | ChildSessionFailureClass::Session
        | ChildSessionFailureClass::Timer
        | ChildSessionFailureClass::Cancellation => DispatchFailureClass::OwnerFailure,
    };
    failure(RouteClass::Subagent, class, source)
}

fn map_subagent_failure(
    class: animus_runtime::subagents::SubagentServiceFailureKind,
    source: &str,
) -> DispatchFailure {
    use animus_runtime::subagents::SubagentServiceFailureKind;
    let class = match class {
        SubagentServiceFailureKind::Unavailable => DispatchFailureClass::Unavailable,
        SubagentServiceFailureKind::Denied => DispatchFailureClass::Denied,
        SubagentServiceFailureKind::TimedOut => DispatchFailureClass::TimedOut,
        SubagentServiceFailureKind::Cancelled => DispatchFailureClass::Cancelled,
        SubagentServiceFailureKind::MalformedReport => DispatchFailureClass::Malformed,
        SubagentServiceFailureKind::StaleHandle => DispatchFailureClass::Stale,
    };
    failure(RouteClass::Subagent, class, source)
}

fn hash_identity(
    route: RouteClass,
    domain: &str,
    parts: &[&[u8]],
) -> Result<BindingIdentity, DispatchFailure> {
    let mut hasher = blake3::Hasher::new();
    crate::contract::frame(&mut hasher, domain.as_bytes());
    for part in parts {
        crate::contract::frame(&mut hasher, part);
    }
    BindingIdentity::new(hasher.finalize().to_hex().as_str())
        .map_err(|_error| failure(route, DispatchFailureClass::Malformed, "owner_identity"))
}

fn file_metadata_identity(metadata: &cap_std::fs::Metadata) -> String {
    let mut hasher = blake3::Hasher::new();
    crate::contract::frame(&mut hasher, b"animus.rlm-host.file-metadata.v1");
    crate::contract::frame(&mut hasher, &metadata.len().to_le_bytes());
    #[cfg(unix)]
    {
        use cap_std::fs::MetadataExt;
        crate::contract::frame(&mut hasher, &metadata.dev().to_le_bytes());
        crate::contract::frame(&mut hasher, &metadata.ino().to_le_bytes());
        crate::contract::frame(&mut hasher, &metadata.mtime().to_le_bytes());
        crate::contract::frame(&mut hasher, &metadata.mtime_nsec().to_le_bytes());
        crate::contract::frame(&mut hasher, &metadata.ctime().to_le_bytes());
        crate::contract::frame(&mut hasher, &metadata.ctime_nsec().to_le_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn failure(route: RouteClass, class: DispatchFailureClass, source: &str) -> DispatchFailure {
    DispatchFailure::new(route, class, source)
}
