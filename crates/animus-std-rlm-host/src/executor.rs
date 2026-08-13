use std::collections::BTreeMap;

use animus_adapter_rlm_kernel::HostRequest;
use animus_runtime::rlm_kernel::RlmKernelHostExecutor;

use crate::{
    BindingIdentity, BindingSet, ConstructionError, CurrentAuthorityObserver, DispatchFailure,
    DispatchFailureClass, DispatchObservation, EditRoute, ExecutorLimits, ExpectedAuthority,
    FileReadPack, KernelClearRoute, ProcessPack, RlmHostResult, RouteBinding, RouteClass,
    RouteScope, ToolRoute,
};

/// Explicit capability packs. Every field is optional only so construction can
/// support a deliberately narrow admitted operation set.
pub struct CapabilityPacks<'a> {
    /// Capability-relative file reader.
    pub file_read: Option<FileReadPack>,
    /// Existing revision-gated edit owner.
    pub file_edit: Option<&'a mut dyn EditRoute>,
    /// Exact bounded process profiles and mechanics.
    pub process: Option<ProcessPack>,
    /// Existing tool-executor-kit owner.
    pub tool: Option<&'a mut dyn ToolRoute>,
    /// Existing context resolver owner.
    pub context: Option<&'a mut dyn crate::ContextRoute>,
    /// Existing child-session service owner.
    pub child: Option<&'a mut dyn crate::ChildRoute>,
    /// Explicit Steel kernel owner.
    pub kernel: Option<&'a mut dyn KernelClearRoute>,
}

/// Construction inputs for one complete executor.
pub struct RlmHostExecutorConfig<'a, Observer> {
    /// Exact selected route bindings.
    pub bindings: BindingSet,
    /// Sealed authority ceiling.
    pub expected_authority: ExpectedAuthority,
    /// Current authority observer.
    pub authority: Observer,
    /// Named host bounds.
    pub limits: ExecutorLimits,
    /// Exact selected capability packs.
    pub packs: CapabilityPacks<'a>,
}

/// Complete non-default RLM host executor.
pub struct RlmHostExecutor<'a, Observer> {
    bindings: BindingSet,
    expected_authority: ExpectedAuthority,
    authority: Observer,
    limits: ExecutorLimits,
    packs: CapabilityPacks<'a>,
    observations: Vec<DispatchObservation>,
}

impl<'a, Observer: CurrentAuthorityObserver> RlmHostExecutor<'a, Observer> {
    /// Validates exact route-pack correspondence and builds one executor.
    ///
    /// # Errors
    ///
    /// Returns a construction error for any missing or mismatched selected pack.
    pub fn new(config: RlmHostExecutorConfig<'a, Observer>) -> Result<Self, ConstructionError> {
        let limits = config.limits.validate()?;
        validate_packs(&config.bindings, &config.packs)?;
        Ok(Self {
            bindings: config.bindings,
            expected_authority: config.expected_authority,
            authority: config.authority,
            limits,
            packs: config.packs,
            observations: Vec::new(),
        })
    }

    /// Returns the stable aggregate binding identity.
    #[must_use]
    pub const fn binding_set_identity(&self) -> &BindingIdentity {
        self.bindings.identity()
    }

    /// Returns metadata-only successful dispatch observations.
    #[must_use]
    pub fn observations(&self) -> &[DispatchObservation] {
        self.observations.as_slice()
    }

    /// Executes one request and preserves the metadata-only observation.
    ///
    /// # Errors
    ///
    /// Returns a typed failure for unavailable packs, current authority drift,
    /// route-owner denial, stale state, cancellation, timeout, or bounds.
    pub fn execute_typed(
        &mut self,
        request: &HostRequest,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::for_operation(request.operation_class());
        let binding = self.bindings.get(route).cloned().ok_or_else(|| {
            DispatchFailure::new(route, DispatchFailureClass::Unavailable, "binding")
        })?;
        let scope = RouteScope::for_request(request).map_err(|_error| {
            DispatchFailure::new(route, DispatchFailureClass::Malformed, "request_scope")
        })?;
        let current = self.authority.observe(route, &binding, &scope.identity)?;
        current.validate(&self.expected_authority, &binding, &scope.identity)?;
        let result = match request {
            HostRequest::FileRead { target } => self.execute_file_read(target),
            HostRequest::FileEdit {
                target,
                replacement,
            } => self.execute_file_edit(&binding, target, replacement),
            HostRequest::Shell { command } => self.execute_process(command),
            HostRequest::Tool { name, payload } => self.execute_tool(&binding, name, payload),
            HostRequest::Subagent(request) => self.execute_child(&binding, request),
            HostRequest::ContextRead { key } => self.execute_context(&binding, key),
            HostRequest::KernelClear => self.execute_kernel_clear(&binding),
        }?;
        self.observations.push(result.observation().clone());
        Ok(result)
    }

