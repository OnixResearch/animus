import Lean.Data.Json
import Lean.Data.Json.FromToJson
import AnimusLean.Core

namespace AnimusLean

open Lean

public def protocolVersion : String := "animus.lean-loop.protocol.v1"
public def protocolSchema : String := "animus.lean-loop.envelope.v1"

public structure Handshake where
  profileIdentity : String
  artifactIdentity : String
  deriving Repr, DecidableEq

public structure TurnEnvelope where
  profileIdentity : String
  artifactIdentity : String
  turn : Turn
  deriving Repr, DecidableEq

public structure Shutdown where
  profileIdentity : String
  artifactIdentity : String
  sequence : Nat
  deriving Repr, DecidableEq

public inductive Request where
  | handshake (value : Handshake)
  | turn (value : TurnEnvelope)
  | shutdown (value : Shutdown)
  deriving Repr, DecidableEq

public structure ProtocolState where
  profileIdentity : String
  artifactIdentity : String
  lastSequence : Nat
  fabricState : FabricState
  deriving Repr, DecidableEq

public inductive ProtocolError where
  | malformed
  | unsupportedVersion
  | unsupportedSchema
  | unknownFields
  | identityMismatch
  | staleSequence
  | kernel (error : FabricError)
  deriving Repr, DecidableEq

deriving instance BEq for Handshake
deriving instance BEq for TurnEnvelope
deriving instance BEq for Shutdown
deriving instance BEq for Request
deriving instance BEq for ProtocolState
deriving instance BEq for ProtocolError

private def requiredString (json : Json) (field : String) : Except ProtocolError String :=
  match json.getObjVal? field >>= Json.getStr? with
  | .ok value => .ok value
  | .error _ => .error .malformed

private def requiredNat (json : Json) (field : String) : Except ProtocolError Nat :=
  match json.getObjVal? field >>= Json.getNat? with
  | .ok value => .ok value
  | .error _ => .error .malformed

private def optionalString (json : Json) (field : String) : Except ProtocolError (Option String) :=
  match json.getObjVal? field with
  | .ok .null => .ok none
  | .ok value =>
      match value.getStr? with
      | .ok text => .ok (some text)
      | .error _ => .error .malformed
  | .error _ => .error .malformed

private def requireObjectSize (json : Json) (expected : Nat) : Except ProtocolError Unit :=
  match json.getObj? with
  | .ok object => if object.size = expected then .ok () else .error .unknownFields
  | .error _ => .error .malformed

private def validateHeader (json : Json) : Except ProtocolError Unit := do
  if (← requiredString json "protocol") != protocolVersion then throw .unsupportedVersion
  if (← requiredString json "schema") != protocolSchema then throw .unsupportedSchema

private def decodeHandshake (json : Json) : Except ProtocolError Request := do
  requireObjectSize json 5
  validateHeader json
  let profileIdentity ← requiredString json "profile_identity"
  let artifactIdentity ← requiredString json "artifact_identity"
  if profileIdentity.trimAscii.isEmpty || artifactIdentity.trimAscii.isEmpty then throw .malformed
  return .handshake { profileIdentity, artifactIdentity }

private def decodeTurn (json : Json) : Except ProtocolError Request := do
  requireObjectSize json 10
  validateHeader json
  let profileIdentity ← requiredString json "profile_identity"
  let artifactIdentity ← requiredString json "artifact_identity"
  let sequence ← requiredNat json "sequence"
  let correlationIdentity ← requiredString json "correlation_identity"
  let summary ← requiredString json "summary"
  let lastModelMessage ← optionalString json "last_model_message"
  let lastToolOutput ← optionalString json "last_tool_output"
  return .turn {
    profileIdentity,
    artifactIdentity,
    turn := { sequence, correlationIdentity, summary, lastModelMessage, lastToolOutput }
  }

private def decodeShutdown (json : Json) : Except ProtocolError Request := do
  requireObjectSize json 6
  validateHeader json
  return .shutdown {
    profileIdentity := ← requiredString json "profile_identity",
    artifactIdentity := ← requiredString json "artifact_identity",
    sequence := ← requiredNat json "sequence"
  }

