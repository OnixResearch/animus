const TEST_STREAM_BYTES: u64 = 16_384;
const CREATED: &[u8] = b"data: {\"type\":\"response.created\",\"sequence_number\":0,\"response\":{\"id\":\"response-1\",\"status\":\"in_progress\"}}\n\n";
const COMPLETED: &[u8] = b"data: {\"type\":\"response.completed\",\"sequence_number\":1,\"response\":{\"id\":\"response-1\",\"status\":\"completed\"}}\n\n";
const UNKNOWN: &[u8] = b"data: {\"type\":\"unsupported\",\"sequence_number\":1}\n\n";
const DONE: &[u8] = b"data: [DONE]\n\n";

fn parser() -> Result<super::OpenAiResponsesSseParser, Box<dyn std::error::Error>> {
    let capability = crate::OpenAiToolCapability::new(
        crate::STEEL_REPL_TOOL_NAME,
        crate::STEEL_REPL_TOOL_SCHEMA,
    )?;
    let matrix = crate::OpenAiToolCapabilityMatrix::new(
        "model-1",
        vec![animus_adapters::CanonicalModelRole::User],
        vec![capability],
        vec![crate::OpenAiToolStreamChannel::Content],
    )?;
    assert_eq!(matrix.model_id(), "model-1");
    assert_eq!(matrix.tools().len(), 1);
    let dispatch = crate::OpenAiToolDispatch::initial(
        "call-1",
        matrix,
        Vec::new(),
        crate::OpenAiToolPolicy::serial(crate::OpenAiToolChoice::auto()),
    )?;
    Ok(super::OpenAiResponsesSseParser::new(
        dispatch,
        crate::OpenAiStreamBounds::reviewed(TEST_STREAM_BYTES),
    )?)
}

#[test]
fn accepts_completion_and_rejects_later_data_without_state_change()
-> Result<(), Box<dyn std::error::Error>> {
    let mut parser = parser()?;
    parser.push_chunk(CREATED)?;
    parser.push_chunk(COMPLETED)?;
    parser.push_chunk(DONE)?;
    let checkpoint = parser.clone();
    assert!(matches!(parser.push_chunk(DONE), Err(error)
        if error.class() == crate::OpenAiStreamFailureClass::Lifecycle));
    assert_unchanged(&parser, &checkpoint);
    let outcome = parser.finish()?;
    assert_eq!(outcome.terminal(), crate::OpenAiStreamTerminal::Completed);
    assert_eq!(outcome.frames().len(), 1);
    Ok(())
}

#[test]
fn rejects_whole_chunk_then_accepts_same_sequence() -> Result<(), Box<dyn std::error::Error>> {
    let mut parser = parser()?;
    let checkpoint = parser.clone();
    let bad_chunk = [CREATED, UNKNOWN].concat();
    assert!(parser.push_chunk(&bad_chunk).is_err());
    assert_unchanged(&parser, &checkpoint);
    parser.push_chunk(CREATED)?;
    let checkpoint = parser.clone();
    assert!(matches!(parser.push_chunk(UNKNOWN), Err(error)
        if error.class() == crate::OpenAiStreamFailureClass::UnsupportedEvent));
    assert_unchanged(&parser, &checkpoint);
    parser.push_chunk(COMPLETED)?;
    assert!(parser.finish().is_ok());
    Ok(())
}

#[test]
fn rejects_byte_and_sequence_overflow_without_state_change()
-> Result<(), Box<dyn std::error::Error>> {
    let mut parser = parser()?;
    parser.bounds.stream_bytes = 1;
    let checkpoint = parser.clone();
    assert!(matches!(parser.push_chunk(CREATED), Err(error)
        if error.class() == crate::OpenAiStreamFailureClass::BoundExceeded));
    assert_unchanged(&parser, &checkpoint);
    parser.bounds = crate::OpenAiStreamBounds::reviewed(TEST_STREAM_BYTES);
    let overflow = b"data: {\"type\":\"response.created\",\"sequence_number\":18446744073709551615,\"response\":{\"id\":\"response-1\",\"status\":\"in_progress\"}}\n\n";
    let checkpoint = parser.clone();
    assert!(matches!(parser.push_chunk(overflow), Err(error)
        if error.class() == crate::OpenAiStreamFailureClass::Lifecycle));
    assert_unchanged(&parser, &checkpoint);
    parser.push_chunk(CREATED)?;
    parser.push_chunk(COMPLETED)?;
    assert!(parser.finish().is_ok());
    Ok(())
}

#[test]
fn rejects_invalid_function_registration_then_accepts_corrected_item()
-> Result<(), Box<dyn std::error::Error>> {
    let mut parser = parser()?;
    parser.push_chunk(CREATED)?;
    let checkpoint = parser.clone();
    let bad = b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":1,\"item\":{\"id\":\"item-1\",\"type\":\"function_call\",\"call_id\":\"tool-1\",\"name\":\"not_admitted\"}}\n\n";
    assert!(matches!(parser.push_chunk(bad), Err(error)
        if error.class() == crate::OpenAiStreamFailureClass::Correlation));
    assert_unchanged(&parser, &checkpoint);
    let good = b"data: {\"type\":\"response.output_item.added\",\"sequence_number\":1,\"item\":{\"id\":\"item-1\",\"type\":\"function_call\",\"call_id\":\"tool-1\",\"name\":\"steel_repl\"}}\n\n";
    parser.push_chunk(good)?;
    assert_eq!(parser.output_items.len(), 1);
    assert_eq!(parser.tool_states.len(), 1);
    Ok(())
}

#[test]
fn accepts_portable_const_bounds_and_rejects_zero_bounds() -> Result<(), Box<dyn std::error::Error>>
{
    const PORTABLE_BYTES: u64 = 4_294_967_296;
    const BOUNDS: crate::OpenAiStreamBounds = crate::OpenAiStreamBounds::reviewed(PORTABLE_BYTES);
    assert_eq!(BOUNDS.stream_bytes, PORTABLE_BYTES);
    assert!(BOUNDS.validate().is_ok());
    assert!(crate::OpenAiStreamBounds::reviewed(0).validate().is_err());
    let mut parser = parser()?;
    parser.cancel()?;
    let checkpoint = parser.clone();
    assert!(parser.cancel().is_err());
    assert_unchanged(&parser, &checkpoint);
    assert_eq!(
        parser.finish()?.terminal(),
        crate::OpenAiStreamTerminal::Cancelled
    );
    Ok(())
}

fn assert_unchanged(
    actual: &super::OpenAiResponsesSseParser,
    prior: &super::OpenAiResponsesSseParser,
) {
    assert_eq!(actual.total_bytes, prior.total_bytes);
    assert_eq!(actual.event_count, prior.event_count);
    assert_eq!(actual.next_sequence, prior.next_sequence);
    assert_eq!(actual.next_order, prior.next_order);
    assert_eq!(actual.response_id, prior.response_id);
    assert_eq!(actual.lifecycle_started, prior.lifecycle_started);
    assert_eq!(actual.terminal, prior.terminal);
    assert_eq!(actual.stream_end_seen, prior.stream_end_seen);
    assert_eq!(actual.buffer, prior.buffer);
    assert_eq!(actual.frames, prior.frames);
    assert_eq!(actual.output_items, prior.output_items);
    assert_eq!(actual.completed_output_items, prior.completed_output_items);
    assert_eq!(actual.tool_states, prior.tool_states);
    assert_eq!(actual.tool_calls, prior.tool_calls);
    assert_eq!(actual.usage_emitted, prior.usage_emitted);
    assert_eq!(actual.retry_progress, prior.retry_progress);
}
