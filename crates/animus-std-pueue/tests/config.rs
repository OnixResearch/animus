//! Positive and negative typed pueue configuration tests.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::no_unwrap,
        tigerstyle::non_trait_imports,
        reason = "configuration-only tests use explicit expect messages while comparing positive and negative fixtures"
    )
)]

mod support;

use animus_std_pueue::{PueueHostConfigIssue, parse_pueue_host_config};

const ROOT: &str = "/tmp/animus-pueue-test";

#[test]
fn valid_generated_shape_resolves_reviewed_profile() {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let config = parse_pueue_host_config(text.as_str()).expect("valid pueue config");
    let resolved = config
        .resolve_profile("fixture-check")
        .expect("reviewed profile");

    assert_eq!(config.namespace_id(), support::NAMESPACE_ID);
    assert_eq!(resolved.request.program.as_deref(), Some(support::FAKE_JOB));
    assert_eq!(resolved.request.command, None);
}

#[test]
fn unknown_raw_command_field_is_rejected() {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    value["profiles"][0]["command"] = serde_json::json!("echo unsafe");
    let error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("raw command denied");

    assert_eq!(error.issue(), PueueHostConfigIssue::Shape);
}

#[test]
fn unsupported_capability_and_missing_non_claim_are_rejected() {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    value["capabilities"]["interactions"] = serde_json::json!(["wait", "send_input"]);
    let capability_error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("send input denied");
    assert_eq!(capability_error.issue(), PueueHostConfigIssue::Value);

    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    value["non_claims"] = serde_json::json!(["sandboxing"]);
    let claim_error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("claim omission denied");
    assert_eq!(claim_error.issue(), PueueHostConfigIssue::Value);
}

#[test]
fn duplicate_profile_names_are_rejected() {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    let duplicate = value["profiles"][0].clone();
    value["profiles"]
        .as_array_mut()
        .expect("profile array")
        .push(duplicate);
    let error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("duplicate profile denied");

    assert_eq!(error.issue(), PueueHostConfigIssue::Profile);
}

#[test]
fn zero_bound_and_relative_executable_are_rejected() {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    value["bounds"]["max_log_lines"] = serde_json::json!(0);
    let bound_error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("zero bound denied");
    assert_eq!(bound_error.issue(), PueueHostConfigIssue::Value);

    let mut value: serde_json::Value = serde_json::from_str(text.as_str()).expect("fixture JSON");
    value["executables"]["pueue"] = serde_json::json!("pueue");
    let path_error =
        parse_pueue_host_config(value.to_string().as_str()).expect_err("relative path denied");
    assert_eq!(path_error.issue(), PueueHostConfigIssue::Value);
}
