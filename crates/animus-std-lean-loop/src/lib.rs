//! Optional bounded process-backed Lean 4 implementation of `animus_runtime::LoopFabric`.
//!
//! The child returns untrusted candidate decisions. Rust reconstructs them through current
//! adapter contracts before the existing runtime can dispatch any model or tool effect.

#![forbid(unsafe_code)]

mod core;
mod model;
mod protocol;
mod transport;

pub use core::{load_generated_profile, project_receipt};
pub use model::{
    ARTIFACT_DOMAIN, ENVELOPE_SCHEMA_IDENTITY, LeanCleanupStatus, LeanLaunchDescriptor,
    LeanLoopBounds, LeanLoopError, LeanLoopFailureClass, LeanLoopProfile, LeanLoopReceipt,
    LeanReceiptContext, LeanSandboxObservation, LeanSandboxPosture, LeanTransportObservation,
    NON_CLAIMS, PROFILE_SCHEMA, PROTOCOL_IDENTITY, REDACTION_MARKER, TOOLCHAIN_COHORT,
};
pub use transport::{LeanTransport, StdioLeanProcess, artifact_identity};

use crate::protocol::{
    LeanResponse, TurnFrame, decode_response, encode_handshake, encode_shutdown, encode_turn,
};

const FIELD_HANDSHAKE: &str = "handshake";
const FIELD_SEQUENCE: &str = "sequence";
const FIELD_SHUTDOWN: &str = "shutdown";
const TURN_CORRELATION_DOMAIN: &str = "animus.lean-loop.turn-correlation.v1";

/// Activated Lean child behind the neutral runtime loop-fabric contract.
pub struct LeanLoopFabric<Transport = StdioLeanProcess> {
    transport: Transport,
    profile: LeanLoopProfile,
    artifact_identity: String,
    sequence: u64,
    terminal: bool,
    last_decision_class: String,
}

impl LeanLoopFabric<StdioLeanProcess> {
    /// Verifies and launches an explicitly selected Lean executable, then performs the handshake.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] when launch policy, artifact identity, child creation, or the
    /// bounded handshake fails.
    pub fn launch(
        profile: LeanLoopProfile,
        descriptor: LeanLaunchDescriptor,
    ) -> Result<Self, LeanLoopError> {
        let (transport, artifact_identity) = StdioLeanProcess::launch(profile.clone(), descriptor)?;
        Self::activate(transport, profile, artifact_identity)
    }
}

impl<Transport> LeanLoopFabric<Transport>
where
    Transport: LeanTransport,
{
    /// Activates an injected bounded transport with the exact reviewed handshake.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] when the transport or handshake response fails closed.
    pub fn activate(
        mut transport: Transport,
        profile: LeanLoopProfile,
        artifact_identity: String,
    ) -> Result<Self, LeanLoopError> {
        let frame = encode_handshake(&profile, artifact_identity.as_str());
        let response = transport.exchange(frame.as_str(), profile.bounds.startup_poll_count)?;
        match decode_response(response.as_str(), &profile, artifact_identity.as_str(), 0)? {
            LeanResponse::HandshakeAck => Ok(Self {
                transport,
                profile,
                artifact_identity,
                sequence: 0,
                terminal: false,
                last_decision_class: String::from("none"),
            }),
            _other => Err(LeanLoopError::new(
                LeanLoopFailureClass::HandshakeFailure,
                FIELD_HANDSHAKE,
            )),
        }
    }

    /// Sends one correlated shutdown frame and performs bounded cleanup.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] when shutdown correlation, transport, exit, or cleanup fails.
    pub fn shutdown(&mut self) -> Result<LeanCleanupStatus, LeanLoopError> {
        if self.terminal {
            return self.transport.cleanup();
        }
        let sequence = self.next_sequence()?;
        let frame = encode_shutdown(&self.profile, self.artifact_identity.as_str(), sequence);
        let response = self
            .transport
            .exchange(frame.as_str(), self.profile.bounds.shutdown_poll_count)?;
        match decode_response(
            response.as_str(),
            &self.profile,
            self.artifact_identity.as_str(),
            sequence,
        )? {
            LeanResponse::ShutdownAck => {
                self.terminal = true;
                self.transport.cleanup()
            }
            _other => Err(LeanLoopError::new(
                LeanLoopFailureClass::MalformedResponse,
                FIELD_SHUTDOWN,
            )),
        }
    }

    /// Returns the current metadata-only transport observation.
    #[must_use]
    pub fn transport_observation(&self) -> LeanTransportObservation {
        self.transport.observation()
    }

    /// Projects a deterministic metadata-only receipt from current safe observations.
    ///
    /// # Errors
    ///
    /// Returns [`LeanLoopError`] if any receipt identity or class is unsafe.
    pub fn receipt(&self) -> Result<LeanLoopReceipt, LeanLoopError> {
        let observation = self.transport.observation();
        crate::project_receipt(
            &self.profile,
            crate::LeanReceiptContext {
                artifact_identity: self.artifact_identity.as_str(),
                decision_class: self.last_decision_class.as_str(),
                sandbox: LeanSandboxObservation::FixtureOnly,
            },
            &observation,
        )
    }

    fn next_sequence(&mut self) -> Result<u64, LeanLoopError> {
        let next = self.sequence.saturating_add(1);
        if next > self.profile.bounds.sequence_limit {
            self.terminal = true;
            return Err(LeanLoopError::new(
                LeanLoopFailureClass::CorrelationMismatch,
                FIELD_SEQUENCE,
            ));
        }
        self.sequence = next;
        Ok(next)
    }
}

