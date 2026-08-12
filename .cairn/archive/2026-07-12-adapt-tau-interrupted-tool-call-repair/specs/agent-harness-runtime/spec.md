# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Active provider context has terminal tool-call bindings

r[agent_harness_runtime.tau_tool_repair.invariant] Before provider dispatch, Animus MUST validate the selected active branch so every model-visible assistant tool call has exactly one compatible terminal tool result and duplicate, reversed, mismatched, unsupported, or stale bindings fail closed.

#### Scenario: Clean active branch needs no repair

- GIVEN a validated active branch in which every assistant tool call is followed by one matching tool result
- WHEN the tool-call binding invariant is evaluated
- THEN validation MUST return a clean outcome with no repair actions
- AND no ledger entry, active leaf, provider request, or tool effect may be changed.

#### Scenario: Duplicate call identity is structurally invalid

- GIVEN two model-visible assistant tool calls share one call identity or one call has multiple terminal results
- WHEN the active branch is validated
- THEN validation MUST return a typed structural error
- AND the branch MUST NOT be treated as repairable by choosing an arbitrary call or result.

### Requirement: Interrupted tool repair planning is pure and deterministic

r[agent_harness_runtime.tau_tool_repair.plan] Animus MUST derive interrupted-tool diagnostics and ordered repair actions as a pure deterministic function of validated active-branch entries, tool bindings, interruption evidence, active-leaf identity, and named bounds.

#### Scenario: Partially completed tool batch is planned

- GIVEN an assistant message contains multiple tool calls, some with matching committed results and some without results
- WHEN repair planning runs with the current active leaf
- THEN the plan MUST preserve every matched result and create one synthetic terminal failure action for each unmatched call
- AND identical inputs MUST produce identical ordered actions and domain-separated BLAKE3 plan identities.

#### Scenario: Planner receives stale branch identity

- GIVEN the supplied active-leaf identity does not match the branch snapshot used for diagnostics
- WHEN repair planning runs
- THEN it MUST return a stale-branch error
- AND it MUST NOT produce an applicable repair plan.

### Requirement: Repair reasons do not invent user intent

r[agent_harness_runtime.tau_tool_repair.reason] Synthetic tool results MUST use an interruption reason derived from correlated evidence and MUST distinguish explicit user cancellation, explicit host cancellation, unclean shutdown, and unknown interruption.

#### Scenario: Missing result has no cancellation evidence

- GIVEN a committed assistant tool call has no result and no correlated cancellation or clean-shutdown observation exists
- WHEN interruption reason reduction runs
- THEN the reason MUST be `unknown_interruption`
- AND the synthetic result MUST NOT state or imply that the user cancelled the call.

#### Scenario: User cancellation is correlated

- GIVEN a validated cancellation observation identifies the same run and affected tool-call batch
- WHEN interruption reason reduction runs
- THEN the affected synthetic results MAY use `user_cancelled`
- AND unrelated calls MUST NOT inherit that reason.

### Requirement: Repair preserves append-only session history

r[agent_harness_runtime.tau_tool_repair.append_only] Applying an interrupted-tool repair MUST append a new tail or repaired active branch, preserve prior committed entries and sibling branches, bind the old and new leaf identities, and remain idempotent without rewriting or deleting history.

#### Scenario: Historical gap has later messages

- GIVEN an unmatched tool call occurs before later model-visible messages on the active branch
- WHEN the host applies an admitted historical repair
- THEN it MUST create an append-only repaired branch or equivalent typed replay mutation that places the synthetic result immediately after its call in reconstructed context
- AND the original branch and later entry identities MUST remain preserved as history.

#### Scenario: Applied repair is resumed again

- GIVEN the same repair plan was durably committed and the repaired leaf is current
- WHEN recovery evaluates the session again
- THEN it MUST return an already-applied or clean outcome
- AND it MUST NOT append a duplicate synthetic result.

### Requirement: Provider continuation requires durable repair

r[agent_harness_runtime.tau_tool_repair.provider_gate] A host MUST NOT send active-branch context to a provider while repairable unmatched calls, malformed bindings, stale repair state, or partially applied repair actions remain, and repair MUST NOT execute or retry a tool.

#### Scenario: Durable repair succeeds

- GIVEN a repair plan is valid, the host commits every repair action, advances the active leaf, and replay revalidation is clean
- WHEN provider continuation is requested
- THEN continuation MAY use the repaired context
- AND no tool executor may have been invoked by the repair path.

#### Scenario: Store fails during repair

- GIVEN the host commits only part of a repair or cannot prove the new active leaf durable
- WHEN provider continuation is requested
- THEN provider dispatch MUST fail closed with a typed persistence or repair failure
- AND the host MUST NOT fall back to an in-memory-only repair or rerun the tool.

### Requirement: Interrupted-tool repair receipts are safe

r[agent_harness_runtime.tau_tool_repair.receipts] Repair receipts MUST contain only bounded status and reason classes, safe session/branch or hashed identities, old/new leaf hashes, call and action counts, redaction status, and domain-separated BLAKE3 plan and receipt identities.

#### Scenario: Safe repair receipt is projected

- GIVEN a repair plan and durable application outcome contain only admitted metadata
- WHEN receipt projection runs
- THEN the receipt MUST bind the plan, old leaf, new leaf, reason classes, counts, and application status
- AND it MUST state that repair does not prove interruption cause or exactly-once external effects.

#### Scenario: Receipt input contains private material

- GIVEN receipt input contains a raw prompt, tool argument, tool output, provider payload, credential, private path, exception body, or unbounded transcript text
- WHEN receipt validation runs
- THEN it MUST reject the receipt without emitting the private material in diagnostics.

### Requirement: Interrupted-tool repair has adversarial evidence

r[agent_harness_runtime.tau_tool_repair.verification] Interrupted-tool repair MUST include positive and negative tests for clean and interrupted branches, partial batches, historical gaps, reason attribution, append-only application, idempotence, provider gating, structural corruption, effect non-reexecution, and receipt safety.

#### Scenario: Change is ready for archive

- GIVEN maintainers intend to sync and archive the repair change
- WHEN closeout validation runs
- THEN focused runtime, session, compaction, provider-projection, safe-receipt, and composition tests plus Cairn gates MUST pass
- AND unavailable checks MUST be recorded with an exact blocker and next-best validation rather than reported as passed.
