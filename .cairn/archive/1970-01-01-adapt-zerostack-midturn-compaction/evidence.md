# Evidence: adapt-zerostack-midturn-compaction

## Baseline

- `adapt-pi-compaction-branch-summaries` was completed and archived first, unblocking this change.
- `scripts/cairn/check-roadmap --root .` passed with zero blocked dependents after the prerequisite archive.
- `nix run .#cairn -- validate --root .` passed before this change was archived.
- `nix run .#cairn -- gate proposal adapt-zerostack-midturn-compaction --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-midturn-compaction --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-midturn-compaction --root .` passed.

## Implementation evidence

- Added pure `animus-runtime::zerostack_midturn` DTOs and helpers for route-aware prompt pressure calibration, cached-token interpretation, mid-turn stop/compact/resume planning, required ledger and summarizer service states, summary output validation, and bounded tool-output artifact descriptors.
- Kept provider calls, token estimation, summarizer execution, ledger persistence, files, clocks, tools, UI, and continuation side effects host-owned.
- Re-exported the Zerostack mid-turn runtime surface from `animus-runtime`.
- Added focused positive and negative tests for included, separate, absent, and malformed cached-token usage fields; enabled and disabled mid-turn policies; threshold boundary behavior; missing-pressure fail policy; missing ledger service fail-closed behavior; malformed, empty/unsafe-kind, unsafe-metadata, and invalid-hash summaries; and bounded tool-output artifact windows that do not embed full output.
- Documented the contract in `docs/zerostack-midturn-compaction.md` and linked it from `README.md`.
- Added Zerostack mid-turn public exports to `cairn/runtime-facade-inventory.ncl`; a Zerostack-specific grep of the inventory rail output found no Zerostack mid-turn issues.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime zerostack_midturn` passed: 5 focused Zerostack mid-turn tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 197 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-zerostack-midturn-compaction --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-midturn-compaction --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-midturn-compaction --root .` passed.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` remains blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a Zerostack-specific grep found no new Zerostack mid-turn inventory issues.

## Requirement coverage

- `r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.prompt_pressure.cache]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.plan]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.plan.disabled]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.ledger_entries.missing]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.summarizer]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.summarizer.malformed]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts]`
- `r[agent_harness_runtime.zerostack_midturn_compaction.tool_artifacts.bounded]`
