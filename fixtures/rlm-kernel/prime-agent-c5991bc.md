# Prime Agent RLM cohort fixture

- Repository: `https://github.com/PrimeIntellect-ai/prime-agent`
- Revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- License: MIT
- Reviewed documentation: `packages/coding-agent/docs/rlm.md`
- Reviewed architecture: `packages/coding-agent/docs/architecture.md`
- Retrieval date: 2026-08-06

## Admitted semantic subset

- The model uses one persistent programming environment as a tool.
- Successful variables and helper definitions remain available across turns.
- Program code returns typed host requests for authoritative host state and effects.
- `rlm(...)` requests bounded child work with explicit context and budgets.
- Prompt compaction does not clear the programming environment.

## Animus adaptation

Animus uses Steel instead of IPython. Rust revalidates every request before an injected host receives it.

The positive Rust fixtures cover persistent definitions, context bindings, typed requests, child admission, bounded results, and compaction preservation.

The negative Rust fixtures cover direct effects, unadmitted operations, bound violations, ambient children, implicit clears, payload leaks, and prohibited claims.

## Limitations

This fixture does not claim Python or IPython compatibility. It does not establish a security sandbox, semantic equivalence, model correctness, task completion, production readiness, or release eligibility.
