# Animus request lifecycle

This document is the maintained ownership map for request processing. It is a review aid, not a second implementation. Runtime changes should update this page when they add, remove, or move a state transition.

## Golden path

1. **Typed runtime input** / typed host runtime input is owned by `animus-std::RuntimeConfig` (`crates/animus-std/src/config.rs`). The source-of-truth constructors are `Runtime::from_intent`, `Runtime::from_env_pairs`, and the thin shell `load_runtime`.
2. **Deterministic core state** is owned by `animus-core::Intent` and `animus-core::AnimusState` (`crates/animus-core/src/lib.rs`). Core validation produces `animus-core::CoreError` and does not read ambient host state.
3. **Reusable loop input/config** is owned by `animus-runtime::RuntimeInput` and `animus-runtime::RuntimeConfig` (`crates/animus-runtime/src/contract.rs`). Runtime config is explicit data such as max-turn budgets, not ambient process state. Optional **workflow command planning** is owned by `animus-runtime::WorkflowCommandManifest`, `WorkflowCommandRegistry`, `WorkflowInvocationBinding`, and `WorkflowCommandOutputContract`; hosts supply already-loaded manifests and already-parsed arguments.
4. **Steel loop decision input** is owned by `animus-steel::LoopTurn` and `animus-steel::SteelLoopFabric` (`crates/animus-steel/src/turn.rs` and `fabric.rs`). `animus-runtime::TurnState` projects validated runtime state into this scriptable loop boundary for each turn.
5. **Steel loop decisions** are owned by `animus-steel::LoopDecision` (`crates/animus-steel/src/action.rs`). Rust revalidates every `CallModel`, current-shape `CallTool`, `Respond`, and `Stop` decision before a host adapter receives it; legacy name-plus-raw-input tool decisions are rejected.
6. **Adapter dispatch DTOs** are owned by `animus-adapters::ModelRequest`, `animus-adapters::ModelResponse`, current-shape `animus-adapters::ToolRequest`, bounded `animus-adapters::ToolResponse`, `ModelAdapter`, and `ToolAdapter` (`crates/animus-adapters/src/`). These are implementation-free contracts.
7. **Host status-observation DTOs** are owned by `animus-adapters::AgentStatusObservation`, `HostStatusRequest`, `HostStatusObservationBatch`, and `HostStatusAdapter`. Hosts may detect agent status however they choose, but reusable crates receive only typed observations.
8. **Reusable status authority selection** is owned by `animus-runtime::select_agent_status`, `AgentStatusSelection`, and `StatusDecisionExplanation`. The selector is pure over a validated target request, logical sequence, and in-memory observations; presentation metadata remains visual-only.
9. **Per-model invocation composition** is owned by `animus-runtime::ModelRuntimeProfileSet`, `ResolvedModelRuntimeProfile`, `ModelInvocationPlan`, and `ModelInvocationReceipt`. Hosts load profiles and validated resources; the pure core resolves exact target/role/default tiers, intersects independent authority ceilings, preserves mandatory prompts and progressive skills, intersects exact existing memory policy, projects exact native tool declarations, and binds fallback/retry/tool-call identities before provider or executor effects.
10. **Semantic role composition** is owned by `animus-runtime::agent_roles`. Hosts load and authorize exact role profiles; the pure core adds role resources as an independent model ceiling, intersects requested typed capabilities across every grant/policy/delegation/task/agent/role/model/workspace/executor/sandbox ceiling, checks explicit separation dimensions, binds immutable plans, requires recomposition across role/model transitions, and admits only capability-scoped metadata outputs. Role findings, checks, judgments, and candidates are not authoritative completion.
11. **Reusable runtime dispatch and projection** are owned by `animus-runtime::run_agent_loop`, `LoopFabric`, `ToolDispatchPlan`, `tool_executor_kit`, `RuntimeEvent`, `RunReport`, and `RuntimeStatus` (`crates/animus-runtime/src/`). The runtime shell admits tool calls through active trusted catalog definitions, exact capability labels, current-schema arguments, approval posture, mutation coordination, output policy, and an explicit ready executor binding before calling injected fabrics/adapters. The executor kit records only bounded safe receipts, while raw tool output and all concrete executor authority remain host-owned. Runtime records ordered bounded events, handles stop/final/max-turn/failure statuses, and does not discover providers or credentials.
12. **Std-shell adapter dispatch** is owned by `animus-std::run_with_model`, `run`, `SummaryModelAdapter`, and the pure `host_adapter_catalog` / `plan_host_adapter` composition boundary. The shell wires typed config, core state, and caller-provided blocks without ambient fallback or provider/credential discovery. Deterministic observability sinks are owned directly by `animus-std-observability` and remain source-compatible through the default `animus-std/observability-host` facade. The legacy **Runtime event/report projection** remains represented by `animus-std::RunReport` and `animus-std::RunError` for this shell path.
13. **Safe receipt construction** for ledger/session replay and restore planning is owned by `animus-runtime::session_persistence::LedgerSessionReceiptDraft` plus `validate_ledger_session_receipt`. Evidence summaries must stay bounded: status, issue codes, counts, safe relative labels, BLAKE3 hashes where content identity is needed, explicit redaction markers, and omitted-surface non-claims.
14. **Host ledger persistence seam** is owned by `animus-runtime::session_persistence`. Hosts inject a `LedgerSessionService` descriptor (the accepted host-owned `LedgerService` marker for this seam) and already-loaded ledger entries; reusable runtime code validates descriptors, plans replay/restore, and validates receipts without discovering stores, reading files, opening databases, contacting networks, using clocks, or writing sessions.