public def decodeRequest (line : String) : Except ProtocolError Request := do
  let json ← match Json.parse line with
    | .ok value => .ok value
    | .error _ => .error .malformed
  match ← requiredString json "type" with
  | "handshake" => decodeHandshake json
  | "turn" => decodeTurn json
  | "shutdown" => decodeShutdown json
  | _ => .error .malformed

private def optionalJson : Option String → Json
  | none => .null
  | some value => toJson value

private def decisionFields : Decision → List (String × Json)
  | .model prompt => [("decision", toJson "model"), ("prompt", toJson prompt)]
  | .respond message => [("decision", toJson "respond"), ("message", toJson message)]
  | .stop => [("decision", toJson "stop")]
  | .tool request => [
      ("decision", toJson "tool"),
      ("tool_name", toJson request.name),
      ("schema_identity", toJson request.schemaIdentity),
      ("arguments", toJson (request.arguments.map fun argument =>
        Json.mkObj [("name", toJson argument.name), ("value", toJson argument.value)])),
      ("correlation_identity", toJson request.correlationIdentity),
      ("visible_output_budget", toJson request.visibleOutputBudget),
      ("mutation_targets", toJson request.mutationTargets),
      ("required_capabilities", toJson request.requiredCapabilities),
      ("approval_posture", toJson request.approvalPosture)
    ]

private def baseResponseFields
    (kind profileIdentity artifactIdentity : String) (sequence : Nat) : List (String × Json) := [
  ("type", toJson kind),
  ("protocol", toJson protocolVersion),
  ("schema", toJson protocolSchema),
  ("profile_identity", toJson profileIdentity),
  ("artifact_identity", toJson artifactIdentity),
  ("sequence", toJson sequence)
]

public def encodeHandshakeAck (value : Handshake) : String :=
  (Json.mkObj (baseResponseFields "handshake_ack" value.profileIdentity value.artifactIdentity 0)).compress

public def encodeDecision
    (profileIdentity artifactIdentity : String) (sequence : Nat) (decision : Decision) : String :=
  (Json.mkObj (baseResponseFields "decision" profileIdentity artifactIdentity sequence ++
    decisionFields decision)).compress

public def encodeShutdownAck (value : Shutdown) : String :=
  (Json.mkObj (baseResponseFields "shutdown_ack" value.profileIdentity value.artifactIdentity
    value.sequence)).compress

public def encodeError
    (profileIdentity artifactIdentity : String) (sequence : Nat) (error : ProtocolError) : String :=
  let label := match error with
    | .malformed => "malformed"
    | .unsupportedVersion => "unsupported_version"
    | .unsupportedSchema => "unsupported_schema"
    | .unknownFields => "unknown_fields"
    | .identityMismatch => "identity_mismatch"
    | .staleSequence => "stale_sequence"
    | .kernel _ => "kernel_rejected"
  (Json.mkObj (baseResponseFields "error" profileIdentity artifactIdentity sequence ++
    [("error", toJson label)])).compress

public def initialProtocolState (handshake : Handshake) (budget : Budget) : ProtocolState := {
  profileIdentity := handshake.profileIdentity
  artifactIdentity := handshake.artifactIdentity
  lastSequence := 0
  fabricState := { phase := .initial, budget }
}

public def handleTurn
    (profile : Profile)
    (state : ProtocolState)
    (envelope : TurnEnvelope) : Except ProtocolError (ProtocolState × Decision) := do
  if envelope.profileIdentity != state.profileIdentity ||
      envelope.artifactIdentity != state.artifactIdentity then
    throw .identityMismatch
  if envelope.turn.sequence ≤ state.lastSequence then throw .staleSequence
  let (fabricState, decision) ← (step profile state.fabricState envelope.turn).mapError .kernel
  return ({ state with lastSequence := envelope.turn.sequence, fabricState }, decision)

public def validToolFixture : ToolRequest := {
  name := "search"
  schemaIdentity := "schema.search.v1"
  arguments := #[{ name := "query", value := "adapter contracts" }]
  correlationIdentity := "tool-call-1"
  visibleOutputBudget := 64
  mutationTargets := #["resource:file:README"]
  requiredCapabilities := #["tool.read"]
  approvalPosture := "pre_approved"
}

end AnimusLean
