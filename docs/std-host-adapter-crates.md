# Authority-scoped std host adapters

Animus treats the standard-library shell as a composition of explicit authority blocks rather than one ambient host bundle. `animus_std::host_adapter_catalog()` is the metadata source for family ownership, feature gates, required authority, compatibility facade paths, smallest direct dependencies, and focused checks.

## Authority catalog

The catalog distinguishes these classes explicitly:

- `no_host_effect`;
- `environment_read`;
- `filesystem_read`;
- `process_execution`;
- `network_transport`;
- `credential_access`;
- `persistence`; and
- `terminal_access`.

Current family records cover config loading, local context, provider registry, the live OpenAI host, the read-only codebase tool, observability, ingest, optional transports, enforcement, persistence, coding sessions, and smoke shells. An authority label describes a possible shell effect; it does not grant authority by itself.

`plan_host_adapter` is pure over an explicit family, availability state, and fallback posture. Missing, disabled, or denied blocks return typed denied plans. Any request to discover an ambient environment, filesystem, provider, network, credential, or persistence fallback is denied even if another block is available.

## Optional live OpenAI provider host

`animus-std-openai` is a separate authority-scoped crate behind the root `openai-provider-host` feature. It requires network-transport and credential-access authority.

The pure core validates one exact OpenAI Responses endpoint and a text-only request cohort. The optional `reqwest-client` shell performs one bounded HTTPS attempt.

The host does not discover endpoints, models, providers, credentials, retries, or fallbacks. See [live OpenAI provider host](live-openai-provider-host.md).

## Optional read-only codebase tool host

The `animus-std/codebase-tool-host` feature adapts one caller-constructed capability-root graph through the reusable executor kit. It binds exact request values, returns bounded model-visible facts, and retains metadata-only durable evidence.

The host does not discover roots or files. It adds no mutation, process, network, provider, or plugin authority. See [read-only codebase tool host](read-only-codebase-tool-host.md).

## First extracted family: observability

`animus-std-observability` is the first direct authority-scoped std crate. It depends only on `animus-adapter-observability` and provides deterministic `NoopSink` and `InMemorySink` implementations. It owns no filesystem, process, network, credential, terminal, clock, global subscriber, or persistence authority.

Existing imports remain valid through the default `animus-std/observability-host` facade:

```rust
use animus_std::{InMemoryObservabilitySink, NoopObservabilitySink};
```

New embedders that need only these sinks can use the smaller direct dependency:

```rust
use animus_std_observability::{InMemorySink, NoopSink};
```

Building `animus-std` with `--no-default-features` omits the observability sink facade. The pure authority catalog remains available so composition code can return a typed `Disabled` or `Missing` plan instead of discovering an ambient sink.

## External agent control

`animus-std-control` is a focused direct std crate. It uses an explicit mutex state latch, condition variable, and bounded event journal over the pure runtime reducer.

The root `agent-control` feature re-exports this crate as `animus::agent_control`. The host owns no socket, network, persistence, clock, process, credential, terminal, or global-registry authority.

A blocked waiter performs no polling and consumes no model tokens. Slow subscribers receive an explicit lag outcome and cannot block command publication. See [external agent control](agent-control-pubsub.md).

## Optional Sandlock enforcement

The non-default Linux-only `sandlock-host` feature adds the `animus_std::sandlock_host` authority block. Its pure planner binds prior authorization, normalized sandbox intent, safe relative capability-root bindings, exact command and policy identities, required mechanisms, and BLAKE3 plan/receipt identities. Its Linux shell alone resolves concrete paths, probes the exact pinned `sandlock-core` cohort, launches a process, drains bounded output, observes timeout/cancellation, and classifies COW and cleanup. Unsupported platforms and unavailable mechanisms fail closed without fallback.

The catalog labels this block with filesystem-read, process-execution, and network-transport authority. Those labels describe possible effects after admission; they do not authorize the action or permit ambient root, command, credential, or adapter discovery.

## Optional BranchFS speculative workspaces

The non-default `branchfs-host` feature adds `animus_std::branchfs_host`. Its pure planner admits only root-level disposable siblings and has no BranchFS commit operation. The thin shell uses injected process, candidate-execution, bounded-manifest, Jujutsu, materialization, and abort services; promotion requires source freshness, a fresh Jujutsu workspace, and post-copy manifest equality. See [BranchFS speculative host](branchfs-host.md).

