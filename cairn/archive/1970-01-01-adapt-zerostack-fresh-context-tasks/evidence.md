# Evidence: adapt-zerostack-fresh-context-tasks

## Implementation evidence

- Added pure `animus-runtime::fresh_context_tasks` DTOs and helpers for fresh-context task descriptors, budgets, read-only side-question action checks, child-run isolation, scheduler dispatch readiness, attributed results, BLAKE3 output hashes, and bounded cancellation receipts.
- Kept scheduling, provider selection, concurrency, cancellation plumbing, process execution, and tool dispatch host-owned.
- Re-exported the fresh-context task runtime surface from `animus-runtime`.
- Added focused positive and negative tests for missing scheduler fail-closed behavior, inherited-authority isolation violations, read/search side-question allowance, write/edit/shell/network/ledger/other-tool denial, malformed descriptors, malformed/unsafe results, and bounded cancellation receipts.
- Documented the contract in `docs/zerostack-fresh-context-tasks.md` and linked it from `README.md`.
- Added fresh-context public exports to `cairn/runtime-facade-inventory.ncl`; a fresh-context-specific grep of the inventory rail output found no fresh-context issues.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime fresh_context_tasks` passed: 4 focused tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 213 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-zerostack-fresh-context-tasks --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-fresh-context-tasks --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-fresh-context-tasks --root .` passed.
- Scoped Clippy remains blocked by pre-existing runtime issues, but grep of `.pi/runtime-clippy.log` showed no fresh-context findings.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` remains blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers.
- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a fresh-context-specific grep found no new fresh-context inventory issues.

## Requirement coverage

- `r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.descriptor.missing_scheduler]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.side_question.readonly]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.isolation.allowlist]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.results]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.results.malformed]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts]`
- `r[agent_harness_runtime.zerostack_fresh_context_tasks.receipts.cancelled]`
