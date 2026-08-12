## Why

Animus already has agent definitions, subagent delegation, router roles, review reports, verifier evidence descriptors, judge-veto rules, UCAN-first capability requests, and per-model runtime profiles under active design. The existing subagent role-profile requirement only narrows model preference, tools, workspace, and output schema for one delegation surface. It does not define first-class semantic roles or say what an implementer, reviewer, verifier, or judge may request, what each may claim, how role assignment is authorized, or when one actor is too closely related to the subject to count as independent.

Leaving roles as prompt labels creates ambiguity. A router `Judge` step can be mistaken for a capability-bearing judge, a review prompt can retain write tools, a verifier can modify the subject or its verifier, an implementer can self-attest completion, and multiple role profiles can be unioned into broader authority. Animus needs capability-bound roles whose behavior is enforced by typed policy and pure admission rather than persona text.

## What Changes

- Add a typed Nickel-authored agent-role catalog with semantic role classes and baseline `implementer`, `reviewer`, `verifier`, and `judge` profiles plus constrained coordination and observation extension classes.
- Distinguish semantic agent roles from router execution roles, provider/model roles, agent-definition names, prompt modes, and UI labels; mappings between those axes are explicit and non-authoritative.
- Extend the capability vocabulary with guarded agent-role assignment, activation, handoff, candidate/report/evidence emission, and judgment selection/veto requests while keeping every role profile ceiling-only: activation authority never grants the role's model, tool, filesystem, process, network, Steel, workspace, output, or evidence abilities.
- Add pure role assignment and capability resolution that intersects verified grants, host policy, parent delegation, task scope, agent definition, semantic role, per-model runtime profile, workspace policy, tool catalog, executor availability, approval, sandbox, and budgets.
- Define class invariants for implementation, review, verification, judgment, coordination, and observation, including mutation posture, evidence/output contracts, abstention/failure behavior, and tighten-only review/judge decisions.
- Add explicit separation-of-duty policy over actor, run/thread, model, workspace, mutation lineage, subject, verifier, rubric, and policy identities; role labels alone never prove independence.
- Recompose prompts, skills, tools, and model eligibility when a role is assigned, handed off, changed, resumed, replayed, or moved to a fallback model, without carrying capabilities or pending calls across role boundaries.
- Add safe role-assignment, decision, handoff, and output receipts plus positive/negative fixtures and focused lifecycle rails.

## Impact

- **Surfaces**: agent-definition activation, UCAN/capability contracts, subagent capability resolution, worker/delegation DTOs, model-runtime-profile composition, tool admission, review/verifier/judge outputs, lineage, and authoritative-completion inputs.
- **Configuration**: typed Nickel role catalog and capability policy fixtures with deterministic checked exports; reusable Rust crates consume loaded validated values only.
- **Dependency**: implementation follows `add-per-model-runtime-profiles` so semantic-role resource requests are intersected with, not duplicated beside, model-specific prompts, skills, and tools.
- **Testing**: role catalog parity, assignment authorization, capability intersection, non-union, role invariants, self-review/self-verification denial, subject/verifier immutability, role transitions, fallback recomposition, output claim bounds, and receipt leak rejection.
- **Non-claims**: a role label does not establish human independence, model competence, verifier soundness, review correctness, judge correctness, completion, release eligibility, or authority absent normal grants and host enforcement. This change does not run providers, checks, tools, or reviewers in reusable cores.
