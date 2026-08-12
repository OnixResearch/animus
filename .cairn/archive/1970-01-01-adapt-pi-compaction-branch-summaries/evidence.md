# Evidence: adapt-pi-compaction-branch-summaries

## Baseline

- `nix run .#cairn -- validate --root .` passed before this change was drained.
- `scripts/cairn/check-roadmap --root .` passed and showed this change as the blocker for `adapt-zerostack-midturn-compaction`.
- `nix run .#cairn -- gate proposal adapt-pi-compaction-branch-summaries --root .` passed.
- `nix run .#cairn -- gate design adapt-pi-compaction-branch-summaries --root .` passed.
- `nix run .#cairn -- gate tasks adapt-pi-compaction-branch-summaries --root .` passed.

## Implementation evidence

- Added pure `animus-runtime::pi_compaction` DTOs and helpers for typed compaction policy, trigger classification, in-memory replay entries, safe cut-point planning, split-turn prefix metadata, branch handoff summary planning, injected summarizer readiness decisions, and bounded summary metadata validation.
- Kept token estimation, persistence, provider/model calls, concrete summarizer execution, clocks, files, UI, and branch navigation side effects host-owned.
- Re-exported the Pi compaction runtime surface from `animus-runtime`.
- Added focused positive and negative tests for enabled, disabled, under-budget, over-budget, and missing-estimate trigger cases; safe tool-call/result cut points; oversized split-turn planning; branch handoff span selection; no-summary navigation; missing summarizer fail-closed behavior; ready summarizer dispatch; valid redacted metadata; invalid hashes; and unsafe raw metadata rejection.
- Documented the contract in `docs/pi-compaction-branch-summaries.md` and linked it from `README.md`.
- Added Pi compaction public exports to `cairn/runtime-facade-inventory.ncl`; a Pi-compaction-specific grep of the inventory rail output found no Pi compaction issues.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime pi_compaction` passed: 6 focused Pi compaction tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 192 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-pi-compaction-branch-summaries --root .` passed.
- `nix run .#cairn -- gate design adapt-pi-compaction-branch-summaries --root .` passed.
- `nix run .#cairn -- gate tasks adapt-pi-compaction-branch-summaries --root .` passed.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` remains blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a Pi-compaction-specific grep found no new Pi compaction inventory issues.

## Requirement coverage

- `r[agent_harness_runtime.pi_compaction.policy]`
- `r[agent_harness_runtime.pi_compaction.policy.trigger]`
- `r[agent_harness_runtime.pi_compaction.cut_points]`
- `r[agent_harness_runtime.pi_compaction.cut_points.reject_tool_result]`
- `r[agent_harness_runtime.pi_compaction.split_turn]`
- `r[agent_harness_runtime.pi_compaction.split_turn.prefix]`
- `r[agent_harness_runtime.pi_compaction.branch_summary]`
- `r[agent_harness_runtime.pi_compaction.branch_summary.navigation]`
- `r[agent_harness_runtime.pi_compaction.injected_summarizer]`
- `r[agent_harness_runtime.pi_compaction.injected_summarizer.missing]`
- `r[agent_harness_runtime.pi_compaction.safe_metadata]`
- `r[agent_harness_runtime.pi_compaction.safe_metadata.reject_raw]`
