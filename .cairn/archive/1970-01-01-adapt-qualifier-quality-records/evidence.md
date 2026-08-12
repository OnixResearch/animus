# Evidence: adapt-qualifier-quality-records

## Baseline

- `scripts/cairn/check-roadmap --root .` passed with zero blocked dependents before this change was drained.
- `nix run .#cairn -- validate --root .` passed before archive.
- `nix run .#cairn -- gate proposal adapt-qualifier-quality-records --root .` passed.
- `nix run .#cairn -- gate design adapt-qualifier-quality-records --root .` passed.
- `nix run .#cairn -- gate tasks adapt-qualifier-quality-records --root .` passed.

## Implementation evidence

- Added pure `animus-runtime::quality_records` DTOs and helpers for Qualifier-inspired safe envelopes, deterministic BLAKE3 IDs, redaction validation, sidecar projection boundaries, unknown sidecar passthrough handles, dependency isolation reporting, lifecycle filtering, no-auto-resolution edit reports, VCS-neutral span freshness, and compact agents guidance pages.
- Kept `.qual` files, JSONL storage, filesystem discovery, clocks, Git operations, sidecar rewrites, and CLI execution host/adapter-owned.
- Re-exported the quality-record runtime surface from `animus-runtime`.
- Added focused positive and negative tests for bounded scope, dependency isolation, unknown sidecar passthrough, deterministic canonical IDs under reordered bodies, malformed envelopes, unsafe raw markers, sidecar export, references, same-subject supersession, cross-subject rejection, supersession cycles, no-auto-resolution behavior, fresh/drifted/missing/no-hash span states, invalid spans, known guidance topics, unknown-topic errors, and no network reads.
- Documented the contract in `docs/qualifier-quality-records.md` and linked it from `README.md`.
- Added quality-record public exports to `cairn/runtime-facade-inventory.ncl`; a quality-specific grep of the inventory rail output found no quality-record issues.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime quality_records` passed: 7 focused quality-record tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 204 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-qualifier-quality-records --root .` passed.
- `nix run .#cairn -- gate design adapt-qualifier-quality-records --root .` passed.
- `nix run .#cairn -- gate tasks adapt-qualifier-quality-records --root .` passed.
- Scoped Clippy remains blocked by pre-existing runtime issues, but grep of `.pi/runtime-clippy.log` showed no remaining findings in `quality_records`, `zerostack_midturn`, `pi_compaction`, or `eval_feedback` after new-module fixes.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` remains blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a quality-specific grep found no new quality-record inventory issues.

## Requirement coverage

- `r[agent_harness_runtime.qualifier_quality.scope]`
- `r[agent_harness_runtime.qualifier_quality.scope.boundary]`
- `r[agent_harness_runtime.qualifier_quality.scope.redaction]`
- `r[agent_harness_runtime.qualifier_quality.envelope]`
- `r[agent_harness_runtime.qualifier_quality.envelope.deterministic]`
- `r[agent_harness_runtime.qualifier_quality.envelope.invalid]`
- `r[agent_harness_runtime.qualifier_quality.sidecar_adapter]`
- `r[agent_harness_runtime.qualifier_quality.sidecar_adapter.export]`
- `r[agent_harness_runtime.qualifier_quality.sidecar_adapter.unknown]`
- `r[agent_harness_runtime.qualifier_quality.sidecar_adapter.isolated]`
- `r[agent_harness_runtime.qualifier_quality.lifecycle]`
- `r[agent_harness_runtime.qualifier_quality.lifecycle.references]`
- `r[agent_harness_runtime.qualifier_quality.lifecycle.supersession]`
- `r[agent_harness_runtime.qualifier_quality.lifecycle.no_auto_resolve]`
- `r[agent_harness_runtime.qualifier_quality.span_freshness]`
- `r[agent_harness_runtime.qualifier_quality.span_freshness.states]`
- `r[agent_harness_runtime.qualifier_quality.span_freshness.invalid]`
- `r[agent_harness_runtime.qualifier_quality.agents_cli]`
- `r[agent_harness_runtime.qualifier_quality.agents_cli.orientation]`
- `r[agent_harness_runtime.qualifier_quality.agents_cli.topics]`
- `r[agent_harness_runtime.qualifier_quality.verification]`
- `r[agent_harness_runtime.qualifier_quality.verification.closeout]`
