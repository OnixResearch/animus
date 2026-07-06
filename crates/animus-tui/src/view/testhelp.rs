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

/// Helper: build a valid tool request.
pub fn tool_req(tool_name: &str, raw_input: &str) -> animus_adapters::ToolRequest {
    unwrap_or_abort(
        animus_adapters::ToolRequest::new(tool_name, raw_input),
        "request should validate",
    )
}

/// Helper: build a tool call card from name+input.
pub fn tool_card(tool_name: &str, raw_input: &str) -> ToolCallCard {
    ToolCallCard::from_request(&tool_req(tool_name, raw_input))
}

/// Helper: build a fresh tool panel state.
pub fn fresh_panel() -> ToolPanelState {
    ToolPanelState::new()
}