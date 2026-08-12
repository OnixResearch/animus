# Design: Add composition conformance rail

## Context

Animus already uses focused rails for boundaries such as runtime facade inventory, safe receipts, injected services, and TUI host surfaces. A composition rail should sit above isolated unit tests and below full workspace checks: fast, deterministic, in-memory, and explicit about which surfaces compose.

## Decisions

### 1. Use a maintained conformance matrix

**Choice:** Define a table of cross-module scenarios, starting with provider-hardening plus model dispatch, tool contract plus provenance receipt, headless event projection plus runtime events, middleware denial plus runtime failure, and capability absence plus no ambient fallback.

**Rationale:** A matrix makes composition coverage reviewable and prevents ad-hoc integration tests from drifting.

### 2. Keep fixtures pure and fake-backed

**Choice:** Use in-memory DTOs, fake adapters, and deterministic fixtures rather than live providers, tools, processes, terminals, networks, or storage.

**Rationale:** Composition semantics should be testable without standing up the world.

### 3. Include negative composition cases

**Choice:** Every positive composition scenario has paired negative cases: malformed response, unauthorized capability, missing service, oversized output, unsafe receipt, invalid feature selection, or disabled surface.

**Rationale:** Composability is safe only when failure paths compose too.

### 4. Produce safe evidence summaries

**Choice:** The rail emits scenario names, classes, counts, and hashes, not raw prompts, provider payloads, credentials, full paths, or unbounded bodies.

**Rationale:** Conformance evidence should be suitable for release and Cairn closeout.

### 5. Make the rail focused and optional in full checks

**Choice:** Add a direct script and, if practical, a Nix check. Full workspace gates may include it once runtime cost is stable.

**Rationale:** Maintainers need a quick command while preserving eventual CI coverage.

## Risks / Trade-offs

- A broad matrix can become slow. Keep scenarios small and deterministic.
- Cross-module tests can overfit implementation details. Assert boundary outcomes, not internal call order beyond deterministic safety requirements.
- Evidence summaries can accidentally leak fixture bodies. Add negative leak fixtures for the rail output itself.
