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

## Prerequisite record

The shell composes prerequisite code from the same checked workspace revision. The closeout commit binds those exact source identities.

The shell package uses `AGPL-3.0-or-later`. The workspace crates otherwise retain their declared `MIT OR Apache-2.0` or AGPL licenses.

The `live-provider` feature is explicit and disabled by default. The package also stays outside workspace `default-members`.

The current compatibility boundary uses public `animus-std-openai` production APIs. It does not use crate-private transport seams.

Live provider availability and model behavior remain unresolved until a separate credentialed current-head smoke runs.

## Configuration and credentials

`config/live-steel-agent-shell.ncl` is the human-authored configuration. It defines exact component labels, provider cohort, model, credential reference, store names, authority identities, operations, skills, and finite bounds.

The shell admits three provider tuples. `openai` binds the official OpenAI Responses endpoint to `brokered:injected-openai-api-key`. `openrouter` binds the OpenRouter Responses endpoint to `brokered:injected-openrouter-api-key`. `local-dspark` binds the exact private Responses endpoint to `brokered:injected-local-dspark-api-key` and requires disabled reasoning.

The shell rejects crossed tuples, arbitrary URLs, unknown providers, credential-reference drift, reasoning drift, discovery, OAuth, and fallback. The existing `animus-std-openai` host remains the only network adapter.

A product can also admit the exact `tool` host operation. The operation remains unavailable unless the product injects a matching route, executor-kit binding, capability, and durable commit service.

`scripts/config/export-nickel-configs` writes the deterministic JSON file. Rust validates the complete file before it opens a store or contacts a provider.

The binary runs the maintained composition activation preflight before it reads the credential. The library entrypoint repeats the same pure check before store or provider effects. A blocked result returns `LiveApplicationError::Activation` without an ambient fallback. See [composition activation graph](composition-activation-graph.md).

The binary accepts `<store-root> <prompt>`. It reads one explicit credential value from standard input for the admitted provider. It does not read an environment variable, provider account, current directory, or ambient credential store.

## Durable ordering

`CodingSessionHost` commits the assistant tool call before `steel_repl` dispatch. The tool adapter then runs the fixed staged execution plan through final validation.

The shell keeps the pending staged result and private safe-point journal bound together. It completes post-commit observation only after the exact correlated result commit.

Provider continuation cannot observe the result before that transition completes. See [tool execution stages](tool-execution-stages.md).

The session and kernel stores use separate journals and exclusive leases. The shell releases both leases after the final safe point.

`LiveSteelAgentComposition::open_session` keeps the provider, model-request snapshot store, coding-session store, Steel tool, and kernel persistence inside one owning session. It begins commands without driving unrelated effects. `drive_one` executes at most one existing `CodingSessionHost` effect.

Workers can queue follow-up or steering input, apply queue edits or removals with exact generation-and-identity guards, request active-run cancellation, and read bounded private event batches. Stale guards, malformed correlations, zero bounds, over-bound batches, and stale cursors fail closed.

A present kernel snapshot must restore exactly. Policy or journal drift fails closed. Interrupted-cell restore returns the prior committed kernel and does not replay an observed host effect.

## Child and host authority

The RLM executor contains only the configured `context_read` and `subagent` routes. Each host effect receives a fresh authority observation.

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
