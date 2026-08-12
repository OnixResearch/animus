# Evidence: add-syndicate-host-dataspace-adapter

r[verify agent_harness_runtime.syndicate_host_adapter.closeout]

## Implementation evidence

- Added non-default `animus-std` feature `syndicate-host`.
- Added a feature-gated host seam with `SyndicateTransport`, `SyndicateHostDataspaceAdapter`, `SyndicateObservationKind`, `SyndicateObservationDraft`, and `SyndicateAdapterError`.
- Kept projection pure through `project_observation`, `build_safe_subscription`, and `admit_scoped_orchestration`; the adapter shell only forwards validated DTOs to caller-owned transport.
- Projection covers runtime, headless, tool lifecycle, background-work, session/provenance, agent, and subagent observations while emitting redaction markers, BLAKE3 safe hashes, counts, status classes, and scope labels only.
- Added positive and negative memory-transport tests for redacted projection, missing redactions, raw sensitive markers, safe subscription, scoped cancellation, cross-scope rejection, and unavailable backend failure.
- Added `docs/syndicate-host-dataspace-adapter.md` and facade inventory entries.

## Checks

- `nix develop -c cargo test -p animus-std --features syndicate-host dataspace --locked` — passed.
- `nix develop -c cargo test -p animus-std --features syndicate-host --locked` — passed.
- `nix develop -c cargo check -p animus-runtime --no-default-features --locked` — passed.
- Dependency isolation: `cargo tree` checks for default `animus-adapters`, `animus-runtime --no-default-features`, and default `animus-std` showed no `syndicate` dependency leak.
- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks add-syndicate-host-dataspace-adapter --root .` — passed.

## Blocked rails

- Live Syndicate smoke testing is intentionally out of scope without explicit host transport config and daemon availability; deterministic memory/fake backend tests are the release evidence.
- Combined `animus-std` clippy remains blocked by existing `crates/animus-std/src/providers/capability/flow.rs` `too_many_arguments` lint; full log captured at `target/focused-clippy-preserves-std-nodeps.log`.
