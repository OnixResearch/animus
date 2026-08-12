# Evidence: adapt Prime Agent RLM kernel

## Baseline and ownership

- Prime Agent source: `https://github.com/PrimeIntellect-ai/prime-agent`
- Reviewed revision: `c5991bc853d27754aed345c13ff4d2e05c40f1f4`
- Reviewed files: `packages/coding-agent/docs/rlm.md` and `packages/coding-agent/docs/architecture.md`
- License: MIT
- Local cohort record: `fixtures/rlm-kernel/prime-agent-c5991bc.md`
- Animus owners: `animus-adapter-rlm-kernel` owns pure contracts, `animus-steel` owns sandboxed persistent evaluation, and `animus-runtime` owns exact injected host execution.

## Implementation evidence

- Added a no-std and Wasm-compatible kernel contract crate.
- Added typed policy, environment, evaluation, host-request, child, compaction, and receipt contracts.
- Added Nickel policy source, deterministic JSON export, positive fixtures, and negative fixtures.
- Added an optional persistent `SteelRlmKernel` and `steel_repl` tool adapter.
- The Steel engine uses `Engine::new_sandboxed()` and a named evaluation timeout.
- Failed cells rebuild from committed sources and do not mutate committed state.
- Direct effects fail inside the sandbox. Typed requests require Rust admission before an injected host executor can run.
- Child requests bind profile, context, depth, fan-out, token, wall-time, result-size, and lineage bounds.
- Compaction plans preserve kernel state. Only an explicit host clear can clear it.
- Receipts reject variable values, code bodies, child transcripts, credentials, raw paths, and prohibited claims.

## Adversarial review

The implementation was reviewed through four separate mechanism lenses:

1. Pure contract state transitions and stale-admission rejection.
2. Sandboxed transactional Steel replay and timeout behavior.
3. Runtime host-executor separation and exact injected dispatch.
4. Metadata-only evidence and compaction-survival failure cases.

The negative fixtures cover bound violations, direct effects, unadmitted operations, ambient child requests, stale admissions, oversized results, implicit clears, leaks, malformed policy, and overclaims.

## Verification

The following checks passed before closeout:

```text
scripts/cairn/check-rlm-kernel --root .
scripts/cairn/check-adapter-family-crates --root .
scripts/cairn/check-component-catalog --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-composition-conformance --root .
scripts/cairn/check-artifact-safe-receipts --root .
scripts/cairn/check-wasm-surface --root .
nix run .#check-nickel-configs
nix build .#checks.x86_64-linux.rlm-kernel --no-link -L
nix run .#cairn -- gate proposal adapt-prime-agent-rlm-kernel --root .
nix run .#cairn -- gate design adapt-prime-agent-rlm-kernel --root .
nix run .#cairn -- gate tasks adapt-prime-agent-rlm-kernel --root .
nix run .#cairn -- sync adapt-prime-agent-rlm-kernel --root . --execute
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo fmt --all -- --check
git diff --check
git diff --cached --check
```

The workspace test run passed. The RLM-focused tests passed for the no-std contract, policy JSON, sandboxed Steel kernel, runtime host executor, root feature, no-default-features build, and Wasm target.

The live Linux bubblewrap test remains intentionally ignored unless its existing opt-in environment is present. This change does not depend on that test and does not add a sandboxing claim.

The post-archive roadmap rail remained blocked by eight unrelated active changes that are absent from the roadmap. The reported changes were `adapt-prime-agent-autonomous-gates`, `adapt-prime-agent-continual-harness`, `adapt-prime-agent-daemon-workers`, `adapt-prime-agent-messaging-receipts`, `adapt-prime-agent-persistent-goals`, `adapt-prime-agent-schedule-reliability`, `add-pijul-vcs-adapter`, and `seal-runtime-authority-units`. The RLM kernel change was archived and was not in this blocker set.

## Non-claims

This evidence does not establish security sandboxing, Python or IPython compatibility, model correctness, arbitrary recursion safety, cross-session sharing, task completion, production readiness, or release eligibility.
