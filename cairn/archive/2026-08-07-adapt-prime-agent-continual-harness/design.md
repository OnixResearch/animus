## Context

Prime Agent's Continual Harness (arXiv:2605.09998) stores supplemental prompts, memories, skill descriptions, and reusable subagent specifications as durable state. The `/refine` command reviews the current trajectory and applies small, evidence-backed updates. The base system prompt is immutable. Recorded snapshots support rollback. Updates are session-local by default.

Animus already owns the target surfaces separately: `model-runtime-profiles` composes per-model prompt, skill, memory, and tool inputs; `hermes-skill-manifests` describes skills; `checkpoint-rollback` provides VCS-neutral rollback DTOs. What is missing is the loop that turns trajectory evidence into reviewed updates to supplemental state. Today supplemental state changes only through host-authored edits.

Completion requires a pure proposal-and-admission core with snapshot rollback, explicit scope rules, and metadata-only history. A refinement path that touches the base prompt, promotes silently to durable state, or records lesson bodies in receipts is false completion.

## Decisions

### Decision: refinement is a pure proposal-and-admission core

**Choice:** A focused no-std crate owns refinement proposal DTOs, admission decisions, snapshot planning, and rollback planning. The host shell owns trajectory review triggers, snapshot storage, and durable promotion. Every proposal carries typed evidence references into the session ledger.

**Rationale:** Refinement decisions are deterministic given proposals, current state, and policy. Keeping them pure preserves the Animus test story: plain assertions, no mocks, no world.

### Decision: the base system prompt is outside the target set

**Choice:** Proposal targets are exactly four classes: supplemental prompts, memories, skill descriptions, and subagent specifications. The base system prompt, repo-owned skill packages, accepted specs, and policy files are not valid targets. Admission rejects them with a typed outcome.

**Rationale:** The immutable-base rule is what keeps self-modification reviewable. Encoding it in the type system and admission, not in prose, makes violations unrepresentable or fail-closed.

### Decision: every admitted update is preceded by a snapshot

**Choice:** Admission produces a snapshot plan before any update applies. Snapshots carry domain-separated BLAKE3 identity over the pre-update supplemental state. Rollback restores an admitted snapshot exactly and emits its own receipt. Retention bounds keep snapshot count and size finite.

**Rationale:** Refinement will sometimes be wrong. Cheap, deterministic rollback converts bad refinements from damage into data.

### Decision: session-local by default, promotion is explicit

**Choice:** Admitted refinement state lives in session scope. Promotion to project or personal durable state is a separate host action with its own admission and receipt. Refinement proposals cannot request promotion.

**Rationale:** Session-local default matches prime-agent behavior and prevents one session's lessons from silently changing other sessions or the repo.

## Alternatives Considered

### Let the model edit prompt files directly through tools

Rejected because it bypasses admission, evidence binding, snapshots, and scope rules, and it would mix refinement with ordinary file edits in the ledger.

### Store refinements only as free-form memory text

Rejected because free-form memory cannot enforce target classes, bounds, or conflict detection, and cannot support deterministic rollback.

### Promote refinements automatically when confidence is high

Rejected because automatic promotion moves authority from the host to heuristic confidence. Promotion stays an explicit host action.

## Risks / Trade-offs

- Refined supplemental state can drift from repo-owned intent over a long session. Mitigation: bounded retention, conflict detection against policy, and receipts that make drift auditable.
- Evidence references can point at weak or irrelevant trajectory spans. Mitigation: admission requires typed evidence references and rejects evidence-free proposals; quality of lessons remains a non-claim.
- Snapshot growth on chatty refinement loops. Mitigation: named count and byte bounds with fail-closed eviction policy.

## Non-Claims

This design does not establish lesson correctness, model improvement, generalization, base-prompt mutability, automatic durable promotion, cross-session consistency, production readiness, or release eligibility.
