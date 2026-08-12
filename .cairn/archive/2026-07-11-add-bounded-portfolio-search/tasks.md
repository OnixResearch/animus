## Phase 1: Pure portfolio data model

- [x] [serial] Define objective, source-policy, and validator-policy fingerprints, finite budgets, success/non-solution criteria, required audit risks, phases, approach states, gap strengths, blockers, audit findings, and safe receipt DTOs. r[agent_harness_runtime.portfolio_search.contract] r[agent_harness_runtime.portfolio_search.phases] r[agent_harness_runtime.portfolio_search.receipts]
- [x] [serial] Define approach-family records with unique mechanism identity, claim identity, safe artifact/evidence references, next checks, and exact blocker metadata. r[agent_harness_runtime.portfolio_search.registry]

## Phase 2: Deterministic transition core

- [x] [serial] Implement pure contract, approach, dispatch, audit, validation-receipt, and safe-label validators. r[agent_harness_runtime.portfolio_search.contract] r[agent_harness_runtime.portfolio_search.independence] r[agent_harness_runtime.portfolio_search.audit] r[agent_harness_runtime.portfolio_search.receipts]
- [x] [serial] Implement pure phase transitions, duplicate-mechanism rejection, equivalent/stronger/unknown gap blocking, novelty-only reopening, and honest exhausted/blocked terminals. r[agent_harness_runtime.portfolio_search.phases] r[agent_harness_runtime.portfolio_search.gap_strength] r[agent_harness_runtime.portfolio_search.redirection] r[agent_harness_runtime.portfolio_search.validation]
- [x] [serial] Implement deterministic underexplored-family selection and domain-separated BLAKE3 portfolio fingerprints/receipts. r[agent_harness_runtime.portfolio_search.redirection] r[agent_harness_runtime.portfolio_search.receipts]

## Phase 3: Evidence and public surface

- [x] [parallel] Add positive tests for independent registration, distinct mechanisms, deterministic redirection, complete adversarial audit, matching validation, and safe receipt projection. r[agent_harness_runtime.portfolio_search.verification]
- [x] [parallel] Add negative tests for incomplete contracts, illegal transitions, early disclosure, duplicate mechanisms, equivalent-strength reduction theater, no-novelty reopening, incomplete audits, stale receipts, unadmitted validator policies, unsafe labels, and exhaustion masquerading as completion. r[agent_harness_runtime.portfolio_search.verification]
- [x] [parallel] Export the portfolio-search facade and add maintained documentation with authority/non-claim boundaries. r[agent_harness_runtime.portfolio_search.receipts] r[agent_harness_runtime.portfolio_search.verification]

## Phase 4: Validation and lifecycle closeout

- [x] [serial] Run focused `animus-runtime` portfolio-search tests, runtime library checks, formatting, and clippy. r[agent_harness_runtime.portfolio_search.verification]
  - Evidence: focused tests, library check, formatting, facade inventory, and safe-receipt rails pass; clippy reaches only the pre-existing unrelated `background/checks/mod.rs:223` finding recorded in `verification.md`.
- [x] [serial] Run canonical Cairn validation plus proposal, design, and tasks gates for this change. r[agent_harness_runtime.portfolio_search.verification]
  - Evidence: canonical validation and all three gates pass with the repository-pinned `.#cairn` command.
- [x] [serial] Sync the accepted runtime spec, dry-run archive, verify accepted requirement text, and archive only after all checks pass. r[agent_harness_runtime.portfolio_search.verification]
  - Evidence: sync executed; accepted requirements begin at `cairn/specs/agent-harness-runtime/spec.md:13020`; the first archive dry-run blocked only on this intentionally unchecked closeout task, which is now satisfied.
