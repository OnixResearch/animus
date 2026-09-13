#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SteelArguments {
    pub(super) code: String,
}

#[derive(Default)]
pub(crate) struct MetadataEventSink;

impl animus_std::coding_session_host::CodingSessionEventSink for MetadataEventSink {
    fn observe(&mut self, _event: &animus_runtime::RuntimeEvent) {}
}

pub(super) fn new_state(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<animus_runtime::coding_session_host::CodingSessionState, super::LiveSteelAgentError> {
    let components = animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL
        .iter()
        .map(
            |class| animus_runtime::session_authority_profile::SessionAuthorityComponentDraft {
                class: *class,
                identity: config.authority_component_identity(),
            },
        )
        .collect::<Vec<_>>();
    debug_assert_eq!(
        components.len(),
        animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL.len()
    );
    debug_assert!(config.authority_generation() > 0);
    let authority_profile =
        animus_runtime::session_authority_profile::build_session_authority_profile(
            animus_runtime::session_authority_profile::SessionAuthorityProfileDraft {
                components: components.as_slice(),
            },
        )
        .map_err(|_error| super::LiveSteelAgentError::Authority)?;
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: config.max_queue_items(),
            allowed_sources: &[super::QUEUE_SOURCE],
            policy_label: super::QUEUE_POLICY_LABEL,
            policy_hash: config.config_identity(),
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .map_err(|_error| super::LiveSteelAgentError::Authority)?;
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
    .map_err(|_error| super::LiveSteelAgentError::Session)
}

pub(super) fn steel_tool_request(
    source: CellSource<'_>,
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
                value: Some(source.code),
            }],
        },
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    let request = animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(source.correlation_id),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget,
        },
        mutation_targets: &[],
        required_capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    debug_assert_eq!(request.name(), animus_std_openai::STEEL_REPL_TOOL_NAME);
    debug_assert_eq!(request.correlation_id(), source.correlation_id);
    Ok(request)
}

pub(super) fn service_issue(
    _error: crate::adapters::responses::ProviderStreamError,
) -> animus_runtime::coding_session_host::CodingSessionServiceIssue {
    animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
}

#[derive(Clone, Copy)]
pub(super) struct CellSource<'a> {
    pub(super) correlation_id: &'a str,
    pub(super) code: &'a str,
}
