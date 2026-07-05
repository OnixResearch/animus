# Evidence: plan-model-provider-dispatch

r[verify agent_harness_runtime.model_dispatch.verification]

## Implementation evidence

- Added pure model dispatch planning in `crates/animus-runtime/src/admission.rs` with selected capability identity, request/provider contract identities, prompt budget, stream policy, receipt policy, cancellation/timeout posture, and deny-by-default fallback policy.
- Runtime shell dispatch now builds an admitted model plan before calling the model adapter and maps denied plans to typed fail-closed adapter outcomes.
- Added positive and negative planning tests for compatibility defaults, scoped capabilities, missing provider, denied capability, budget overflow, unsupported stream policy, malformed shape, missing receipt policy, cancellation, timeout, and fallback denial.
- Updated runtime composition docs, provider-contract notes, and facade inventory entries for model dispatch planning.

## Checks

- `nix develop -c cargo test -p animus-runtime admission --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-runtime shell --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
- `nix develop -c cargo check -p animus-runtime --no-default-features --locked` — passed.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks plan-model-provider-dispatch --root .` — passed.

## Blocked rails

- Crate-wide clippy remains blocked by existing runtime/std lint debt outside this focused evidence; next best validation is the passing focused runtime tests and no-default runtime check.
