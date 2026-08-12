# Evidence: Clankers runtime pattern adaptation

## Baseline and gates

- PASS: `nix run path:<repo>#cairn -- validate --root <repo>` before closeout; lifecycle was valid with 30 active changes.
- PASS: `nix run path:<repo>#cairn -- gate proposal adapt-clankers-runtime-patterns --root <repo>`.
- PASS: `nix run path:<repo>#cairn -- gate design adapt-clankers-runtime-patterns --root <repo>`.
- PASS: `nix run path:<repo>#cairn -- gate tasks adapt-clankers-runtime-patterns --root <repo>`.
- PASS: `scripts/cairn/check-roadmap --root .`; reported only active dependency-order blockers for other changes.

## Focused rails

- PASS: `nix develop -c sh -c 'scripts/cairn/check-steel-runtime-policy --root . && scripts/cairn/check-safe-receipts-artifacts --root . && scripts/cairn/check-neutral-ledger --root . && scripts/cairn/check-injected-services --root .'`.
- Steel runtime policy receipt: `target/cairn/focused-rails/steel-runtime-policy.md` recorded `policy_drift: skipped-explicit-source-unset` with next check `set CAIRN_POLICY_SOURCE to a canonical generated Cairn policy artifact`.
- Injected services receipt: `target/cairn/focused-rails/injected-services.md` recorded fail-closed model, tool, and effect-service fixtures.

## Workspace checks

- PASS: `nix develop -c cargo test --workspace --locked`.
- BLOCKED: `nix develop -c cargo clippy --workspace --all-targets --locked -- -D warnings` failed in existing `animus-adapters` lint debt. First captured issue class: duplicated `allow` attributes in `crates/animus-adapters/src/front/guidance/assembly.rs`, followed by existing `vec_init_then_push`, `struct_field_names`, `too_many_arguments`, `assertions_on_constants`, lifetime-elision, and pass-by-value diagnostics. Scratch log: `.pi/cairn-drain/clippy-workspace.log`.
- BLOCKED: `nix flake check path:<repo> -L` failed because `checks.x86_64-linux.cargo-clippy` hit the same `animus-adapters` lint debt. Remote builder SSH fallback warning was non-blocking. Scratch log: `.pi/cairn-drain/flake-check.log`.
- BLOCKED: `scripts/cairn/check-policy --root .` reported generated Cairn policy drift against the sibling local Cairn policy source `local-cairn:fb118a4`; next best validation is the Steel runtime policy rail's explicit source-unset mode above.

## Non-claims

This closeout does not claim live-provider behavior, daemon/TUI dogfood, production readiness, or resolution of unrelated `animus-adapters` clippy debt.
