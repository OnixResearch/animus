## Phase 1: Public facade and state model

- [x] [serial] Inventory existing RuntimeEvent, headless command, queue, continuation, tool, session-tree, compaction, repair, persistence, TUI reducer, and dogfood owners and publish a no-duplication mapping. r[agent_harness_runtime.coding_session_host.facade] r[agent_harness_runtime.coding_session_host.conformance]
- [x] [serial] Define the narrow prompt, continue, steer, follow-up, cancel, resume, compact, state/messages snapshot, and event-subscription API plus typed effect requests/outcomes. r[agent_harness_runtime.coding_session_host.facade]
- [x] [depends:agent_harness_runtime.coding_session_host.facade] Define pure session state for run identity/status, queue state, active leaf, durable cursor, pending effect, cancellation, repair/recovery posture, and terminal outcome. r[agent_harness_runtime.coding_session_host.functional_core]

## Phase 2: Functional reducer

- [x] [depends:agent_harness_runtime.coding_session_host.functional_core] Implement deterministic command reduction and enforce one active run, stale-run rejection, explicit busy prompt behavior, and no overlapping transcript mutation. r[agent_harness_runtime.coding_session_host.single_run]
- [x] [depends:agent_harness_runtime.coding_session_host.single_run] Integrate existing steering/follow-up reducers so pending queue edits remain non-transcript state and selected injection requires a durable user-message commit. r[agent_harness_runtime.coding_session_host.queues]
- [x] [depends:agent_harness_runtime.coding_session_host.functional_core] Implement commit-before-effect transitions for user message/provider, assistant message/tool batch, each tool result/provider continuation, final assistant/completion, compaction, branch movement, and repair. r[agent_harness_runtime.coding_session_host.durable_boundaries]
- [x] [depends:agent_harness_runtime.coding_session_host.durable_boundaries] Implement recovery reduction over validated store/replay/repair outcomes and block continuation on stale heads, malformed bindings, partial commits, or unavailable required services. r[agent_harness_runtime.coding_session_host.recovery]
- [x] [depends:agent_harness_runtime.coding_session_host.functional_core] Map transitions and effect outcomes to ordered existing RuntimeEvent values plus narrowly required session commit/recovery variants. r[agent_harness_runtime.coding_session_host.events]

## Phase 3: Thin host shell

- [x] [depends:agent_harness_runtime.coding_session_host.durable_boundaries] Implement a non-default host shell that wires only explicit provider, tool, persistence, cancellation, timer, policy, and summarizer services and feeds outcomes back to the reducer. r[agent_harness_runtime.coding_session_host.facade] r[agent_harness_runtime.coding_session_host.functional_core]
- [x] [depends:agent_harness_runtime.coding_session_host.recovery] Wire durable store open/resume, active-leaf replay, interrupted-tool repair, and reducer reconstruction before prompt or continuation admission. r[agent_harness_runtime.coding_session_host.recovery]
- [x] [depends:agent_harness_runtime.coding_session_host.events] Add headless/SDK adapters and a TUI reducer adapter that consume the same event stream without moving keybindings, rendering, resource discovery, command parsing, or provider login into the facade. r[agent_harness_runtime.coding_session_host.events]

## Phase 4: Behavior conformance

- [x] [parallel] Add positive black-box scenarios for prompt/final response, multi-turn replay, tool call/result continuation, one-at-a-time and all-at-once queue profiles when admitted, steering, follow-up, cancellation request, compaction, branch resume, interrupted-tool repair, process restart, and deterministic event/store ordering. r[agent_harness_runtime.coding_session_host.conformance]
- [x] [parallel] Add negative scenarios for overlapping prompts, ambiguous busy input, stale run/cancellation IDs, provider failure, tool denial/failure, store failure before provider, store failure before tool, partial commit, stale leaf, malformed replay, unresolved repair, timer/service absence, unbounded queue input, and event/receipt leaks. r[agent_harness_runtime.coding_session_host.conformance]
- [x] [depends:agent_harness_runtime.coding_session_host.events] Feed identical event fixtures through headless and TUI reducers and prove they agree on lifecycle status, transcript boundaries, queue counts, tool correlation, cancellation posture, and terminal state without frontend-owned transcript mutation. r[agent_harness_runtime.coding_session_host.events] r[agent_harness_runtime.coding_session_host.conformance]
- [x] [depends:agent_harness_runtime.coding_session_host.recovery] Add restart fault-injection scenarios at every commit/effect boundary and prove no dependent provider or tool effect starts before its prerequisite durable commit. r[agent_harness_runtime.coding_session_host.durable_boundaries] r[agent_harness_runtime.coding_session_host.recovery] r[agent_harness_runtime.coding_session_host.conformance]

## Phase 5: Product rung and closeout

- [x] [serial] Add a packaged non-default durable fake-backed session smoke and safe receipt projection, naming live providers, concrete product tools, TUI dogfood, daemon, plugins, OAuth, and production readiness as omissions. r[agent_harness_runtime.coding_session_host.conformance]
- [x] [serial] Update request lifecycle, dogfood readiness/evidence ladder, headless, TUI, session, component catalog, runtime facade, safety, and README reference docs. r[agent_harness_runtime.coding_session_host.facade] r[agent_harness_runtime.coding_session_host.events]
- [x] [serial] Run focused reducer/facade/store/headless/TUI tests, formatting, Clippy, composition conformance, packaged smoke, safe receipts, dogfood claim hygiene, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.coding_session_host.conformance]