## Boundary owners

| Boundary | Owning crate | Source-of-truth type or trait | Host authority |
| --- | --- | --- | --- |
| Host runtime input | `animus-std` | `RuntimeConfig`, `Runtime::from_env_pairs`, `load_runtime` | Process environment is read only by `load_runtime`. |
| Model-profile config loading | `animus-std` | `load_model_runtime_profiles`, `parse_model_runtime_profiles_json` behind `model-runtime-profiles` | Exact generated-JSON parsing is pure; explicit path reads are shell-owned. Nickel is never evaluated at runtime. |
| Semantic-role config loading | `animus-std` | `load_agent_role_profiles`, `parse_agent_role_profiles_json` behind `agent-role-profiles` | Exact generated-JSON parsing is pure; explicit path reads are shell-owned. Nickel is never evaluated at runtime. |
| Core state | `animus-core` | `Intent`, `AnimusState`, `CoreError` | No host authority. |
| Reusable loop input/config | `animus-runtime` | `RuntimeInput`, `RuntimeConfig` | No ambient host authority. |
| Workflow command planning | `animus-runtime` | `WorkflowCommandManifest`, `WorkflowCommandRegistry`, `WorkflowInvocationBinding`, `WorkflowCommandOutputContract` | Manifests and parsed arguments are host-supplied; reusable planning only validates, binds, and policy-checks. |
| Steel turn input | `animus-steel` / `animus-runtime` | `LoopTurn`, `TurnState`, `LoopFabric` | Fabric is injected; no concrete providers or tools are constructed. |
| Steel decision | `animus-steel` | `LoopDecision`, `SteelLoopFabric`, `LoopError` | No adapter dispatch until Rust revalidation passes. |
| Model/tool DTOs | `animus-adapters` | `ModelRequest`, `ModelResponse`, current-shape `ToolRequest`, bounded `ToolResponse` | No implementation authority; no legacy raw tool payloads. |
| Status observation DTOs | `animus-adapters` | `AgentStatusObservation`, `HostStatusRequest`, `HostStatusObservationBatch` | Typed observations only. |
| Adapter ports | `animus-adapters` | `ComponentAdapter`, `ModelAdapter`, `ToolAdapter`, `HostStatusAdapter` | Concrete host implementation is injected by callers. |
| Status authority selection | `animus-runtime` | `select_agent_status`, `AgentStatusSelection`, `StatusDecisionExplanation` | Pure selection over typed inputs only. |
| Per-model invocation composition | `animus-runtime` | `ModelRuntimeProfileSet`, `ResolvedModelRuntimeProfile`, `ModelInvocationPlan`, `ModelInvocationReceipt` | Pure over host-loaded profiles, prompts, skills, opt-in memory policy, model capabilities, tool catalogs, grants, approvals, readiness, and budgets; no profile grants authority or crosses provider/executor effects. |
| Capability-bound semantic roles | `animus-runtime` | `agent_roles::{AgentRoleCatalog, AgentRoleAssignment, AgentRoleCapabilityResolution, AgentRoleSeparationDecision, AgentRolePlan, AgentRoleOutput, AgentRoleReceipt}` | Pure over host-loaded and host-authorized identities; roles only narrow authority. Providers, tools, checks, mutation, persistence, clocks, and completion remain host-owned. |
| Runtime dispatch | `animus-runtime` | `run_agent_loop`, `run_with_steel_program`, `ToolDispatchPlan`, `tool_executor_kit`, `RuntimeEvent`, `RunReport`, `RuntimeStatus` | Calls injected fabric/model/tool boundaries only after exact catalog/capability/argument/executor-binding admission; executor receipts omit raw output. |
| Std dispatch | `animus-std` / `animus-std-observability` | `run_with_model`, `run`, `host_adapter_catalog`, `plan_host_adapter`, `NoopSink`, `InMemorySink` | Explicit authority-scoped shell composition only; missing/disabled blocks and ambient fallback fail closed. |
| Ledger/session receipts | `animus-runtime` | `LedgerSessionReceiptDraft`, `validate_ledger_session_receipt` | Safe labels, counts, hashes, redaction/freshness classes, and non-claims only. |
| Ledger/session seam | `animus-runtime` | `LedgerSessionService`, `LedgerReplayRequest`, `SessionRestoreRequest`, `LedgerEntry`, `plan_ledger_replay`, `plan_session_restore` | Service descriptors and ledger entries are host-injected; durable storage remains host-owned. |

## Forbidden dependencies and side effects

### Reusable core boundary

