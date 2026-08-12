# Agent Harness Runtime

## ADDED Requirements

### Requirement: Agent messaging remains an exact optional boundary

r[agent_harness_runtime.messaging_receipts.boundary] Animus MUST provide agent-to-agent messaging semantics as a focused non-default contract family pinned to the reviewed prime-agent cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire transport, network, process, filesystem, credential, or persistence authority from messaging.

#### Scenario: Messaging profile is selected

r[agent_harness_runtime.messaging_receipts.boundary.selected]
- GIVEN an embedder selects the reviewed messaging profile and supplies roster state, counters, and a host transport
- WHEN messaging is composed
- THEN Animus MAY produce routing decisions, limits, and receipts for family-scoped messages
- AND the existing interrupt classifier, orchestration, Iroh/Syndicate, subagent, and receipt owners MUST remain authoritative.

#### Scenario: Messaging expects ambient authority

r[agent_harness_runtime.messaging_receipts.boundary.no_ambient]
- GIVEN a caller or the model expects the messaging layer to open connections, discover peers, or persist queues directly
- WHEN a decision runs
- THEN Animus MUST return a typed unsupported-authority outcome
- AND it MUST NOT substitute an ambient implementation.

### Requirement: Messaging policy is Nickel-authored and revalidated

r[agent_harness_runtime.messaging_receipts.profile] Human-authored messaging policy MUST use typed Nickel contracts for roster shape, admitted modes per role, size, rate, and pending-queue limits, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.messaging_receipts.profile.valid]
- GIVEN a checked profile names the exact cohort, roster shape, per-role modes, finite limits, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no routing decision or receipt may proceed before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.messaging_receipts.profile.invalid]
- GIVEN generated policy is stale, omits a limit, admits cross-family broadcast, or weakens claim boundaries
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no message may be routed.

### Requirement: Sender identity is host-derived and roster-bound

r[agent_harness_runtime.messaging_receipts.roster] Sender identity MUST come from the host session registry and never from message content, targets MUST resolve against the admitted family roster, and broadcast MUST resolve only within that roster.

#### Scenario: Message routes within the family roster

r[agent_harness_runtime.messaging_receipts.roster.in_family]
- GIVEN a message whose sender the host attests and whose target is an admitted roster member
- WHEN target resolution runs
- THEN it MUST return a routable decision carrying the host-derived sender identity
- AND the message content MUST NOT be able to override sender or target authority.

#### Scenario: Spoofed or unknown identity is supplied

r[agent_harness_runtime.messaging_receipts.roster.rejected]
- GIVEN a message claims a sender in its content, names an unknown target, or requests cross-family broadcast
- WHEN resolution runs
- THEN it MUST fail closed with a typed outcome before routing
- AND no delivery, queue write, or ambient registration may occur.

### Requirement: Delivery modes are pure routing decisions

r[agent_harness_runtime.messaging_receipts.modes] Routing MUST decide `auto`, `steer`, or `follow_up` semantics from the message, supplied target state, and policy, with `auto` delivering to idle targets and steering busy targets, `steer` injecting into active work, and `follow_up` waiting for the current turn to finish.

#### Scenario: Mode resolves over target state

r[agent_harness_runtime.messaging_receipts.modes.resolved]
- GIVEN an admitted message and supplied target state
- WHEN the routing decision runs
- THEN it MUST return the mode-correct decision (`auto` on idle yields deliver, on busy yields steer; `follow_up` yields enqueue-after-turn)
- AND identical inputs MUST produce identical decisions.

#### Scenario: Mode is denied for the sender role

r[agent_harness_runtime.messaging_receipts.modes.denied]
- GIVEN policy denies `steer` for the sender's roster role
- WHEN a `steer` or busy-target `auto` message routes
- THEN it MUST fail closed with a typed denial receipt
- AND no interrupt or injection may reach the target.

### Requirement: Delivery receipts distinguish delivered from queued

r[agent_harness_runtime.messaging_receipts.receipts] Delivery receipts MUST carry exactly the success variants `delivered` and `queued`, `delivered` MUST require host attestation of entry into an idle target's context, and `queued` MUST require host attestation of accepted pending storage.

#### Scenario: Delivery produces the correct receipt variant

r[agent_harness_runtime.messaging_receipts.receipts.variants]
- GIVEN a routed message and host attestation of delivery or queue acceptance
- WHEN the receipt is projected
- THEN it MUST carry the matching variant with attestation identity
- AND no receipt variant MAY claim reading, understanding, ordering, or exactly-once delivery.

#### Scenario: Receipt lacks attestation

r[agent_harness_runtime.messaging_receipts.receipts.unattested]
- GIVEN a delivery claim without host attestation
- WHEN receipt validation runs
- THEN it MUST fail with a bounded typed issue
- AND the message MUST NOT be recorded as delivered or queued.

### Requirement: Messaging limits enforce before routing

r[agent_harness_runtime.messaging_receipts.limits] Message-size, rate, and pending-queue limits MUST be pure checks over host-supplied counters that run before any routing decision, and violations MUST return typed limit receipts without reaching a target.

#### Scenario: Message fits all limits

r[agent_harness_runtime.messaging_receipts.limits.admitted]
- GIVEN a message and counters within admitted size, rate, and queue bounds
- WHEN limit checks run
- THEN routing MUST proceed
- AND the limit decision MUST record which counter identities decided.

#### Scenario: Message violates a limit

r[agent_harness_runtime.messaging_receipts.limits.rejected]
- GIVEN a message exceeding size, rate, or pending-queue bounds
- WHEN limit checks run
- THEN the check MUST fail closed with a typed limit receipt naming the violated class
- AND no routing decision, delivery, or queue write may follow.

### Requirement: Messaging evidence is metadata-only

r[agent_harness_runtime.messaging_receipts.evidence] Messaging receipts MUST bind only policy identity, roster identity, sender and target identities, mode, receipt variant, counter identities, bounded counts, required non-claims, and domain-separated BLAKE3 identities, and MUST exclude message bodies.

#### Scenario: Safe messaging receipt is projected

r[agent_harness_runtime.messaging_receipts.evidence.safe]
- GIVEN routing, limits, and delivery complete under policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, mode, variant, and required non-claims
- AND it MUST describe only bounded routing and delivery facts.

#### Scenario: Receipt contains content or overclaims

r[agent_harness_runtime.messaging_receipts.evidence.rejects]
- GIVEN a receipt draft includes message bodies, credentials, or a read, acted, ordered, or exactly-once claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, or release evidence.

### Requirement: Messaging adoption has positive and negative conformance evidence

r[agent_harness_runtime.messaging_receipts.verification] The messaging contract MUST include positive and negative tests through production roster, routing, limit, and receipt entrypoints; cohort fixture parity; default and no-std dependency isolation; and a focused deterministic rail.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.messaging_receipts.verification.positive]
- GIVEN fixtures cover all three modes on busy and idle targets, both receipt variants, roster-bound broadcast, and admitted limit edges
- WHEN the focused conformance rail runs
- THEN production entrypoints MUST agree with the reviewed prime-agent semantics for the admitted subset
- AND fixture provenance and any semantic limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.messaging_receipts.verification.negative]
- GIVEN a fixture attempts spoofed identity, unknown targets, cross-family broadcast, limit violations, ambient transport use, receipt content leakage, or authority widening
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of new effect authority.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.messaging_receipts.verification.closeout]
- GIVEN maintainers intend to sync and archive the messaging change
- WHEN closeout validation runs
- THEN focused messaging, runtime, Nickel, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND transport security, exactly-once delivery, comprehension, production readiness, and release eligibility MUST remain separate non-claims.
