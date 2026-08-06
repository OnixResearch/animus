## Context

Prime Agent's `/autonomous` mode adds follow-up continuations until quality gates pass or a continuation, turn, token, or wall-clock limit is reached. Gate commands run before the session may finish. A failed gate returns its bounded output to the agent for another attempt. The harness avoids rerunning the same failed gate when the workspace has not changed. The docs state the boundary explicitly: a passed gate checks only what that gate verifies, and reaching a limit does not imply task success.

Animus owns the continuation side. `controller-continuation-policy` handles post-turn continuation state and priority. `ralph-loop` handles bounded repeated-task loops with completion promises. Neither has a gate abstraction or workspace change detection. A ralph loop today can rerun an unchanged failing check until budget exhaustion, and nothing in contracts prevents a limit stop from being read as success.

Completion requires gates as host-injected executors, bounded output, deterministic rerun decisions from workspace fingerprints, and an explicit success boundary. Reusable-crate gate execution, unbounded gate output, or limit-as-success reporting is false completion.

## Decisions

### Decision: gates are host-injected executors with typed results

**Choice:** A gate is a descriptor (identity, command class, output bound) admitted by policy. The host shell executes it and returns a typed result: pass, or fail with bounded output. The contract layer consumes results and decides continuation; it never spawns processes.

**Rationale:** Gate execution is an effect with arbitrary authority. Keeping it host-owned preserves the reusable boundary and lets each host sandbox or admit gates by its own rules.

### Decision: bounded failed-gate output returns to the agent

**Choice:** On failure, the contract truncates gate output to the admitted byte bound and classifies it (stderr class, exit class, timeout class) before it enters the prompt. Raw unbounded output never enters context.

**Rationale:** Failed-gate feedback is the mechanism that lets the agent fix the cause. Bounding it keeps one bad gate from consuming the context window.

### Decision: rerun decisions use workspace fingerprints

**Choice:** The host supplies a domain-separated BLAKE3 fingerprint over the admitted file set after each attempt. The contract records (gate identity, fingerprint, outcome) and denies a rerun of the same failed gate when the fingerprint is unchanged. Fingerprint inputs name the admitted file set; coverage outside it is a non-claim.

**Rationale:** This makes prime-agent's "don't rerun an unchanged failure" rule deterministic and auditable instead of heuristic.

### Decision: success boundary is a contract rule, not a doc note

**Choice:** Gate results carry a typed `verified_scope` field naming what the gate checks. Continuation outcomes carry an explicit `limit_reached` or `gates_passed` class, and receipt validation rejects any projection that maps these to task success or objective completion.

**Rationale:** Autonomous runs end in ambiguous states. Encoding the boundary in types and receipt validation keeps the ambiguity visible downstream.

## Alternatives Considered

### Extend ralph-loop with gate fields in place

Rejected because ralph-loop is a Steel policy profile, not the owner of gate execution semantics. Gates get a focused contract that ralph and continuation policy both consume.

### Hash the whole workspace tree for change detection

Rejected because unbounded trees make fingerprints expensive and noisy. The admitted file set is explicit in policy, and uncovered paths are a documented non-claim.

### Treat all gates passed as completion

Rejected because a gate checks only its own scope. Completion remains with goal and ralph completion-promise owners.

## Risks / Trade-offs

- A narrow admitted file set can miss relevant changes and skip a needed rerun. Mitigation: the admitted set is explicit in policy, and receipts bind the fingerprint input identity.
- Bounded gate output can truncate the one line the agent needs. Mitigation: output classes and head/tail preservation rules are named in policy.
- Hosts can inject weak gates and claim strong verification. Mitigation: receipts bind gate identity and verified scope, so downstream consumers see exactly what was checked.

## Non-Claims

This design does not establish task success, objective completion, verification completeness outside a gate's scope, change-detection completeness outside the admitted file set, gate sandboxing, production readiness, or release eligibility.
