# Design: Replace legacy tool calling contracts

## Context

The accepted Pi-derived tool contracts define rich metadata, argument preparation, bounded output, mutation planning, and typed failure classes. The runtime path still falls back to a minimal DTO shape and broad compatibility behavior: a fabric can request a tool by name plus raw input text, runtime checks only broad tool capability presence, the adapter returns raw output-shaped text, and failure detail is often collapsed into execution failure.

Because compatibility is explicitly out of scope, this change removes the legacy shape rather than layering a v2 wrapper beside it. Old ledgers, fixtures, and embedders that still produce raw string input must fail closed until they are rewritten to the current contract.

## Decisions

### 1. Current-shape tool calls are the only runtime contract

**Choice:** Replace runtime-visible tool calls with a structured DTO carrying tool name, public schema identity, validated argument fields or host-validated argument body, tool-call correlation identity, output policy, declared mutation targets, required capability labels, and approval posture.

**Rationale:** Every dispatch decision needs these facts before executor invocation. Keeping raw string input means policy and schema validation remain optional or host-specific.

### 2. No compatibility preparer for legacy tool arguments

**Choice:** Delete or disable legacy argument compatibility shims on the runtime dispatch path. Resumed legacy calls are rejected with a typed current-shape-required diagnostic.

**Rationale:** The project has chosen safety and simplicity over migration compatibility. A single shape gives tests, receipts, and embedders one set of invariants.

### 3. Catalog admission precedes prompt and dispatch

**Choice:** Build model-facing tool context and runtime dispatch admission from the same active trusted catalog or manifest definitions. A tool is callable only when its definition is active, trusted, schema-valid, executor-backed, and capability-admitted.

**Rationale:** The model should not see tools that dispatch cannot execute, and runtime should not execute tools that were not advertised through reviewed metadata.

### 4. Tool dispatch gets a pure plan

**Choice:** Add `ToolDispatchPlan` and `ToolDispatchPlanIssue` beside model dispatch planning. The pure plan records definition identity, request contract identity, schema identity, capability resolutions, approval decision, output policy, mutation plan, cancellation/timeout posture, and fallback denial.

**Rationale:** Runtime shells need one deterministic admission object to test and receipt before effects. The shell remains a thin executor over an admitted plan.

### 5. Results are bounded DTOs, not raw output strings

**Choice:** Replace runtime-visible raw tool output with `BoundedToolResult`-style data: visible text, status, byte/char/line counts, BLAKE3 content hash when required, redaction class, and optional host-owned full-output handle.

**Rationale:** Runtime state, events, reports, and model context must not carry unbounded executor bodies or unsafe host paths.

### 6. Execution outcomes stay typed end-to-end

**Choice:** Tool execution services return typed completed or fail-closed outcomes. Denied, missing, unavailable, malformed output, cancelled, timed out, unsafe output reference, unbounded oversized output, and execution failure are preserved as distinct classes in runtime events and reports.

**Rationale:** Receipts, retry decisions, and UI affordances cannot safely infer semantics from prose errors.

### 7. Mutation scheduling remains host-owned but plan-driven

**Choice:** The pure core plans mutation windows from opaque target keys; the host-owned coordinator serializes conflicting mutation windows and may run independent read-only or non-overlapping calls concurrently.

**Rationale:** Reusable crates should decide conflicts over explicit labels without reading files or applying mutations.

## Breaking behavior

- Raw string `ToolRequest` input is rejected before dispatch.
- Raw string `ToolResponse` output is not accepted as a completed runtime result.
- Broad `default-tool` capability admission is removed from the dispatch path.
- Legacy argument compatibility rules are not applied for runtime tool calls.
- Preserves, headless, TUI, docs, and tests must project the structured current result shape rather than legacy raw payloads.

## Validation strategy

Run the smallest focused checks during implementation: `cargo test -p animus-adapters tool`, `cargo test -p animus-runtime tool_dispatch`, Preserves/headless projection tests, `scripts/cairn/check-tool-catalog --root .`, `scripts/cairn/check-request-lifecycle --root .`, `scripts/cairn/check-evidence-safety --root .`, and `nix run .#cairn -- validate --root .`. Closeout evidence must name skipped checks with blockers and next-best validation.