## Context

Animus already separates pure runtime decisions from host effects. Existing contracts cover session replay, interrupted tool repair, provider retry, steering commands, and authoritative completion.

The missing contract begins when a host accepts work. It ends when the turn reaches one durable terminal state or a typed blocked state.

camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2` is a bounded design reference. Its turn journal records accepted user and steering messages before later model work.

The useful invariant is write ordering, not its Cloudflare storage model. Animus must use explicit transactions, BLAKE3 identities, injected services, and fail-closed corruption handling.

Completion requires deterministic plans for every declared crash window. Source comments, a successful continuation, or a fake that cannot interrupt writes are false completion.

## Decisions

### Decision: define a pure recovery state machine

**Choice:** Add typed drafts and validated values for the session, branch, turn, accepted message sequence, journal generation, stable turn identity, checkpoint identity, steering sequence, and policy.

The pure core validates state and returns ordered actions. It does not read storage, use a clock, allocate identifiers, serialize host records, dispatch providers, deliver steering, or run tools.

**Rationale:** Recovery logic must be testable without a database, provider, process, network, or UI.

### Decision: commit acceptance before provider eligibility

**Choice:** A new turn becomes provider-eligible only after the host returns durable evidence for the accepted user message and active marker.

The durable acceptance transaction binds the session, branch, turn sequence, accepted message identity, policy identity, and expected prior journal generation. The transaction creates one stable active-turn identity.

A steering message follows the same rule. Runtime delivery requires a durable append observation bound to the active turn and steering sequence.

**Rationale:** The accepted message is the earliest fact that must survive. Later provider events cannot repair a message that never reached durable state.

### Decision: derive stable identities from typed facts

**Choice:** Use domain-separated BLAKE3 identities over admitted typed fields. The turn identity includes session, branch, monotonic turn sequence, accepted message identity, and policy identity.

The checkpoint identity includes the turn identity, journal generation, ordered entry identities, and replacement policy. The steering identity includes the turn identity and monotonic steering sequence.

Host timestamps are optional observations for display and retention. Random identifiers can exist as host correlation values, but they cannot replace canonical identities.

**Rationale:** Repeated text and equal timestamps must not alias. Content identities also make stale recovery and receipt replay detectable.

### Decision: replace checkpoints through one explicit transaction

**Choice:** The host prepares and bounds every replacement entry before it starts mutation. The pure core checks the complete prepared manifest and returns one compare-and-swap transaction request.

The host transaction checks the expected prior generation, writes the complete new checkpoint, updates the active marker, and exposes the new generation atomically. It never deletes the old checkpoint before replacement preparation finishes.

A partial transaction, mismatched generation, serialization failure, oversized entry, or store error returns no success evidence. Recovery keeps the last complete generation or blocks if store facts conflict.

**Rationale:** The absence of an `await` or process yield is not atomicity. Explicit transaction evidence is required.

### Decision: compose existing interrupted tool repair without rerunning tools

**Choice:** Recovery first validates the active-turn checkpoint and accepted work. It then delegates model-visible dangling tool-call diagnosis to the existing interrupted tool repair core.

Completed results remain unchanged. Unmatched calls can receive only the existing evidence-bound synthetic terminal failure plan. Recovery cannot construct or dispatch a tool request.

Provider continuation requires all journal, replay, repair, authority, cancellation, and exact-plan gates to pass. Provider retry remains a separate same-plan decision after a provider failure.

**Rationale:** Active-turn recovery closes an earlier durability gap. It must not duplicate or weaken the established repair and retry owners.

### Decision: fail closed on corruption and ambiguity

**Choice:** Malformed records, gaps, duplicates, unknown kinds, invalid order, identity mismatch, unsupported schema, stale generations, conflicting terminal states, and over-budget collections produce typed blocked outcomes.

A host can quarantine corrupt bytes outside reusable code. Reusable recovery does not skip a row, guess intent, truncate silently, or continue from a partial checkpoint.

**Rationale:** Silent row skipping can remove an accepted message or tool result while making the remainder appear valid.

### Decision: keep terminal cleanup evidence-bound

**Choice:** Active state can close only after a durable terminal ledger commit matches the active turn and final journal generation.

Cleanup of the active marker, checkpoint, and steering queue is a separate host mutation. Failed cleanup records retained recovery state and does not change the terminal turn outcome.

Cancellation and abandonment remain explicit terminal classes. They do not imply provider cancellation or absence of remote effects.

**Rationale:** Clearing recovery state before the terminal commit creates another loss window.

### Decision: emit metadata-only receipts

**Choice:** Receipts bind schema, policy, session and branch fingerprints, turn and checkpoint identities, generation and bounded counts, crash-window class, recovery action class, repair status, terminal class, redaction marker, and non-claims.

Receipts reject messages, prompts, tool arguments or results, provider bodies, credentials, environment values, physical paths, raw store errors, and timestamps used as identity.

**Rationale:** Recovery evidence needs correlation and status. It does not need private transcript content.

## Crash-Window Matrix

| Window | Required result |
| --- | --- |
| After input validation and before durable acceptance | No provider dispatch. The caller receives no accepted-work claim. |
| After durable acceptance and before provider dispatch | Recovery returns the same active turn with the accepted user message. |
| During replacement preparation | The prior complete checkpoint remains current. |
| During the atomic replacement transaction | The store exposes one complete generation or returns blocked evidence. |
| After a tool call and before its result | Existing interrupted tool repair plans one synthetic failure. No tool rerun occurs. |
| After a real tool result and before model continuation | Recovery preserves the real result and continues without another tool effect. |
| After steering acceptance and before delivery | Recovery retains the steering message for one correlated delivery attempt. |
| After terminal ledger commit and before cleanup | Recovery observes the terminal state and performs no provider or tool dispatch. |

## Alternatives Considered

### Extend interrupted tool repair only

Rejected because that contract starts after a model-visible tool call. It does not protect accepted work before provider output.

### Store only the latest rendered message

Rejected because rendered state is a projection. It cannot replace model-visible ledger and tool-correlation facts.

### Skip malformed records and recover the remainder

Rejected because skipped records can contain the accepted user message or the only real tool result.

### Use timestamps or random UUIDs as canonical identity

Rejected because equal timestamps and repeated content can collide semantically. Random values do not bind the recovered facts.

## Risks and Trade-offs

- Explicit transactions narrow supported host stores. This cost is preferable to ambiguous durability claims.
- Fail-closed corruption handling can block useful work. Silent continuation creates a larger integrity risk.
- Retained terminal state can require operator cleanup after a store failure. The receipt must make this visible.
- The contract does not prove host transaction or storage implementation correctness.

## Non-Claims

This design does not prove power-loss durability, store correctness, tool non-execution, provider cancellation, provider acceptance, exactly-once remote effects, transcript completeness, UI continuity, distributed agreement, task completion, production readiness, or release eligibility.
