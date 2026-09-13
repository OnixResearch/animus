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

## Risks / Trade-offs

- Approval fatigue can convert review into rubber-stamping; hashed decision artifacts make the review visible and auditable.
- Over-strict handoffs interrupt agent flow; handoff frequency and subset size stay configurable bounds.
- Diff identity alone does not prove correctness; artifacts exclude raw transcripts and carry no semantic claim.
