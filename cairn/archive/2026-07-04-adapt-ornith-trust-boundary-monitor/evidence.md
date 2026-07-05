# Evidence: adapt-ornith-trust-boundary-monitor

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed after archiving and removing `adapt-ornith-scaffold-profiles` from active roadmap classification.
- `nix run .#cairn -- gate proposal adapt-ornith-trust-boundary-monitor --root .` passed before implementation.
- `nix run .#cairn -- gate design adapt-ornith-trust-boundary-monitor --root .` passed before implementation.
- `nix run .#cairn -- gate tasks adapt-ornith-trust-boundary-monitor --root .` passed before implementation.

## Implementation evidence

- Added pure `animus-runtime::trust_boundary` DTOs and helpers for immutable outer boundary descriptors, verifier hashes, sandbox identities, approval/evidence policy, deterministic action monitoring, judge-veto binding, anti-gaming issue codes, and redacted violation receipts.
- Kept filesystem access, sandbox enforcement, verifier execution, provider calls, approval UX, optional judge execution, and receipt storage host-owned.
- Re-exported the trust-boundary runtime surface from `animus-runtime`.
- Added focused positive and negative tests for valid/invalid boundary descriptors, missing verifier hashes, duplicate labels, empty sanctioned tools, unsafe raw path labels, mutable-boundary requests, allowed actions, withheld-path reads, verifier modification, unsanctioned tools, sandbox weakening, approval suppression, evidence-policy bypass, judge approval non-override, judge veto rejection, anti-gaming fixtures, and receipt leak rejection.
- Documented the monitor in `docs/ornith-trust-boundary-monitor.md` and linked it from `README.md`.
- Added trust-boundary public exports to `cairn/runtime-facade-inventory.ncl`; a trust-boundary-specific grep of the inventory rail output found no trust-boundary issues.
- After `cairn sync --execute`, manually merged the trust-boundary requirement delta into `cairn/specs/agent-harness-runtime/spec.md` because the sync plan executed without promoting the full delta text. A grep for `ornith_trust_boundary` confirms the accepted spec now contains the new requirement IDs.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime trust_boundary` passed: 5 focused trust-boundary tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 181 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `nix run .#cairn -- validate --root .` passed after accepted-spec merge.
- `nix run .#cairn -- gate proposal adapt-ornith-trust-boundary-monitor --root .` passed.
- `nix run .#cairn -- gate design adapt-ornith-trust-boundary-monitor --root .` passed.
- `nix run .#cairn -- gate tasks adapt-ornith-trust-boundary-monitor --root .` passed after task completion.
- `nix run .#cairn -- sync adapt-ornith-trust-boundary-monitor --root .` dry-run returned `reasons: []`.
- `nix run .#cairn -- sync adapt-ornith-trust-boundary-monitor --root . --execute` completed.
- `rg ornith_trust_boundary cairn/specs/agent-harness-runtime/spec.md` confirmed accepted spec sync after the manual merge repair.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked by pre-existing `animus-adapters` Clippy errors, including `clippy::needless_pass_by_value` in `crates/animus-adapters/src/workspace/receipt/mod.rs`.
- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` is blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a trust-boundary-specific grep found no remaining trust-boundary issues after adding inventory records.

## Requirement coverage

- `r[agent_harness_runtime.ornith_trust_boundary.outer_boundary]`
- `r[agent_harness_runtime.ornith_trust_boundary.outer_boundary.invalid]`
- `r[agent_harness_runtime.ornith_trust_boundary.monitor]`
- `r[agent_harness_runtime.ornith_trust_boundary.monitor.violations]`
- `r[agent_harness_runtime.ornith_trust_boundary.judge_veto]`
- `r[agent_harness_runtime.ornith_trust_boundary.judge_veto.no_override]`
- `r[agent_harness_runtime.ornith_trust_boundary.anti_gaming]`
- `r[agent_harness_runtime.ornith_trust_boundary.anti_gaming.fixtures]`
- `r[agent_harness_runtime.ornith_trust_boundary.receipts]`
- `r[agent_harness_runtime.ornith_trust_boundary.receipts.redacted]`
- `r[agent_harness_runtime.ornith_trust_boundary.verification]`
- `r[agent_harness_runtime.ornith_trust_boundary.verification.closeout]`
