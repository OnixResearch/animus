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

Current family records cover config loading, local context, provider registry and credential diagnostics, observability, document ingest, model-evidence ingest, optional Iroh and Syndicate transports, and the packaged smoke/demo shell. An authority label describes a possible shell effect; it does not grant authority by itself.

`plan_host_adapter` is pure over an explicit family, availability state, and fallback posture. Missing, disabled, or denied blocks return typed denied plans. Any request to discover an ambient environment, filesystem, provider, network, credential, or persistence fallback is denied even if another block is available.

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

## Functional core and shell

Deterministic config parsing, provider selection, context planning, document/model-evidence ingest planning, observability record validation, and host adapter admission remain pure over explicit values. Filesystem opens, environment reads, process calls, transport calls, credential lookup, durable storage, terminal access, and other effects stay in thin owner shells after a plan is admitted.

## Verification and non-claims

Focused checks:

```sh
cargo test -p animus-std-observability --locked
cargo test -p animus-std authority --locked
cargo test -p animus-std observability --locked
cargo check -p animus-std --no-default-features --locked
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-request-lifecycle --root .
```

Positive fixtures cover explicit available blocks and direct/facade observability sinks. Negative fixtures cover missing, disabled, denied, unavailable, closed, unsafe-record, and ambient-fallback paths.

This split does not add live providers, default credential discovery, default persistence, default network transport, daemon behavior, terminal ownership, or product CLI behavior.
