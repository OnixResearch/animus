# Live Steel agent shell

The `animus-live-steel-agent` package is a non-default headless application. It has an `AGPL-3.0-or-later` license because it composes `animus-std-rlm-host`.

## Composition boundary

`run_live_application` is the visible composition root. It selects these concrete parts:

- the tool-capable OpenAI-compatible Responses host with an exact OpenAI, OpenRouter, or private local-dspark endpoint tuple;
- separate durable coding-session and private Steel-kernel journals;
- `CodingSessionHost` for message and tool-loop ordering;
- the fixed staged tool shell with no optional handlers by default;
- `SteelRlmKernel` and `execute_bridged_cell`;
- `RlmHostExecutor` with explicit context and child routes;
- `ConcreteChildSessionService` with `NormalCodingSessionController`;
- reviewed Steel skill modules;
- explicit cancellation checkpoints;
- metadata-only events and receipts.

The package is a workspace member, but it is not a default member. Its `live-provider` feature is also off by default.

## Argument and recovery boundaries

The binary acquires process arguments through the locked `clap_lex` 1.1.0 lexer after activation preflight. Its adapter admits exactly two positional values. Spaces, empty strings, and flag spellings remain literal values. Missing values, extra values, and invalid UTF-8 return the existing usage text. Credential input and product admission keep their existing order.

`MemoryCodingSessionPersistence::empty()` constructs the canonical empty in-memory store without I/O. `Default` delegates to this constructor.

`CodingSessionHost::resume_with_gate` adds a caller-owned effect gate to recovery. It retains the current-authority probe after successful recovery. The existing `resume` method keeps its prior behavior. This product selects read-only recovery and refuses automatic repair writes.

## Portable count migration

`FixtureStreamProvider::new` now accepts a `u64` response bound. Its `remaining()` method now returns `Result<u64, ProviderStreamError>`. `MetadataChildEvents::len()` returns `u64`. The crate root re-exports `ProviderStreamError`.

These public changes are not source-compatible with machine-width counts or infallible fixture-count callers. Callers must handle conversion errors and compare counts as `u64`.

Private modules now separate configuration admission, provider adapters, composition, session ownership, evaluation, persistence, and evidence. The public facade retains the existing concept names.

## Prerequisite record

The shell composes prerequisite code from the same checked workspace revision. The closeout commit binds those exact source identities.

The shell package uses `AGPL-3.0-or-later`. The workspace crates otherwise retain their declared `MIT OR Apache-2.0` or AGPL licenses.

The `live-provider` feature is explicit and disabled by default. The package also stays outside workspace `default-members`.

The current compatibility boundary uses public `animus-std-openai` production APIs. It does not use crate-private transport seams.

Live provider availability and model behavior remain unresolved until a separate credentialed current-head smoke runs.

## Configuration and credentials

`config/live-steel-agent-shell.ncl` is the human-authored configuration. It defines exact component labels, provider cohort, model, credential reference, store names, authority identities, operations, skills, and finite bounds.

The shell admits three provider tuples. `openai` binds the official OpenAI Responses endpoint to `brokered:injected-openai-api-key`. `openrouter` binds the OpenRouter Responses endpoint to `brokered:injected-openrouter-api-key`. `local-dspark` binds the exact private Responses endpoint to `brokered:injected-local-dspark-api-key` and requires disabled reasoning.

The shell rejects crossed tuples, arbitrary URLs, unknown providers, credential-reference drift, reasoning drift, discovery, OAuth, and fallback. Live request planning preserves the validated `none` or `low` reasoning setting. The existing `animus-std-openai` host remains the only network adapter.

A product can also admit the exact `tool` host operation. The operation remains unavailable unless the product injects a matching route, executor-kit binding, capability, and durable commit service.

`scripts/config/export-nickel-configs` writes the deterministic JSON file. Rust validates the complete file before it opens a store or contacts a provider.

The binary runs the maintained composition activation preflight before it reads the credential. The library entrypoint repeats the same pure check before store or provider effects. A blocked result returns `LiveApplicationError::Activation` without an ambient fallback. See [composition activation graph](composition-activation-graph.md).

The binary accepts `<store-root> <prompt>`. It reads one explicit credential value from standard input for the admitted provider. It does not read an environment variable, provider account, current directory, or ambient credential store.

