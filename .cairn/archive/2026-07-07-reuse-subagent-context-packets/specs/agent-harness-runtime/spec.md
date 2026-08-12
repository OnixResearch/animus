# agent-harness-runtime Change Spec: Shared subagent context packets

## Requirements

### Requirement: Subagents use shared minimized context packets

r[agent_harness_runtime.subagent_context_packets.shared_packet] Animus MUST provide a shared minimized context-packet contract for subagent child runs, covering selected parent excerpts, summaries, file observations, memory descriptors, bounded tool-result summaries, and admitted context references.

#### Scenario: Recipe-backed subagent receives minimized context

- GIVEN a recipe-backed review subagent needs parent context
- WHEN the child-run draft is assembled
- THEN the context is represented as a shared minimized packet with selected safe items
- AND the full parent transcript is not included by default.

### Requirement: Unsafe context requires admission and leak validation

r[agent_harness_runtime.subagent_context_packets.unsafe_admission] Shared subagent context packets MUST reject full transcripts, raw hidden memory, credentials, provider-native payloads, unbounded tool output, and unsafe raw material unless the context class is explicitly admitted and leak validation succeeds.

#### Scenario: Credential context is rejected

- GIVEN a child-run context packet contains credential-looking material without explicit admission
- WHEN packet validation runs
- THEN validation returns an unsafe-context diagnostic
- AND the child run cannot start with that packet.

### Requirement: Context packet identity is deterministic and safe

r[agent_harness_runtime.subagent_context_packets.identity] Shared subagent context packets MUST carry deterministic BLAKE3 packet identities and budget summaries derived from stable safe packet fields, and MUST NOT include raw prompt bodies, credentials, provider payloads, full paths, or unbounded output in receipts.

#### Scenario: Packet identity can be recorded without raw context

- GIVEN a valid minimized packet contains safe summaries and evidence references
- WHEN a child-run receipt is produced
- THEN the receipt records the packet BLAKE3 identity and budget summary
- AND it does not include raw parent prompt or tool output bodies.

### Requirement: Every child-run kind can project context packets

r[agent_harness_runtime.subagent_context_packets.kind_projection] Recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks MUST consume or losslessly project their context inputs through the shared context-packet contract.

#### Scenario: Fresh-context task keeps admitted references

- GIVEN a fresh-context task declares admitted context references
- WHEN the task is projected into the shared packet form
- THEN each admitted reference is represented with its safe label and budget metadata
- AND no parent-only secret or hidden prompt is inherited.

### Requirement: Context-packet failures fail closed

r[agent_harness_runtime.subagent_context_packets.fail_closed] Context-packet validation MUST fail closed for missing packet identity, empty required safe items, over-budget packets, unsafe raw markers, unadmitted unsafe classes, duplicate item labels, and lossy child-run projections.

#### Scenario: Over-budget packet is rejected

- GIVEN a child-run context packet exceeds its configured item or token budget
- WHEN packet validation runs
- THEN validation returns an over-budget diagnostic
- AND no scheduler, worker, model, tool, or external adapter receives the packet.

### Requirement: Context-packet closeout is evidenced

r[agent_harness_runtime.subagent_context_packets.verification] Shared context-packet adoption MUST include positive minimized packet tests, negative unsafe-context tests, identity tests, child-kind projection tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Context-packet evidence is complete

- GIVEN the shared context-packet change is ready to archive
- WHEN closeout validation runs
- THEN evidence names packet validation tests, projection tests, leak tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
