## Context

Animus already validates prompt sources, model profiles, skills, memory policy, tools, and context references. Each owner has its own trust and authority rules.

The missing layer is a neutral request-local selection algebra. It must select already-admitted contributions without acquiring source authority or replacing the existing prompt assembler.

Selected skill bodies also need a logical-turn lifetime. A tool call that selects a skill cannot count as successful use before a later provider request consumes the body.

Autolith provides the reference scenarios for priority, supersession, conflict groups, edge-triggered context, and next-response skill behavior.

## Decisions

### Decision: define one typed contribution envelope

**Choice:** A contribution contains a nominal contribution ID, source class, trust class, mandatory posture, priority, finite budget cost, payload identity, deduplication key, superseded IDs, optional conflict group, lifetime, sensitivity class, and safe label.

Payload bytes remain host-loaded values. The reducer operates on metadata and bounded borrowed payload views only where prompt assembly requires them.

**Rationale:** One envelope lets independent owners participate without sharing source or persistence authority.

### Decision: keep the reducer pure and order-independent

**Choice:** The reducer validates all contributions first. It then applies exact supersession, deduplication, conflict, priority, and budget rules.

Canonical identity order breaks ties only after all semantic fields match. Source enumeration order, filesystem order, and hash-map iteration never affect selection.

**Rationale:** Request composition must replay deterministically from the same admitted inputs.

### Decision: mandatory contributions fail closed

**Choice:** A missing, denied, conflicting, superseded, malformed, or over-budget mandatory contribution blocks provider dispatch.

Advisory contributions can be omitted with bounded diagnostics. An advisory contribution cannot displace a mandatory contribution through priority alone.

**Rationale:** Mandatory policy and safety context cannot degrade silently.

### Decision: model finite lifetimes as explicit state

**Choice:** Initial lifetimes are `CurrentRequest`, `UntilNextProviderResponse`, and `CurrentLogicalTurn`.

The reducer returns a selected plan and a follow-up lifetime transition. The host supplies exact provider request and response observations before transition.

No default lifetime persists into another logical turn.

**Rationale:** Explicit expiration prevents hidden prompt state and stale instruction reuse.

### Decision: make skill selection an edge-triggered contribution

**Choice:** An explicit skill-selection action binds the current session, turn, provider target, model profile, manifest identity, body identity, and selection identity.

The host loads the exact current body after selection. The body enters one `UntilNextProviderResponse` contribution.

Logical turn completion remains blocked until one admitted provider request binds that contribution and its correlated response completes or fails terminally.

The skill-selection result exposes metadata only. It does not echo the skill body.

**Rationale:** This proves that a selected skill affects a later model step instead of only returning a successful tool result.

### Decision: integrate under model invocation composition

**Choice:** Existing prompt, skill, memory, project-context, MCP, and runtime owners produce admitted contributions. The new reducer selects them before `compose_model_invocation_plan` seals the prompt plan.

Model fallback, profile drift, policy drift, or target changes require full contribution recomposition. A selected skill cannot cross targets by inheritance.

**Rationale:** A second top-level prompt path would weaken the exact model-profile binding.

### Decision: separate payload retention from selection evidence

**Choice:** Request-local and sensitive payloads cannot enter the session ledger, compaction source, memory generation, quality records, or receipts by default.

Receipts contain contribution IDs, source classes, lifetime classes, payload hashes, selection classes, omission classes, budget counts, and BLAKE3 plan identity.

**Rationale:** Context selection needs auditable structure, not durable copies of private instructions.

## Reduction Order

1. Validate every contribution and payload identity.
2. Reject duplicate contribution IDs with different facts.
3. Apply explicit supersession.
4. Collapse exact deduplication groups.
5. Resolve conflict groups.
6. Reserve budget for mandatory contributions.
7. Select advisory contributions by priority and canonical identity.
8. Produce ordered prompt inputs and omission diagnostics.
9. Bind the plan to the exact model invocation.
10. Advance lifetimes only from correlated provider observations.

## Alternatives Considered

### Extend prompt-source precedence only

Rejected because skills, MCP instructions, edge-triggered notices, and memory results need finite lifetimes and conflict semantics beyond source precedence.

### Persist selected context in the transcript

Rejected because request-local context is not conversation history and can contain sensitive instructions.

### Let each contributor resolve conflicts independently

Rejected because independent conflict rules can produce order-dependent or contradictory prompt plans.

### Treat skill selection as complete after body load

Rejected because loading does not prove that a provider request consumed the body.

## Risks and Trade-offs

- One neutral envelope can become too broad. Keep domain authority and validation in current owners.
- Mandatory contributions can block requests under tight budgets. The result must name the bounded conflict without exposing payloads.
- Provider cancellation can leave a skill contribution unconsumed. The terminal transition must expire or explicitly retain it under policy.

## Non-Claims

This design does not prove prompt correctness, model compliance, skill quality, memory truth, MCP server trust, provider delivery, hidden-context confidentiality, task completion, production readiness, or release eligibility.
