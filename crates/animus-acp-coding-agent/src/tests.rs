// r[verify agent_harness_runtime.acp_coding_agent.verification]

use crate::admission::{
    Edit, EditIssue, MAX_EDIT_TEXT_BYTES, MAX_EDITS_PER_CALL, MAX_PATH_BYTES, PathIssue, ToolCall,
    ToolIssue, WorkspacePath, apply_edits, parse_tool_call,
};
use crate::checks::{AdmittedChecks, CHECKS_SCHEMA, ChecksIssue, MAX_TIMEOUT_MS, parse_checks};

fn edit(old: &str, new: &str) -> Edit {
    Edit {
        old: old.to_owned(),
        new: new.to_owned(),
    }
}

#[test]
fn relative_paths_are_admitted() {
    let path = WorkspacePath::parse("crates/core/src/lib.rs").unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(
        path.components().collect::<Vec<_>>(),
        ["crates", "core", "src", "lib.rs"]
    );
    assert!(WorkspacePath::parse(".").is_ok_and(|p| p.is_root()));
    assert!(WorkspacePath::parse(".hidden/file").is_ok());
}

#[test]
fn escaping_paths_are_refused() {
    let refused = [
        ("", PathIssue::Empty),
        ("/etc/passwd", PathIssue::Absolute),
        ("../secret", PathIssue::ForbiddenComponent),
        ("a/../../b", PathIssue::ForbiddenComponent),
        ("a/./b", PathIssue::ForbiddenComponent),
        ("a//b", PathIssue::ForbiddenComponent),
        ("a/", PathIssue::ForbiddenComponent),
        ("a\0b", PathIssue::ForbiddenCharacter),
        ("a\\..\\b", PathIssue::ForbiddenCharacter),
    ];
    for (text, issue) in refused {
        assert_eq!(WorkspacePath::parse(text), Err(issue), "{text:?}");
    }
    let long = "a".repeat(MAX_PATH_BYTES + 1);
    assert_eq!(WorkspacePath::parse(&long), Err(PathIssue::TooLong));
}

