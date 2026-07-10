# Adapter family crates

Animus is splitting low-risk adapter families into focused no-std crates while preserving `animus-adapters` as the compatibility facade.

## Extracted families

| Family | Direct crate | Compatibility facade | Authority posture | Focused checks |
| --- | --- | --- | --- | --- |
| Safe receipts | `animus-adapter-safe` | `animus-adapters` safe receipt aliases remain available | reusable DTO validation only; no filesystem, process, network, credential, terminal, clock, or persistence authority | `cargo test -p animus-adapter-safe --locked`; `cargo test -p animus-adapters adapter_family_crates --locked`; `scripts/cairn/check-adapter-family-crates --root .` |
| Observability | `animus-adapter-observability` | `animus-adapters::{MetricSample, LogRecord, MetricsAdapter, LoggingAdapter}` re-export direct crate items | reusable metric/log DTO validation only; concrete sinks remain host-owned | `cargo test -p animus-adapter-observability --locked`; `cargo test -p animus-adapters adapter_family_crates --locked`; `scripts/cairn/check-adapter-family-crates --root .` |

## Migration rule

New embedders should depend on the smallest direct family crate. Existing imports through `animus-adapters` continue to compile while the migration window stays open.

## Non-claims

This split does not add concrete providers, tools, memory stores, workspaces, networks, plugin runtimes, credentials, persistence, or new adapter behavior.
