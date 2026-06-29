# Design: Introduce a Steel-backed agent runtime loop

## Context

The workspace is intentionally modular: `animus-core` owns deterministic state, `animus-adapters` owns implementation-free model/tool contracts, `animus-preserves` owns portable boundary serialization, `animus-steel` owns scriptable turn decisions, and `animus-std` owns the current standard-library shell. The missing layer is a reusable runtime that wires those pieces into an embeddable harness loop without hard-coding provider or tool implementations.

## Decisions

### 1. Runtime owns orchestration, not providers

**Choice:** Add `animus-runtime` for agent loop orchestration. It will accept adapter trait implementations and loop-fabric inputs instead of depending on concrete model/tool/provider crates.

**Rationale:** Crates remain barriers. Embedders can use the runtime with their own model/tool implementations and can ignore provider crates they do not need.

### 2. Steel is the default loop fabric, not a core dependency

**Choice:** Treat `animus-steel` as the default scriptable turn-decision fabric and expose runtime constructors that use it, while keeping the runtime interface generic enough to swap future fabrics.

**Rationale:** Steel gives the harness a programmable agent loop without putting an interpreter in `animus-core` or `animus-adapters`.

### 3. Runtime dispatch consumes validated decisions only

**Choice:** Runtime dispatch will consume `LoopDecision` values already validated by `animus-steel` and convert adapter responses back into typed runtime state/events.

**Rationale:** Steel scripts are untrusted boundary logic. Rust DTO constructors remain the source of validation before any adapter call.

### 4. Runtime events are first-class

**Choice:** Define typed turn events such as loop-started, model-requested, model-responded, tool-requested, tool-responded, final-response, stopped, max-turns-exceeded, and adapter-error.

**Rationale:** Embedders need deterministic reports and observability without scraping logs or coupling to provider internals.

### 5. Preserves schema follows the runtime event model

**Choice:** Add Preserves schema after the event/report shape is stable enough to encode as wire contract.

**Rationale:** Schema should describe the durable boundary, not churn with every internal refactor.

## Risks / Trade-offs

- Steel script evaluation can fail at load time or turn time; runtime must surface typed failures and never call adapters after invalid decisions.
- A generic runtime abstraction can become too wide; start with model/tool decisions and final response/stop behavior only.
- Event schemas can ossify too early; stage schema after initial runtime tests prove event shape.
- Max-turn behavior is required to prevent runaway loops, but the limit must be caller-configurable rather than hidden in code.
