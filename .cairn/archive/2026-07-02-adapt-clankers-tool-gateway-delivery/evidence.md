# Evidence: adapt-clankers-tool-gateway-delivery

## Implementation

- Added `front::tool_gateway` adapter DTOs and pure policy/outbox/receipt logic in `crates/animus-adapters/src/front/tool/gateway.rs` and split implementation files under `crates/animus-adapters/src/front/tool/gateway/`.
- Exported the gateway surface from `crates/animus-adapters/src/front/mod.rs`.
- Added positive and negative focused tests for local/session request normalization, raw artifact byte rejection, raw destination/header/cloud/webhook/platform payload policy denial, unsupported target and missing-context denial, missing adapter failure, adapter success, retryable adapter failure, non-retryable policy-denied retry, deterministic receipts, and receipt leak rejection.
- Added `docs/clankers-tool-gateway-delivery.md` and README linkage documenting request, policy, adapter, outbox, retry, receipt, and non-claim boundaries.
- Synced accepted spec IDs into `cairn/specs/agent-harness-runtime/spec.md`; the Cairn sync execution did not promote the delta text automatically, so the accepted spec was manually repaired and revalidated.

## Checks

- Baseline before implementation: `nix develop -c cargo test -p animus-adapters` — PASS, 334 tests.
- Baseline Cairn gates: `gate proposal`, `gate design`, and `gate tasks` for `adapt-clankers-tool-gateway-delivery` — PASS.
- Focused gateway tests: `nix develop -c cargo test -p animus-adapters front::tool_gateway::tests` — PASS, 7 tests.
- Focused crate tests: `nix develop -c cargo test -p animus-adapters` — PASS, 341 tests.
- Formatting: `nix develop -c cargo fmt --check` — PASS.
- Tigerstyle: `nix run .#cargo-tigerstyle -- check` — PASS after staging the new gateway files so Nix could see them.
- Cairn gates after implementation: `gate proposal`, `gate design`, and `gate tasks` — PASS.
- Cairn validation after sync and after final implementation split: `nix run .#cairn -- validate --root .` — PASS.
- Accepted spec verification: `rg agent_harness_runtime\.clankers_tool_gateway cairn/specs/agent-harness-runtime/spec.md` found the new requirement and scenario IDs.

## Blocked rails

- `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` remains globally blocked by pre-existing diagnostics outside the gateway files; `/tmp/animus-adapters-clippy.log` contained no `front/tool/gateway` diagnostics after local fixes.
- `nix run .#cairn -- tracey coverage --root .` remains globally blocked by pre-existing missing requirement markers; the local coverage artifact did not list `agent_harness_runtime.clankers_tool_gateway.*` IDs as missing.
- `scripts/cairn/check-clankers-product-surfaces --root .` remains globally blocked by pre-existing Clankers marker findings and by the repo's current Tracey marker convention for accepted `clankers_*` requirement IDs in reusable Rust source.
