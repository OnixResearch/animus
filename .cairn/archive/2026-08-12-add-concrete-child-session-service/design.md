# Design: Concrete child-session service

## Context

The reusable subagent facade already owns child kinds, admissions, state transitions, lifecycle events, lineage, fan-in, and safe results. It deliberately does not create provider clients, queues, stores, processes, or coding sessions.

The missing unit is a host implementation of that port.

## Success Contract

An admitted child request creates one child coding session through an explicit factory. The service returns a stable handle and supports status, cancel, and collect.

The child uses its own session identity, private journal, queue, kernel, and event stream. Its effective authority is never wider than its parent and profile ceilings.

A global agent registry, direct provider construction, inherited ambient credentials, or raw child transcript returned to the parent is false completion.

## Decisions

### 1. Use a host-owned session factory

The child service receives a factory that can build a coding-session host from explicit values:

- child identity and parent lineage
- admitted profile and model selection
- effective capabilities and workspace scope
- minimized context packet
- provider, tool, persistence, timer, cancellation, and event services
- optional Steel kernel and skill selection

The service does not discover these values.

### 2. Keep one explicit handle registry per service instance

The registry maps stable child handles to owned session controllers and safe lifecycle state. It is not process-global.

All operations require parent identity, child handle, generation, and current authority. Stale or foreign handles fail closed.

### 3. Use normal coding-session semantics

A child prompt follows the same durable message, provider, tool, cancellation, repair, and event ordering as a root session.

The child service does not add a second agent loop.

### 4. Preserve private content isolation

The parent receives only the admitted bounded result and safe lineage metadata. Full child transcripts remain in the child session's private channel.

Detailed import requires a separate admitted context operation.

### 5. Make restart explicit

Safe lifecycle records bind parent, child, generation, state, session head, result identity, and failure class. Restart reopens only admitted non-terminal children.

A missing provider, store, or tool service produces a blocked or failed child state. It does not create a new ambient service.

## Authority Flow

```text
admitted child draft
  -> pure capability and budget intersection
  -> explicit session factory
  -> child coding session
  -> safe lifecycle observations
  -> bounded parent result + lineage
```

## Alternatives Considered

### Run child prompts through a special model-call helper

Rejected because this bypasses normal session persistence, tools, cancellation, events, and recovery.

### Use a global child registry

Rejected because it creates hidden process authority and cross-session discovery.

### Return the complete child transcript

Rejected because parent context and evidence require explicit bounds and admission.

## Risks and Controls

- Child sessions can exhaust resources. Depth, fan-out, token, turn, tool, time, and result limits apply before creation.
- Parent cancellation can race completion. Generation-fenced state transitions preserve the observed outcome.
- Restart can duplicate work. Durable child state and coding-session recovery gate continuation.
- Child content can leak. Bounded results and safe lineage remain separate from private journals.

## Non-Claims

This design does not establish child correctness, automatic delegation quality, remote execution, cross-process isolation, sandboxing, exactly-once effects, task completion, or production readiness.
