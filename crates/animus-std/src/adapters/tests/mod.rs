use super::*;
use animus_adapters::ComponentAdapter;

const SAFE_PAYLOAD: &str = "safe fixture payload";
const RAW_PROMPT_PAYLOAD: &str = "prompt: summarize private material";
const SECRET_PAYLOAD: &str = "token=sk-redacted";
const PRIVATE_PATH_PAYLOAD: &str = "/home/example/private.txt";
const MODEL_REQUEST_TEXT: &str = "intent: build reliable systems";
const TOOL_NAME: &str = "fixture-tool";
const TOOL_INPUT: &str = "fixture input";
const TOOL_ARGUMENT_NAME: &str = "input";
const TOOL_SCHEMA: &str = "schema.fixture-tool.v1";
const TOOL_CALL_ID: &str = "fixture-tool-call";
const TOOL_VISIBLE_BUDGET: u32 = 64;
const TOOL_ARGUMENT_COUNT: usize = 1;
const TOOL_MUTATION_TARGET: &str = "resource:fixture";
const TOOL_CAPABILITY: &str = "tool.fixture";
const CONCRETE_DEPENDENCY_MARKERS: &[&str] = &[
    "reqwest",
    "ureq",
    "tokio",
    "redb",
    "rusqlite",
    "keyring",
    "octocrab",
    "crossterm",
];
const REUSABLE_CARGO_TOMLS: &[(&str, &str)] = &[
    (
        "animus-core",
        include_str!("../../../../animus-core/Cargo.toml"),
    ),
    (
        "animus-adapters",
        include_str!("../../../../animus-adapters/Cargo.toml"),
    ),
    (
        "animus-runtime",
        include_str!("../../../../animus-runtime/Cargo.toml"),
    ),
];

#[test]
fn adapter_families_cover_required_shell_edges() {
    let families = [
        AdapterFamily::ModelProvider,
        AdapterFamily::FilesystemTool,
        AdapterFamily::ProcessExecution,
        AdapterFamily::HttpNetworkHelper,
        AdapterFamily::FixtureBackend,
    ];

    for family in families {
        assert!(family.is_shell_owned());
        assert!(!family.as_str().is_empty());
    }
}

#[test]
fn effect_plan_allows_gated_model_dispatch() {
    let descriptor = AdapterDescriptor::host_owned(AdapterFamily::ModelProvider, "openai_class");
    let plan = plan_adapter_effect(
        descriptor,
        EffectPrerequisites::satisfied().with_credentials_present(),
        SAFE_PAYLOAD,
    )
    .expect("safe plan receipt should build");

    assert_eq!(plan.family(), AdapterFamily::ModelProvider);
    assert!(plan.verdict().is_allowed());
    assert_eq!(
        plan.receipt().status(),
        animus_adapters::ReceiptStatus::Completed
    );
    assert_eq!(plan.receipt().artifact_hashes().len(), usize::from(1_u8));
}

#[test]
fn fixtures_cover_model_filesystem_process_network_clock_and_credentials() {
    let fixtures = Fixtures::new();
    let kinds = [
        FixtureKind::Model,
        FixtureKind::Filesystem,
        FixtureKind::Process,
        FixtureKind::Network,
        FixtureKind::Clock,
        FixtureKind::Credential,
    ];

    for kind in kinds {
        let output = fixtures
            .dispatch(kind, SAFE_PAYLOAD)
            .expect("fixture output should be safe");
        assert_eq!(output.kind(), kind);
        assert!(output.plan().verdict().is_allowed());
        assert!(output.output_label().starts_with("fixture_"));
    }
}

#[test]
fn fixture_model_and_tool_implement_injected_adapter_contracts() {
    let request = animus_adapters::ModelRequest::new(MODEL_REQUEST_TEXT)
        .expect("fixture model request should parse");
    let mut model = FixtureModelAdapter;
    let response = model.call(&request).expect("fixture model should respond");
    assert_eq!(
        response.message(),
        super::deterministic::test_support::FIXTURE_MODEL_MESSAGE
    );

    let request = tool_request();
    let mut tool = FixtureToolAdapter;
    let response = tool.call(&request).expect("fixture tool should respond");
    assert_eq!(
        response.output(),
        super::deterministic::test_support::FIXTURE_TOOL_OUTPUT
    );
}