`animus-core` must not read environment variables, filesystem paths, credentials, provider config, network resources, clocks, randomness, process state, terminal state, or concrete host persistence. Validate with `cargo test -p animus-core` and the no-std/Wasm rail.

### Adapter contract boundary

`animus-adapters` must not implement providers, discover tools, read credentials, open network connections, execute processes, read files, or persist sessions. It owns DTO validation and implementation-free traits only, including current-shape tool request validation, bounded tool response validation, typed tool failure classes, host status-observation DTOs, and the `HostStatusAdapter` port. Validate with `cargo test -p animus-adapters` and dependency rails.

### Status authority boundary

`animus-runtime::select_agent_status` must remain pure over typed in-memory observations, a logical sequence, and a validated target request. Trusted current lifecycle observations suppress heuristic fallback; stale, untrusted, or conflicting reports degrade deterministically without reading host state. Presentation metadata must remain visual-only and must not drive waits, notifications, rollups, or policy decisions.

### Steel boundary

`animus-steel` may evaluate Steel scripts, but every script-produced action must re-enter Rust validation through `LoopDecision`, `ModelRequest`, current-shape `ToolRequest`, bounded `ToolResponse`, or `ModelResponse` before dispatch. Tool calls must use the structured `(animus-tool name schema args correlation budget targets caps approval)` shape. It must not silently bypass adapter contracts, weaken policy, or persist host state. Validate with `cargo test -p animus-steel`.

### Reusable runtime boundary

`animus-runtime` owns pure turn-state transitions, model-runtime profile validation/resolution, prompt/skill/memory/tool composition, capability-bound semantic role validation/composition/intersection/separation/transition/output decisions, identity binding, and metadata-only receipt projection, workflow command manifest/registry/argument/output validation, worker task/context/report validation, deterministic worker report merge logic, pure `ToolDispatchPlan` and `tool_executor_kit` admission, and thin dispatch shells over injected loop fabrics, tool executors, worker services, and model/tool adapters. Tool dispatch must fail closed for unknown, inactive, untrusted, executor-less, capability-denied, approval-denied, schema- or argument-mismatched, malformed-output, unsafe-output, unbounded-output, mutation-conflict, cancelled, timed-out, binding-mismatched, or fallback-denied plans before executor effects. Executor receipts reject credentials, private paths, raw file bodies, provider payloads, and unbounded stdout/stderr. It must not construct provider clients, discover tools, read provider credentials, open network connections, execute processes, read files, inspect environment variables, use clocks, or perform concrete persistence. Validate with `cargo test -p animus-runtime` and `cargo test -p animus-runtime workflow_command --locked` for command contracts.

### Std-shell boundary

`animus-std` owns process-facing orchestration such as `load_runtime`, but deterministic parsing and authority planning belong in testable helpers such as `Runtime::from_env_pairs` and `plan_host_adapter`. Missing or disabled blocks must not fall back to environment discovery, filesystem scans, provider probing, network defaults, storage creation, process execution, or terminal access. `animus-std-observability` owns only deterministic no-op/in-memory sinks over validated DTOs. Concrete providers, credentials, transports, and persistence require separately admitted host blocks. Validate with `cargo test -p animus-std`, `cargo test -p animus-std-observability`, and `cargo check -p animus-std --no-default-features`.

### Receipt boundary

Receipt and evidence code must not embed raw prompts, credentials, provider payloads, raw tool output, raw terminal snapshots, unbounded stdout/stderr, unbounded detector transcripts, full private paths, or raw private file bodies. Status-decision explanations and ledger/session receipts must use safe hashes, bounded redacted summaries, freshness classes, sequence ranges, counts, and omitted-surface non-claims. Validate current lifecycle evidence with `scripts/cairn/check-evidence-safety --root .`.

### Host ledger persistence seam

The ledger/session service is host-injected through `animus-runtime::session_persistence`. `LedgerSessionService` is the concrete `LedgerService` marker for the accepted seam. Reusable crates validate descriptors, plan replay/restore over already-loaded entries, and reject unsafe receipts; they must not discover sessions, read host persistence, create default global stores, or resume conversations by ambient global state. Durable backends, locking, freshness collection, writes, and migrations remain host-owned implementation work.

## Validation commands

Use the smallest command that covers the changed boundary:

```sh
cargo test -p animus-core
cargo test -p animus-adapters
cargo test -p animus-steel
cargo test -p animus-runtime
cargo test -p animus-runtime tool_executor_kit --locked
cargo test -p animus-runtime modelprofiles --locked
scripts/cairn/check-model-runtime-profiles --root .
cargo test -p animus-runtime agent_roles --locked
scripts/cairn/check-agent-role-profiles --root .
cargo test -p animus-std
cargo test -p animus-std-observability
cargo check -p animus-std --no-default-features
scripts/cairn/check-tool-catalog --root .
cargo test -p animus-runtime session_persistence --locked
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-evidence-safety --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cairn -- validate --root .
```

Closeout evidence should name which commands ran, which boundaries they cover, and any skipped checks with a blocker and next-best validation path.
