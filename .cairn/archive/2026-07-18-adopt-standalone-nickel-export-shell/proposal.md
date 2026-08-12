## Why

Animus currently uses Nix-owned Nickel generation commands for TOML, YAML, JSON, and raw field projections, followed by exact byte comparisons. `OnixResearch/nickel-export` now publishes those generic shell mechanics and evaluator-neutral identities at immutable revision `257fafc1c746f1faf156207043a4c826bfb16d49`. Adoption should reduce duplicate infrastructure without moving agent/runtime contracts, destination authority, provider policy, or release gates outside Animus.

## What Changes

- Pin the standalone Nickel export source and shell to the exact published revision. r[agent_harness_runtime.nickel_export_cutover.source]
- Add a Nix-owned wrapper that invokes the pinned external shell for checked destinations while keeping reusable crates free of process and filesystem authority. r[agent_harness_runtime.nickel_export_cutover.boundary]
- Preserve Animus agent/runtime profile contracts, generated destinations, provider policy, and release gates. r[agent_harness_runtime.nickel_export_cutover.authority]
- Dual-run Cargo and lint config, pre-commit YAML, the recipe catalog, and runtime/role profile JSON outputs. r[agent_harness_runtime.nickel_export_cutover.dual_run]
- Keep current generation commands authoritative until every positive and negative contract fixture agrees and rollback evidence is recorded. r[agent_harness_runtime.nickel_export_cutover.rollback]

## Impact

- **Shared dependency**: exact Git revision `257fafc1c746f1faf156207043a4c826bfb16d49` of `github.com/OnixResearch/nickel-export`.
- **Animus surfaces**: Nix generation wrappers, source pins, checked config outputs, negative contract fixtures, docs, and release evidence; ordinary runtime crates remain unchanged.
- **Authority**: Animus remains the sole owner of profile contracts, destinations, provider and runtime policy, and agent-harness release decisions.
- **Claims**: migration proves checked request, exact byte, selector, diagnostic, and freshness compatibility only. It does not prove evaluator equivalence, policy correctness, runtime correctness, deployability, or release eligibility.
