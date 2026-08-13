# Proposal: Add durable Steel kernel restore

## Why

The Steel kernel preserves successful cells only while its process remains alive. Session restart cannot restore helper definitions, variables, or prior host results.

Replaying model-written source without captured results can repeat external effects. Serializing engine internals can create unsafe version and compatibility claims.

Animus needs restart-safe kernel state at committed cell boundaries. Restore must use the private session channel and must not repeat host effects.

## What Changes

- Add a versioned private kernel journal for committed cells, clear markers, context bindings, and captured host results. r[agent_harness_runtime.steel_kernel_restore.journal]
- Rebuild a kernel by deterministic replay with host execution disabled and recorded results supplied in exact order. r[agent_harness_runtime.steel_kernel_restore.replay]
- Admit restore only for matching session, policy, Steel cohort, source chain, and journal head. r[agent_harness_runtime.steel_kernel_restore.admission]
- Restore only complete committed cell boundaries and classify interrupted cells explicitly. r[agent_harness_runtime.steel_kernel_restore.safe_points]
- Keep durable receipts metadata-only while private records remain in the existing protected journal channel. r[agent_harness_runtime.steel_kernel_restore.evidence]
- Add positive, negative, corruption, drift, and no-effect-replay tests. r[agent_harness_runtime.steel_kernel_restore.verification]

## Prerequisites

- `add-resumable-steel-host-bridge`
- `agent_harness_runtime.durable_session_store.private_journal`
- `agent_harness_runtime.durable_session_store.framing`
- `agent_harness_runtime.session_restore.restore_matrix`
- `agent_harness_runtime.active_turn_recovery.recovery`

## Impact

- **Crates**: the RLM kernel contracts, `animus-steel`, runtime restore planning, and the durable session host.
- **Storage**: model-written source and host results use the existing replay-private channel.
- **Compatibility**: no Steel VM memory image becomes a stable public format.
- **Recovery**: interrupted active cells remain explicit degraded outcomes.

## Out of Scope

- Restoring an active continuation in the middle of a cell.
- Reversing external effects that completed before a crash.
- Cross-session kernel cloning, cross-version VM snapshots, encrypted storage guarantees, or production durability.

## Affected Specs

- `agent-harness-runtime`: private kernel journal, replay, admission, safe points, evidence, and verification.
