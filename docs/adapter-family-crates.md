# Adapter family crates

Animus is splitting low-risk adapter families into focused no-std crates while preserving `animus-adapters` as the compatibility facade.

## Extracted families

| Family | Direct crate | Compatibility facade | Authority posture | Focused checks |
| --- | --- | --- | --- | --- |
| Safe receipts | `animus-adapter-safe` | `animus-adapters` safe receipt aliases remain available | reusable DTO validation only; no filesystem, process, network, credential, terminal, clock, or persistence authority | `cargo test -p animus-adapter-safe --locked`; `cargo test -p animus-adapters familycrates --locked`; `scripts/cairn/check-adapter-family-crates --root .` |
| Observability | `animus-adapter-observability` | `animus-adapters::{MetricSample, LogRecord, MetricsAdapter, LoggingAdapter}` re-export direct crate items | reusable metric/log DTO validation only; concrete sinks remain host-owned | `cargo test -p animus-adapter-observability --locked`; `cargo test -p animus-adapters familycrates --locked`; `scripts/cairn/check-adapter-family-crates --root .` |
| RLM kernel | `animus-adapter-rlm-kernel` | `animus-adapters::rlm_kernel` re-exports pure contracts | reusable policy, state, request, child, compaction, and receipt validation only; Steel evaluation and all host effects remain outside this crate | `cargo test -p animus-adapter-rlm-kernel --locked`; `cargo check -p animus-adapter-rlm-kernel --target wasm32-unknown-unknown --locked`; `scripts/cairn/check-rlm-kernel --root .` |
| Native evaluation services | `animus-adapter-eval` | `animus-adapters::evaluation` and `animus::evaluation_adapters` re-export contracts only under `evaluation-adapters` | reusable service catalog, separate ports, exact binding, bounded gateway, Harbor conversion, and receipt validation only; every effect remains host-owned | `cargo test -p animus-adapter-eval --locked`; `cargo check -p animus-adapter-eval --target wasm32-unknown-unknown --locked`; `scripts/cairn/check-evaluation-adapter-family --root .` |
| Harbor ATIF-v1.7 | `animus-adapter-atif` | `animus-adapters::atif` and `animus::harbor_atif` re-export contracts only under `harbor-atif`; JSON is under `json-codec` or `harbor-atif-json` | caller-byte codec, pure validation, graph checks, owner-specific drafts, loss reports, and metadata-only receipts; acquisition, resolution, persistence, promotion, and evaluation remain host-owned | `cargo test -p animus-adapter-atif --features json-codec --locked`; `cargo check -p animus-adapter-atif --target wasm32-unknown-unknown --no-default-features --locked`; `scripts/cairn/check-harbor-atif-adapter --root .` |
| Native evaluation host | `animus-std-eval` | `animus::evaluation_host` exposes the selected crate under `evaluation-host` | standard-library shell over explicit task, model, tool, agent, verifier, artifact, timer, cancellation, cleanup, and result services; no ambient discovery or default activation | `cargo test -p animus-std-eval --locked`; `cargo clippy -p animus-std-eval --all-targets -- -D warnings`; `scripts/cairn/check-std-evaluation-host --root .` |
| Basalt authority | `animus-std-basalt-authority` | `animus::basalt_authority` exposes the selected crate under `basalt-authority-adapter`; generic contracts remain in `animus-adapters` | non-default live raw-holder verification over caller-owned services; Basalt can deny but cannot widen local gates; `AGPL-3.0-or-later` review is required | `cargo test -p animus-std-basalt-authority --locked`; `cargo clippy -p animus-std-basalt-authority --all-targets -- -D warnings`; `scripts/cairn/check-basalt-authority-adapter --root .` |

## Migration rule

New embedders should depend on the smallest direct family crate. Existing imports through `animus-adapters` continue to compile while the migration window stays open.

## Non-claims

This split does not add concrete providers, tools, memory stores, workspaces, networks, plugin runtimes, credentials, persistence, or new adapter behavior.
