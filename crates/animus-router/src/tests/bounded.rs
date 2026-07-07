// Each case delegates the eight-bucket assertions to `assert_signal_labels`
// so the scenario setup stays readable while assertion coverage remains high.
#![allow(unknown_lints, assertion_density)]

use super::*;
use crate::SignalSource;

#[test]
fn simple_remote_request_projects_low_bands() {
    let metadata = metadata(crate::RequestMetadataDraft {
        prompt_char_estimate: SIMPLE_PROMPT_CHARS,
        tool_call_count: TOOL_COUNT_NONE,
        contains_code: false,
        requires_structured_output: false,
        context_window_fill_per_mille: LOW_CONTEXT_PER_MILLE,
        privacy_policy: "remote_allowed",
        fallback_hint: "eligible",
    });

    assert_signal_labels(
        metadata,
        [
            "simple",
            "low",
            "low",
            "low",
            "low",
            "low",
            "remote_allowed",
            "eligible",
        ],
    );
}

#[test]
fn local_code_request_projects_coding_bands() {
    let metadata = metadata(crate::RequestMetadataDraft {
        prompt_char_estimate: SIMPLE_PROMPT_CHARS,
        tool_call_count: TOOL_COUNT_NONE,
        contains_code: true,
        requires_structured_output: false,
        context_window_fill_per_mille: LOW_CONTEXT_PER_MILLE,
        privacy_policy: "local_only",
        fallback_hint: "ineligible",
    });

    assert_signal_labels(
        metadata,
        [
            "coding",
            "low",
            "low",
            "low",
            "low",
            "low",
            "local_only",
            "ineligible",
        ],
    );
}

#[test]
fn structured_review_request_projects_pressure_bands() {
    let metadata = metadata(crate::RequestMetadataDraft {
        prompt_char_estimate: REVIEW_PROMPT_CHARS,
        tool_call_count: TOOL_COUNT_NONE,
        contains_code: false,
        requires_structured_output: true,
        context_window_fill_per_mille: HIGH_CONTEXT_PER_MILLE,
        privacy_policy: "remote_allowed",
        fallback_hint: "eligible",
    });

    assert_signal_labels(
        metadata,
        [
            "review",
            "medium",
            "strict",
            "high",
            "medium",
            "medium",
            "remote_allowed",
            "eligible",
        ],
    );
}

#[test]
fn workflow_request_projects_strict_bands() {
    let metadata = metadata(crate::RequestMetadataDraft {
        prompt_char_estimate: REASONING_PROMPT_CHARS,
        tool_call_count: TOOL_COUNT_TWO,
        contains_code: false,
        requires_structured_output: false,
        context_window_fill_per_mille: STRICT_CONTEXT_PER_MILLE,
        privacy_policy: "remote_allowed",
        fallback_hint: "eligible",
    });

    assert_signal_labels(
        metadata,
        [
            "workflow",
            "strict",
            "high",
            "strict",
            "high",
            "high",
            "remote_allowed",
            "eligible",
        ],
    );
}

#[test]
fn rule_signal_source_feeds_signal_projection() {
    let source = crate::RuleSignalSource;
    let metadata = metadata(crate::RequestMetadataDraft {
        prompt_char_estimate: STRICT_PROMPT_CHARS,
        tool_call_count: TOOL_COUNT_ONE,
        contains_code: false,
        requires_structured_output: false,
        context_window_fill_per_mille: STRICT_CONTEXT_PER_MILLE,
        privacy_policy: "remote_allowed",
        fallback_hint: "eligible",
    });
    let draft = source
        .project(&metadata)
        .expect("rule source should project");
    let projected = crate::Signals::project(draft).expect("projected labels should parse");

    assert_eq!(projected.task_shape(), crate::TaskShape::Reasoning);
    assert_eq!(projected.risk(), crate::SignalBand::Strict);
    assert_eq!(projected.privacy(), crate::PrivacyBand::RemoteAllowed);
}

fn assert_signal_labels(
    metadata: crate::RequestMetadata,
    expected: [&str; crate::projection::SIGNAL_BUCKET_COUNT],
) {
    let draft = crate::project_signals(&metadata).expect("request metadata should project");
    let projected = crate::Signals::project(draft).expect("signal labels should parse");
    let actual = projected.bucket_labels();

    assert_eq!(actual.len(), expected.len());
    for (actual_label, expected_label) in actual.iter().zip(expected) {
        assert_eq!(actual_label.as_str(), expected_label);
    }
}

fn metadata(draft: crate::RequestMetadataDraft<'_>) -> crate::RequestMetadata {
    crate::RequestMetadata::new(draft).expect("metadata should validate")
}
