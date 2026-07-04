# Proposal: Preserve typed adapter call outcomes

## Why

Runtime dispatch currently accepts typed model and tool requests, but concrete adapter failures can be collapsed into display strings before they reach runtime reports. That loses safe machine-readable failure class, outcome kind, request identity, and receipt context at the exact boundary where embedders need deterministic diagnostics.

This change adds a richer adapter-call outcome contract while preserving the existing simple `ComponentAdapter::call` path for compatibility.

## What Changes

- Define a typed adapter call outcome/envelope that can carry completed responses, fail-closed outcome kinds, safe reason codes, request correlation metadata, and optional safe receipts.
- Add model/tool adapter v2 compatibility helpers so embedders can preserve typed failures without exposing raw prompts, provider payloads, tool bodies, paths, credentials, or unbounded text.
- Update runtime dispatch to prefer the typed outcome path when available and to keep legacy string failures as a compatibility fallback.
- Add positive and negative tests for completed responses, denied/absent/unavailable/malformed/cancelled/timed-out outcomes, unsafe failure-message rejection, and no raw payload leakage.

## Impact

- **Files**: `crates/animus-adapters` adapter-call DTOs/traits, `crates/animus-runtime` dispatch normalization, `crates/animus-std` fixtures, tests, docs, facade inventory, and Cairn evidence.
- **Testing**: focused adapter, runtime, std-fixture, leak-rejection, and Cairn gate checks.
- **Non-claims**: this does not add concrete provider clients, network calls, process execution, or global service discovery.
