import AnimusLean.Protocol

open AnimusLean

private def maxFrameBytes : Nat := 65536
private def initialStepBudget : Nat := 64
private def initialPayloadBudget : Nat := 262144
private def maxTextBytes : Nat := 8192
private def maxArguments : Nat := 32
private def maxTargets : Nat := 32
private def maxCapabilities : Nat := 32
private def exitSuccess : UInt32 := 0
private def exitFailure : UInt32 := 1
private def newlineByte : UInt8 := 10

private inductive ReadError where
  | frameLimit
  | invalidUtf8

private partial def readBoundedLineLoop
    (input : IO.FS.Stream)
    (remaining : Nat)
    (buffer : ByteArray) : IO (Except ReadError (Option String)) := do
  if remaining = 0 then return .error .frameLimit
  let chunk ← input.read 1
  if chunk.isEmpty then
    if buffer.isEmpty then return .ok none
    else
      match String.fromUTF8? buffer with
      | some line => return .ok (some line)
      | none => return .error .invalidUtf8
  let byte := chunk[0]!
  if byte = newlineByte then
    match String.fromUTF8? buffer with
    | some line => return .ok (some line)
    | none => return .error .invalidUtf8
  readBoundedLineLoop input (remaining - 1) (buffer.push byte)

private def readBoundedLine (input : IO.FS.Stream) : IO (Except ReadError (Option String)) :=
  readBoundedLineLoop input (maxFrameBytes + 1) ByteArray.empty

private def profile : Profile := {
  maxTextBytes,
  maxArguments,
  maxTargets,
  maxCapabilities
}

private def initialBudget : Budget := {
  remainingSteps := initialStepBudget,
  remainingPayloadBytes := initialPayloadBudget
}

private def writeFrame (output : IO.FS.Stream) (frame : String) : IO Unit := do
  output.putStrLn frame
  output.flush

private def errorFrame (state : Option ProtocolState) (error : ProtocolError) : String :=
  match state with
  | some value => encodeError value.profileIdentity value.artifactIdentity value.lastSequence error
  | none => encodeError "unbound" "unbound" 0 error

private partial def serve
    (input output : IO.FS.Stream)
    (state : Option ProtocolState) : IO UInt32 := do
  match ← readBoundedLine input with
  | .error _ =>
      writeFrame output (errorFrame state .malformed)
      return exitFailure
  | .ok none => return exitSuccess
  | .ok (some line) =>
      match decodeRequest line with
      | .error error =>
          writeFrame output (errorFrame state error)
          return exitFailure
      | .ok (.handshake handshake) =>
          match state with
          | some _ =>
              writeFrame output (errorFrame state .staleSequence)
              return exitFailure
          | none =>
              let next := initialProtocolState handshake initialBudget
              writeFrame output (encodeHandshakeAck handshake)
              serve input output (some next)
      | .ok (.turn envelope) =>
          match state with
          | none =>
              writeFrame output (errorFrame none .identityMismatch)
              return exitFailure
          | some current =>
              match handleTurn profile current envelope with
              | .error error =>
                  writeFrame output (errorFrame state error)
                  return exitFailure
              | .ok (next, decision) =>
                  writeFrame output (encodeDecision next.profileIdentity next.artifactIdentity
                    next.lastSequence decision)
                  serve input output (some next)
      | .ok (.shutdown shutdown) =>
          match state with
          | none =>
              writeFrame output (errorFrame none .identityMismatch)
              return exitFailure
          | some current =>
              if shutdown.profileIdentity != current.profileIdentity ||
                  shutdown.artifactIdentity != current.artifactIdentity then
                writeFrame output (errorFrame state .identityMismatch)
                return exitFailure
              if shutdown.sequence ≤ current.lastSequence then
                writeFrame output (errorFrame state .staleSequence)
                return exitFailure
              writeFrame output (encodeShutdownAck shutdown)
              return exitSuccess

public def main : IO UInt32 := do
  let input ← IO.getStdin
  let output ← IO.getStdout
  serve input output none
