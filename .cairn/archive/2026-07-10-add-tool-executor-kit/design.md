# Design: Add an explicit tool executor kit

## Context

The current repo has current-shape `ToolRequest`, bounded `ToolResponse`, dispatch planning, permission/capability DTOs, plugin manifests, and product-surface non-claims. The missing Lego block is a coherent host-injected executor kit that combines those contracts without granting ambient execution authority.

## Decisions

### 1. Tool execution is an injected service boundary

**Choice:** Define a tool executor service trait or DTO boundary that hosts implement. Reusable runtime code produces admitted execution plans and consumes typed outcomes; it never executes shell commands, reads files, opens networks, or discovers tools by itself.

**Rationale:** Tool execution is useful only when policy and host authority are explicit. Injection keeps reusable crates deterministic.

### 2. Catalog definitions are data, not authority

**Choice:** Tool definitions describe name, schema identity, argument schema, capability requirements, mutation posture, output policy, approval posture, trust status, provenance, and executor binding labels. A definition does not execute anything until policy admission succeeds and a matching executor is injected.

**Rationale:** Tool metadata should be composable and reviewable without creating side effects.

### 3. Execution plans are pure final gates

**Choice:** The runtime tool executor kit produces pure plans that either admit a specific executor binding or return typed fail-closed diagnostics for missing, denied, malformed, inactive, untrusted, over-budget, conflicting, cancelled, timed-out, or fallback-denied requests.

**Rationale:** Hosts need one deterministic decision point before effects occur.

### 4. Receipts are safe and bounded

**Choice:** Execution receipts contain status, failure class, tool label, schema identity, capability labels, mutation target summaries, counts, redaction markers, safe references, and BLAKE3 hashes where content identity is needed. They must not store raw credentials, raw private file bodies, unbounded stdout/stderr, full private paths, raw provider payloads, or unbounded tool output.

**Rationale:** Durable evidence must prove what boundary was crossed without leaking sensitive material.

## Risks / Trade-offs

- A generic executor kit can become a product tool runtime. Mitigate by forbidding default tools and keeping concrete packs separate host adapters.
- Tool schemas may overlap plugin/MCP manifests. Mitigate by treating plugin/MCP descriptors as optional adapters into the kit, not the source of authority.
- Too many failure classes can be hard to use. Mitigate with a small canonical failure taxonomy plus safe metadata.
