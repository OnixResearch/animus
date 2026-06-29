# Evidence: adapt Clankers focused verification rails

## Baseline before edits

- `nix run .#cairn -- validate --root .`: PASS; lifecycle was valid with 74 active changes.
- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-focused-verification-rails --root .`: PASS.
- `scripts/cairn/check-roadmap --root .`: PASS; dependency-order blockers remained for downstream active changes.
- Existing focused rails and tests passed before edits: request lifecycle, safe receipts/artifacts, neutral ledger, `animus-steel`, selected Steel runtime positive/negative filters, selected injected-service runtime filters, and adapter effect outcome tests.
- `scripts/cairn/check-policy --root .`: pre-existing local drift against the moving sibling Cairn checkout (`local-cairn:fb118a4`). The new Steel runtime policy rail therefore requires an explicit `CAIRN_POLICY_SOURCE` for drift checking and records a skipped mode with next-best validation when unset.

## Implementation evidence

- Added `docs/focused-verification-rails.md` documenting the one-boundary rail convention, safe evidence schema, target receipt path, current rail index, and closeout non-claim handling.
- Added `scripts/cairn/check-steel-runtime-policy` for current Steel activation, invalid Steel rejection before adapter dispatch, and explicit policy-drift source handling.
- Added `scripts/cairn/check-injected-services` for missing-service, adapter-failure, denied-outcome, and non-completed effect outcome fixtures.
- Added named Nix checks `steel-runtime-policy` and `injected-services`, and linked the rails from README and the safety contract.

## Validation evidence

- `nix develop -c scripts/cairn/check-steel-runtime-policy --root .`: PASS; wrote `target/cairn/focused-rails/steel-runtime-policy.md` with fixture names and safe metadata only.
- `nix develop -c scripts/cairn/check-injected-services --root .`: PASS; wrote `target/cairn/focused-rails/injected-services.md` with fixture names and safe metadata only.
- `nix build .#checks.x86_64-linux.steel-runtime-policy --no-link -L`: PASS.
- `nix build .#checks.x86_64-linux.injected-services --no-link -L`: PASS.
- `nix develop -c scripts/cairn/check-evidence-safety --root .`: PASS.
- `nix run .#cairn -- validate --root .`: PASS after implementation.
- `nix run .#cairn -- gate proposal/design/tasks adapt-clankers-focused-verification-rails --root .`: PASS after implementation; tasks gate receipt hash `90701d100799b432d85631db1c76faea2d77eddf27af9bc5017d1481d5f0ca19`.
- `nix run .#cairn -- sync adapt-clankers-focused-verification-rails --root .`: dry-run PASS; plan hash `c24b18d264efdce5306485a3cb2419be99d30ead3d2f1574e88212c907bdb317`, receipt hash `0e082f2ae2a86f9932392be738a72fc40a1d4d6fe4bb93f0fcbd0fff7ee578e9`.
- `nix run .#cairn -- sync adapt-clankers-focused-verification-rails --root . --execute`: PASS; receipt hash `e10228022f1b69e69270b485ba5f5b430414e311ec9841f2c5d4c97a1c6429a4`.
- Sync verification: the accepted spec initially lacked the new `clankers_focused_rails` IDs after sync, so the delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` and revalidated.
- `nix run .#cairn -- archive adapt-clankers-focused-verification-rails --root .`: dry-run PASS; plan hash `ba678a9d4aeb4b4be362647a5889c1e553926ebc8a22318a3b3bbdc213e6cca4`, receipt hash `79511cc27757d7c02347886892df5986540935146e0450ff2e02d481eeb78e83`.
- `CAIRN_ARCHIVE_DATE=$(date +%F) nix run .#cairn -- archive adapt-clankers-focused-verification-rails --root . --execute`: PASS; receipt hash `57e4336febc2fefaaa48e127a902d13c7143c1073823e3afa3cb2301361ad0cc`.
- `nix run .#cairn -- validate --root .`: PASS after archive; 73 active changes and 74 specs validated.
- Post-archive roadmap cleanup removed the archived change from active classification, replaced product-surface prerequisites with the accepted `agent_harness_runtime.clankers_focused_rails.closeout` requirement, and updated the roadmap rail to traverse accepted safety requirement IDs.
- `nix develop -c rustfmt --edition 2021 scripts/cairn/check-roadmap.rs`: PASS.
- `nix develop -c scripts/cairn/check-roadmap --root .`: PASS after cleanup; remaining blocked entries are dependency-order reports.
- `nix develop -c scripts/cairn/check-evidence-safety --root .`: PASS after final evidence update.
- `nix run .#cairn -- validate --root .`: PASS after roadmap cleanup.

## Non-claims

This change does not implement the full Nickel-governed Steel runtime policy, live-provider dogfood, TUI or daemon dogfood, VM checks, remote services, plugin runtime, performance claims, or production readiness. The full Steel runtime policy remains owned by `adapt-clankers-steel-runtime-policy`.
