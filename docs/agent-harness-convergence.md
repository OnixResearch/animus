# Agent harness convergence rails

Animus binds seven result and supervision rules without adding a new agent protocol.

## Typed results and projections

`animus_runtime::convergence::Datum` is the canonical result value. It wraps the existing bounded adapter output with:

- call and result-kind identities
- complete or truncated status
- original byte, character, and line counts
- optional BLAKE3 content identity
- an optional store-typed handle
- optional bounded resource telemetry

Model, human, and persisted views are projections of the same datum identity. `inspect_record` rejects a persisted projection that drifts from its source datum.

## Honest size handling

The existing adapter output bound remains authoritative for the visible prefix. A truncated result cannot enter the convergence rail without a store-typed handle.

The model projection states the exact omitted character count and gives a follow command. Silent truncation fails closed.

## Store-typed handles

A handle contains an enum store namespace and a short opaque token. The runtime does not parse links or guess another namespace.

`resolve` calls the injected `StorePort`. An unknown handle returns the exact token in a typed error. A substituted datum fails the identity check.

## Data-first history

`History` stores bounded append-only call, result, and span events. `render_history` is a pure query with no executor or effect port. Rendering cannot rerun a tool.

## Explainable silence

`plan_silence` consumes explicit elapsed-time and output-byte facts. A non-terminal call that exceeds either bound detaches with a typed handle and follow command.

`project_suspension_dump` requires a bounded, non-empty list. Each item names the call, handle, and supplied suspension duration.

## Supervised cycles

The pure cycle reducer separates these phases:

1. new
2. prepared
3. ready to commit
4. parked
5. finished or failed

Parking is not finishing. Commit requires exact call count, released guard evidence, and no side effects inside the guard.

The optional `animus_std::convergencehost` shell runs every prepared call in a scoped worker thread. It joins all workers before it invokes the injected commit service. Tests use a barrier to prove that a two-call batch runs concurrently.

## Telemetry

Optional CPU-time and output-byte telemetry rides with each result. Every observation names its bound and must stay within it.

Telemetry does not prove tool correctness, result freshness, authorization, provider behavior, model quality, production readiness, or release eligibility.
