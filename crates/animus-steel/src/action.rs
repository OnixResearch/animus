const MODEL_TAG: &str = "model";
const TOOL_TAG: &str = "tool";
const RESPOND_TAG: &str = "respond";
const STOP_TAG: &str = "stop";
const MODEL_ARITY: usize = 2;
const TOOL_ARITY: usize = 3;
const RESPOND_ARITY: usize = 2;
const STOP_ARITY: usize = 1;

/// Next action selected by the Steel loop fabric.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Decision {
    /// Send a request to a model adapter.
    CallModel {
        /// Validated model request.
        request: animus_adapters::ModelRequest,
    },
    /// Send a request to a tool adapter.
    CallTool {
        /// Validated tool request.
        request: animus_adapters::ToolRequest,
    },
    /// Return a final response from the loop.
    Respond {
        /// Validated response message.
        response: animus_adapters::ModelResponse,
    },
    /// Stop the loop without producing another adapter request.
    Stop,
}

impl Decision {
    pub(crate) fn from_value(value: steel::SteelVal) -> Result<Self, crate::LoopError> {
        let items = list_items(value)?;
        let tag = required_symbol(&items, TAG_INDEX)?;
        match tag {
            MODEL_TAG => from_model_items(&items),
            TOOL_TAG => from_tool_items(&items),
            RESPOND_TAG => from_respond_items(&items),
            STOP_TAG => from_stop_items(&items),
            other => Err(crate::LoopError::InvalidDecision {
                reason: format!("unknown decision tag {other}"),
            }),
        }
    }
}

const TAG_INDEX: usize = 0;
const MODEL_PROMPT_INDEX: usize = 1;
const TOOL_NAME_INDEX: usize = 1;
const TOOL_INPUT_INDEX: usize = 2;
const RESPONSE_MESSAGE_INDEX: usize = 1;

fn from_model_items(items: &[steel::SteelVal]) -> Result<Decision, crate::LoopError> {
    validate_arity(items, MODEL_TAG, MODEL_ARITY)?;
    let prompt = required_string(items, MODEL_PROMPT_INDEX)?;
    Ok(Decision::CallModel {
        request: animus_adapters::ModelRequest::new(prompt)?,
    })
}

fn from_tool_items(items: &[steel::SteelVal]) -> Result<Decision, crate::LoopError> {
    validate_arity(items, TOOL_TAG, TOOL_ARITY)?;
    let name = required_string(items, TOOL_NAME_INDEX)?;
    let input = required_string(items, TOOL_INPUT_INDEX)?;
    Ok(Decision::CallTool {
        request: animus_adapters::ToolRequest::new(name, input)?,
    })
}

fn from_respond_items(items: &[steel::SteelVal]) -> Result<Decision, crate::LoopError> {
    validate_arity(items, RESPOND_TAG, RESPOND_ARITY)?;
    let message = required_string(items, RESPONSE_MESSAGE_INDEX)?;
    Ok(Decision::Respond {
        response: animus_adapters::ModelResponse::new(message)?,
    })
}

fn from_stop_items(items: &[steel::SteelVal]) -> Result<Decision, crate::LoopError> {
    validate_arity(items, STOP_TAG, STOP_ARITY)?;
    Ok(Decision::Stop)
}

fn list_items(value: steel::SteelVal) -> Result<Vec<steel::SteelVal>, crate::LoopError> {
    match value {
        steel::SteelVal::ListV(list) => Ok(list.iter().cloned().collect()),
        other => Err(crate::LoopError::InvalidDecision {
            reason: format!("expected list decision, got {other:?}"),
        }),
    }
}

fn validate_arity(
    items: &[steel::SteelVal],
    tag: &'static str,
    expected: usize,
) -> Result<(), crate::LoopError> {
    let actual = items.len();
    if actual != expected {
        return Err(crate::LoopError::InvalidDecision {
            reason: format!("decision {tag} expected {expected} fields, got {actual}"),
        });
    }

    Ok(())
}

fn required_symbol(items: &[steel::SteelVal], index: usize) -> Result<&str, crate::LoopError> {
    match items.get(index) {
        Some(steel::SteelVal::SymbolV(symbol)) => Ok(symbol.as_str()),
        Some(other) => Err(crate::LoopError::InvalidDecision {
            reason: format!("expected symbol at index {index}, got {other:?}"),
        }),
        None => Err(crate::LoopError::InvalidDecision {
            reason: format!("missing symbol at index {index}"),
        }),
    }
}

fn required_string(items: &[steel::SteelVal], index: usize) -> Result<&str, crate::LoopError> {
    match items.get(index) {
        Some(steel::SteelVal::StringV(text)) => Ok(text.as_str()),
        Some(other) => Err(crate::LoopError::InvalidDecision {
            reason: format!("expected string at index {index}, got {other:?}"),
        }),
        None => Err(crate::LoopError::InvalidDecision {
            reason: format!("missing string at index {index}"),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_PROMPT: &str = "intent: build reliable systems";
    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const RESPONSE_MESSAGE: &str = "done";
    const BLANK_TEXT: &str = "   ";

    #[test]
    fn parses_model_decision() {
        let value = steel::list!(steel::SteelVal::SymbolV(MODEL_TAG.into()), MODEL_PROMPT);
        let decision = Decision::from_value(value).expect("model decision should parse");

        assert!(matches!(decision, Decision::CallModel { .. }));
    }

    #[test]
    fn parses_tool_decision() {
        let value = steel::list!(
            steel::SteelVal::SymbolV(TOOL_TAG.into()),
            TOOL_NAME,
            TOOL_INPUT
        );
        let decision = Decision::from_value(value).expect("tool decision should parse");

        assert!(matches!(decision, Decision::CallTool { .. }));
    }

    #[test]
    fn parses_response_decision() {
        let value = steel::list!(
            steel::SteelVal::SymbolV(RESPOND_TAG.into()),
            RESPONSE_MESSAGE
        );
        let decision = Decision::from_value(value).expect("response decision should parse");

        assert!(matches!(decision, Decision::Respond { .. }));
    }

    #[test]
    fn parses_stop_decision() {
        let value = steel::list!(steel::SteelVal::SymbolV(STOP_TAG.into()));
        let decision = Decision::from_value(value).expect("stop decision should parse");

        assert_eq!(decision, Decision::Stop);
    }

    #[test]
    fn rejects_invalid_shape() {
        let error = Decision::from_value(steel::SteelVal::BoolV(true))
            .expect_err("non-list decision should fail");

        assert!(matches!(error, crate::LoopError::InvalidDecision { .. }));
    }

    #[test]
    fn rejects_invalid_payload() {
        let value = steel::list!(steel::SteelVal::SymbolV(MODEL_TAG.into()), BLANK_TEXT);
        let error = Decision::from_value(value).expect_err("blank model prompt should fail");

        assert!(matches!(error, crate::LoopError::AdapterContract { .. }));
    }
}
