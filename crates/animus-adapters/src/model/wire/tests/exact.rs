//! Delta bodies are protocol bytes, not normalized identifier labels.

#[test]
fn content_thought_and_tool_deltas_preserve_whitespace_and_unicode() {
    for kind in [
        super::super::parts::Kind::ContentDelta,
        super::super::parts::Kind::ThoughtDelta,
        super::super::parts::Kind::ToolCallDelta,
    ] {
        for text in [" ", " padded \n", "\tα\n"] {
            let frame = frame(kind, text).expect("nonempty delta");
            assert_eq!(frame.body(), Some(text));
            assert_eq!(frame.kind(), kind);
        }
    }
}

#[test]
fn empty_nul_and_oversized_deltas_remain_invalid() {
    let invalid_text_chars = crate::validation::MAX_ADAPTER_FIELD_CHARS
        .checked_add(1)
        .expect("the test bound must have a larger representable value");
    let invalid_text = "x".repeat(invalid_text_chars);
    for text in ["", "\0", &invalid_text] {
        assert!(frame(super::super::parts::Kind::ContentDelta, text).is_err());
        assert!(frame(super::super::parts::Kind::ThoughtDelta, text).is_err());
        assert!(frame(super::super::parts::Kind::ToolCallDelta, text).is_err());
    }
}

fn frame(
    kind: super::super::parts::Kind,
    body: &str,
) -> Result<super::super::types::Frame, super::super::failure::Error> {
    super::super::types::Frame::from_draft(super::super::types::Draft {
        call_id: super::CALL_ID,
        order: super::FIRST_ORDER,
        kind,
        body: Some(body),
        tool_call_id: if kind == super::super::parts::Kind::ToolCallDelta {
            Some(super::TOOL_ID)
        } else {
            None
        },
        usage: None,
        visibility: super::super::parts::Visibility::Redacted,
    })
}
