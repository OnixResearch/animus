// Shared test helpers for animus-tui view module tests.
// Included from each test module via `use super::*;` — types brought into scope
// by the test wrapper in view.rs.

/// Helper: build a content-delta stream frame.
pub fn delta_frame(call_id: &str, order: u64, body: &str) -> animus_adapters::ModelStreamFrame {
    animus_adapters::ModelStreamFrame::from_draft(animus_adapters::ModelStreamFrameDraft {
        call_id,
        order,
        kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
        body: Some(body),
        tool_call_id: None,
        usage: None,
        visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
    })
    .expect("frame should validate")
}

/// Helper: build a valid stream call ID.
pub fn call_id(raw: &str) -> animus_adapters::ModelStreamCallId {
    animus_adapters::ModelStreamCallId::new(raw).expect("call id should validate")
}

/// Helper: build a model response.
pub fn msg_response(content: &str) -> animus_adapters::ModelResponse {
    animus_adapters::ModelResponse::new(content).expect("response should validate")
}

/// Helper: build a finish frame kind.
pub fn finish_kind() -> animus_adapters::ModelStreamFrameKind {
    animus_adapters::ModelStreamFrameKind::Finish
}

/// Helper: build a valid image handle.
pub fn img_handle(media_type: &str) -> animus_adapters::ModelImagePayloadHandle {
    animus_adapters::ModelImagePayloadHandle::from_draft(
        animus_adapters::ModelImagePayloadHandleDraft {
            kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
            safe_handle: "safe_abc",
            content_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            media_type,
        },
    )
    .expect("handle should validate")
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
pub fn tool_req(nm: &str, val: &str) -> animus_adapters::ToolRequest {
    animus_adapters::ToolRequest::new(nm, val).expect("request should validate")
}

/// Helper: build a tool call card from name+input.
pub fn tool_card(nm: &str, val: &str) -> ToolCallCard {
    ToolCallCard::from_request(&tool_req(nm, val))
}

/// Helper: build a fresh tool panel state.
pub fn fresh_panel() -> ToolPanelState {
    ToolPanelState::new()
}