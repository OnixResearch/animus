# Evidence: native evaluation core

## Baseline

Before implementation, the current Harbor, batch resume, evaluation-feedback, dataset-export, semantic-role, safe-artifact receipt, neutral-ledger, no-default, and Wasm rails passed. The Harbor rail reported its pre-existing stale source-path grep warning but passed its Rust, adapter, std-shell, Clippy, and dependency checks. No baseline regression was admitted.

## Ownership inventory

- `animus-runtime::harbor_eval` previously owned Harbor-shaped trial matrix, ATIF, reward, and receipt behavior. It now retains Harbor and ATIF compatibility while generic matrix and reward reducers delegate to `animus-eval-core`.
- `animus-runtime::batch` retains execution ordering and batch resume manifests.
- The session ledger retains transcript, branch, leaf, replay, and trace authority. Native episodes contain ledger references only.
- `animus-runtime::agent_roles` retains semantic role capability and separation policy.
- Dataset export retains training-data admission. Evaluation feedback retains reviewed feedback activation. Safe receipts and Cairn retain claim admission.
- Future adapter and std-host changes retain task loading, model/tool/verifier execution, artifacts, model-call capture, cancellation, deadlines, and result storage.

## Reviewed reference cohort

The bounded design reference is `PrimeIntellect-ai/verifiers` commit `1a997814e85a2e4f409add960aa852737388dbd8`, MIT. The domain-separated source-cohort identity is `b3:069f20983a0d523d84028b76f85b3dd63a712f952e99cdef628824ef1ffe6bfe`.

Reviewed v1 owners:

- `verifiers/v1/task.py`
- `verifiers/v1/taskset.py`
- `verifiers/v1/env.py`
- `verifiers/v1/episode.py`
- `verifiers/v1/trace.py`
- `verifiers/v1/cli/eval/resume.py`
- `verifiers/v1/cli/resume.py`
- `verifiers/v1/envs/single_agent/env.py`
- `verifiers/v1/envs/best_of_n/env.py`
- `verifiers/v1/envs/user_sim/env.py`
- `verifiers/v1/envs/agentic_judge/env.py`
- `verifiers/v1/envs/shared_agentic_judge/__init__.py`

Adopted patterns are immutable task data, explicit tasksets, bounded lazy generation, environment and episode structure, built-in recipe separation, ledger-like trace references, and content-addressed resume. Rejected behavior is Python execution, decorators, dynamic discovery, package installation, provider or server traffic, containers, training services, raw trace retention, and mutable shared state.

## Implementation evidence

- `animus-eval-core` is `#![no_std]` plus `alloc` and depends only on no-default BLAKE3.
- Typed Nickel owns deny-by-default policy and named bounds. Generated JSON is strict-parsed with unknown-field rejection, then revalidated by the no-std core.
- Exact and lazy tasksets bind data, behavior, scorer, policy, generator, version, seed, order, finiteness, and selection facts.
- Environment and recipe planners produce deterministic seats and actions without dispatch. Pass-at-N reduction checks exact planned order.
- Episodes bind terminal observations to exact canonical ledger references and expose no transcript-body field.
- Rewards are finite, in-domain, ordered, attributed, and loss-visible. Judge values remain advisory.
- Resume keys bind every material identity. Only one exact complete terminal safe within-budget record can produce `Keep`.
- Receipts accept bounded metadata and required non-claims only. Leak and overclaim drafts fail before identity projection.

## Verification evidence

- Focused static checker positive and negative self-tests: PASS.
- `cargo test -p animus-eval-core --locked`: PASS with positive and negative coverage.
- Strict core Clippy: PASS.
- Core, root facade, and runtime native-evaluation Wasm checks: PASS.
- Strict generated-profile parser positive, unknown-field, and source-drift tests: PASS.
- Harbor matrix and reward compatibility tests through the native core: PASS.
- Default root and default-free runtime dependency isolation: PASS.
- Component catalog and runtime facade inventory: PASS after adding the evaluation feature and family.

## Lifecycle and build receipts

- Proposal gate receipt `f35eec95a7def44a9693a68a5662a75a82ffed662cb6159d8aa921891f03d99a`: PASS.
- Design gate receipt `899523b9648f0923d294727faf5ad489c76577b17274d79919219849bbe7552c`: PASS.
- Tasks gate receipt `b597da2b35af4a27679a83bf8e5f57fbffc9b15da60ef1862778a4d79813bfb5`: PASS with all 27 tasks complete.
- Sync dry-run receipt `29178279edb5d993853bb7cdda8b7303bc75d63c6e697772405131c50471388a`: PASS with nine added requirements and expected accepted-spec identity `97bddae750df3830487987049f157d585021460cfebc039f1bcc9525279a8d29`.
- Sync execute receipt `2fc0bff0c04dd5d732435614afeb5d26fd24064a134175602aa7950da6798797`: PASS. The accepted specification changed from `c393adc70177e727343f5b14bea27aefb0898fb8ad87cb4de719838f500c16eb` to `97bddae750df3830487987049f157d585021460cfebc039f1bcc9525279a8d29`.
- Archive dry-run receipt `d422208a3339ac15b833c54afabba95fdf9c17ab5c239f9c582291b294b2c887`: PASS and unblocked.
- Archive execute receipt `43fd4a138c4174ce11ebe0c6e7e652f9a5c91ea14ecf5fad138ac850b20ca19c`: PASS. The active package moved to `cairn/archive/2026-08-08-add-native-evaluation-core/`.
- `checks.x86_64-linux.native-evaluation-core`: PASS after local fallback from the unavailable SSH builder.
- Post-archive focused rail, evidence-safety rail, Cairn validation, and `git diff --check`: PASS.
- Post-archive roadmap validation reports only the pre-existing unclassified active changes `add-optional-basalt-authority-adapter` and `add-pijul-vcs-adapter`; it reports no native-evaluation-core issue.

## Non-claims

This evidence does not prove task correctness, trace completeness, scorer validity, verifier soundness, model quality, benchmark comparability, training fitness, host execution, sandboxing, authoritative completion, production readiness, or release eligibility.
