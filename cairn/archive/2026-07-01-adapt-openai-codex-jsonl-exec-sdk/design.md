# Design: Adapt OpenAI Codex JSONL exec and SDK control contracts for Animus

## Context

Codex documents a non-interactive mode that streams progress to a machine-readable event stream and writes the final agent message separately. It also supports schema-constrained final output and SDK thread control. Animus already plans first-class runtime events; this change turns those events into an automation surface.

## Decisions

### 1. Non-interactive execution is a runtime request

**Choice:** Model non-interactive execution as a typed run request containing prompt input, optional attached context, sandbox profile, output preferences, and optional resume identity.

**Rationale:** CLI, SDK, and hosted controllers can share the same contract without duplicating orchestration logic.

### 2. Event streams are append-only and typed

**Choice:** Emit ordered runtime events for thread start, turn start, item lifecycle, model/tool dispatch, policy decisions, hook outcomes, usage, completion, and failure.

**Rationale:** Automation should consume durable events rather than scraping human-readable logs.

### 3. Final output is selected from the event log

**Choice:** The final response is a typed item in the event stream and may also be written through a shell-owned output sink.

**Rationale:** There is one source of truth for both streaming observers and final-result consumers.

### 4. Structured output is adapter-neutral

**Choice:** Runtime accepts a schema identity and validator interface instead of hard-coding one schema library. Shells may provide JSON Schema, Preserves, Nickel-derived contracts, or other validators.

**Rationale:** Animus stays flexible while still enforcing structured outputs before declaring success.

### 5. SDK control uses the same thread API

**Choice:** Expose start, resume, run, inspect, and cancel operations as controller traits over typed thread identifiers.

**Rationale:** A future TypeScript, Python, Rust, MCP, or app-server binding can share the same runtime semantics.

## Risks / Trade-offs

- Event schema churn can break automation. Version event envelopes and keep compatibility tests for stable fields.
- Structured-output validation can tempt shells to parse model text too late. Validation must occur before success is reported.
- Resume semantics require careful state identity. Thread IDs and ledger hashes should be validated before continuing a run.
