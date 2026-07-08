# Proposal: Add memory control surface

## Why

Swappable memory adapters need a safe control surface so users and hosts can inspect, approve, reject, explain, update, and forget memory without knowing which backend is active. Existing agents emphasize this in different ways: Claude Code exposes `/memory` and editable files, Codex exposes per-thread memory use/generation controls, Goose exposes remember/retrieve/remove tools, and managed memory layers expose add/search/update/delete APIs. Animus should provide adapter-neutral commands and receipts instead of backend-specific UI assumptions.

## What Changes

- Define adapter-neutral memory control commands for list, show, search, why, approve, reject, forget, update/supersede, and safe export summaries.
- Validate commands as pure DTOs before any adapter mutation, remote call, or prompt influence.
- Add review states for extracted candidates so admission is explicit and auditable.
- Route forget/update/supersede through injected adapters with typed success, denied, unavailable, partial, and stale outcomes.
- Explain prompt influence using safe ledger entries, ranks, match reasons, policy decisions, and BLAKE3 hashes rather than raw private bodies.

## Impact

- **Files**: future control DTOs, validation core, host shell command/RPC bindings, receipts, tests, docs, and Cairn artifacts.
- **Testing**: positive list/show/why/approve/forget cases and negative unauthorized, unavailable backend, stale candidate, raw-body leak, partial delete, and remote admission fixtures.
- **Non-claims**: this does not mandate a TUI, CLI, web UI, or specific memory backend; hosts may expose any subset allowed by policy.
