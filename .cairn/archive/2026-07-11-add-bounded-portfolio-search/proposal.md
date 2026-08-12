## Why

Animus already validates prompt sources, fresh-context tasks, worker teams, subagent fan-out/fan-in, retrieval budgets, and evidence-safe receipts. It does not yet own a typed state machine for diversity-preserving search across hard proofs, architecture decisions, root-cause hypotheses, security reviews, or optimization routes.

Without a portfolio contract, hosts can fan out several correlated paraphrases, disclose a favored route before independent exploration, present an equivalent-strength reduction as progress, reopen blocked approaches without new evidence, or treat model synthesis as validated completion. These are orchestration failures that should be represented and rejected by pure runtime logic rather than left only in prompt prose.

## What Changes

- Add a typed portfolio contract for objective identity, completion evidence, false-completion cases, audit risks, source and validator policies, and explicit budgets. r[agent_harness_runtime.portfolio_search.contract]
- Add pure portfolio phases for framing, independent exploration, synthesis, targeted redirection, adversarial audit, validation, validated result, exhaustion, and blocking. r[agent_harness_runtime.portfolio_search.phases]
- Add an approach-family registry keyed by BLAKE3 mechanism fingerprints with claims, safe artifact/evidence references, gap strength, exact blockers, next discriminating checks, and state. r[agent_harness_runtime.portfolio_search.registry]
- Deny early cross-pollination and duplicate mechanisms, select underexplored live families deterministically, and reopen blocked routes only for new evidence or a new mechanism. r[agent_harness_runtime.portfolio_search.independence] r[agent_harness_runtime.portfolio_search.redirection]
- Require equivalent or stronger missing steps to remain blocked and prevent them from entering audit or validation as solved routes. r[agent_harness_runtime.portfolio_search.gap_strength]
- Add typed adversarial audit findings and pass-only validation receipts before a portfolio result becomes validated. r[agent_harness_runtime.portfolio_search.audit] r[agent_harness_runtime.portfolio_search.validation]
- Add safe hash-only portfolio receipts, positive and negative tests, public exports, and documentation. r[agent_harness_runtime.portfolio_search.receipts] r[agent_harness_runtime.portfolio_search.verification]

## Impact

- **Rust**: new pure `animus-runtime::portfolio_search` module plus facade exports.
- **Docs**: a maintained portfolio-search contract and README link.
- **Host boundary**: hosts still own model calls, worker scheduling, concurrency, clocks, retrieval, tools, files, networks, persistence, and validation execution.
- **Compatibility**: this change does not implement or replace `receipt-authoritative-agent-completion`. A `Validated` portfolio phase means only that a supplied audit/validation receipt matches the portfolio contract and selected route; it does not mean repository, release, or agent completion.
- **Non-claims**: mechanism-family labels and gap-strength classifications are validated supplied metadata, not semantic proof that two natural-language approaches are equivalent or that a validator is sound.
