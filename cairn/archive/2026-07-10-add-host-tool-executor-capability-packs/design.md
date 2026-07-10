# Design: Host tool executor capability packs

## Context

Tool definitions and dispatch gates already separate model-visible metadata from executor authority. Plugin and tool manifest work also records missing executors as fail-closed outcomes. The missing adapter slice is a reusable way for host shells to package executor availability, exact capability labels, output policy, and mutation policy into reviewable packs.

## Decisions

### 1. Packs describe executor authority, not tool implementation

**Choice:** A pack descriptor names safe tool definitions, exact capability labels, schema identities, executor service labels, output policies, mutation windows, and approval posture.

**Rationale:** Concrete tool code remains host-owned; reusable code validates authority and dispatch prerequisites.

### 2. Prompt exposure requires admitted executor packs

**Choice:** Model-facing tool context is shown only when the tool definition is active, trusted, and backed by an admitted executor pack or explicit prompt-only policy.

**Rationale:** Showing executor-less tools leads to hallucinated calls and ambiguous authority.

### 3. Deterministic fake executors provide first evidence

**Choice:** The first pack implementation uses fake executors for read-only and bounded-output cases.

**Rationale:** Dogfood readiness can prove execution shape without shipping product tools or granting host effects.

### 4. Mutation and output receipts are mandatory

**Choice:** Every executor outcome projects a safe receipt with failure class, capability label, bounded output status, mutation policy, and redaction marker.

**Rationale:** Tool output and mutation are high-risk adapter surfaces.

## Risks / Trade-offs

- Pack descriptors can duplicate manifest metadata. Treat manifests as source descriptors and packs as runtime executor admission evidence.
- Prompt-only tools may be useful but dangerous. They must be explicit and not imply executor availability.
- Fake executors do not prove host tool quality. Docs must scope the evidence rung.
