const STRICT_CODE: &[u8] = br#"{"type":"object","properties":{"code":{"type":"string"}},"required":["code"],"additionalProperties":false}"#;

#[test]
fn accepts_strict_code_and_rejects_missing_required() {
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, STRICT_CODE)
            .is_ok()
    );
    let missing = br#"{"type":"object","properties":{"code":{"type":"string"}},"required":[],"additionalProperties":false}"#;
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, missing).is_err()
    );
}

#[test]
fn accepts_nested_cohort_and_rejects_invalid_children() {
    let nested = br#"{"type":"object","properties":{"code":{"type":"array","items":{"anyOf":[{"type":"string"},{"type":"null"}]}}},"required":["code"],"additionalProperties":false,"$defs":{"empty":{"type":"object","properties":{},"required":[],"additionalProperties":false}}}"#;
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, nested).is_ok()
    );
    for bad in [
        br#"{"type":"object","properties":{"code":{"type":"array"}},"required":["code"],"additionalProperties":false}"#.as_slice(),
        br#"{"type":"object","properties":{},"required":[],"additionalProperties":false,"anyOf":[]}"#.as_slice(),
        br#"{"type":"object","properties":{},"required":[],"additionalProperties":false,"$defs":{"bad":false}}"#.as_slice(),
        br#"{"type":"object","properties":{},"required":[],"additionalProperties":true}"#.as_slice(),
        br#"{"type":"string"}"#.as_slice(),
        b"{",
        b"",
    ] {
        assert!(super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, bad).is_err());
    }
}

#[test]
fn preserves_duplicate_required_names_and_rejects_wrong_types() {
    let duplicated = br#"{"type":"object","properties":{"code":{"type":"string"}},"required":["code","code"],"additionalProperties":false}"#;
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, duplicated)
            .is_ok()
    );
    let wrong = br#"{"type":"object","properties":{"code":{"type":"string"}},"required":[0],"additionalProperties":false}"#;
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, wrong).is_err()
    );
}

#[test]
fn accepts_exact_byte_limit_and_rejects_larger_input() {
    let mut bytes = STRICT_CODE.to_vec();
    bytes.resize(crate::declarations::MAX_SCHEMA_BYTES, b' ');
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, &bytes).is_ok()
    );
    bytes.push(b' ');
    assert!(
        super::OpenAiToolSchemaBinding::from_json(crate::STEEL_REPL_TOOL_SCHEMA, &bytes).is_err()
    );
    assert!(super::OpenAiToolSchemaBinding::from_json("", STRICT_CODE).is_err());
}

#[test]
fn rejects_excessive_depth_and_accepts_shallow_arrays() {
    const ADMITTED_DEPTH: u32 = 4;
    const REFUSED_DEPTH: u32 = 128;
    for (depth, is_admitted) in [(ADMITTED_DEPTH, true), (REFUSED_DEPTH, false)] {
        let mut child = String::from(r#"{"type":"string"}"#);
        for _level in 0..depth {
            child = format!(r#"{{"type":"array","items":{child}}}"#);
        }
        let schema = format!(
            r#"{{"type":"object","properties":{{"code":{child}}},"required":["code"],"additionalProperties":false}}"#
        );
        assert!(schema.len() < crate::declarations::MAX_SCHEMA_BYTES);
        assert_eq!(
            super::OpenAiToolSchemaBinding::from_json(
                crate::STEEL_REPL_TOOL_SCHEMA,
                schema.as_bytes()
            )
            .is_ok(),
            is_admitted
        );
    }
}
