# agent-harness-runtime Change Spec: Subagent fan-out/fan-in

## Requirements

### Requirement: Fan-out plans are declarative and bounded

r[agent_harness_runtime.subagent_fanout_fanin.plan] Animus MUST represent subagent fan-out as declarative plans containing child-run admissions or drafts, concurrency policy, shared budget, cancellation policy, aggregation schema, and mutation-arbitration policy without starting child execution during plan validation.

#### Scenario: Valid fan-out plan is accepted without execution

- GIVEN a fan-out plan contains two read-only child-run admissions, a bounded concurrency policy, and a shared budget
- WHEN plan validation runs
- THEN validation accepts the plan and returns safe plan metadata
- AND no scheduler, worker, model, tool, or external adapter is invoked.

### Requirement: Fan-in aggregation is deterministic

r[agent_harness_runtime.subagent_fanout_fanin.aggregation] Animus MUST aggregate fan-in results with a deterministic reducer that distinguishes success, partial success, failure, timeout, cancellation, malformed output, denied capability, budget exhaustion, and write conflict outcomes.

#### Scenario: Partial success is explicit

- GIVEN one child succeeds and one child times out
- WHEN fan-in aggregation runs
- THEN the aggregate status is partial success or timeout-bearing partial result according to policy
- AND the timed-out child is not counted as successful.

### Requirement: Cancellation and budget exhaustion are terminal evidence

r[agent_harness_runtime.subagent_fanout_fanin.cancellation_budget] Fan-out/fan-in orchestration MUST treat cancellation, timeout, and budget exhaustion as explicit terminal or blocking outcomes with safe diagnostics and receipt evidence.

#### Scenario: Shared budget exhaustion cancels remaining children

- GIVEN fan-out execution exhausts its shared budget before all children complete
- WHEN the coordinator records remaining child outcomes
- THEN unfinished children receive budget-exhausted or cancelled outcomes
- AND the aggregate report explains the budget stop without raw child logs.

### Requirement: Mutation intents require arbitration

r[agent_harness_runtime.subagent_fanout_fanin.mutation_arbitration] Fan-in aggregation MUST preserve, deny, or mark child mutation intents unresolved, and MUST NOT apply workspace, ledger, filesystem, process, network, or provider effects during aggregation.

#### Scenario: Conflicting mutation intents do not apply

- GIVEN two child reports propose incompatible mutation intents
- WHEN fan-in aggregation runs
- THEN the aggregate records a structured write-conflict or unresolved-mutation result
- AND neither mutation is dispatched before normal policy and coordination accept a later action.

### Requirement: Fan-out/fan-in failures fail closed

r[agent_harness_runtime.subagent_fanout_fanin.fail_closed] Fan-out/fan-in validation and aggregation MUST fail closed for missing child admissions, unsafe concurrency policies, over-budget plans, malformed child reports, denied capabilities, duplicate child ids, write conflicts, mutation bypass attempts, and lossy aggregate receipts.

#### Scenario: Duplicate child id is rejected

- GIVEN a fan-out plan contains two child runs with the same safe child identity
- WHEN plan validation runs
- THEN validation returns a duplicate-child diagnostic
- AND no child is enqueued from that plan.

### Requirement: Fan-out/fan-in closeout is evidenced

r[agent_harness_runtime.subagent_fanout_fanin.verification] Fan-out/fan-in adoption MUST include positive plan and aggregation tests, negative fail-closed tests, mutation arbitration tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Fan-out/fan-in evidence is complete

- GIVEN the fan-out/fan-in change is ready to archive
- WHEN closeout validation runs
- THEN evidence names plan tests, aggregate reducer tests, mutation arbitration tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
