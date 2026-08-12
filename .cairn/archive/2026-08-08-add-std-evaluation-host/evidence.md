# Evidence: standard native evaluation host

## Scope

- Change: `add-std-evaluation-host`
- Host crate: `animus-std-eval`
- Root feature: `evaluation-host`, non-default
- Core owners: `animus-eval-core` and `animus-adapter-eval`
- Trace authority: canonical Animus session ledger only

## Prerequisites and baseline

- `add-native-evaluation-core`: accepted and archived.
- `add-evaluation-adapter-family`: accepted and archived.
- Active-turn durable acceptance and commit ordering: accepted in `agent_harness_runtime.active_turn_recovery`.
- Pre-change native-core, evaluation-adapter, Harbor evaluation, coding-session, durable-session, active-turn, provider-retry, process-job, and safe-receipt rails: PASS.

## Ownership inventory

- Checked policy loading: this std host reads one explicit caller-selected path; pure parsing is separate.
- Task, agent, verifier, artifact, model capture, append, replace, and reopen effects: nine existing `animus-adapter-eval` ports.
- Tool, timer, cancellation, and cleanup effects: four explicit host-only auxiliary ports.
- Model and task state capabilities: separate gateway identities supplied by the caller.
- Durable accepted-turn and terminal commit evidence: existing active-turn contract facts.
- Trace and replay: existing session ledger references; no second transcript store.
- Storage: explicit root capability identity plus logical relative target; no ambient path.
- Reduction, reward, and resume decisions: existing `animus-eval-core` owners.

## Reviewed source cohort

- Repository: `PrimeIntellect-ai/verifiers`
- Revision: `1a997814e85a2e4f409add960aa852737388dbd8`
- License: MIT
- Ordered cohort identity: `b3:c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f`
- Paths: `verifiers/v1/cli/eval/runner.py`, `verifiers/v1/cli/eval/resume.py`, `verifiers/v1/episode.py`, `verifiers/v1/interception/{base,pool,server}.py`, `verifiers/v1/rollout.py`, `verifiers/v1/runtimes/limiters.py`, `verifiers/v1/state.py`, and `verifiers/v1/utils/artifacts.py`.

Adopted mechanisms are bounded episode fan-out, separate requested inner concurrency, borrowed-resource lifetime checks, exact non-streaming retry coalescing, sealed late-write handling, content-keyed resume classification, complete replacement, and bounded artifact transfer.

Rejected behavior includes dynamic installation, provider or executable discovery, permissive networking, user-global limiter files, raw trace persistence, absolute artifact restoration, mutable response rewriting, and built-in Docker, Modal, Prime, cloud, Hub, registry, scheduler, or training behavior.

## Implemented evidence

- Strict Nickel host policy and generated JSON freshness: PASS.
- Public-entrypoint fake-backed coverage for all five built-in recipes: PASS.
- Nine adapter services and four auxiliary services require explicit current admission.
- Separate episode and per-episode agent permit limits reject overflow before dispatch.
- Cancellation, deadlines, service failures, run sealing, and cleanup obligations remain typed.
- Gateway validation requires admitted binds, separate capabilities, durable acceptance, exact retry evidence, and terminal-after-commit evidence.
- Verifier execution uses a distinct environment and an allowlisted collision-free no-link artifact manifest.
- Result append, complete replacement, strict reopen, and malformed or stale rerun posture remain explicit.
- Receipt leak and overclaim fixtures reject raw evaluation content and unsupported claims.
- Default root dependency isolation and reusable Wasm checks: PASS.
- Component catalog, facade inventory, composition, active-turn, durable-store, and evidence-safety rails: PASS.
- `nix build .#checks.x86_64-linux.std-evaluation-host --no-link -L`: PASS. The configured remote builder was unavailable, so Nix completed the focused build locally.

## Lifecycle receipts

- Proposal gate `74617e8bfc261123fb68dc1dcdc2ffbecc124a768e1a462f5add1b2ba12d34bb`: PASS.
- Design gate `ba31c17a3459f0b7b15d5f1114298d238213dd663a738f8acf464fdece41540d`: PASS.
- Initial tasks gate `212b1a92f2513a144df46fb6f4249b3958247b66e3de3e42c7c3ae93f3a7ccf7`: PASS with only the final dry-run review task open.
- Sync dry-run `29e7b38d0ef521a5a7d8e7f667598689ac3f366e62da0a9b701582127650904a`: PASS. It adds exactly seven `agent_harness_runtime.native_evaluation_host` requirements once.
- Initial archive dry-run `a6fc8031ea1077769d1e127167e403a2b8a046e6ca92f89f3418aef7fbcb4ad8`: correctly blocked while the final review task remained open.
- Final tasks gate `255793cd97889749ec6a8bb56bebc850358c6be44b45687ea0b217d833f4bd38`: PASS with 32 of 32 tasks complete.
- Final sync dry-run `1fe3eba5213cfb468c37a4bec6e686f9dd557e7b495170b4b569fcec512bede0`: PASS with the same seven one-time requirement additions.
- Final archive dry-run `8ee7888da0dc3b398794f48bb6a0ec69d9d7b8ac367a10775f20e177258320a3`: PASS and planned one archive move.
- Sync execute `9afc551ffd6d581875c19f8d20abbdfc1edeb6206de504b38e8e7f3607fb97a7`: PASS. The accepted specification changed to `e90e8973b0939d2e1952926ebeeb45fe444343d36cc2b7cc93aad059b63446e3`.
- Archive execute `2695a0d81eaa5d9b75a32e394714c61a233214e1bcca5ee35414cfd40a37528c`: PASS. The Nix-packaged Cairn process supplied the epoch date in its archive path; the path was normalized to the current repository date after the move.

## Non-claims

This evidence does not prove host confinement, network enforcement, provider correctness, tool safety, persistence durability, exactly-once remote effects, verifier soundness, artifact authenticity, benchmark validity, training fitness, authoritative completion, production readiness, or release eligibility.
