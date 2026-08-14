# Live Steel agent shell

The `animus-live-steel-agent` package is a non-default headless application. It has an `AGPL-3.0-or-later` license because it composes `animus-std-rlm-host`.

## Composition boundary

`run_live_application` is the visible composition root. It selects these concrete parts:

- the tool-capable OpenAI Responses host;
- separate durable coding-session and private Steel-kernel journals;
- `CodingSessionHost` for message and tool-loop ordering;
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

`scripts/config/export-nickel-configs` writes the deterministic JSON file. Rust validates the complete file before it opens a store or contacts a provider.

The binary runs the maintained composition activation preflight before it reads the credential. The library entrypoint repeats the same pure check before store or provider effects. A blocked result returns `LiveApplicationError::Activation` without an ambient fallback. See [composition activation graph](composition-activation-graph.md).

The binary accepts `<store-root> <prompt>`. It reads one explicit credential value from standard input. It does not read an environment variable, provider account, current directory, or ambient credential store.

## Durable ordering

`CodingSessionHost` commits the assistant tool call before `steel_repl` dispatch. It commits the correlated tool result before provider continuation.

The session and kernel stores use separate journals and exclusive leases. The shell releases both leases after the final safe point.

A present kernel snapshot must restore exactly. Policy or journal drift fails closed. Interrupted-cell restore returns the prior committed kernel and does not replay an observed host effect.

## Child and host authority

The RLM executor contains only the configured `context_read` and `subagent` routes. Each host effect receives a fresh authority observation.

Child requests must match the configured profile and bounds. The child service creates a normal `CodingSessionHost` controller. It does not create a second agent loop.

Steel modules are reviewed source values. Native modules, FFI, dynamic libraries, arbitrary imports, package discovery, and automatic activation are denied.

## Model-request snapshot prerequisite

The maintained activation profile requires the pure snapshot core and the snapshot host shell. This requirement makes the private commit boundary explicit in the live composition.

Provider-native projection, credentials, and transport remain outside the snapshot. See [durable model-request snapshots](model-request-snapshots.md).

## Evidence

The deterministic fixture rail is:

```sh
scripts/cairn/check-live-steel-agent-shell --root .
```

The rail runs positive and negative tool-loop, child, configuration, cancellation, host-failure, durable restart, and restore-drift checks. The composition activation rail adds startup-order and fail-loud coverage. It also runs Clippy with warnings denied.

Receipts contain safe labels, counts, BLAKE3 identities, one fixed omission marker, and explicit non-claims. They omit prompts, provider bodies, tool arguments, Steel source, host results, child content, credentials, and paths.

A real provider run is separate and opt-in. Without that current-head run, provider correctness remains a non-claim.

The shell also does not claim tool correctness, task success, sandboxing, daemon readiness, terminal readiness, production readiness, or release eligibility.