fn tool_arguments() -> animus_adapters::tool::PreparedArguments {
    let fields = [animus_adapters::tool::ArgumentFieldDraft {
        name: Some(TOOL_ARGUMENT_NAME),
        value: Some(TOOL_INPUT),
    }];
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: TOOL_SCHEMA,
            allowed_field_names: &[TOOL_ARGUMENT_NAME],
            compatibility_rules: &[],
            fields: &fields,
        },
    )
    .expect("tool arguments parse");
    assert_eq!(arguments.schema_identity(), TOOL_SCHEMA);
    assert_eq!(arguments.fields().len(), TOOL_ARGUMENT_COUNT);
    arguments
}

fn tool_request() -> animus_adapters::ToolRequest {
    let arguments = tool_arguments();
    let request = animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(TOOL_CALL_ID),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: TOOL_VISIBLE_BUDGET,
        },
        mutation_targets: &[TOOL_MUTATION_TARGET],
        required_capability_labels: &[TOOL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .expect("tool request parses");
    assert_eq!(request.name(), TOOL_NAME);
    assert_eq!(request.correlation_id(), TOOL_CALL_ID);
    request
}

#[test]
fn missing_credentials_fail_closed_before_dispatch() {
    let descriptor = AdapterDescriptor::host_owned(AdapterFamily::HttpNetworkHelper, "http_api");
    let plan = plan_adapter_effect(
        descriptor,
        EffectPrerequisites::satisfied().with_missing_credentials(),
        SAFE_PAYLOAD,
    )
    .expect("blocked plan receipt should build");

    assert_eq!(
        plan.verdict(),
        AdapterVerdict::Blocked {
            issue: AdapterIssue::MissingCredentials,
        }
    );
    assert_eq!(
        plan.receipt().status(),
        animus_adapters::ReceiptStatus::Blocked
    );
}

#[test]
fn denied_policy_sandbox_mismatch_malformed_output_timeout_and_cancel_fail_closed() {
    let descriptor = AdapterDescriptor::host_owned(AdapterFamily::ProcessExecution, "command");
    let cases = [
        (
            EffectPrerequisites::satisfied().with_policy_denied(),
            AdapterIssue::DeniedCapability,
        ),
        (
            EffectPrerequisites::satisfied().with_sandbox_mismatch(),
            AdapterIssue::SandboxMismatch,
        ),
        (
            EffectPrerequisites::satisfied().with_malformed_output(),
            AdapterIssue::MalformedOutput,
        ),
        (
            EffectPrerequisites::satisfied().with_timeout(),
            AdapterIssue::TimedOut,
        ),
        (
            EffectPrerequisites::satisfied().with_cancellation(),
            AdapterIssue::Cancelled,
        ),
    ];

    for (prerequisites, issue) in cases {
        let plan = plan_adapter_effect(descriptor, prerequisites, SAFE_PAYLOAD)
            .expect("failure receipt should be safe");
        assert_eq!(plan.verdict(), AdapterVerdict::Blocked { issue });
        assert!(!plan.verdict().is_allowed());
    }
}

#[test]
fn safe_receipts_omit_raw_prompt_secret_path_and_payload_text() {
    let descriptor = AdapterDescriptor::host_owned(AdapterFamily::FilesystemTool, "read_file");
    let payload = format!("{RAW_PROMPT_PAYLOAD}\n{SECRET_PAYLOAD}\n{PRIVATE_PATH_PAYLOAD}");
    let plan = plan_adapter_effect(descriptor, EffectPrerequisites::satisfied(), &payload)
        .expect("receipt should hash payload without copying it");
    let summary = plan.receipt().review_summary();

    assert!(!summary.contains(RAW_PROMPT_PAYLOAD));
    assert!(!summary.contains(SECRET_PAYLOAD));
    assert!(!summary.contains(PRIVATE_PATH_PAYLOAD));
    assert!(summary.contains(animus_adapters::ReceiptRedaction::PayloadHashOnly.as_str()));
}

#[test]
fn unsafe_metadata_labels_are_rejected() {
    let descriptor =
        AdapterDescriptor::host_owned(AdapterFamily::ModelProvider, RAW_PROMPT_PAYLOAD);
    let error = plan_adapter_effect(descriptor, EffectPrerequisites::satisfied(), SAFE_PAYLOAD)
        .expect_err("unsafe labels should be rejected");

    assert!(matches!(
        error,
        animus_adapters::SafeReceiptError::UnsafeMetadata { .. }
    ));
}

#[test]
fn reusable_crates_do_not_depend_on_concrete_host_effect_crates() {
    for (crate_name, cargo_toml) in REUSABLE_CARGO_TOMLS {
        for marker in CONCRETE_DEPENDENCY_MARKERS {
            assert!(
                !cargo_toml.contains(marker),
                "{crate_name} must not depend on concrete host crate {marker}"
            );
        }
    }
}
