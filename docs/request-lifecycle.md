# Animus request lifecycle

This document is the maintained ownership map for request processing. It is a review aid, not a second implementation. Runtime changes should update this page when they add, remove, or move a state transition.

## Golden path

1. **Typed runtime input** / typed host runtime input is owned by `animus-std::RuntimeConfig` (`crates/animus-std/src/config.rs`). The source-of-truth constructors are `Runtime::from_intent`, `Runtime::from_env_pairs`, and the thin shell `load_runtime`.
2. **Deterministic core state** is owned by `animus-core::Intent` and `animus-core::AnimusState` (`crates/animus-core/src/lib.rs`). Core validation produces `animus-core::CoreError` and does not read ambient host state.
3. **Reusable loop input/config** is owned by `animus-runtime::RuntimeInput` and `animus-runtime::RuntimeConfig` (`crates/animus-runtime/src/contract.rs`). Runtime config is explicit data such as max-turn budgets, not ambient process state.
4. **Steel loop decision input** is owned by `animus-steel::LoopTurn` and `animus-steel::SteelLoopFabric` (`crates/animus-steel/src/turn.rs` and `fabric.rs`). `animus-runtime::TurnState` projects validated runtime state into this scriptable loop boundary for each turn.
5. **Steel loop decisions** are owned by `animus-steel::LoopDecision` (`crates/animus-steel/src/action.rs`). Rust revalidates every `CallModel`, `CallTool`, `Respond`, and `Stop` decision before a host adapter receives it.
6. **Adapter dispatch DTOs** are owned by `animus-adapters::ModelRequest`, `animus-adapters::ModelResponse`, `animus-adapters::ToolRequest`, `animus-adapters::ToolResponse`, `ModelAdapter`, and `ToolAdapter` (`crates/animus-adapters/src/`). These are implementation-free contracts.
7. **Host status-observation DTOs** are owned by `animus-adapters::AgentStatusObservation`, `HostStatusRequest`, `HostStatusObservationBatch`, and `HostStatusAdapter`. Hosts may detect agent status however they choose, but reusable crates receive only typed observations.
8. **Reusable status authority selection** is owned by `animus-runtime::select_agent_status`, `AgentStatusSelection`, and `StatusDecisionExplanation`. The selector is pure over a validated target request, logical sequence, and in-memory observations; presentation metadata remains visual-only.
9. **Reusable runtime dispatch and projection** are owned by `animus-runtime::run_agent_loop`, `LoopFabric`, `RuntimeEvent`, `RunReport`, and `RuntimeStatus` (`crates/animus-runtime/src/`). The runtime shell calls injected fabrics/adapters, records ordered events, handles stop/final/max-turn/failure statuses, and does not discover providers or credentials.
10. **Std-shell adapter dispatch** is owned by `animus-std::run_with_model`, `run`, and `SummaryModelAdapter` (`crates/animus-std/src/execution.rs`). The shell wires typed config, core state, and caller-provided adapters without discovering providers or credentials. The legacy **Runtime event/report projection** remains represented by `animus-std::RunReport` and `animus-std::RunError` for this shell path.
11. **Safe receipt construction** is future host-owned behavior. Until a receipt DTO lands, evidence summaries must follow `docs/cairn-lifecycle.md`, `docs/status-authority.md`, and `docs/safety-contract.md`: bounded status, issue codes, counts, safe relative labels, BLAKE3 hashes where content identity is needed, and explicit redaction markers.
12. **Host ledger persistence seam** is future host-owned behavior. The source-of-truth placeholder is a future host-owned `LedgerService` trait or DTO set; reusable core, adapter, Steel, and runtime crates must not persist sessions directly.

## Boundary owners

