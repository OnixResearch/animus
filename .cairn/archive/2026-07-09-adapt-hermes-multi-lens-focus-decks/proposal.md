# Proposal: Adapt Hermes multi-lens focus deck contracts

## Why

Hermes' multi-lens and property-discovery skills are useful because they turn broad codebase analysis into independent focus passes with synthesis and contradiction handling. Animus should adapt the shape as SDK contracts for analysis decks, not as built-in subagents or mandatory prompts.

Embedders should be able to provide their own focus decks and worker services, assemble bounded context, and validate synthesis results while Animus keeps analysis orchestration optional and host-owned.

## What Changes

- Define typed focus deck, focus item, lens task packet, lens report, synthesis report, and review criteria DTOs.
- Integrate retrieval-budget and worker-task boundaries so hosts can run lenses through injected worker services, sequential checklists, or external systems.
- Validate provenance, evidence references, contradictions, skipped lenses, and confidence without requiring specific agent names or parallel execution.
- Add positive and negative tests for valid decks, malformed focus items, overbroad context, missing worker services, unsafe reports, contradiction handling, and no default execution.

## Impact

- **Files**: future runtime/subagent DTOs, analysis docs, tests, and focused fixtures.
- **Testing**: positive deck/task/synthesis cases plus negative unsafe-context, duplicate-focus, missing-service, stale-evidence, and leak fixtures.
- **Non-claims**: this does not ship Hermes `multi-lens`, spawn subagents by default, require property-discovery, depend on a specific UI, or make any analysis focus set mandatory.