impl<Transport> animus_runtime::LoopFabric for LeanLoopFabric<Transport>
where
    Transport: LeanTransport,
{
    type Error = LeanLoopError;

    fn decide(
        &mut self,
        turn: &animus_runtime::RuntimeLoopTurn,
    ) -> Result<animus_runtime::RuntimeLoopDecision, Self::Error> {
        if self.terminal {
            return Err(LeanLoopError::new(
                LeanLoopFailureClass::TerminalTransport,
                FIELD_SEQUENCE,
            ));
        }
        let sequence = self.next_sequence()?;
        let correlation_identity = turn_correlation_identity(sequence, turn);
        let frame = encode_turn(
            &self.profile,
            self.artifact_identity.as_str(),
            TurnFrame {
                sequence,
                correlation_identity: correlation_identity.as_str(),
                turn,
            },
        );
        let response = match self
            .transport
            .exchange(frame.as_str(), self.profile.bounds.response_poll_count)
        {
            Ok(response) => response,
            Err(error) => {
                self.terminal = true;
                return Err(error);
            }
        };
        let candidate = match decode_response(
            response.as_str(),
            &self.profile,
            self.artifact_identity.as_str(),
            sequence,
        )? {
            LeanResponse::Decision(candidate) => candidate,
            LeanResponse::Error(_safe_class) => {
                self.terminal = true;
                return Err(LeanLoopError::new(
                    LeanLoopFailureClass::DecisionRejected,
                    FIELD_SEQUENCE,
                ));
            }
            _other => {
                self.terminal = true;
                return Err(LeanLoopError::new(
                    LeanLoopFailureClass::MalformedResponse,
                    FIELD_SEQUENCE,
                ));
            }
        };
        let decision = match crate::core::reconstruct_decision(
            &self.profile,
            correlation_identity.as_str(),
            candidate,
        ) {
            Ok(decision) => decision,
            Err(error) => {
                self.terminal = true;
                return Err(error);
            }
        };
        self.last_decision_class = String::from(decision_class(&decision));
        Ok(decision)
    }
}

fn turn_correlation_identity(sequence: u64, turn: &animus_runtime::RuntimeLoopTurn) -> String {
    let body = format!(
        "{sequence}\0{}\0{}\0{}",
        turn.summary(),
        turn.last_model_message().unwrap_or("none"),
        turn.last_tool_output().unwrap_or("none")
    );
    crate::core::domain_hash(TURN_CORRELATION_DOMAIN, body.as_bytes())
}

const fn decision_class(decision: &animus_runtime::RuntimeLoopDecision) -> &'static str {
    match decision {
        animus_runtime::RuntimeLoopDecision::CallModel { .. } => "model",
        animus_runtime::RuntimeLoopDecision::CallTool { .. } => "tool",
        animus_runtime::RuntimeLoopDecision::Respond { .. } => "respond",
        animus_runtime::RuntimeLoopDecision::CompletionCandidate { .. } => "completion_candidate",
        animus_runtime::RuntimeLoopDecision::Stop => "stop",
    }
}

#[cfg(test)]
pub(crate) mod test_support {
    use std::collections::VecDeque;

    use super::*;

    pub(crate) const ARTIFACT_IDENTITY: &str =
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const PROFILE_IDENTITY: &str =
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const PROOF_IDENTITY: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

