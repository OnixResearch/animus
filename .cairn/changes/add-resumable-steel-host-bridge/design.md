# Design: Resumable Steel host bridge

## Context

`SteelRlmKernel::evaluate` creates an evaluation engine, replays successful cells, and appends the new source after successful evaluation. A host-request value is currently a terminal result.

Prime Agent lets model-written code call host functions and continue with returned values. Animus must provide this control flow without giving Steel ambient operating-system authority.

## Success Contract

One Steel cell can make a bounded sequence of admitted host calls. Each call receives one correlated typed result before evaluation continues.

The kernel commits the cell only after a successful final Steel value. A denied, cancelled, malformed, stale, or failed call does not commit cell state.

External effects that completed before a later error remain observed facts. The bridge must not describe them as rolled back.

A marker string, a one-shot request, an untyped callback, or direct Steel filesystem access is false completion.

## Decisions

### 1. Use an explicit bridge driver

The runtime owns a driver state machine with these states:

1. `Evaluating`
2. `AwaitingHostResult`
3. `Resuming`
4. `Completed`
5. `Failed`
6. `Cancelled`

The driver owns call sequence numbers and correlation identities. The Steel engine receives only an injected bridge function.

### 2. Keep host operations outside Steel

The bridge function yields a typed request to the runtime. The runtime admits the request before it calls the injected executor.

The executor result returns through an application-owned DTO. Raw adapter, filesystem, process, or provider types never enter the kernel contract.

### 3. Keep cell-state commit atomic

Evaluation uses staged kernel state. A successful final value commits source history and environment state.

A bridge denial, host error, cancellation, timeout, or later Steel error discards the staged kernel state. This rule does not reverse completed external effects.

### 4. Record partial external effects

Each completed host call gets an observation before the next continuation. A later cell failure references the completed call identities and the terminal error class.

The receipt must not claim transaction rollback, exactly-once execution, or absence of external change.

### 5. Keep continuations process-local

An active continuation is opaque and process-local. Durable recovery occurs only at committed cell boundaries in `add-durable-steel-kernel-restore`.

A crash during an active cell produces an interrupted-cell outcome. The host must use existing repair and recovery policy for any completed effect.

### 6. Bound every continuation path

Policy names the maximum host calls per cell, result characters, cumulative result characters, nested bridge depth, evaluator fuel, and wall time.

Cancellation is checked before admission, before execution, and before resumption.

## Effect and Authority Flow

```text
model-written Steel cell
  -> staged Steel evaluation
  -> typed host request
  -> pure runtime admission
  -> injected host executor
  -> bounded typed result
  -> correlated Steel continuation
  -> final value
  -> commit staged kernel state
```

## Alternatives Considered

### Keep one terminal request per cell

Rejected because the model cannot use a host result inside the same program.

### Give Steel direct Rust filesystem and process functions

Rejected because direct functions bypass application admission and merge interpreter authority with host authority.

### Roll back all prior host effects after a later error

Rejected because many external effects are not reversible. The bridge records partial effects instead.

### Serialize active Steel continuations

Rejected for the first cohort. Engine-specific continuation serialization adds unsafe compatibility claims and weakens restore review.

## Risks and Controls

- A loop can request many effects. Named call and time bounds stop the loop.
- A later error can follow a successful mutation. Receipts preserve this partial outcome.
- A stale result can enter the wrong call. Correlation and sequence checks fail closed.
- A callback can hide authority. The public bridge accepts only the application-owned executor port.

## Non-Claims

This design does not establish sandboxing, durable mid-cell continuation, external effect rollback, exactly-once execution, model correctness, task completion, or production readiness.
