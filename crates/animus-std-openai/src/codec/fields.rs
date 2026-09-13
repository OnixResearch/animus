#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum OutputItemKind {
    Message,
    Reasoning,
    FunctionCall,
}

impl OutputItemKind {
    pub(super) fn parse(value: &str) -> Result<Self, super::OpenAiStreamError> {
        match value {
            "message" => Ok(Self::Message),
            "reasoning" => Ok(Self::Reasoning),
            "function_call" => Ok(Self::FunctionCall),
            _unsupported => Err(super::OpenAiStreamError::new(
                super::OpenAiStreamFailureClass::UnsupportedEvent,
                super::vocabulary::UNKNOWN_EVENT_REASON,
            )),
        }
    }
}

#[derive(serde::Deserialize)]
pub(super) struct EventEnvelope {
    #[serde(rename = "type")]
    pub(super) kind: String,
    pub(super) sequence_number: u64,
    #[serde(flatten)]
    pub(super) value: serde_json::Value,
}

pub(super) fn required_object<'a>(
    value: &'a serde_json::Value,
    key: &str,
) -> Result<&'a serde_json::Map<String, serde_json::Value>, super::OpenAiStreamError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| super::contracts::malformed(super::vocabulary::MALFORMED_SSE_REASON))
}

pub(super) trait JsonFields {
    fn field(&self, key: &str) -> Option<&serde_json::Value>;
}

impl JsonFields for serde_json::Value {
    fn field(&self, key: &str) -> Option<&serde_json::Value> {
        self.get(key)
    }
}

impl JsonFields for serde_json::Map<String, serde_json::Value> {
    fn field(&self, key: &str) -> Option<&serde_json::Value> {
        self.get(key)
    }
}

pub(super) fn required_string<'a, T: JsonFields + ?Sized>(
    value: &'a T,
    key: &str,
) -> Result<&'a str, super::OpenAiStreamError> {
    value
        .field(key)
        .and_then(serde_json::Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| super::contracts::malformed(super::vocabulary::MALFORMED_SSE_REASON))
}

pub(super) fn required_u32(
    value: &serde_json::Value,
    key: &str,
) -> Result<u32, super::OpenAiStreamError> {
    let raw = value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| super::contracts::malformed(super::vocabulary::MALFORMED_SSE_REASON))?;
    u32::try_from(raw).map_err(|_error| super::contracts::bound_error())
}
