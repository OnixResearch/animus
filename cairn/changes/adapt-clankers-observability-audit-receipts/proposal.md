# Proposal: Adapt Clankers observability audit receipts for Animus

## Why

Animus needs operator-visible audit evidence for runtime behavior without turning observability into transcript or tool-output storage. Clankers' observability/audit receipt brick exposes bounded counts, states, and timing summaries for pending and completed work while redacting tool names, arguments, outputs, prompts, provider payloads, and credentials. Animus should add a focused audit receipt contract that composes with safe receipts, event lifecycle, tool dispatch, and dataset/export surfaces.

## What Changes

- Define audit snapshot DTOs for pending/completed model calls, tool calls, confirmations, background work, delivery attempts, retries, and error classes.
- Bound public counts and durations, clamp hidden pending sets, and report over-limit diagnostics instead of serializing maps of private call state.
- Keep metrics sinks, telemetry export, clocks, storage, and UI rendering in host adapters.
- Emit safe audit receipts with schema version, policy hash, event window identity, bounded counts, status classes, timing buckets, and BLAKE3 receipt hash.
- Add positive and negative fixtures for empty audit state, pending-over-limit clamping, completed summaries, failed work classes, sink-unavailable behavior, and receipt leak attempts.

## Impact

- **Files**: future audit DTOs, bounded projection core, receipt fixtures, tests, docs, and Cairn artifacts.
- **Testing**: positive audit snapshot fixtures and negative over-limit/sink/leak fixtures.
- **Non-claims**: this does not add external telemetry, raw logs, full session tracing, or provider/tool payload persistence.
