# Evidence: adapt-clankers-cost-model-routing

r[verify agent_harness_runtime.clankers_cost_model_routing.verification.closeout]

## Implementation evidence

- Added pure cost-aware routing policy, catalog, decision, budget, and receipt DTOs in `crates/animus-adapters/src/front/cost_model_routing.rs`.
- Exported the facade through `crates/animus-adapters/src/front/mod.rs`.
- Added Nickel-authored routing policy contract fixture at `policy/model-routing/policy.ncl`.
- Added focused positive and negative tests for invalid policy fixtures, simple/medium/complex routing, explicit user hints, soft and hard budget behavior, missing prices, unavailable models, catalog/usage bounds, and receipt leak rejection.
- Added `docs/clankers-cost-model-routing.md` documenting policy, inputs, decisions, receipts, and non-claims.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters front::audit_receipts::tests` — passed, 6 tests.
- Baseline before edits: `nix develop -c cargo test -p animus-adapters front::guidance::tests` — passed, 6 tests.
- Baseline before edits: `nix develop -c cargo test -p animus-adapters front::prompt_assembly::tests` — passed, 5 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters front::cost_model_routing::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters front::audit_receipts::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters front::guidance::tests` — passed, 6 tests.
- After implementation: `nix develop -c cargo test -p animus-adapters front::prompt_assembly::tests` — passed, 5 tests.
- After implementation: `nix develop -c cargo check -p animus-adapters --all-targets` — passed.
- After implementation: `nix develop -c nickel export policy/model-routing/policy.ncl` — passed.
- Focused clippy: `nix develop -c cargo clippy -p animus-adapters --lib --no-deps -- -D warnings` — globally blocked by pre-existing lint failures outside `front/cost_model_routing.rs`, `front/audit_receipts.rs`, `front/guidance.rs`, `front/mod.rs`, and `front/prompt_assembly.rs`; recaptured log had no touched-front-file diagnostics.
- Cairn proposal/design/tasks gates, validation, and evidence safety are run during closeout.

## Next-best validation for blocked rails

- Fix the existing `animus-adapters` clippy backlog outside touched front modules before using crate-wide clippy as closeout evidence for this change.
