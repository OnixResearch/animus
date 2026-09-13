//! Visible live-application composition root.

/// Explicit inputs accepted by the packaged live entrypoint.
pub struct LiveApplicationInput<'a> {
    /// Deterministic Nickel-exported JSON bytes.
    pub config_json: &'a [u8],
    /// Exact capability root for both dedicated journals.
    pub store_root: &'a std::path::Path,
    /// One explicit user prompt.
    pub prompt: &'a str,
    /// Injected credential read by the outer binary from standard input.
    pub credential: String,
    /// Explicit finite context values. No filesystem discovery occurs.
    pub context: crate::ExplicitContextMap,
}

/// Safe packaged-entrypoint failure.
#[derive(Debug, Eq, PartialEq)]
pub enum LiveApplicationError {
    /// Typed application configuration failed validation.
    Configuration,
    /// The selected composition failed activation preflight.
    Activation,
    /// Store root could not be opened as an explicit capability.
    StoreRoot,
    /// Store policy, recovery, lock, or persistence failed.
    Store(crate::LiveStoreError),
    /// The explicit credential or HTTPS host was rejected.
    Provider,
    /// Concrete child composition failed.
    Child,
    /// Concrete RLM host composition failed.
    Host(crate::LiveHostError),
    /// Main agent composition or execution failed.
    Agent(crate::LiveSteelAgentError),
}

impl core::fmt::Display for LiveApplicationError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live application failed: {self:?}")
    }
}

impl std::error::Error for LiveApplicationError {}

/// Runs the fully composed live shell from explicit inputs.
///
/// The root selects provider, durable stores, coding session, Steel kernel,
/// resumable bridge, RLM executor, child service, skills, control, events, and
/// metadata-only receipts before the first provider request.
///
/// # Errors
///
/// Returns [`LiveApplicationError`] without ambient provider, credential,
/// current-directory, account, skill, child, or fallback discovery.
#[cfg(feature = "live-provider")]
pub fn run_live_application(
    input: LiveApplicationInput<'_>,
) -> Result<crate::LiveSteelAgentRunOutput, LiveApplicationError> {
    let parsed = crate::parse_live_steel_agent_config(input.config_json)
        .map_err(|_error| LiveApplicationError::Configuration)?;
    let _activation = crate::readiness::preflight_live_application_activation()
        .map_err(|_error| LiveApplicationError::Activation)?;
    if input.credential.trim().is_empty() {
        return Err(LiveApplicationError::Provider);
    }
    let api_key =
        animus_std_openai::OpenAiApiKey::new(parsed.credential_reference(), input.credential)
            .map_err(|_error| LiveApplicationError::Provider)?;
    let root = cap_std::fs::Dir::open_ambient_dir(input.store_root, cap_std::ambient_authority())
        .map_err(|_error| LiveApplicationError::StoreRoot)?;
    let definitions = crate::store_definitions(&parsed).map_err(LiveApplicationError::Store)?;
    let is_resume_required = crate::session_resume_required(&root, &parsed, &definitions)
        .map_err(LiveApplicationError::Store)?;
    let config = parsed.with_session_resume_required(is_resume_required);
    debug_assert_eq!(config.session_resume_required(), is_resume_required);
    debug_assert_eq!(config.config_identity(), parsed.config_identity());
    let session_persistence = crate::open_session_persistence(&root, &config, &definitions)
        .map_err(LiveApplicationError::Store)?;
    let snapshot_persistence = crate::open_snapshot_persistence(&root, &definitions)
        .map_err(LiveApplicationError::Store)?;
    let kernel_persistence = crate::open_kernel_persistence(&root, &config, &definitions)
        .map_err(LiveApplicationError::Store)?;
    let kernel = crate::restore_or_create_kernel(&config, &kernel_persistence)
        .map_err(LiveApplicationError::Store)?;
    let provider_host = animus_std_openai::ReqwestOpenAiHost::new(
        config
            .provider_host_config()
            .map_err(|_error| LiveApplicationError::Provider)?,
    )
    .map_err(|_error| LiveApplicationError::Provider)?;
    let provider = crate::LiveOpenAiProvider::new(provider_host, api_key);
    let mut child_service = crate::build_child_service(&config, "bounded child result")
        .map_err(|_error| LiveApplicationError::Child)?;
    let child_config = config.clone();
    let mut child_admission = move |request: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
        crate::admit_child_request(&child_config, request)
    };
    let mut child_route = animus_std_rlm_host::ConcreteChildRoute {
        service: &mut child_service,
        admission: &mut child_admission,
    };
    let mut context_route = crate::context_route(&input.context);
    let executor = crate::build_executor(&config, &mut context_route, &mut child_route)
        .map_err(LiveApplicationError::Host)?;
    crate::LiveSteelAgentComposition::new(crate::LiveSteelAgentCompositionInput {
        config: config.clone(),
        provider,
        session_persistence,
        snapshot_persistence,
        kernel,
        host_executor: executor,
        kernel_persistence,
    })
    .map_err(LiveApplicationError::Agent)?
    .run_prompt(input.prompt)
    .map_err(LiveApplicationError::Agent)
}

/// Stub that makes the non-default feature boundary explicit.
///
/// # Errors
///
/// Always returns [`LiveApplicationError::Provider`] when the live feature is absent.
#[cfg(not(feature = "live-provider"))]
pub fn run_live_application(
    _input: LiveApplicationInput<'_>,
) -> Result<crate::LiveSteelAgentRunOutput, LiveApplicationError> {
    Err(LiveApplicationError::Provider)
}
