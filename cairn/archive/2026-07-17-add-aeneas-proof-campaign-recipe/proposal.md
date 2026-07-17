## Why

Animus already has typed recipe workflows, capability-bound semantic roles, per-model runtime profiles, skill manifests, focused verification rails, safe receipts, and authoritative-completion boundaries. It does not yet compose those pieces into a proof campaign where theorem statement review, generated-model handling, proof mutation, independent verification, resource budgets, and honest blocked/counterexample outcomes are explicit workflow contracts.

Aeneas provides useful proof-engineering guidance, but copying its prompts or repository-specific workflow would bypass Animus's authority, role, and receipt model. Animus should adapt the reusable campaign structure as an Animus-owned typed recipe over injected Octet/Aeneas evidence services.

## What Changes

- Add a Nickel-authored Aeneas proof-campaign recipe profile that composes existing recipe, role, model, skill, tool, budget, and safe-receipt contracts rather than creating a second agent runtime.
- Define scoped statement-author, statement-reviewer, prover, and verifier assignments using existing semantic role classes, explicit separation policies, immutable subject identities, and typed handoffs.
- Require theorem statement approval before proof execution, including requirement ID, statement BLAKE3, claim class, source/model identity, adequacy/vacuity decision, and declared trust closure.
- Separate generated translation artifacts from maintained theorem source and reject workflows that let generation overwrite maintained files or let agents mutate generated output as a substitute for regeneration.
- Use one canonical compact skill source with BLAKE3 identity and generated manifest/catalog freshness; do not copy Aeneas's Git workflow, magic thresholds, confirmation pauses, or project-specific tactic bans.
- Model named proof-time, tool-call, output, retrieval, log, and retained-artifact budgets plus typed terminal outcomes for proved, blocked, counterexample, invalid statement/model, budget exhausted, tool failure, denied, cancelled, and abstained.
- Keep proof-tool execution, filesystem/process/network authority, provider dispatch, clocks, and artifact storage in injected host services; reusable cores validate plans, handoffs, outputs, and metadata-only receipts.
- Add a focused proof-campaign rail with positive and negative fixtures for role separation, statement approval, vacuity, trust closure, generated/maintained boundaries, manifest drift, budget exhaustion, tool failure, unsafe handoffs, and false completion.

## Impact

- **Configuration**: typed Nickel campaign profile and deterministic generated projection.
- **Runtime core**: pure campaign-plan, transition, handoff, terminal-outcome, and receipt validation over existing recipe/role contracts.
- **Host shell**: injected proof/evidence services only; no bundled Aeneas, Charon, Lean, Git, or ambient shell authority.
- **Documentation**: compact campaign usage and claim boundaries linked from existing recipe/role/skill docs.
- **Testing**: paired positive/negative fixtures and a named focused rail.
- **Claims**: a valid campaign receipt proves only that Animus enforced the declared workflow, authority, separation, budget, handoff, and metadata contracts. It does not prove theorem truth, proof-tool soundness, reviewer independence, source/model equivalence, requirement adequacy, or product correctness.
