import AnimusLean.Protocol

open AnimusLean

private def fixtureProfile : Profile := {
  maxTextBytes := 8192
  maxArguments := 32
  maxTargets := 32
  maxCapabilities := 32
}

private def fixtureBudget : Budget := {
  remainingSteps := 4
  remainingPayloadBytes := 16384
}

private def fixtureTurn : Turn := {
  sequence := 1
  correlationIdentity := "turn-1"
  summary := "build reliable systems"
  lastModelMessage := none
  lastToolOutput := none
}

private def require (condition : Bool) (message : String) : IO Unit :=
  unless condition do throw (IO.userError message)

private def exceptEq [BEq error] [BEq value] : Except error value → Except error value → Bool
  | .ok left, .ok right => left == right
  | .error left, .error right => left == right
  | _, _ => false

private def handshakeJson : String :=
  "{\"type\":\"handshake\",\"protocol\":\"animus.lean-loop.protocol.v1\",\"schema\":\"animus.lean-loop.envelope.v1\",\"profile_identity\":\"profile.fixture\",\"artifact_identity\":\"artifact.fixture\"}"

private def turnJson : String :=
  "{\"type\":\"turn\",\"protocol\":\"animus.lean-loop.protocol.v1\",\"schema\":\"animus.lean-loop.envelope.v1\",\"profile_identity\":\"profile.fixture\",\"artifact_identity\":\"artifact.fixture\",\"sequence\":1,\"correlation_identity\":\"turn-1\",\"summary\":\"build reliable systems\",\"last_model_message\":null,\"last_tool_output\":null}"

private def malformedTurnJson : String :=
  "{\"type\":\"turn\",\"protocol\":\"unsupported\",\"schema\":\"animus.lean-loop.envelope.v1\",\"profile_identity\":\"profile.fixture\",\"artifact_identity\":\"artifact.fixture\",\"sequence\":1,\"correlation_identity\":\"turn-1\",\"summary\":\"build reliable systems\",\"last_model_message\":null,\"last_tool_output\":null}"

private def unknownFieldTurnJson : String :=
  "{\"type\":\"turn\",\"protocol\":\"animus.lean-loop.protocol.v1\",\"schema\":\"animus.lean-loop.envelope.v1\",\"profile_identity\":\"profile.fixture\",\"artifact_identity\":\"artifact.fixture\",\"sequence\":1,\"correlation_identity\":\"turn-1\",\"summary\":\"build reliable systems\",\"last_model_message\":null,\"last_tool_output\":null,\"unknown\":true}"

public def main : IO UInt32 := do
  let initial : FabricState := { phase := .initial, budget := fixtureBudget }
  let first := step fixtureProfile initial fixtureTurn
  require (exceptEq first (.ok ({ phase := .waitingModel, budget := consumeStep fixtureBudget },
    .model fixtureTurn.summary))) "initial model transition failed"
  require (exceptEq (step fixtureProfile initial fixtureTurn) first) "deterministic replay failed"

  let modelTurn := { fixtureTurn with sequence := 2, lastModelMessage := some "model answer" }
  let second := step fixtureProfile { phase := .waitingModel, budget := fixtureBudget } modelTurn
  require (exceptEq second (.ok ({ phase := .terminal, budget := consumeStep fixtureBudget },
    .respond "model answer"))) "model-then-response transition failed"

  require (exceptEq (validateDecision fixtureProfile (.tool validToolFixture)) (.ok ()))
    "current-shape tool fixture failed"
  require (exceptEq (validateDecision fixtureProfile .stop) (.ok ())) "stop decision failed"
  require (exceptEq (validateDecision fixtureProfile (.model "")) (.error .invalidDecision))
    "empty model prompt was accepted"
  require (exceptEq (validateTool fixtureProfile { validToolFixture with approvalPosture := "legacy" })
    (.error .invalidDecision)) "legacy approval posture was accepted"
  require (exceptEq (step fixtureProfile { phase := .terminal, budget := fixtureBudget } fixtureTurn)
    (.error .terminalState)) "terminal state emitted an effect"
  require (exceptEq (step fixtureProfile {
      phase := .initial,
      budget := { fixtureBudget with remainingSteps := 0 }
    } fixtureTurn) (.error .budgetExhausted)) "exhausted budget emitted an effect"

  let handshake ← match decodeRequest handshakeJson with
    | .ok (.handshake value) => pure value
    | _ => throw (IO.userError "handshake fixture failed")
  let protocolState := initialProtocolState handshake fixtureBudget
  let envelope ← match decodeRequest turnJson with
    | .ok (.turn value) => pure value
    | _ => throw (IO.userError "turn fixture failed")
  let handled := handleTurn fixtureProfile protocolState envelope
  require (handled.isOk) "valid protocol turn failed"
  require (exceptEq (decodeRequest malformedTurnJson) (.error .unsupportedVersion))
    "unsupported protocol was accepted"
  require (exceptEq (handleTurn fixtureProfile { protocolState with lastSequence := 1 } envelope)
    (.error .staleSequence)) "stale protocol sequence was accepted"
  require (exceptEq (decodeRequest unknownFieldTurnJson) (.error .unknownFields))
    "unknown protocol field was accepted"

  IO.println "animus Lean core/protocol tests: passed"
  return 0
