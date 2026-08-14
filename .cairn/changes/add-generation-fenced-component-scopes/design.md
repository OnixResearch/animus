# Design: Add generation-fenced component scopes

## Context

Animus uses explicit injected services, lifecycle hooks, cancellation gates, daemon generations, and transactional handoff. These contracts do not provide a common lifecycle for a group of related registrations and children.

A reusable scope must model visibility and ownership without becoming an ambient mutable context.

## Success Contract

A composition root can create an explicit scope, register declared components, resolve only admitted members, and close the scope through one generation-fenced sequence. After close starts, no new resolution or delivery succeeds. Completion requires quiescence and reverse-order release observations for every owned member.

## Decisions

### 1. Use explicit scope values and leases

The core defines `ComponentScope`, `ComponentRegistration`, `ComponentLease`, `ScopeGeneration`, and typed states. Callers pass a scope or lease explicitly.

There is no process-global current scope, thread-local lookup, hidden parent fallback, or name-only service locator.

### 2. Separate visibility from effect authority

A valid registration permits exact resolution within its declared scope. It does not grant model, tool, filesystem, process, network, credential, persistence, terminal, or mutation authority.

Resolved services still pass all existing authority and runtime-seal gates.

### 3. Use a pure lifecycle reducer

The core owns these states:

```text
open -> closing -> quiescing -> releasing -> closed
                    \-> failed_cleanup
```

The reducer consumes explicit observations and returns the next state plus typed effect requests. It performs no cancellation, waiting, task control, listener control, or resource release.

### 4. Make teardown order explicit

The close plan performs these logical steps:

1. revoke new resolution;
2. close new event or message delivery;
3. request cancellation for owned work;
4. await generation-bound quiescence observations;
5. release child scopes and component resources in reverse dependency order;
6. commit one terminal outcome.

A release observation for the wrong generation or component cannot advance the plan.

### 5. Converge repeated close requests

The first admitted close command fixes the close generation and plan identity. Repeated or concurrent commands return the same in-progress or terminal identity.

They cannot start a second teardown, change order, restore visibility, or suppress unresolved cleanup.

### 6. Integrate with existing daemon ownership

The local daemon supervisor can map one worker generation to one root component scope. A client lease is not the worker scope. Client detach removes only client-owned delivery, while authorized worker stop closes the worker scope.

### 7. Keep evidence metadata-only

Receipts contain scope and generation identities, component counts, transition classes, requested and observed cleanup counts, unresolved safe labels, and non-claims. They omit handles, paths, commands, payloads, prompts, credentials, and raw errors.

## Alternatives Considered

### Reuse lifecycle hooks as teardown ownership

Rejected because hooks describe extension points. They do not own registration visibility, child quiescence, or reverse-order release.

### Let Drop prove cleanup

Rejected because asynchronous work, process trees, listeners, and partial initialization need explicit observations and unresolved outcomes.

### Adopt an ambient Cordis context

Rejected because hidden service lookup conflicts with explicit dependencies and inward dependency direction.

## Risks and Controls

- Quiescence observations can be false. Receipts state that Animus validates supplied facts and does not prove external termination.
- Dependency cycles can prevent reverse release. The activation graph must reject unsupported ownership cycles before startup.
- A stalled member can block close. Named host deadlines return explicit unresolved cleanup instead of false completion.

## Non-Claims

This design does not prove task cancellation, process death, resource reclamation, sandboxing, authorization, external liveness, production readiness, or release eligibility.