## Durable ordering

`CodingSessionHost` commits the assistant tool call before `steel_repl` dispatch. The tool adapter then runs the fixed staged execution plan through final validation.

The tool adapter persists the private kernel safe point before it returns the staged result. `CodingSessionHost` then commits the tool result. The adapter completes post-commit observation only after the exact correlated result commit.

Provider continuation cannot observe the result before that transition completes. See [tool execution stages](tool-execution-stages.md).

The session and kernel stores use separate journals and exclusive leases. Their commits do not form one transaction. An interruption between these commits can leave the kernel head ahead of the session tool result. The product refuses a recovery repair before that repair writes to the session journal. A completed turn reopens at `Idle` without provider contact.

The adapter validates stage commit facts before it consumes the pending stage. Rejected result identities, unsynchronized observations, and duplicate result bindings retain that stage. The exact accepted observation completes it once. A later duplicate returns `Stale`.

A continuation becomes blocked before snapshot persistence. Snapshot refusal, provider failure, or terminal-persistence refusal cannot turn that attempt into a fresh request. Direct adapter controls attempt a second dispatch and verify unchanged provider input and persisted-record counts.

The interrupted-write control stops after a kernel commit and before the session result commit. It closes the leases and attempts recovery. It does not simulate a process kill, prove general journal-pair equivalence, or establish rollback or cross-journal atomicity.

The shell releases both leases after the final safe point.

`LiveSteelAgentComposition::open_session` keeps the provider, model-request snapshot store, coding-session store, Steel tool, and kernel persistence inside one owning session. It begins commands without driving unrelated effects. `drive_one` executes at most one existing `CodingSessionHost` effect.

Workers can queue follow-up or steering input, apply queue edits or removals with exact generation-and-identity guards, request active-run cancellation, and read bounded private event batches. Stale guards, malformed correlations, zero bounds, over-bound batches, and stale cursors fail closed.

A present kernel snapshot must restore exactly. Policy or journal drift fails closed. Interrupted-cell restore returns the prior committed kernel and does not replay an observed host effect.

## Child and host authority

The RLM executor contains only the configured `context_read` and `subagent` routes. Each host effect receives a fresh authority observation.

The product shell explicitly acquires its existing monotonic clock capability for each admitted cell. It uses the already-locked `cap-std` 3.4.5 adapter. The private checkpoint control receives that capability and the cell start instant. Reusable runtime code still receives elapsed-time facts through `RlmBridgeControl` and gains no clock authority.

The four-input `SteelToolService::new` constructor remains unchanged. Time starts after request admission and identity construction, immediately before bridge execution. Checkpoints remain cooperative. This repair adds no preemptive deadline and no companion cancellation API.

Child requests must match the configured profile and bounds. The child service creates a normal `CodingSessionHost` controller. It does not create a second agent loop.

Steel modules are reviewed source values. Native modules, FFI, dynamic libraries, arbitrary imports, package discovery, and automatic activation are denied.

## Model-request snapshot prerequisite

The maintained activation profile requires the pure snapshot core and the snapshot host shell. This requirement makes the private commit boundary explicit in the live composition.

The profile also requires the pure tool-stage core and its std shell. Missing staged execution blocks activation before tool effects.

Provider-native projection, credentials, and transport remain outside the snapshot. See [durable model-request snapshots](model-request-snapshots.md).

## Evidence

The deterministic fixture rail is:

```sh
scripts/cairn/check-live-steel-agent-shell --root .
```

The rail runs positive and negative tool-loop, persistent-session, incremental-drive, queue-control, cancellation, child, configuration, host-failure, durable restart, and restore-drift checks. The composition activation rail adds startup-order and fail-loud coverage. It also runs Clippy with warnings denied.

Completed run output exposes its exact ordered `RuntimeEvent` snapshot to the worker-owned delivery path. This private event batch is not evidence.

Receipts contain safe labels, counts, BLAKE3 identities, one fixed omission marker, and explicit non-claims. They omit runtime-event payloads, prompts, provider bodies, tool arguments, Steel source, host results, child content, credentials, and paths.

A real provider run is separate and opt-in. Without that current-head run, provider correctness remains a non-claim.

The shell also does not claim tool correctness, task success, sandboxing, daemon readiness, terminal readiness, production readiness, or release eligibility.
