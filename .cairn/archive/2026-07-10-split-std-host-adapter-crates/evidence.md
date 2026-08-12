# Evidence: Split std host adapters into authority-scoped crates

## Baseline

- PASS: `nix develop -c cargo test -p animus-std --locked` before implementation.
- PASS: `nix develop -c cargo test -p animus-adapter-observability --locked` before extraction.
- PASS: lifecycle validation, roadmap validation, and proposal/design/tasks gates before implementation.

## Implementation

- Added the pure `animus_std::host_adapter_catalog` inventory for config, local context, provider registry, observability, document ingest, model-evidence ingest, optional Iroh/Syndicate transports, and smoke/demo shells.
- Added explicit environment, filesystem, process, network, credential, persistence, terminal, and no-host-effect authority classes.
- Added pure `plan_host_adapter` admission over explicit availability and fallback posture. Missing, disabled, denied, and ambient-fallback requests fail closed without host discovery.
- Extracted deterministic no-op and in-memory sinks into `animus-std-observability`, which depends only on `animus-adapter-observability`.
- Preserved `animus_std::{NoopObservabilitySink, InMemoryObservabilitySink}` through the default `observability-host` compatibility feature and added direct dependency guidance.
- Updated component catalog, adapter conformance index, runtime facade inventory/checker, request lifecycle, safety contract, std/observability docs, contracts index, and README guidance.

## Verification

- PASS: `nix develop -c cargo test -p animus-std-observability --locked`; positive direct-sink and negative unavailable/closed/leak fixtures passed.
- PASS: `nix develop -c cargo test -p animus-std authority --locked`; explicit-authority and missing/disabled/denied/fallback cases passed.
- PASS: `nix develop -c cargo test -p animus-std observability --locked`; compatibility facade parity passed.
- PASS: `nix develop -c cargo test -p animus-std --locked`.
- PASS: `nix develop -c cargo check -p animus-std --no-default-features --locked`.
- PASS: `nix develop -c cargo check -p animus --locked`.
- PASS: `nix develop -c cargo check --workspace --all-targets --locked`.
- PASS: focused Clippy for `animus-std-observability` and `animus-std` with warnings denied.
- PASS: `nix develop -c cargo fmt --all -- --check`.
- PASS: `scripts/cairn/check-adapter-conformance-index --root .`.
- PASS: `scripts/cairn/check-component-catalog --root .`.
- PASS: `scripts/cairn/check-runtime-facade-inventory --root .`.
- PASS: `scripts/cairn/check-request-lifecycle --root .`.
- PASS: `scripts/cairn/check-composition-conformance --root .`.
- PASS: `nix run .#cairn -- validate --root .`.
- PASS: proposal, design, and tasks gates before task closeout.

## Non-claims

This change does not add live providers, default credential discovery, default persistence, default network transport, daemon behavior, terminal ownership, product CLI behavior, ambient filesystem scans, or ambient fallback.
