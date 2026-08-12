# Proposal: Add concrete std runtime adapter follow-ups

## Why

Several planned changes intentionally stop at reusable contracts: the runtime loop does not add real providers, tool contracts do not add filesystem/process tools, provider registry work does not create network clients, and policy work does not execute commands. Animus needs explicit follow-up Cairns for the first concrete std-shell adapters so implementation work does not leak into core crates or land as ad hoc examples.

This change defines the boundary for concrete host adapters while preserving the adapter-first architecture.

## What Changes

- Define the initial `animus-std` adapter set for model provider clients, filesystem tools, process execution, HTTP/network helpers, and local fixture adapters.
- Require every concrete adapter to consume accepted runtime, policy, sandbox, receipt, event, and tool contracts before performing effects.
- Add typed adapter capability descriptors, denial/failure classes, redacted receipts, and fixture backends.
- Add positive tests for successful injected adapter calls and negative tests for missing credentials, denied capabilities, sandbox mismatch, malformed outputs, timeout/cancellation, and leak attempts.
- Keep live provider and host-effect tests opt-in and separate from deterministic default checks.

## Impact

- **Files**: `animus-std` adapter modules, fixture adapters, policy/sandbox integration points, docs, tests, and Cairn artifacts.
- **Testing**: deterministic fake-adapter tests by default; opt-in live/smoke tests only behind explicit host configuration.
- **Non-claims**: this does not move providers, filesystem mutation, process spawning, HTTP clients, credentials, or sandbox enforcement into reusable runtime/core crates.
