#![allow(clippy::wildcard_imports, non_trait_imports)]
use super::*;
use crate::model::*;

    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        text: Some(text),
        ..base_response_fixture(finish_reason)
    }
}

fn structured_response_fixture(
    payload_label: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        structured_payload: Some(payload_label),
        ..base_response_fixture(finish_reason)
    }
}

fn tool_response_fixture(
    call_id: &'static str,
    tool_name: &'static str,
    arguments_label: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        tool_call_id: Some(call_id),
        tool_name: Some(tool_name),
        tool_arguments: Some(arguments_label),
        ..base_response_fixture(finish_reason)
    }
}

fn refusal_response_fixture(
    reason: &'static str,
    finish_reason: Option<&'static str>,
) -> ProviderResponseFixtureDraft<'static> {
    ProviderResponseFixtureDraft {
        refusal: Some(reason),
        ..base_response_fixture(finish_reason)
    }
}
