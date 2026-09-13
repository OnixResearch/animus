## Context

Animus records agent activity through safe receipts but has no typed notion of a proposed change set awaiting review. A task can appear finished while its edits are unowned and unapproved. SeekMoon reframes the development environment around review ownership: the agent drives implementation, the developer takes ownership, and only that decision completes the task.

## Decisions

### Decision: Change scopes are typed first-class facts

**Choice:** every agent change is recorded as a typed change scope that binds files, hunks, the driving task, and validation obligations.

**Rationale:** an untyped edit trail cannot be reviewed, subsetted, or evidenced. Typed scopes make ownership decisions deterministic and auditable.

### Decision: Nothing completes without a recorded handoff decision

**Choice:** the handoff is an explicit boundary; acceptance, rejection, or refinement is recorded before continuation.

**Rationale:** the developer, not the agent, owns the result. Completion claims must cite the recorded decision.

### Decision: Continuation is subset-bounded

**Choice:** after a decision, the agent continues only within the approved subset; rejected items open a fresh scope.

**Rationale:** bounded continuation keeps review authority real and prevents silent scope creep.

### Decision: Scope identities preserve typed collection boundaries

**Choice:** the scope digest uses `animus.change-scope-handoff.scope.v2`.
The encoded fields retain the contract identity, driving task, and the declared order of files, obligations, and hunks.
Each collection starts with its count as canonical decimal UTF-8 text.
Each value retains the existing unsigned 64-bit little-endian byte-length prefix.
Each hunk retains its six fields and unsigned 32-bit little-endian coordinates.

**Rationale:** value lengths do not separate adjacent collections.
The retained public-API counterexample moves one label from files to obligations without changing the old encoded bytes.
Collection counts make that partition explicit without a new dependency or another authority boundary.
Nested group digests also separate the collections but add hashing that this fixed schema does not need.

This correction changes scope identities.
Old scope identities cannot authorize newly validated scopes, even when their visible facts match.
Callers must validate the scope again and obtain a fresh decision.
There is no automatic legacy fallback.
The encoding correction retained the public Rust signatures, handoff contract label, receipt schema, and non-claims.
The later portable API decision changes only the declared integer types and their checked internal conversions.
The receipt validator still checks bounded metadata, not historical scope facts or external execution.

### Decision: Public counts and approved indexes use portable integers

**Choice:** public count bounds and approved hunk indexes use `u32`.
Declared artifact bytes remain `u64`, and receipt counts remain `u32`.
Conversions from collection lengths fail explicitly instead of using a sentinel value.
Continuation derives rejected hunks through checked enumeration, without a silent missing-index filter.

**Rationale:** review contracts must not change their integer width with the host pointer width.
This is a source-breaking API migration, not a source-compatible alias.
The admitted numeric range, order, refusal classes, and v2 encoded identities remain unchanged.
Maintained public controls retain pre-migration scope, decision, pending-receipt, and completed-receipt digest values.

The private data owners separate scope facts, judgments, and artifact records.
The existing public runtime and host facades re-export the same named capabilities.
The core gains no external authority, dependency, or port.

## Risks / Trade-offs

- Approval fatigue can convert review into rubber-stamping; hashed decision artifacts make the review visible and auditable.
- Over-strict handoffs interrupt agent flow; handoff frequency and subset size stay configurable bounds.
- Diff identity alone does not prove correctness; artifacts exclude raw transcripts and carry no semantic claim.
