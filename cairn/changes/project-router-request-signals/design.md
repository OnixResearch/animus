# Design: Project router request signals from typed metadata

## Context

`Signals::project` is the entry to recipe selection, but its input `SignalDraft` is already classified. The gap between a real model request and a `SignalDraft` is unowned. Filling it deterministically is what makes the router usable without per-caller hand-classification, while keeping a pluggable seam preserves the option to inject a learned classifier later.

## Decisions

### 1. Rule-based default projector is pure over bounded metadata

**Choice:** Add `project_signals(metadata: &RequestMetadata) -> Result<SignalDraft, SignalProjectionError>` as a pure function. It maps bounded request metadata (prompt char/token estimate, tool-call count, code-block presence, structured-output demand, context-window fill ratio, privacy policy label, fallback hint) to the eight `SignalDraft` bands using explicit thresholds.

**Rationale:** A deterministic default makes routing testable and reproducible without any classifier dependency. Thresholds are named constants, not magic numbers.

### 2. SignalSource adapter trait is the classifier seam

**Choice:** Define `trait SignalSource { fn project(&self, metadata: &RequestMetadata) -> Result<SignalDraft, SignalProjectionError>; }` in `animus-adapters`. The rule projector is a `RuleSignalSource` struct that implements it. Hosts may inject any `SignalSource`.

**Rationale:** Keeps the classifier pluggable so a later Supra-Router adapter can replace the rule projector without touching router core. The trait lives in adapters (implementation-free contracts), the rule implementation lives in router.

### 3. Projection consumes only bounded, safe metadata

**Choice:** `RequestMetadata` carries counts, booleans, and labels — never raw prompt bodies, provider payloads, credentials, or unbounded text. The projector returns `SignalProjectionError::UnsafePayload` if any field looks like a raw prompt or credential.

**Rationale:** Signals flow into receipts; they must stay safe. This connects to the existing `vllm_semantic_router_recipes.signals.safe` requirement.

### 4. Thresholds are named constants with documented bands

**Choice:** Each threshold (e.g. `REASONING_TOKEN_THRESHOLD`, `HIGH_CONTEXT_PRESSURE_RATIO`, `STRICT_CONTRACT_TOOL_COUNT`) is a named constant with a doc comment mapping to a band. No numeric literal appears unexplained.

**Rationale:** Matches the repo's no-magic-numbers rule and makes band boundaries auditable.

## Risks / Trade-offs

- Rule thresholds are approximate. Mitigate by treating them as policy (later `add-router-policy-contracts` can move thresholds into Nickel) and by keeping the `SignalSource` seam so hosts override them.
- Token estimation without a tokenizer is lossy. Use char-based heuristics with a documented ratio constant and mark it approximate; a host `SignalSource` can supply real token counts.
- `RequestMetadata` may overlap existing request DTOs. Reuse `ModelRequest`-derived metadata where possible and add only the bounded fields the projector needs.
