use animus_adapters::codegraph::{
    CodeGraphEventKind, CodeGraphProfile, CodeGraphProfileDraft, validate_code_graph_profile,
};
use animus_runtime::tool_executor_kit::{
    ToolExecutionFailureClass, ToolExecutorAdmission, ToolExecutorKitError,
};
use cap_std::fs::Dir;

use super::*;
use crate::{CodeGraphHost, CodeGraphHostEvent};

const FILE_BUDGET: usize = 4;
const FILE_BYTE_BUDGET: usize = 4_096;
const RESULT_BUDGET: usize = 8;
const QUERY_LIMIT: usize = 2;
const FIXTURE_PATH: &str = "src/lib.rs";
const QUERY_SYMBOL: &str = "run";
const CORRELATION_ID: &str = "codebase-query-test";
const SOURCE: &str = "fn run() { run(); }";
const INVALID_KIND: &str = "semantic";
const INVALID_LIMIT: usize = 0;

#[test]
fn pure_catalog_request_decode_and_render_are_exact() {
    // r[verify agent_harness_runtime.read_only_codebase_tool.host.query]
    let catalog = build_codebase_query_catalog().expect("catalog");
    let request =
        build_codebase_query_request(QUERY_SYMBOL, "definition", QUERY_LIMIT, CORRELATION_ID)
            .expect("request");
    let query = decode_codebase_query(&request).expect("query");

    assert_eq!(catalog.definitions().len(), 1);
    assert_eq!(query.symbol(), QUERY_SYMBOL);
    assert_eq!(
        query.kind(),
        Some(animus_adapters::codegraph::CodeGraphSymbolKind::Definition)
    );
    assert_eq!(query.max_results(), QUERY_LIMIT);
}

#[test]
fn malformed_kind_and_zero_limit_fail_closed() {
    // r[verify agent_harness_runtime.read_only_codebase_tool.host.failure]
    let invalid_kind =
        build_codebase_query_request(QUERY_SYMBOL, INVALID_KIND, QUERY_LIMIT, CORRELATION_ID)
            .expect("structurally valid request");
    assert_eq!(
        decode_codebase_query(&invalid_kind),
        Err(CodebaseToolError::InvalidKind)
    );
    let invalid_limit =
        build_codebase_query_request(QUERY_SYMBOL, "any", INVALID_LIMIT, CORRELATION_ID)
            .expect("structurally valid request");
    assert_eq!(
        decode_codebase_query(&invalid_limit),
        Err(CodebaseToolError::InvalidLimit)
    );
}

#[test]
fn capability_root_query_returns_bounded_model_output_and_safe_receipt() {
    // r[verify agent_harness_runtime.read_only_codebase_tool.outcome.bounded]
    // r[verify agent_harness_runtime.read_only_codebase_tool.smoke.pass]
    let root_path = test_root("positive");
    create_fixture(&root_path);
    let root = Dir::open_ambient_dir(&root_path, cap_std::ambient_authority())
        .expect("open explicit capability root");
    let host = CodeGraphHost::index_rust_files(&root, profile(), &[String::from(FIXTURE_PATH)])
        .expect("index fixture");
    let mut executor = CodebaseQueryToolExecutor::new(host);
    let request =
        build_codebase_query_request(QUERY_SYMBOL, "definition", QUERY_LIMIT, CORRELATION_ID)
            .expect("request");

    let outcome =
        execute_codebase_query_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
            .expect("execute query");

    assert!(
        outcome
            .response()
            .model_visible_text()
            .contains(FIXTURE_PATH)
    );
    assert!(
        outcome
            .response()
            .model_visible_text()
            .contains("kind=definition")
    );
    assert_eq!(outcome.receipt().tool_name(), CODEBASE_QUERY_TOOL_NAME);
    assert_eq!(
        outcome.receipt().executor_binding(),
        CODEBASE_QUERY_EXECUTOR_BINDING
    );
    assert!(!format!("{:?}", outcome.receipt()).contains(FIXTURE_PATH));
    std::fs::remove_dir_all(root_path).expect("cleanup fixture");
}

#[test]
fn stale_graph_and_denied_capability_return_no_success_outcome() {
    // r[verify agent_harness_runtime.read_only_codebase_tool.verification]
    let root_path = test_root("stale");
    create_fixture(&root_path);
    let root = Dir::open_ambient_dir(&root_path, cap_std::ambient_authority())
        .expect("open explicit capability root");
    let mut host = CodeGraphHost::index_rust_files(&root, profile(), &[String::from(FIXTURE_PATH)])
        .expect("index fixture");
    host.apply_event(
        &root,
        CodeGraphHostEvent {
            kind: CodeGraphEventKind::Overflow,
            relative_path: FIXTURE_PATH,
            new_relative_path: None,
        },
    )
    .expect("mark stale");
    let mut executor = CodebaseQueryToolExecutor::new(host);
    let request = build_codebase_query_request(QUERY_SYMBOL, "any", QUERY_LIMIT, CORRELATION_ID)
        .expect("request");
    assert!(matches!(
        execute_codebase_query_tool(&mut executor, &request, ToolExecutorAdmission::admitted(),),
        Err(CodebaseToolError::ExecutorKit(
            ToolExecutorKitError::ServiceFailed { .. }
        ))
    ));

    let host = CodeGraphHost::index_rust_files(&root, profile(), &[String::from(FIXTURE_PATH)])
        .expect("fresh index");
    let mut executor = CodebaseQueryToolExecutor::new(host);
    let mut denied = ToolExecutorAdmission::admitted();
    denied.capability = animus_runtime::tool_executor_kit::ToolCapabilityAdmission::Denied;
    assert_eq!(
        execute_codebase_query_tool(&mut executor, &request, denied),
        Err(CodebaseToolError::ExecutorKit(
            ToolExecutorKitError::PlanDenied {
                failure: ToolExecutionFailureClass::DeniedCapability,
            }
        ))
    );
    std::fs::remove_dir_all(root_path).expect("cleanup fixture");
}

fn profile() -> CodeGraphProfile {
    validate_code_graph_profile(&CodeGraphProfileDraft {
        profile_id: "codebase-tool-test",
        language: "rust",
        parser_cohort: "tree-sitter-rust-0.24",
        query_cohort: "rust-query-v1",
        root_label: "explicit-test-root",
        max_files: FILE_BUDGET,
        max_file_bytes: FILE_BYTE_BUDGET,
        max_results: RESULT_BUDGET,
    })
    .expect("valid profile")
}

fn create_fixture(root: &std::path::Path) {
    if root.exists() {
        std::fs::remove_dir_all(root).expect("remove stale fixture");
    }
    std::fs::create_dir_all(root.join("src")).expect("create fixture root");
    std::fs::write(root.join(FIXTURE_PATH), SOURCE).expect("write fixture");
}

fn test_root(label: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "animus-codebase-tool-{label}-{}",
        std::process::id()
    ))
}