    pub(crate) fn profile() -> LeanLoopProfile {
        LeanLoopProfile {
            profile_identity: String::from(PROFILE_IDENTITY),
            proof_source_identity: String::from(PROOF_IDENTITY),
            bounds: LeanLoopBounds {
                frame_bytes: 65_536,
                text_bytes: 8_192,
                sequence_limit: 64,
                diagnostic_bytes: 4_096,
                startup_poll_count: 4,
                response_poll_count: 4,
                shutdown_poll_count: 4,
                poll_interval: std::time::Duration::from_millis(1),
                cleanup_poll_count: 4,
                argument_count: 8,
                target_count: 8,
                capability_count: 8,
            },
            sandbox_posture: LeanSandboxPosture::FixtureOnly,
        }
    }

    pub(crate) struct FakeTransport {
        responses: VecDeque<Result<String, LeanLoopError>>,
        observation: LeanTransportObservation,
    }

    impl FakeTransport {
        pub(crate) fn new(responses: Vec<Result<String, LeanLoopError>>) -> Self {
            Self {
                responses: responses.into(),
                observation: LeanTransportObservation {
                    frame_count: 0,
                    byte_count: 0,
                    failure: None,
                    cleanup: LeanCleanupStatus::Completed,
                },
            }
        }
    }

    impl LeanTransport for FakeTransport {
        fn exchange(&mut self, frame: &str, _poll_count: u32) -> Result<String, LeanLoopError> {
            self.observation.frame_count = self.observation.frame_count.saturating_add(1);
            self.observation.byte_count = self
                .observation
                .byte_count
                .saturating_add(u64::try_from(frame.len()).unwrap_or(u64::MAX));
            self.responses.pop_front().unwrap_or_else(|| {
                Err(LeanLoopError::new(
                    LeanLoopFailureClass::InputEof,
                    "fixture",
                ))
            })
        }

        fn cleanup(&mut self) -> Result<LeanCleanupStatus, LeanLoopError> {
            Ok(LeanCleanupStatus::Completed)
        }

        fn observation(&self) -> LeanTransportObservation {
            self.observation.clone()
        }
    }

    pub(crate) fn response(sequence: u64, decision: &serde_json::Value) -> String {
        let mut value = serde_json::json!({
            "type": "decision",
            "protocol": PROTOCOL_IDENTITY,
            "schema": ENVELOPE_SCHEMA_IDENTITY,
            "profile_identity": PROFILE_IDENTITY,
            "artifact_identity": ARTIFACT_IDENTITY,
            "sequence": sequence,
        });
        let object = value.as_object_mut().expect("fixture object");
        for (key, value) in decision.as_object().expect("fixture decision") {
            object.insert(key.clone(), value.clone());
        }
        value.to_string()
    }

    pub(crate) fn handshake_response() -> String {
        serde_json::json!({
            "type": "handshake_ack",
            "protocol": PROTOCOL_IDENTITY,
            "schema": ENVELOPE_SCHEMA_IDENTITY,
            "profile_identity": PROFILE_IDENTITY,
            "artifact_identity": ARTIFACT_IDENTITY,
            "sequence": 0,
        })
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use animus_runtime::LoopFabric;

    use super::test_support::*;
    use super::*;

    const SUMMARY: &str = "build reliable systems";
    const MODEL_MESSAGE: &str = "model answer";
    const EXPECTED_MODEL_CALLS: usize = 1;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct AdapterFailure;

    impl std::fmt::Display for AdapterFailure {
        fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("fixture adapter failure")
        }
    }

    #[derive(Default)]
    struct ModelStub {
        calls: usize,
    }

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
        > for ModelStub
    {
        type Error = AdapterFailure;

        fn call(
            &mut self,
            request: &animus_adapters::ModelRequest,
        ) -> Result<animus_adapters::ModelResponse, Self::Error> {
            assert_eq!(request.prompt(), SUMMARY);
            self.calls = self.calls.saturating_add(EXPECTED_MODEL_CALLS);
            animus_adapters::ModelResponse::new(MODEL_MESSAGE).map_err(|_error| AdapterFailure)
        }
    }

    struct ToolStub;

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ToolRequest,
            animus_adapters::ToolResponse,
        > for ToolStub
    {
        type Error = AdapterFailure;

        fn call(
            &mut self,
            _request: &animus_adapters::ToolRequest,
        ) -> Result<animus_adapters::ToolResponse, Self::Error> {
            Err(AdapterFailure)
        }
    }

