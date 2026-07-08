// Shared test helpers for animus-tui view module tests.

fn unwrap_or_abort<T>(result: Result<T, impl std::fmt::Debug>, _msg: &str) -> T {
    match result {
        Ok(v) => v,
        Err(_e) => std::process::abort(),
    }
}

/// Helper: build a content-delta stream frame.
pub fn delta_frame(cid: &str, order: u64, body: &str) -> animus_adapters::ModelStreamFrame {
    unwrap_or_abort(
        animus_adapters::ModelStreamFrame::from_draft(animus_adapters::ModelStreamFrameDraft {
            call_id: cid,
            order,
            kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
            body: Some(body),
            tool_call_id: None,
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
        }),
        "frame should validate",
    )
}

/// Helper: build a valid stream call ID.
pub fn call_id(raw: &str) -> animus_adapters::ModelStreamCallId {
    unwrap_or_abort(
        animus_adapters::ModelStreamCallId::new(raw),
        "call id should validate",
    )
}

/// Helper: build a model response.
pub fn msg_response(content: &str) -> animus_adapters::ModelResponse {
    unwrap_or_abort(
        animus_adapters::ModelResponse::new(content),
        "response should validate",
    )
}

/// Helper: build a finish frame kind.
pub fn finish_kind() -> animus_adapters::ModelStreamFrameKind {
    animus_adapters::ModelStreamFrameKind::Finish
}

/// Helper: build a valid image handle.
pub fn img_handle(media_type: &str) -> animus_adapters::ModelImagePayloadHandle {
    unwrap_or_abort(
        animus_adapters::ModelImagePayloadHandle::from_draft(
            animus_adapters::ModelImagePayloadHandleDraft {
                kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
                safe_handle: "safe_abc",
                content_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
                media_type,
            },
        ),
        "handle should validate",
    )
}

/// Helper: build a fresh conversation render model.
pub fn fresh_model() -> ConversationRenderModel {
    ConversationRenderModel::new()
}

/// Helper: build a sized conversation render model.
pub fn sized_model(n: u64) -> ConversationRenderModel {
    ConversationRenderModel::with_window_size(n)
}

/// Helper: build a fresh app shell state.
pub fn fresh_shell() -> AppShellState {
    AppShellState::new()
}

/// Helper: build a fresh idle status rail.
pub fn idle_rail() -> StatusRailState {
    StatusRailState::idle()
}

const TEST_TOOL_SCHEMA: &str = "schema.tui-tool.v1";
const TEST_TOOL_ARGUMENT: &str = "query";
const TEST_TOOL_CALL_ID: &str = "tool-call-tui";
const TEST_TOOL_VISIBLE_BUDGET: u32 = 64;
const TEST_TOOL_MUTATION_TARGET: &str = "resource:tui";
const TEST_TOOL_CAPABILITY: &str = "tool.tui";
const TEST_TOOL_ARGUMENT_COUNT: usize = 1;

/// Helper: build a valid current-shape tool request.
#[allow(unknown_lints, ambiguous_params)]
pub fn make_req(request_name: &str, request_text: &str) -> animus_adapters::ToolRequest {
    let fields = [animus_adapters::tool::ArgumentFieldDraft {
        name: Some(TEST_TOOL_ARGUMENT),
        value: Some(request_text),
    }];
    let arguments = unwrap_or_abort(
        animus_adapters::tool::PreparedArguments::prepare(
            &animus_adapters::tool::ArgumentPreparationRequest {
                current_schema_identity: TEST_TOOL_SCHEMA,
                allowed_field_names: &[TEST_TOOL_ARGUMENT],
                compatibility_rules: &[],
                fields: &fields,
            },
        ),
        "arguments should validate",
    );
    assert_eq!(arguments.schema_identity(), TEST_TOOL_SCHEMA);
    assert_eq!(arguments.fields().len(), TEST_TOOL_ARGUMENT_COUNT);
    let request = unwrap_or_abort(
        animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
            name: Some(request_name),
            arguments: Some(&arguments),
            correlation_id: Some(TEST_TOOL_CALL_ID),
            output_policy: animus_adapters::tool::OutputPolicy {
                visible_char_budget: TEST_TOOL_VISIBLE_BUDGET,
            },
            mutation_targets: &[TEST_TOOL_MUTATION_TARGET],
            required_capability_labels: &[TEST_TOOL_CAPABILITY],
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
        }),
        "request should validate",
    );
    assert_eq!(request.name(), request_name);
    assert_eq!(request.correlation_id(), TEST_TOOL_CALL_ID);
    request
}

/// Helper: build a tool call card from name+input.
#[allow(unknown_lints, ambiguous_params)]
pub fn build_card(card_name: &str, card_text: &str) -> ToolCallCard {
    ToolCallCard::from_request(&make_req(card_name, card_text))
}

/// Helper: build a fresh tool panel state.
pub fn fresh_panel() -> ToolPanelState {
    ToolPanelState::new()
}