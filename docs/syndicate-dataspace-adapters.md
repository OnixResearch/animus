# Syndicate dataspace adapter boundary

Animus treats Syndicate-style dataspaces as an optional host adapter boundary, not as reusable runtime state. The default `animus-core`, `animus-runtime`, and `animus-adapters` dependency graphs must remain free of Syndicate crates, daemon clients, and live transport dependencies.

## Contract

The `animus-adapters` crate exposes dataspace DTOs for:

- bounded assertion projection over runtime events, tool states, background work, session/provenance summaries, and agent/subagent lifecycle states;
- explicit tenant, workspace, session, branch, facet, and actor scoping;
- safe subscriptions that reject raw-payload requests, cross-workspace queries, and unsupported query shapes;
- host orchestration events for cancellation, expiry, publish acknowledgement, and ownership transfer.

Assertions carry only stable IDs, status classes, BLAKE3 hashes, counts, and redaction markers. Prompts, provider payloads, credentials, full paths, file bodies, tool payloads, model output, and unbounded logs are omitted by default.

## Optional live integration

A future host may configure a live Syndicate transport outside default reusable crates. That integration must provide its own host configuration, credentials handling, daemon/network setup, and smoke-test evidence. Until then, live integration is unavailable by design; deterministic fake-dataspace tests are the release evidence for this boundary.

## Verification

Run the focused rail:

```sh
cargo test -p animus-adapters --locked dataspace::
scripts/cairn/check-syndicate-dataspace-isolation --root .
nix build .#checks.x86_64-linux.syndicate-dataspace-isolation --no-link -L
```
