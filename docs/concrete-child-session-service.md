# Concrete child-session service

Animus provides a non-default host for admitted child work. The host uses normal coding-session controllers. It does not add a second agent loop.

## Ownership

The implementation keeps three owners separate:

1. `animus-runtime::child_session_service` owns pure admission, limits, lifecycle records, and bounded result contracts.
2. `animus-std::child_session_host` owns the service-local registry, orchestration, persistence calls, timers, cancellation, and events.
3. The injected `ChildSessionFactory` creates or restores one normal `CodingSessionHost` controller from explicit services.

The service has no process-global registry. It does not read environment values, provider files, credentials, or ambient configuration.

## Admission

The pure admission function intersects these values before construction:

- the parent authority ceiling;
- the child profile ceiling;
- the semantic role;
- the requested typed capabilities;
- the workspace scope;
- the model grant;
- the tool grants;
- the host admission;
- the minimized context packet;
- depth, fan-out, wall-time, token, turn, tool-call, and result limits.

A widening request returns a typed denial. No factory, store, provider, tool, workspace, timer, or event effect occurs after this denial.

## Explicit services

The factory reports its exact service inventory and a BLAKE3 service identity. Normal child sessions require these injected services:

- provider;
- tool;
- persistence;
- context;
- timer;
- cancellation;
- event;
- policy;
- authority;
- summarizer;
- recovery.

A selected Steel kernel or skill also requires its matching explicit service. A missing service blocks construction. The host does not use a fallback service.

## Lifecycle and handles

Each service instance owns one handle registry. A handle binds the parent, child, registry generation, and current parent-authority identity.

Status, cancellation, and collection validate this binding. They also recheck current parent authority. A stale, legacy, foreign, or authority-drifted handle fails before child mutation.

The host records safe lifecycle metadata after each accepted operation. The record includes:

- parent and child identity;
- generation;
- admission and capability identities;
- context packet identity;
- parent authority identity;
- factory service identity;
- lifecycle state;
- coding-session head and durable cursor;
- result identity;
- failure class.

Private prompts, child messages, provider payloads, tool bodies, credentials, and full transcripts stay in the child session.

## Results and restart

Collection returns a typed terminal class and optional admitted body. Complete, partial, failed, timed-out, cancelled, and bound-exceeded classes remain distinct.

The service computes a domain-separated BLAKE3 result identity. An oversized or transcript-shaped body is not returned to the parent.

Restart loads safe lifecycle records from the explicit store. It reopens only non-terminal records with matching admissions, authority, factory services, generation, and coding-session head.

The factory must use the coding-session recovery path. Restore does not submit the child prompt again. It does not repeat committed provider or tool effects.

## RLM projection

`project_rlm_child_session_result` maps an admitted result to the existing RLM child contract. The projection includes bounded model-visible text and hashed lineage.

The projection does not import the child transcript. Detailed imports need a separate admitted context operation.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-concrete-child-session-service --root .
```

The rail covers positive and negative lifecycle behavior, normal coding-session composition, cancellation races, restart, authority drift, missing services, bounds, leaks, RLM projection, and dependency isolation.

## Non-claims

This service does not establish automatic delegation, remote workers, process isolation, sandboxing, child correctness, task success, exactly-once cancellation, exactly-once external effects, or production readiness.