    fn execute_file_read(&self, target: &str) -> Result<RlmHostResult, DispatchFailure> {
        self.packs
            .file_read
            .as_ref()
            .ok_or_else(|| unavailable(RouteClass::FileRead))?
            .execute(target, self.limits)
    }

    fn execute_process(&self, profile: &str) -> Result<RlmHostResult, DispatchFailure> {
        self.packs
            .process
            .as_ref()
            .ok_or_else(|| unavailable(RouteClass::Shell))?
            .execute(profile, self.limits)
    }

    fn execute_file_edit(
        &mut self,
        binding: &RouteBinding,
        target: &str,
        replacement: &str,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::FileEdit;
        let owner = self
            .packs
            .file_edit
            .as_deref_mut()
            .ok_or_else(|| unavailable(route))?;
        let owner_identity = owner.execute_edit(target, replacement)?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding,
                value: "workspace edit published",
                owner_identity,
                max_result_chars: self.limits.max_result_chars,
            },
        )
    }

    fn execute_tool(
        &mut self,
        binding: &RouteBinding,
        name: &str,
        payload: &str,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::Tool;
        let owner = self
            .packs
            .tool
            .as_deref_mut()
            .ok_or_else(|| unavailable(route))?;
        let (value, owner_identity) =
            owner.execute_tool(name, payload, self.limits.max_result_chars)?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding,
                value: value.as_str(),
                owner_identity,
                max_result_chars: self.limits.max_result_chars,
            },
        )
    }

    fn execute_context(
        &mut self,
        binding: &RouteBinding,
        key: &str,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::ContextRead;
        let owner = self
            .packs
            .context
            .as_deref_mut()
            .ok_or_else(|| unavailable(route))?;
        let (value, owner_identity) = owner.resolve_context(
            key,
            self.limits.max_context_bytes,
            self.limits.max_context_lines,
        )?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding,
                value: value.as_str(),
                owner_identity,
                max_result_chars: self.limits.max_result_chars,
            },
        )
    }

    fn execute_child(
        &mut self,
        binding: &RouteBinding,
        request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::Subagent;
        let owner = self
            .packs
            .child
            .as_deref_mut()
            .ok_or_else(|| unavailable(route))?;
        let (value, owner_identity) =
            owner.execute_child(request, self.limits.max_child_status_checks)?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding,
                value: value.as_str(),
                owner_identity,
                max_result_chars: self.limits.max_result_chars,
            },
        )
    }

    fn execute_kernel_clear(
        &mut self,
        binding: &RouteBinding,
    ) -> Result<RlmHostResult, DispatchFailure> {
        let route = RouteClass::KernelClear;
        let owner = self
            .packs
            .kernel
            .as_deref_mut()
            .ok_or_else(|| unavailable(route))?;
        let owner_identity = owner.clear_kernel()?;
        RlmHostResult::new(
            route,
            crate::RlmHostResultDraft {
                binding,
                value: "kernel cleared",
                owner_identity,
                max_result_chars: self.limits.max_result_chars,
            },
        )
    }
}

impl<Observer: CurrentAuthorityObserver> RlmKernelHostExecutor for RlmHostExecutor<'_, Observer> {
    type Error = DispatchFailure;

    fn execute(&mut self, request: &HostRequest) -> Result<String, Self::Error> {
        self.execute_typed(request)
            .map(|result| result.value().to_owned())
    }
}

fn validate_packs(
    bindings: &BindingSet,
    packs: &CapabilityPacks<'_>,
) -> Result<(), ConstructionError> {
    let pack_routes = BTreeMap::from([
        (RouteClass::FileRead, packs.file_read.is_some()),
        (RouteClass::FileEdit, packs.file_edit.is_some()),
        (RouteClass::Shell, packs.process.is_some()),
        (RouteClass::Tool, packs.tool.is_some()),
        (RouteClass::Subagent, packs.child.is_some()),
        (RouteClass::ContextRead, packs.context.is_some()),
        (RouteClass::KernelClear, packs.kernel.is_some()),
    ]);
    for (route, present) in pack_routes {
        let selected = bindings.get(route).is_some();
        if present != selected {
            return Err(ConstructionError::PackBindingMismatch(route));
        }
    }
    if let Some(pack) = packs.file_read.as_ref() {
        require_same_binding(bindings, pack.binding())?;
    }
    if let Some(pack) = packs.process.as_ref() {
        require_same_binding(bindings, pack.binding())?;
    }
    Ok(())
}

fn require_same_binding(
    bindings: &BindingSet,
    supplied: &RouteBinding,
) -> Result<(), ConstructionError> {
    if bindings.get(supplied.route()) == Some(supplied) {
        Ok(())
    } else {
        Err(ConstructionError::PackBindingMismatch(supplied.route()))
    }
}

fn unavailable(route: RouteClass) -> DispatchFailure {
    DispatchFailure::new(route, DispatchFailureClass::Unavailable, "capability_pack")
}