#[test]
fn the_four_tools_parse() {
    assert!(matches!(
        parse_tool_call("read_file", r#"{"path":"src/lib.rs"}"#),
        Ok(ToolCall::ReadFile { .. })
    ));
    assert!(matches!(
        parse_tool_call("list_files", r#"{"path":"."}"#),
        Ok(ToolCall::ListFiles { .. })
    ));
    assert!(matches!(
        parse_tool_call(
            "edit_file",
            r#"{"path":"a.rs","edits":[{"old":"x","new":"y"}]}"#
        ),
        Ok(ToolCall::EditFile { .. })
    ));
    assert_eq!(
        parse_tool_call("run_check", r#"{"check_id":"cargo-test"}"#),
        Ok(ToolCall::RunCheck {
            check_id: "cargo-test".to_owned()
        })
    );
}

#[test]
fn malformed_tool_calls_are_refused() {
    assert_eq!(
        parse_tool_call("shell", r#"{"cmd":"ls"}"#),
        Err(ToolIssue::UnknownTool)
    );
    assert_eq!(
        parse_tool_call("read_file", r#"{"path":"a","extra":1}"#),
        Err(ToolIssue::MalformedArguments)
    );
    assert_eq!(
        parse_tool_call("read_file", "not json"),
        Err(ToolIssue::MalformedArguments)
    );
    assert_eq!(
        parse_tool_call("read_file", r#"{"path":"../x"}"#),
        Err(ToolIssue::Path(PathIssue::ForbiddenComponent))
    );
    assert_eq!(
        parse_tool_call("read_file", r#"{"path":"."}"#),
        Err(ToolIssue::RootIsNotAFile)
    );
    assert_eq!(
        parse_tool_call("edit_file", r#"{"path":"a","edits":[]}"#),
        Err(ToolIssue::EditBounds)
    );
    let too_many = format!(
        r#"{{"path":"a","edits":[{}]}}"#,
        vec![r#"{"old":"x","new":"y"}"#; MAX_EDITS_PER_CALL + 1].join(",")
    );
    assert_eq!(
        parse_tool_call("edit_file", &too_many),
        Err(ToolIssue::EditBounds)
    );
    let oversized = format!(
        r#"{{"path":"a","edits":[{{"old":"x","new":"{}"}}]}}"#,
        "y".repeat(MAX_EDIT_TEXT_BYTES + 1)
    );
    assert_eq!(
        parse_tool_call("edit_file", &oversized),
        Err(ToolIssue::EditBounds)
    );
    for bad in ["", "-lead", "Upper", "has space", &"a".repeat(65)] {
        let json = format!(r#"{{"check_id":"{bad}"}}"#);
        assert_eq!(
            parse_tool_call("run_check", &json),
            Err(ToolIssue::CheckId),
            "{bad:?}"
        );
    }
}

#[test]
fn exact_edits_apply_in_order() {
    let result = apply_edits(
        Some("fn a() {}\nfn b() {}\n"),
        &[edit("fn a", "fn first"), edit("fn b", "fn second")],
    );
    assert_eq!(result, Ok("fn first() {}\nfn second() {}\n".to_owned()));
    assert_eq!(
        apply_edits(None, &[edit("", "new file\n")]),
        Ok("new file\n".to_owned())
    );
}

#[test]
fn ambiguous_missing_and_conflicting_edits_are_refused() {
    assert_eq!(
        apply_edits(Some("x x"), &[edit("x", "y")]),
        Err(EditIssue::Ambiguous { index: 0 })
    );
    assert_eq!(
        apply_edits(Some("abc"), &[edit("z", "y")]),
        Err(EditIssue::NotFound { index: 0 })
    );
    assert_eq!(
        apply_edits(Some("abc"), &[edit("a", "b"), edit("a", "c")]),
        Err(EditIssue::NotFound { index: 1 })
    );
    assert_eq!(
        apply_edits(Some("abc"), &[edit("", "x")]),
        Err(EditIssue::CreateConflict)
    );
    assert_eq!(
        apply_edits(None, &[edit("", "x"), edit("x", "y")]),
        Err(EditIssue::CreateConflict)
    );
    assert_eq!(
        apply_edits(None, &[edit("a", "b")]),
        Err(EditIssue::Missing)
    );
    assert_eq!(
        apply_edits(Some("abc"), &[edit("a", "b"), edit("", "y")]),
        Err(EditIssue::EmptyOld { index: 1 })
    );
}

fn checks_json(checks: &str) -> String {
    format!(r#"{{"schema":"{CHECKS_SCHEMA}","checks":[{checks}]}}"#)
}

const CARGO_TEST: &str = r#"{"id":"cargo-test","argv":["cargo","test","--offline"],"timeout_ms":600000,"stdout_max_bytes":65536,"stderr_max_bytes":65536}"#;

#[test]
fn a_declared_checks_file_is_admitted() {
    let bytes = checks_json(CARGO_TEST);
    let admitted = parse_checks(bytes.as_bytes()).unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(admitted.checks().len(), 1);
    assert!(admitted.find("cargo-test").is_some());
    assert!(admitted.find("rm-all").is_none());
    assert_eq!(
        admitted.identity(),
        blake3::hash(bytes.as_bytes()).to_hex().as_str()
    );
    assert!(AdmittedChecks::none().find("cargo-test").is_none());
}

#[test]
fn malformed_checks_files_are_refused() {
    assert_eq!(parse_checks(b"{}"), Err(ChecksIssue::Malformed));
    assert_eq!(
        parse_checks(br#"{"schema":"other","checks":[]}"#),
        Err(ChecksIssue::Schema)
    );
    let unknown_field = checks_json(&CARGO_TEST.replace("\"argv\"", "\"shell\":true,\"argv\""));
    assert_eq!(
        parse_checks(unknown_field.as_bytes()),
        Err(ChecksIssue::Malformed)
    );
    let duplicate = checks_json(&format!("{CARGO_TEST},{CARGO_TEST}"));
    assert_eq!(
        parse_checks(duplicate.as_bytes()),
        Err(ChecksIssue::Duplicate("cargo-test".to_owned()))
    );
    let bad_id = checks_json(&CARGO_TEST.replace("cargo-test", "Cargo Test"));
    assert!(matches!(
        parse_checks(bad_id.as_bytes()),
        Err(ChecksIssue::Id(_))
    ));
    let empty_argv = checks_json(&CARGO_TEST.replace(r#"["cargo","test","--offline"]"#, "[]"));
    assert!(matches!(
        parse_checks(empty_argv.as_bytes()),
        Err(ChecksIssue::Argv(_))
    ));
    let empty_arg = checks_json(&CARGO_TEST.replace(r#""--offline""#, r#""""#));
    assert!(matches!(
        parse_checks(empty_arg.as_bytes()),
        Err(ChecksIssue::Argv(_))
    ));
    let zero_timeout = checks_json(&CARGO_TEST.replace("600000", "0"));
    assert!(matches!(
        parse_checks(zero_timeout.as_bytes()),
        Err(ChecksIssue::Budget(_))
    ));
    let long_timeout =
        checks_json(&CARGO_TEST.replace("600000", &(MAX_TIMEOUT_MS + 1).to_string()));
    assert!(matches!(
        parse_checks(long_timeout.as_bytes()),
        Err(ChecksIssue::Budget(_))
    ));
    let missing_budget = checks_json(&CARGO_TEST.replace(r#","stderr_max_bytes":65536"#, ""));
    assert_eq!(
        parse_checks(missing_budget.as_bytes()),
        Err(ChecksIssue::Malformed)
    );
}

#[test]
fn the_workspace_tool_schema_is_admitted_by_the_openai_host() {
    let binding = animus_std_openai::OpenAiToolSchemaBinding::from_json(
        "animus.acp-coding-agent.workspace.v1",
        crate::admission::WORKSPACE_TOOL_SCHEMA.as_bytes(),
    );
    assert!(binding.is_ok(), "{binding:?}");
}

#[test]
fn workspace_calls_route_by_op() {
    use crate::admission::parse_workspace_call;
    assert!(matches!(
        parse_workspace_call(r#"{"request":{"op":"read_file","path":"src/lib.rs"}}"#),
        Ok(ToolCall::ReadFile { .. })
    ));
    assert!(matches!(
        parse_workspace_call(
            r#"{"request":{"op":"edit_file","path":"a","edits":[{"old":"x","new":"y"}]}}"#
        ),
        Ok(ToolCall::EditFile { .. })
    ));
    assert_eq!(
        parse_workspace_call(r#"{"request":{"op":"run_check","check_id":"cargo-test"}}"#),
        Ok(ToolCall::RunCheck {
            check_id: "cargo-test".to_owned()
        })
    );
    assert_eq!(
        parse_workspace_call(r#"{"request":{"op":"shell","cmd":"ls"}}"#),
        Err(ToolIssue::UnknownTool)
    );
    assert_eq!(
        parse_workspace_call(r#"{"request":{"path":"a"}}"#),
        Err(ToolIssue::MalformedArguments)
    );
    assert_eq!(
        parse_workspace_call(r#"{"request":{"op":"read_file","path":"../x"}}"#),
        Err(ToolIssue::Path(PathIssue::ForbiddenComponent))
    );
    assert_eq!(
        parse_workspace_call(r#"{"request":{"op":"read_file","path":"a"},"extra":1}"#),
        Err(ToolIssue::MalformedArguments)
    );
}
