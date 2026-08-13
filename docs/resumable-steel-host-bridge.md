# Resumable Steel host bridge

Animus provides a non-default bridge for model-written Steel cells. The bridge returns bounded host results to the same staged cell.

## Ownership

The bridge keeps three owners separate:

1. `animus-adapter-rlm-kernel` owns pure state, correlation, limits, and failure classes.
2. `animus-steel` owns sandboxed evaluation and staged replay.
3. `animus-runtime::rlm_kernel` owns admission, host execution, cancellation checks, and orchestration.

Steel does not receive direct filesystem, process, network, credential, clock, persistence, or child-lifecycle authority.

## Reviewed cohort

The implementation baseline is Animus commit `c85fdb35e4e546edcafb368fcb32e0353bf65fc4`. The Steel package is `steel-core` version `0.8.2`.

The Prime Agent reference is commit `c5991bc853d27754aed345c13ff4d2e05c40f1f4`. The reviewed source is `packages/coding-agent/docs/rlm.md`.

Before this change, `SteelRlmKernel::evaluate` returned one admitted host-request marker. It committed that cell without executing or returning a host result.

## Execution flow

The runtime starts a bridge run with BLAKE3 session and cell identities. It also supplies an explicit nesting depth.

For each host call, the runtime uses this flow:

1. Evaluate the cell from the committed kernel state.
2. Replay earlier correlated results without executor calls.
3. Stop at the next typed host request.
4. Check cancellation and elapsed time before admission.
5. Revalidate the request against the current policy.
6. Check cancellation and elapsed time before execution.
7. Run the exact injected executor.
8. Bind the bounded result to the pending correlation.
9. Check cancellation and elapsed time before resumption.
10. Repeat evaluation with all prior results in exact order.

The runtime commits the cell only after a successful final value. Final commit also checks each replayed request identity.

This implementation uses deterministic staged replay. It does not serialize or expose a Steel VM continuation.

## Bounds

`config/rlm-kernel-profile.ncl` owns these bridge limits:

- host calls per cell
- one host-result size
- cumulative host-result size
- bridge nesting
- cumulative evaluator fuel
- bridge wall time

All limits are finite and positive. Rust revalidates the generated policy before use.

## Partial effects

A later Steel error does not erase earlier host observations. The terminal error retains completed call identities, result hashes, counts, and operation classes.

The bridge discards staged Steel state after denial, cancellation, timeout, host failure, result failure, or Steel failure.

Discarding staged state does not reverse an external effect. The bridge does not claim rollback or exactly-once execution.

## Compatibility

`SteelRlmKernel::evaluate` keeps the marker-only behavior for existing callers. New resumable callers use `execute_bridged_cell`.

The marker-only path does not execute effects. It returns one admitted request as model-visible text.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-steel-host-bridge --root .
```

The rail covers positive and negative Rust tests, Nickel policy freshness, no-std isolation, Wasm isolation, and metadata-only receipts.

## Non-claims

The bridge does not establish durable mid-cell restore, external effect rollback, exactly-once execution, security sandboxing, task correctness, or production readiness.
