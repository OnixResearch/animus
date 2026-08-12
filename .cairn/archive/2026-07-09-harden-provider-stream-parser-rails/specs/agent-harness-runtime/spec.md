# agent-harness-runtime Change Spec: Provider stream parser rails

## Requirements

### Requirement: Stream parser entrypoints are inventoried

r[agent_harness_runtime.provider_stream_parser_rails.inventory] Animus MUST maintain an inventory of provider stream parser entrypoints and raw-boundary assertions or panics that can be reached from provider-like chunks, frames, records, or terminal markers.

#### Scenario: New parser entrypoint requires inventory

r[agent_harness_runtime.provider_stream_parser_rails.inventory.new_entrypoint]
- GIVEN a provider stream parser entrypoint is added or renamed
- WHEN parser hardening validation runs
- THEN the inventory names the entrypoint, input framing, normalized output event type, budget policy, and receipt projection
- AND unreviewed parser entrypoints fail validation.

### Requirement: Parser failures are typed outcomes

r[agent_harness_runtime.provider_stream_parser_rails.typed_failures] Provider stream parser entrypoints MUST return typed failures instead of parser-facing raw-boundary assertions, panics, or unchecked unwraps for malformed data, invalid encoding, unsupported events, invalid order, duplicate terminals, missing terminals, and unknown terminal state.

#### Scenario: Malformed provider record returns typed failure

r[agent_harness_runtime.provider_stream_parser_rails.typed_failures.malformed_record]
- GIVEN a provider-like stream chunk contains malformed JSON or malformed SSE data
- WHEN the parser entrypoint processes the chunk
- THEN the parser returns a malformed-record failure class
- AND it does not panic, assert, unwrap, or fabricate a terminal success event.

### Requirement: Stream budgets apply before aggregation

r[agent_harness_runtime.provider_stream_parser_rails.budgets] Provider stream parsers MUST enforce frame-byte, record-byte, aggregate-byte, record-count, event-count, and completion budgets before buffering or concatenating stream data beyond the admitted budget.

#### Scenario: Oversized frame is rejected before concatenation

r[agent_harness_runtime.provider_stream_parser_rails.budgets.oversized_frame]
- GIVEN a stream frame exceeds the admitted frame-byte budget
- WHEN the parser entrypoint receives the frame
- THEN the parser returns an oversized-frame failure before concatenating it into aggregate state
- AND durable evidence records only safe size class, budget kind, and optional BLAKE3 identity.

### Requirement: Parser-entrypoint fixtures use raw provider-like chunks

r[agent_harness_runtime.provider_stream_parser_rails.entrypoint_fixtures] Stream parser tests MUST include provider-like raw chunks, SSE lines, JSON records, split boundaries, malformed fragments, and terminal markers that enter through parser entrypoints instead of only normalized event constructors.

#### Scenario: Split boundary stream normalizes correctly

r[agent_harness_runtime.provider_stream_parser_rails.entrypoint_fixtures.split_boundary]
- GIVEN a valid provider event is split across two admitted chunks
- WHEN the parser processes both chunks in order
- THEN it emits the same normalized event as the unsplit fixture
- AND budget accounting reflects both chunks.

### Requirement: Stream parser receipts are safe summaries

r[agent_harness_runtime.provider_stream_parser_rails.receipts] Stream parser successes and failures MUST project to safe receipts containing parser class, budget kind, counts, safe provider family, status class, BLAKE3 identities where useful, and redaction markers without raw chunks, hidden reasoning, provider payloads, credentials, private paths, or unbounded logs.

#### Scenario: Hidden reasoning leak is rejected

r[agent_harness_runtime.provider_stream_parser_rails.receipts.hidden_reasoning]
- GIVEN a provider stream exposes hidden reasoning or raw provider payload data not admitted for durable evidence
- WHEN receipt projection runs
- THEN validation rejects the leak or records only an allowed redaction marker
- AND the raw hidden reasoning or provider payload is not persisted.

### Requirement: Provider stream parser verification

r[agent_harness_runtime.provider_stream_parser_rails.verification] Stream parser hardening MUST include positive ordered-stream and split-boundary tests plus negative invalid-encoding, malformed-record, oversized-frame, record-budget, aggregate-budget, duplicate-terminal, missing-terminal, out-of-order-delta, unsupported-event, hidden-reasoning-leak, and receipt-leak tests before archive.

#### Scenario: Closeout proves parser hardening but not live provider parity

r[agent_harness_runtime.provider_stream_parser_rails.verification.closeout]
- GIVEN maintainers prepare to archive parser hardening
- WHEN closeout validation runs
- THEN evidence names parser inventory, typed failures, budget-before-aggregation tests, raw-entrypoint fixtures, receipt safety, docs, Cairn validation, and Cairn gates
- AND live provider clients, network transport, retries, SDK-specific streaming guarantees, and broad provider parity remain omitted non-claims unless separately indexed.
