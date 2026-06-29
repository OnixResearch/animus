# Animus runtime Preserves envelope

This schema note records the stable runtime boundary variants implemented by `animus-preserves::BoundaryMessage`.

Runtime event envelope variants:

- `RuntimeLoopStarted { summary }`
- `RuntimeModelRequested { prompt }`
- `RuntimeModelResponded { message }`
- `RuntimeToolRequested { name, input }`
- `RuntimeToolResponded { output }`
- `RuntimeFinalResponse { message }`
- `RuntimeStopped`
- `RuntimeMaxTurnsExceeded { max_turns }`
- `RuntimeAdapterError { component, message }` where `component` is `model` or `tool`
- `RuntimeFabricError { message }`

Runtime report envelope variant:

- `RuntimeReport { status, events }`

Runtime report statuses:

- `FinalResponse { message }`
- `Stopped`
- `MaxTurnsExceeded { max_turns }`
- `Failed { failure }`

Runtime failures:

- `Fabric { message }`
- `ModelAdapter { message }`
- `ToolAdapter { message }`

Round-trip and malformed-payload coverage lives in `crates/animus-preserves/src/message.rs` tests.
