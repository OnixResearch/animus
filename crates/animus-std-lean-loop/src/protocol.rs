use std::collections::{BTreeMap, BTreeSet};

use serde::de::{Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

use crate::{
    ENVELOPE_SCHEMA_IDENTITY, LeanLoopError, LeanLoopFailureClass, LeanLoopProfile,
    PROTOCOL_IDENTITY,
};

const FIELD_RESPONSE: &str = "response";
const FIELD_SEQUENCE: &str = "sequence";
const FIELD_DECISION: &str = "decision";
const MAX_JSON_DEPTH: usize = 32;

#[derive(Clone, Debug, PartialEq)]
enum UniqueValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<Self>),
    Object(BTreeMap<String, Self>),
}

impl<'de> Deserialize<'de> for UniqueValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UniqueValueVisitor)
    }
}

struct UniqueValueVisitor;

impl<'de> Visitor<'de> for UniqueValueVisitor {
    type Value = UniqueValue;

    fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("a JSON value without duplicate object keys")
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(UniqueValue::Null)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(UniqueValue::Null)
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(UniqueValue::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(UniqueValue::Number(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(UniqueValue::Number(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        serde_json::Number::from_f64(value)
            .map(UniqueValue::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(UniqueValue::String(String::from(value)))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(UniqueValue::String(value))
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element::<UniqueValue>()? {
            values.push(value);
        }
        Ok(UniqueValue::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = BTreeMap::new();
        while let Some((key, value)) = map.next_entry::<String, UniqueValue>()? {
            if values.insert(key, value).is_some() {
                return Err(serde::de::Error::custom("duplicate JSON object key"));
            }
        }
        Ok(UniqueValue::Object(values))
    }
}

impl UniqueValue {
    fn depth(&self) -> usize {
        match self {
            Self::Array(values) => values
                .iter()
                .map(Self::depth)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
            Self::Object(values) => values
                .values()
                .map(Self::depth)
                .max()
                .unwrap_or(0)
                .saturating_add(1),
            _scalar => 1,
        }
    }

    fn object(self) -> Result<BTreeMap<String, Self>, LeanLoopError> {
        match self {
            Self::Object(value) => Ok(value),
            _other => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ToolArgumentCandidate {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ToolCandidate {
    pub name: String,
    pub schema_identity: String,
    pub arguments: Vec<ToolArgumentCandidate>,
    pub correlation_identity: String,
    pub visible_output_budget: u32,
    pub mutation_targets: Vec<String>,
    pub required_capabilities: Vec<String>,
    pub approval_posture: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LeanCandidateDecision {
    Model(String),
    Tool(ToolCandidate),
    Respond(String),
    Stop,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LeanResponse {
    HandshakeAck,
    Decision(LeanCandidateDecision),
    ShutdownAck,
    Error(String),
}

pub(crate) fn encode_handshake(profile: &LeanLoopProfile, artifact_identity: &str) -> String {
    serde_json::json!({
        "type": "handshake",
        "protocol": PROTOCOL_IDENTITY,
        "schema": ENVELOPE_SCHEMA_IDENTITY,
        "profile_identity": profile.profile_identity,
        "artifact_identity": artifact_identity,
    })
    .to_string()
}

#[derive(Clone, Copy)]
pub(crate) struct TurnFrame<'a> {
    pub sequence: u64,
    pub correlation_identity: &'a str,
    pub turn: &'a animus_runtime::RuntimeLoopTurn,
}

pub(crate) fn encode_turn(
    profile: &LeanLoopProfile,
    artifact_identity: &str,
    frame: TurnFrame<'_>,
) -> String {
    serde_json::json!({
        "type": "turn",
        "protocol": PROTOCOL_IDENTITY,
        "schema": ENVELOPE_SCHEMA_IDENTITY,
        "profile_identity": profile.profile_identity,
        "artifact_identity": artifact_identity,
        "sequence": frame.sequence,
        "correlation_identity": frame.correlation_identity,
        "summary": frame.turn.summary(),
        "last_model_message": frame.turn.last_model_message(),
        "last_tool_output": frame.turn.last_tool_output(),
    })
    .to_string()
}

pub(crate) fn encode_shutdown(
    profile: &LeanLoopProfile,
    artifact_identity: &str,
    sequence: u64,
) -> String {
    serde_json::json!({
        "type": "shutdown",
        "protocol": PROTOCOL_IDENTITY,
        "schema": ENVELOPE_SCHEMA_IDENTITY,
        "profile_identity": profile.profile_identity,
        "artifact_identity": artifact_identity,
        "sequence": sequence,
    })
    .to_string()
}

pub(crate) fn decode_response(
    line: &str,
    profile: &LeanLoopProfile,
    artifact_identity: &str,
    expected_sequence: u64,
) -> Result<LeanResponse, LeanLoopError> {
    if line.len() > profile.bounds.frame_bytes {
        return Err(protocol_error(LeanLoopFailureClass::FrameLimitExceeded));
    }
    let mut deserializer = serde_json::Deserializer::from_str(line);
    let value = UniqueValue::deserialize(&mut deserializer)
        .map_err(|_error| protocol_error(LeanLoopFailureClass::MalformedResponse))?;
    deserializer
        .end()
        .map_err(|_error| protocol_error(LeanLoopFailureClass::MalformedResponse))?;
    if value.depth() > MAX_JSON_DEPTH {
        return Err(protocol_error(LeanLoopFailureClass::MalformedResponse));
    }
    let object = value.object()?;
    require_string(&object, "protocol", PROTOCOL_IDENTITY)?;
    require_string(&object, "schema", ENVELOPE_SCHEMA_IDENTITY)?;
    require_string(&object, "profile_identity", &profile.profile_identity)?;
    require_string(&object, "artifact_identity", artifact_identity)?;
    let sequence = required_u64(&object, FIELD_SEQUENCE)?;
    let response_type = required_string(&object, "type")?;
    match response_type.as_str() {
        "handshake_ack" => {
            exact_keys(
                &object,
                &[
                    "type",
                    "protocol",
                    "schema",
                    "profile_identity",
                    "artifact_identity",
                    "sequence",
                ],
            )?;
            if sequence != 0 || expected_sequence != 0 {
                return Err(protocol_error(LeanLoopFailureClass::CorrelationMismatch));
            }
            Ok(LeanResponse::HandshakeAck)
        }
        "decision" => {
            if sequence != expected_sequence {
                return Err(protocol_error(LeanLoopFailureClass::CorrelationMismatch));
            }
            decode_decision(&object).map(LeanResponse::Decision)
        }
        "shutdown_ack" => {
            exact_keys(
                &object,
                &[
                    "type",
                    "protocol",
                    "schema",
                    "profile_identity",
                    "artifact_identity",
                    "sequence",
                ],
            )?;
            if sequence != expected_sequence {
                return Err(protocol_error(LeanLoopFailureClass::CorrelationMismatch));
            }
            Ok(LeanResponse::ShutdownAck)
        }
        "error" => {
            exact_keys(
                &object,
                &[
                    "type",
                    "protocol",
                    "schema",
                    "profile_identity",
                    "artifact_identity",
                    "sequence",
                    "error",
                ],
            )?;
            Ok(LeanResponse::Error(required_string(&object, "error")?))
        }
        _unknown => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
    }
}

fn decode_decision(
    object: &BTreeMap<String, UniqueValue>,
) -> Result<LeanCandidateDecision, LeanLoopError> {
    match required_string(object, FIELD_DECISION)?.as_str() {
        "model" => {
            exact_keys(object, &decision_keys(&["prompt"]))?;
            Ok(LeanCandidateDecision::Model(required_string(
                object, "prompt",
            )?))
        }
        "respond" => {
            exact_keys(object, &decision_keys(&["message"]))?;
            Ok(LeanCandidateDecision::Respond(required_string(
                object, "message",
            )?))
        }
        "stop" => {
            exact_keys(object, &decision_keys(&[]))?;
            Ok(LeanCandidateDecision::Stop)
        }
        "tool" => decode_tool(object).map(LeanCandidateDecision::Tool),
        _unknown => Err(protocol_error(LeanLoopFailureClass::UnknownDecision)),
    }
}

fn decode_tool(object: &BTreeMap<String, UniqueValue>) -> Result<ToolCandidate, LeanLoopError> {
    exact_keys(
        object,
        &decision_keys(&[
            "tool_name",
            "schema_identity",
            "arguments",
            "correlation_identity",
            "visible_output_budget",
            "mutation_targets",
            "required_capabilities",
            "approval_posture",
        ]),
    )?;
    let visible_output_budget = u32::try_from(required_u64(object, "visible_output_budget")?)
        .map_err(|_error| protocol_error(LeanLoopFailureClass::DecisionRejected))?;
    Ok(ToolCandidate {
        name: required_string(object, "tool_name")?,
        schema_identity: required_string(object, "schema_identity")?,
        arguments: required_arguments(object, "arguments")?,
        correlation_identity: required_string(object, "correlation_identity")?,
        visible_output_budget,
        mutation_targets: required_strings(object, "mutation_targets")?,
        required_capabilities: required_strings(object, "required_capabilities")?,
        approval_posture: required_string(object, "approval_posture")?,
    })
}

fn decision_keys(extra: &[&'static str]) -> Vec<&'static str> {
    let mut keys = vec![
        "type",
        "protocol",
        "schema",
        "profile_identity",
        "artifact_identity",
        "sequence",
        "decision",
    ];
    keys.extend_from_slice(extra);
    keys
}

fn required_arguments(
    object: &BTreeMap<String, UniqueValue>,
    field: &'static str,
) -> Result<Vec<ToolArgumentCandidate>, LeanLoopError> {
    let values = required_array(object, field)?;
    let mut arguments = Vec::with_capacity(values.len());
    for value in values {
        let UniqueValue::Object(argument) = value else {
            return Err(protocol_error(LeanLoopFailureClass::MalformedResponse));
        };
        exact_keys(argument, &["name", "value"])?;
        arguments.push(ToolArgumentCandidate {
            name: required_string(argument, "name")?,
            value: required_string(argument, "value")?,
        });
    }
    Ok(arguments)
}

fn required_strings(
    object: &BTreeMap<String, UniqueValue>,
    field: &'static str,
) -> Result<Vec<String>, LeanLoopError> {
    required_array(object, field)?
        .iter()
        .map(|value| match value {
            UniqueValue::String(text) => Ok(text.clone()),
            _other => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
        })
        .collect()
}

fn required_array<'a>(
    object: &'a BTreeMap<String, UniqueValue>,
    field: &'static str,
) -> Result<&'a [UniqueValue], LeanLoopError> {
    match object.get(field) {
        Some(UniqueValue::Array(values)) => Ok(values.as_slice()),
        _other => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
    }
}

fn required_string(
    object: &BTreeMap<String, UniqueValue>,
    field: &'static str,
) -> Result<String, LeanLoopError> {
    match object.get(field) {
        Some(UniqueValue::String(value)) => Ok(value.clone()),
        _other => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
    }
}

fn require_string(
    object: &BTreeMap<String, UniqueValue>,
    field: &'static str,
    expected: &str,
) -> Result<(), LeanLoopError> {
    if required_string(object, field)? == expected {
        Ok(())
    } else {
        Err(protocol_error(LeanLoopFailureClass::CorrelationMismatch))
    }
}

fn required_u64(
    object: &BTreeMap<String, UniqueValue>,
    field: &'static str,
) -> Result<u64, LeanLoopError> {
    match object.get(field) {
        Some(UniqueValue::Number(value)) => value
            .as_u64()
            .ok_or_else(|| protocol_error(LeanLoopFailureClass::MalformedResponse)),
        _other => Err(protocol_error(LeanLoopFailureClass::MalformedResponse)),
    }
}

fn exact_keys(
    object: &BTreeMap<String, UniqueValue>,
    expected: &[&str],
) -> Result<(), LeanLoopError> {
    let actual = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(protocol_error(LeanLoopFailureClass::MalformedResponse))
    }
}

const fn protocol_error(failure: LeanLoopFailureClass) -> LeanLoopError {
    LeanLoopError::new(failure, FIELD_RESPONSE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::profile;

    const ARTIFACT: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn duplicate_and_unknown_fields_fail_closed() {
        let duplicate = format!(
            "{{\"type\":\"decision\",\"type\":\"decision\",\"protocol\":\"{PROTOCOL_IDENTITY}\",\"schema\":\"{ENVELOPE_SCHEMA_IDENTITY}\",\"profile_identity\":\"{}\",\"artifact_identity\":\"{ARTIFACT}\",\"sequence\":1,\"decision\":\"stop\"}}",
            profile().profile_identity
        );
        assert_eq!(
            decode_response(duplicate.as_str(), &profile(), ARTIFACT, 1)
                .expect_err("duplicate denied")
                .failure(),
            LeanLoopFailureClass::MalformedResponse
        );

        let unknown = serde_json::json!({
            "type": "decision",
            "protocol": PROTOCOL_IDENTITY,
            "schema": ENVELOPE_SCHEMA_IDENTITY,
            "profile_identity": profile().profile_identity,
            "artifact_identity": ARTIFACT,
            "sequence": 1,
            "decision": "stop",
            "extra": true,
        })
        .to_string();
        assert!(decode_response(unknown.as_str(), &profile(), ARTIFACT, 1).is_err());
    }

    #[test]
    fn stale_sequence_and_legacy_tool_shape_fail_closed() {
        let stop = serde_json::json!({
            "type": "decision",
            "protocol": PROTOCOL_IDENTITY,
            "schema": ENVELOPE_SCHEMA_IDENTITY,
            "profile_identity": profile().profile_identity,
            "artifact_identity": ARTIFACT,
            "sequence": 1,
            "decision": "stop",
        })
        .to_string();
        assert_eq!(
            decode_response(stop.as_str(), &profile(), ARTIFACT, 2)
                .expect_err("stale sequence denied")
                .failure(),
            LeanLoopFailureClass::CorrelationMismatch
        );

        let legacy = serde_json::json!({
            "type": "decision",
            "protocol": PROTOCOL_IDENTITY,
            "schema": ENVELOPE_SCHEMA_IDENTITY,
            "profile_identity": profile().profile_identity,
            "artifact_identity": ARTIFACT,
            "sequence": 1,
            "decision": "tool",
            "tool_name": "search",
            "raw_input": "query",
        })
        .to_string();
        assert!(decode_response(legacy.as_str(), &profile(), ARTIFACT, 1).is_err());
    }
}
