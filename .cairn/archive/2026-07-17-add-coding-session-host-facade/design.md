## Context

Animus currently exposes many composable contracts but no single host object proves their end-to-end lifecycle. A frontend or SDK integrator would otherwise need to recreate run exclusivity, queue semantics, event ordering, durable boundaries, repair, and resume behavior. That duplication risks divergent transcript and active-leaf state.

Tau's `CodingSession` demonstrates the value of one product seam, while its broad responsibilities show why Animus needs a smaller composition boundary. Resource discovery, slash-command UX, provider credential selection, and terminal policy remain outside this facade.

## Decisions

### 1. Split a pure reducer from a thin host shell

**Choice:** A deterministic session-host reducer consumes validated state plus commands and returns new state, ordered runtime events, and explicit effect requests. A non-default std shell executes injected provider, tool, persistence, cancellation, timer, policy, and summarizer services and feeds typed outcomes back to the reducer.

**Rationale:** Run and persistence logic remains testable without standing up files, providers, tools, clocks, or a terminal.

### 2. Keep the facade intentionally small

**Choice:** The public surface covers prompt, continue, steer, follow-up, cancel, resume, compact, state snapshot, message snapshot, and event subscription. Inputs are already expanded and admitted. Provider login, model catalog UX, skills, prompt templates, project discovery, shell commands, theme, keybindings, and slash-command parsing are caller-owned.

**Rationale:** The session boundary should compose the agent lifecycle, not become an application monolith.

### 3. Enforce one active run

**Choice:** A session has at most one run identity in an active or cancelling state. A second ordinary prompt is rejected before transcript, queue, store, or provider mutation. Busy input must explicitly select steering or follow-up semantics through existing queue contracts.

**Rationale:** Concurrent runs cannot safely mutate one transcript or active leaf without a more complex transactional model.

### 4. Persist admitted user input before provider contact

**Choice:** A new prompt is validated, reduced into a user-message entry, and durably committed with its leaf transition before the provider effect request is emitted. Persistence denial or failure blocks provider contact.

**Rationale:** A provider must not observe a user turn that the declared durable session cannot restore.

### 5. Persist every dependency boundary before the next effect

**Choice:** Assistant messages are committed before tool dispatch; each tool result is committed before another provider dispatch; final assistant messages are committed before terminal completion is emitted. Summary, branch, and repair changes follow the same commit-before-dependent-effect rule.

**Rationale:** This ordering makes crash recovery local and exposes exactly where an interrupted tool result can be synthesized.

### 6. Queue messages become transcript messages only on injection

**Choice:** Steering and follow-up queues remain in session-host state while pending. A queued body becomes durable model-visible history only when the reducer selects it for injection and a user-message commit succeeds. Queue receipts and status events remain safe and bounded.

**Rationale:** Persisting every edited pending prompt as conversation history would conflate UI intent with provider-visible context.

### 7. Existing RuntimeEvent is the frontend contract

**Choice:** The facade emits existing provider/UI-neutral runtime lifecycle events plus any narrowly required session commit, queue, retry, repair, and restore variants. Headless, SDK, and TUI adapters consume the same stream. Frontends do not infer authoritative run state from spinners, widgets, or provider chunks.

**Rationale:** Events already form Animus's portable observation boundary.

### 8. Recovery runs before continuation

**Choice:** Resume validates store framing and active-leaf replay, applies or blocks interrupted-tool repair, rebuilds reducer state, and only then admits continue or prompt. A stale head, unresolved malformed tool binding, partial commit, or unavailable durable store blocks continuation.

**Rationale:** Provider continuation over a guessed transcript would weaken both persistence and tool safety.

### 9. Cancellation does not imply effect rollback

**Choice:** Cancellation changes run state, signals injected services, and records terminal observations. It does not claim that an in-flight provider or tool effect stopped. Missing tool results flow through the repair planner using evidence-bound interruption reasons.

**Rationale:** Cancellation tokens express requests, not proof of external effect termination.

### 10. Conformance is behavior-first

**Choice:** A shared scenario suite drives the facade through fake provider/tool/store services and the local durable store. It asserts event order, store order, active leaf, provider inputs, tool admission, queue injection, cancellation, repair, restart, compaction, and failure behavior. The same event fixtures feed headless and TUI reducers.

**Rationale:** Tau's strongest value is its working vertical behavior. Animus should adopt that as a conformance oracle, not copy application code.

### 11. Keep the first evidence rung bounded

**Choice:** The first packaged scenario uses deterministic fake provider and tool services plus the explicit local store. Live providers, concrete product tools, raw terminal operation, daemon attach, plugin runtime, OAuth, and global session discovery remain omitted and named.

**Rationale:** A durable fake-backed session is a meaningful next rung without overstating product readiness.

## Risks / Trade-offs

- Commit-before-effect increases latency and exposes store availability as part of prompt admission.
- A facade can accumulate unrelated application behavior. Public inventory and crate-dependency rails must enforce the narrow surface.
- Event consumers may assume emitted progress is durable. Commit status and transient progress must remain distinguishable.

## Non-Goals

- Replacing the Steel loop fabric, model router, tool executor kit, or headless protocol.
- Owning resource discovery, command parsing, credentials, model catalogs, keybindings, themes, or terminal rendering.
- Supporting concurrent runs in one session.
- Claiming cancellation rolled back provider, tool, process, filesystem, or network effects.
- Default adoption as the production Animus application shell.
