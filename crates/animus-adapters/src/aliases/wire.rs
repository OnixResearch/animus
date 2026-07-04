/// Stable model-call correlation identifier carried by stream frames.
pub type ModelStreamCallId = crate::model::wire::parts::CallId;
/// Canonical model stream frame emitted by model adapters.
pub type ModelStreamFrame = crate::model::wire::types::Frame;
/// Borrowed model stream frame fields before validation.
pub type ModelStreamFrameDraft<'a> = crate::model::wire::types::Draft<'a>;
/// Canonical model stream frame kind.
pub type ModelStreamFrameKind = crate::model::wire::parts::Kind;
/// Model streaming validation, parsing, and assembly failure.
pub type ModelStreamError = crate::model::wire::failure::Error;
/// Strictly increasing per-call stream order.
pub type ModelStreamOrder = crate::model::wire::parts::Order;
/// Stable tool-call correlation identifier carried by tool-call deltas.
pub type ModelStreamToolCallId = crate::model::wire::parts::ToolCallId;
/// Usage counts supplied by a model stream.
pub type ModelStreamUsage = crate::model::wire::parts::Usage;
/// Default sink visibility attached to streamed text channels.
pub type ModelStreamVisibility = crate::model::wire::parts::Visibility;
/// Request sent from the harness to a model component.
pub type ModelRequest = crate::model::Request;
/// Response returned by a model component.
pub type ModelResponse = crate::model::Response;
/// Assembles assistant-visible response text from validated stream frames.
pub fn assemble_visible_model_stream_response(
    frames: &[ModelStreamFrame],
) -> Result<ModelResponse, ModelStreamError> {
    crate::model::wire::validate::assemble(frames)
}

/// Parses raw provider-stream fixture chunks into canonical model frames.
pub fn parse_model_stream_fixture(
    chunks: &[&str],
) -> Result<alloc::vec::Vec<ModelStreamFrame>, ModelStreamError> {
    crate::model::wire::fixture::parse(chunks)
}

/// Validates model stream ordering, correlation, and terminal status.
pub fn validate_model_stream_frames(frames: &[ModelStreamFrame]) -> Result<(), ModelStreamError> {
    crate::model::wire::validate::frames(frames)
}
