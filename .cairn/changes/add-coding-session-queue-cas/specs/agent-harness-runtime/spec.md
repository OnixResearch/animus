# Agent Harness Runtime

## ADDED Requirements

### Requirement: Queue mutations carry explicit preconditions

r[agent_harness_runtime.coding_session_queue_cas.commands] External queue edit and removal commands MUST bind stable entry identity, expected checked generation, and expected private text identity before mutation.

#### Scenario: A client edits the item that it displayed

r[agent_harness_runtime.coding_session_queue_cas.commands.exact]
- GIVEN a client holds the current entry identity, queue generation, and expected text identity
- WHEN it submits a valid bounded replacement
- THEN the command MUST carry all preconditions to the queue reducer
- AND no position-only authority may substitute for them.

### Requirement: Queue mutation is a pure compare-and-swap transition

r[agent_harness_runtime.coding_session_queue_cas.reducer] Queue edit and removal MUST use a deterministic reducer that changes state only when all preconditions match and advances the checked generation exactly once on success.

#### Scenario: All preconditions match

r[agent_harness_runtime.coding_session_queue_cas.reducer.success]
- GIVEN current queue state matches the command entry, generation, and expected text identity
- WHEN the reducer applies the command
- THEN it MUST return the exact changed queue and next checked generation
- AND transcript, active-run, and unrelated queue entries MUST remain unchanged.

### Requirement: Stale queue commands return conflicts without mutation

r[agent_harness_runtime.coding_session_queue_cas.conflicts] A precondition mismatch MUST return a typed conflict with bounded current safe snapshot metadata and MUST NOT mutate queue, transcript, active-run, or durable-cursor state.

#### Scenario: Another client changed the text

r[agent_harness_runtime.coding_session_queue_cas.conflicts.text]
- GIVEN the entry still exists but its current private text identity differs from the command
- WHEN the reducer evaluates the command
- THEN it MUST return a text-conflict outcome with current safe queue identity
- AND it MUST NOT reveal current or expected text in safe diagnostics or receipts.

### Requirement: Queue mutation evidence omits private text

r[agent_harness_runtime.coding_session_queue_cas.evidence] Queue mutation events and receipts MUST contain only bounded entry, generation, position, operation, conflict, redaction, and BLAKE3 identity metadata without queued text bodies.

#### Scenario: A conflict receipt includes text

r[agent_harness_runtime.coding_session_queue_cas.evidence.leak]
- GIVEN a queue conflict receipt draft includes current, expected, or replacement text
- WHEN receipt validation runs
- THEN it MUST reject the draft without echoing the text
- AND the receipt MUST NOT contribute to readiness evidence.

### Requirement: Compatibility wrappers require exclusive ownership

r[agent_harness_runtime.coding_session_queue_cas.compatibility] A convenience API that derives queue preconditions MAY exist only for one explicit exclusive local owner using a current snapshot, and ownership-crossing clients MUST supply explicit preconditions.

#### Scenario: A remote command omits preconditions

r[agent_harness_runtime.coding_session_queue_cas.compatibility.remote]
- GIVEN a headless, daemon, SDK, or TUI command crosses an ownership boundary without explicit preconditions
- WHEN command admission runs
- THEN it MUST reject the command before queue mutation
- AND it MUST NOT invoke a last-writer-wins compatibility path.

### Requirement: Queue compare-and-swap has race evidence

r[agent_harness_runtime.coding_session_queue_cas.verification] Queue compare-and-swap MUST include positive and negative tests for exact edits and removals, generation order, stale text, stale generations, wrong entries, moved positions, concurrent injection, overflow, duplicates, unsafe evidence, frontend mapping, and fallback rejection.

#### Scenario: The queue change is ready to archive

r[agent_harness_runtime.coding_session_queue_cas.verification.closeout]
- GIVEN maintainers intend to sync and archive queue compare-and-swap
- WHEN focused and lifecycle validation runs
- THEN session core, coding-session, headless, daemon, TUI, receipt, Cairn, and relevant Nix checks MUST pass
- AND distributed consensus, replicated queues, automatic merge, exactly-once delivery, and production readiness MUST remain non-claims.