    #[test]
    fn fake_transport_drives_existing_runtime_model_path() {
        let transport = FakeTransport::new(vec![
            Ok(handshake_response()),
            Ok(response(
                1,
                &serde_json::json!({"decision": "model", "prompt": SUMMARY}),
            )),
            Ok(response(
                2,
                &serde_json::json!({"decision": "respond", "message": MODEL_MESSAGE}),
            )),
        ]);
        let mut fabric =
            LeanLoopFabric::activate(transport, profile(), String::from(ARTIFACT_IDENTITY))
                .expect("activate");
        let mut model = ModelStub::default();
        let mut tool = ToolStub;
        let report = animus_runtime::run_agent_loop(
            &animus_runtime::RuntimeInput::new(SUMMARY).expect("runtime input"),
            animus_runtime::RuntimeConfig::default(),
            &mut fabric,
            animus_runtime::RuntimeAdapters::new(&mut model, &mut tool),
        );

        assert_eq!(model.calls, EXPECTED_MODEL_CALLS);
        assert!(matches!(
            report.status(),
            animus_runtime::RuntimeStatus::FinalResponse { .. }
        ));
        assert!(
            report
                .events()
                .iter()
                .any(|event| event.kind() == animus_runtime::RuntimeEventKind::ModelRequested)
        );
        assert!(
            report
                .events()
                .iter()
                .any(|event| event.kind() == animus_runtime::RuntimeEventKind::FinalResponse)
        );
    }

    #[test]
    fn existing_runtime_retains_max_turn_enforcement() {
        let transport = FakeTransport::new(vec![
            Ok(handshake_response()),
            Ok(response(
                1,
                &serde_json::json!({"decision": "model", "prompt": SUMMARY}),
            )),
        ]);
        let mut fabric =
            LeanLoopFabric::activate(transport, profile(), String::from(ARTIFACT_IDENTITY))
                .expect("activate");
        let mut model = ModelStub::default();
        let mut tool = ToolStub;
        let report = animus_runtime::run_agent_loop(
            &animus_runtime::RuntimeInput::new(SUMMARY).expect("runtime input"),
            animus_runtime::RuntimeConfig::new(animus_runtime::MIN_MAX_TURNS),
            &mut fabric,
            animus_runtime::RuntimeAdapters::new(&mut model, &mut tool),
        );

        assert_eq!(model.calls, EXPECTED_MODEL_CALLS);
        assert!(matches!(
            report.status(),
            animus_runtime::RuntimeStatus::MaxTurnsExceeded { .. }
        ));
        assert!(
            report
                .events()
                .iter()
                .any(|event| event.kind() == animus_runtime::RuntimeEventKind::MaxTurnsExceeded)
        );
    }

    #[test]
    fn fake_transport_drives_model_then_response_through_neutral_trait() {
        let transport = FakeTransport::new(vec![
            Ok(handshake_response()),
            Ok(response(
                1,
                &serde_json::json!({"decision": "model", "prompt": SUMMARY}),
            )),
            Ok(response(
                2,
                &serde_json::json!({"decision": "respond", "message": MODEL_MESSAGE}),
            )),
        ]);
        let mut fabric =
            LeanLoopFabric::activate(transport, profile(), String::from(ARTIFACT_IDENTITY))
                .expect("activate");
        let initial = animus_runtime::RuntimeLoopTurn::new(SUMMARY, None, None).expect("turn");
        let next = fabric.decide(&initial).expect("model decision");
        assert!(matches!(
            next,
            animus_runtime::RuntimeLoopDecision::CallModel { .. }
        ));
        let model_turn = animus_runtime::RuntimeLoopTurn::new(SUMMARY, Some(MODEL_MESSAGE), None)
            .expect("model turn");
        let final_decision = fabric.decide(&model_turn).expect("response decision");
        assert!(matches!(
            final_decision,
            animus_runtime::RuntimeLoopDecision::Respond { .. }
        ));
    }

    #[test]
    fn transport_failure_is_terminal_without_replay_or_fallback() {
        let transport = FakeTransport::new(vec![
            Ok(handshake_response()),
            Err(LeanLoopError::new(
                LeanLoopFailureClass::ResponseDeadline,
                "fixture",
            )),
        ]);
        let mut fabric =
            LeanLoopFabric::activate(transport, profile(), String::from(ARTIFACT_IDENTITY))
                .expect("activate");
        let turn = animus_runtime::RuntimeLoopTurn::new(SUMMARY, None, None).expect("turn");
        assert_eq!(
            fabric.decide(&turn).expect_err("deadline denied").failure(),
            LeanLoopFailureClass::ResponseDeadline
        );
        assert_eq!(
            fabric
                .decide(&turn)
                .expect_err("terminal transport denies replay")
                .failure(),
            LeanLoopFailureClass::TerminalTransport
        );
    }
}
