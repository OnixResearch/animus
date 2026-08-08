# Prime Agent continual-harness cohort fixture

- Repository: `https://github.com/PrimeIntellect-ai/prime-agent`
- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Reviewed cohort: `prime-agent-continual-harness`
- Reviewed surface: `/refine` supplemental prompt, memory, skill-description, and reusable subagent-specification updates.
- Reviewed semantics: evidence-backed proposals, immutable base system prompt, snapshot-before-update, bounded rollback, and session-local state by default.

Animus adopts only this bounded semantic subset. Negative coverage includes `base_prompt`, `not-automatic-promotion`, and `not-lesson-correctness`. The fixture does not establish `not-model-improvement`, `not-generalization`, `not-production-readiness`, or `not-release-eligibility`.
