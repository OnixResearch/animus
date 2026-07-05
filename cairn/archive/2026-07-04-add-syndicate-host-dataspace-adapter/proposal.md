# Proposal: Add optional Syndicate host dataspace adapter

## Why

Animus has a typed dataspace boundary and deterministic fake backend, but no host-owned path for live coordination. The places where Syndicate-style dataspaces help most are cross-agent status, subagent/task orchestration, background work progress, TUI/headless fan-out, cancellation, leases, and ownership transfer. Those are coordination concerns, not deterministic core state.

This change adds a feature-gated host adapter plan for publishing redacted Animus coordination assertions to a Syndicate-compatible dataspace while keeping reusable runtime/core crates free of live transport dependencies.

## What Changes

- Add an optional `animus-std` host dataspace adapter that implements `animus_adapters::dataspace::AdapterService` behind a non-default feature or explicit host wiring.
- Map runtime, headless, tool lifecycle, background-work, session/provenance, and agent/subagent states into bounded redacted dataspace assertions.
- Accept only validated safe subscription shapes and host orchestration events for cancellation, expiry, publish acknowledgement, and ownership transfer.
- Add host configuration for live transport location and credentials outside reusable core/runtime/adapter crates.
- Preserve deterministic fake-dataspace tests as release evidence and make live transport smoke tests opt-in.
- Extend dependency-isolation checks so Syndicate crates, daemon clients, and network transports cannot leak into default reusable dependency graphs.

## Impact

- **Files**: `crates/animus-std` optional host adapter, dataspace projection helpers, docs, dependency-isolation script/checks, tests, Cairn evidence.
- **Testing**: fake backend, redacted projection fixtures, safe subscription/orchestration tests, unavailable-backend failures, dependency isolation, optional live smoke test when configured, workspace checks, and Cairn gates.
- **Non-claims**: this does not make Syndicate a default dependency, replace the runtime event ledger, expose raw prompts/provider/tool/model/file material, or require a live daemon for normal tests.