## Optional durable local session store

The non-default `durable-session-store` feature adds the persistence-authority block exported as `animus_std::{DurableSessionLocalStore, open_durable_session_store}`. It requires an explicit capability directory, relative journal/lock names, generated-policy-bound descriptor, and lease identity. `animus-runtime` owns the pure bounded framing, lineage, identity, recovery, and receipt core; the std block alone owns file permissions, exclusive lease creation, append, `sync_all`, and reopen effects. It never selects an ambient or global store. See [durable local session store](durable-session-store.md).

## Optional coding-session host

The non-default `coding-session-host` feature adds `animus_std::coding_session_host`. It composes the pure `animus_runtime::coding_session_host` reducer with explicit provider, tool, persistence, policy, cancellation, timer, summarizer, and event services. `DurableCodingSessionPersistence` adapts only the separately enabled explicit durable local store. The shell never discovers providers, tools, credentials, resources, commands, sessions, stores, or terminal behavior. A prerequisite message commit must succeed before a dependent provider or tool effect. See [coding-session host facade](coding-session-host.md).

## Functional core and shell

Deterministic config parsing, provider selection, context planning, document/model-evidence ingest planning, observability record validation, and host adapter admission remain pure over explicit values. Filesystem opens, environment reads, process calls, transport calls, credential lookup, durable storage, terminal access, and other effects stay in thin owner shells after a plan is admitted.

## Optional ACP stdio host crate

`animus-acp-stdio-host` is intentionally separate from `animus-std` and absent from default members. It depends on the non-default coding-session feature and owns only the reviewed ACP schema mapping plus bounded local stdio shell. Reusable crates and default/no-std/Wasm feature graphs do not acquire its ACP cohort or entrypoint. See [`acp-stdio-host.md`](acp-stdio-host.md).

## Optional Bubblewrap agent-guardrail crate

`animus-std-agent-guardrail` is intentionally separate from `animus-std` and absent from default members. The Linux-only `agent-guardrail` root feature supplies a pure authorization-bound planner plus an explicit-file/executable/process shell using a private read-only root and sealed Bubblewrap option memfd. It performs no project-policy, host-root, environment-value, executable, shell, or fallback discovery and remains claim-distinct from Sandlock and Mantle. See [`agent-guardrail.md`](agent-guardrail.md).

## Optional Lean 4 loop-fabric crate

`animus-std-lean-loop` is intentionally separate from `animus-std` and absent from default members. The `lean-loop` root feature supplies one exact process-backed implementation of `animus_runtime::LoopFabric`; it verifies an explicit compiled Lean artifact, clears the environment, owns bounded JSON-line pipes/deadlines/cancellation/cleanup, and reconstructs candidates through current Rust model/tool/response contracts. It does not own runtime sequencing, adapters, sessions, transcripts, or completion, and it never falls back to Steel. See [`lean4-loop-fabric.md`](lean4-loop-fabric.md).

## Verification and non-claims

Focused checks:

```sh
cargo test -p animus-std-observability --locked
cargo test -p animus-std-control --locked
cargo check -p animus --features agent-control --locked
scripts/cairn/check-live-openai-provider-host --root .
scripts/cairn/check-codebase-tool-host --root .
cargo test -p animus-std hostadapters --locked
cargo test -p animus-std observability --locked
cargo check -p animus-std --no-default-features --locked
cargo test -p animus-std --features sandlock-host sandlock_host --locked
scripts/cairn/check-sandlock-host --root .
scripts/cairn/check-branchfs-host
scripts/cairn/check-durable-session-store --root .
scripts/cairn/check-coding-session-host --root .
scripts/cairn/check-agent-guardrail --root .
scripts/cairn/check-lean4-loop-fabric --root .
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-request-lifecycle --root .
```

Positive fixtures cover explicit available blocks and direct/facade observability sinks. Negative fixtures cover missing, disabled, denied, unavailable, closed, unsafe-record, and ambient-fallback paths.

This split does not add live providers, default credential discovery, default persistence, default network transport, daemon behavior, terminal ownership, or product CLI behavior. Agent-control waiting does not prove external delivery, durable pause, provider cancellation, tool rollback, or exactly-once effects.
