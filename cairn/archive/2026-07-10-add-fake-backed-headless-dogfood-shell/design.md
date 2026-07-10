# Design: Fake-backed headless dogfood shell

## Context

`docs/dogfoodable-host-readiness-roadmap.md` defines the first dogfood target as a non-default headless/CLI host shell with deterministic fake adapters. Existing reusable crates already own DTOs, runtime events, policy gates, and fixture adapters, but no executable shell stitches them together into a bounded prompt path.

## Decisions

### 1. Non-default host shell

**Choice:** The shell is feature-gated, packaged separately, or otherwise opt-in so default reusable crates do not gain product behavior.

**Rationale:** Dogfoodability should prove integration without making live providers, tools, persistence, or UI default dependencies.

### 2. Deterministic fake services first

**Choice:** The first smoke uses injected deterministic model and tool fixtures, admitted tool metadata, and host-provided policy state.

**Rationale:** Fake-backed integration provides stable evidence before optional live provider or packaged tool work.

### 3. Ordered events are the integration spine

**Choice:** The shell records prompt admission, model request, model response, optional tool-admission decision, completion, and shutdown as ordered runtime events.

**Rationale:** Events let headless, SDK, receipts, and future UI consumers share the same evidence shape.

### 4. Receipts prove only safe facts

**Choice:** The shell receipt index stores status classes, counts, adapter labels, BLAKE3 identities, redaction markers, and omitted-surface non-claims.

**Rationale:** Dogfood evidence is likely durable; raw prompts, model output bodies, private paths, credentials, and provider payloads must stay out.

## Risks / Trade-offs

- Fake-backed integration may be mistaken for production readiness. Docs and receipts must state the evidence rung.
- A thin shell can drift from runtime behavior if it bypasses runtime events. The smoke must run through runtime-owned orchestration rather than duplicating logic.
- Adding examples creates maintenance load. Keep one minimal positive path and focused negative fixtures.
