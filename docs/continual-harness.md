# Continual harness

Animus provides an optional `continual-harness` contract family. It adapts the reviewed Prime Agent continual-harness subset at revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`.

The family admits four target classes:

- supplemental prompts;
- memories;
- skill descriptions;
- reusable subagent specifications.

The base system prompt, repo-owned skill packages, accepted specifications, and policy files are always rejected.

## Authority boundary

`animus-adapter-continual` is `no_std` and pure. It does not read trajectories, files, VCS state, clocks, processes, networks, credentials, or persistence stores. The host supplies proposals, current session-local state, and typed ledger evidence.

Every admitted proposal requires evidence, bounded content, conflict checks, and a valid operation. An admitted plan contains a deterministic BLAKE3 snapshot of the pre-update state. The host records the snapshot before it applies the new state. Rollback restores a retained snapshot exactly. Unknown, evicted, or forged snapshots fail closed.

Refinement state is session-local. A proposal cannot request promotion. A separate host action can create a project or personal promotion plan. The plan does not perform a durable write.

## Safe receipts

Receipts contain policy and cohort identities, target class, operation, status, snapshot and state identities, bounded counts, and required non-claims. Receipts do not contain lesson bodies, prompt text, memory contents, skill bodies, trajectory spans, or improvement, correctness, generalization, or authority claims.

## Validation

```sh
cargo test -p animus-adapter-continual --locked
cargo check -p animus-adapter-continual --target wasm32-unknown-unknown --locked
scripts/cairn/check-continual-harness --root .
nix build .#checks.x86_64-linux.continual-harness --no-link -L
```

This family does not claim lesson correctness, model improvement, generalization, automatic promotion, production readiness, or release eligibility.
