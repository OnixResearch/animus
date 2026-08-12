## Why

Animus has strong individual contracts for runtime events, headless commands, queues, continuation, tools, session trees, compaction, persistence, and TUI reducers, but embedders still lack one small executable session boundary that composes those contracts. The current packaged smoke proves a fake-backed prompt path, not durable multi-turn session behavior.

Tau's `CodingSession` demonstrates the missing product seam, but its large application object should not be copied. Animus should expose a narrow host facade backed by a pure state reducer and injected services, so headless, SDK, and future TUI shells share one lifecycle without importing UI, resource-discovery, credential, or command policy into runtime core.

## What Changes

- Define a small session-host API for prompt, continuation, steering, follow-up, cancellation, resume, compaction, state snapshots, and event subscription. r[agent_harness_runtime.coding_session_host.facade]
- Keep run, queue, durable-cursor, active-leaf, and recovery transitions in a deterministic functional core. r[agent_harness_runtime.coding_session_host.functional_core]
- Enforce one active run per session and require explicit steering or follow-up behavior for prompt-like input while busy. r[agent_harness_runtime.coding_session_host.single_run] r[agent_harness_runtime.coding_session_host.queues]
- Commit user, assistant, and tool-result boundaries before the next dependent effect or provider dispatch. r[agent_harness_runtime.coding_session_host.durable_boundaries]
- Project the existing provider/UI-neutral event stream as the sole frontend lifecycle contract. r[agent_harness_runtime.coding_session_host.events]
- Integrate interrupted-tool repair and deterministic restart/resume handling before continuation. r[agent_harness_runtime.coding_session_host.recovery]
- Add black-box fake and durable-store conformance scenarios shared by headless and TUI reducers. r[agent_harness_runtime.coding_session_host.conformance]

## Impact

- **Runtime**: gains a pure session-host reducer and typed effect requests, not filesystem, provider, tool, timer, terminal, or credential authority.
- **Std host**: wires explicit provider, tool, persistence, cancellation, timer, and policy services behind a non-default facade.
- **Frontends**: headless and TUI consumers render the same ordered events and do not mutate transcript state independently.
- **Claims**: the first rung proves a fake-provider/fake-tool durable local session path. It does not claim live-provider quality, default tools, terminal dogfood, daemon attach, plugin runtime, OAuth, or production readiness.