| Boundary | Owning crate | Source-of-truth type or trait | Host authority |
| --- | --- | --- | --- |
| Host runtime input | `animus-std` | `RuntimeConfig`, `Runtime::from_env_pairs`, `load_runtime` | Process environment is read only by `load_runtime`. |
| Core state | `animus-core` | `Intent`, `AnimusState`, `CoreError` | No host authority. |
| Reusable loop input/config | `animus-runtime` | `RuntimeInput`, `RuntimeConfig` | No ambient host authority. |
| Steel turn input | `animus-steel` / `animus-runtime` | `LoopTurn`, `TurnState`, `LoopFabric` | Fabric is injected; no concrete providers or tools are constructed. |
| Steel decision | `animus-steel` | `LoopDecision`, `SteelLoopFabric`, `LoopError` | No adapter dispatch until Rust revalidation passes. |
| Model/tool DTOs | `animus-adapters` | `ModelRequest`, `ModelResponse`, `ToolRequest`, `ToolResponse` | No implementation authority. |
| Status observation DTOs | `animus-adapters` | `AgentStatusObservation`, `HostStatusRequest`, `HostStatusObservationBatch` | Typed observations only. |
| Adapter ports | `animus-adapters` | `ComponentAdapter`, `ModelAdapter`, `ToolAdapter`, `HostStatusAdapter` | Concrete host implementation is injected by callers. |
| Status authority selection | `animus-runtime` | `select_agent_status`, `AgentStatusSelection`, `StatusDecisionExplanation` | Pure selection over typed inputs only. |
| Runtime dispatch | `animus-runtime` | `run_agent_loop`, `run_with_steel_program`, `RuntimeEvent`, `RunReport`, `RuntimeStatus` | Calls injected fabric/model/tool boundaries only. |
| Std dispatch | `animus-std` | `run_with_model`, `run`, `SummaryModelAdapter` | Shell orchestration only; no provider discovery. |
| Receipts | host-owned future surface | Future receipt DTO | Must be redacted and bounded before persistence. |
| Ledger persistence | host-owned future surface | Future host-owned `LedgerService` trait | Host-owned persistence only. |

## Forbidden dependencies and side effects

### Reusable core boundary

`animus-core` must not read environment variables, filesystem paths, credentials, provider config, network resources, clocks, randomness, process state, terminal state, or concrete host persistence. Validate with `cargo test -p animus-core` and the no-std/Wasm rail.

### Adapter contract boundary

`animus-adapters` must not implement providers, discover tools, read credentials, open network connections, execute processes, read files, or persist sessions. It owns DTO validation and implementation-free traits only, including host status-observation DTOs and the `HostStatusAdapter` port. Validate with `cargo test -p animus-adapters` and dependency rails.

### Status authority boundary

`animus-runtime::select_agent_status` must remain pure over typed in-memory observations, a logical sequence, and a validated target request. Trusted current lifecycle observations suppress heuristic fallback; stale, untrusted, or conflicting reports degrade deterministically without reading host state. Presentation metadata must remain visual-only and must not drive waits, notifications, rollups, or policy decisions.

### Steel boundary

`animus-steel` may evaluate Steel scripts, but every script-produced action must re-enter Rust validation through `LoopDecision`, `ModelRequest`, `ToolRequest`, or `ModelResponse` before dispatch. It must not silently bypass adapter contracts, weaken policy, or persist host state. Validate with `cargo test -p animus-steel`.

### Reusable runtime boundary

`animus-runtime` owns pure turn-state transitions and a thin dispatch shell over injected loop fabrics plus model/tool adapters. It must not construct provider clients, discover tools, read provider credentials, open network connections, execute processes, read files, inspect environment variables, use clocks, or perform concrete persistence. Validate with `cargo test -p animus-runtime`.

### Std-shell boundary

`animus-std` owns process-facing orchestration such as `load_runtime`, but deterministic parsing belongs in testable helpers such as `Runtime::from_env_pairs`. It must not construct provider clients, read provider credentials, open network connections, or perform concrete persistence unless a future adapter contract and Cairn change accepts that surface. Validate with `cargo test -p animus-std`.

### Receipt boundary

Receipt and evidence code must not embed raw prompts, credentials, provider payloads, raw tool output, raw terminal snapshots, unbounded stdout/stderr, unbounded detector transcripts, full private paths, or raw private file bodies. Status-decision explanations must use safe hashes and bounded redacted summaries. Validate current lifecycle evidence with `scripts/cairn/check-evidence-safety --root .`.

### Future ledger boundary

A future ledger service must be host-injected. Reusable crates must not discover sessions, read host persistence, or resume conversations by ambient global state. Until the service exists, resume behavior is a documented seam, not an implemented claim.

## Validation commands

Use the smallest command that covers the changed boundary:

```sh
cargo test -p animus-core
cargo test -p animus-adapters
cargo test -p animus-steel
cargo test -p animus-runtime
cargo test -p animus-std
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-evidence-safety --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cairn -- validate --root .
```

Closeout evidence should name which commands ran, which boundaries they cover, and any skipped checks with a blocker and next-best validation path.
