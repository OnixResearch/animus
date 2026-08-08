## Context

Prime Agent's `/goal` keeps an objective and its progress active across turns. The goal records token usage, elapsed time, continuation count, and an optional token budget. The harness keeps prompting an active goal after ordinary turns. Only `goal.complete()` marks successful completion. Creating a persistent goal is an explicit user or host action, never something the agent infers.

Animus has adjacent machinery. `ralph-loop` repeats a fresh-task prompt until a completion promise appears. `controller-continuation-policy` owns post-turn continuation state, priority, and acknowledgement. Neither provides a durable cross-turn objective with accounting, and neither enforces the rule that stopping is not succeeding. This change fills that gap as a pure state contract that existing continuation owners consume.

Completion requires a durable goal object, an explicit lifecycle, budget accounting, and metadata-only receipts. A goal that the agent can create for itself, a budget stop reported as success, or receipts containing objective text is false completion.

## Decisions

### Decision: goal state is a pure durable object

**Choice:** A focused contract module owns the goal record: objective identity, status, creation provenance, token and elapsed accounting, continuation count, and optional budget. Clocks and token counters are host-injected inputs to pure transition functions. Persistence stays host-owned through the existing ledger.

**Rationale:** Goal semantics are deterministic transitions over typed state. Pure transitions keep the Animus test story intact and let any host shell drive the same lifecycle.

### Decision: completion is explicit and distinct from stopping

**Choice:** The status machine separates `active`, `paused`, `completed`, `budget_limited`, `errored`, and `cleared`. Only an explicit completion transition produces `completed`. Budget exhaustion produces `budget_limited`. Detach, compaction, and turn limits never produce `completed`.

**Rationale:** The prime-agent lesson is that autonomous runs die many deaths that are not success. Encoding the distinction in the status type makes success-by-exhaustion unrepresentable.

### Decision: goals are host-created, agent-completable

**Choice:** Creation, pause, resume, and clear require explicit user or host actions with provenance. The model receives a narrow kernel-side surface to inspect the goal and to request completion, which admission validates against the active goal. The model cannot create, pause, resume, or clear goals.

**Rationale:** This matches prime-agent's explicit-creation rule and prevents the agent from minting objectives or erasing accountability.

### Decision: continuation consumes goal state through existing owners

**Choice:** `controller-continuation-policy` gains a goal-aware input: an active goal raises continuation priority after ordinary turns. The goal module never injects prompts itself; it supplies state that the continuation owner turns into decisions.

**Rationale:** One owner per decision. Goal state stays a fact; continuation stays a policy.

## Alternatives Considered

### Extend ralph-loop promises into goals

Rejected because ralph-loop is a bounded single-task loop profile, not a durable cross-turn objective with accounting. A separate goal contract keeps both surfaces honest.

### Track goals as free-form ledger notes

Rejected because free-form notes cannot enforce the status machine, accounting fields, or the explicit-completion rule.

### Let the agent clear or complete goals freely

Rejected because unrestricted completion removes the evidence value of the goal. Completion requests pass admission against the active goal and produce receipts.

## Risks / Trade-offs

- A sticky active goal can keep a session working after the user lost interest. Mitigation: explicit pause and clear, budget bounds, and accounting receipts that make continuation visible.
- Accounting depends on host-injected counters; wrong inputs produce wrong budgets. Mitigation: counters are typed inputs with provenance, and receipts record which counter identities produced the accounting.
- Multiple concurrent goals complicate continuation priority. Mitigation: the initial contract admits at most one active goal per session; additional goals queue as paused.

## Non-Claims

This design does not establish objective achievement, work quality, model correctness, automatic goal inference, cross-session goal consistency, production readiness, or release eligibility.
