# Evidence: adapt-zerostack-context-file-layering

## Baseline

- `scripts/cairn/check-roadmap --root .` passed with zero blocked dependents before this change was drained.
- `nix run .#cairn -- validate --root .` passed before archive.
- `nix run .#cairn -- gate proposal adapt-zerostack-context-file-layering --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-context-file-layering --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-context-file-layering --root .` passed.

## Implementation evidence

- Added pure `animus-runtime::context_layering` DTOs and helpers for context file descriptors, source classes, trust/freshness/redaction states, inclusion policy, no-I/O discovery boundary proof fields, admission policy, deterministic precedence, and safe included/omitted receipts.
- Kept ancestor walking, file reads, ignore handling, path canonicalization, environment reads, clocks, and persistence host-owned.
- Re-exported the context-layering runtime surface from `animus-runtime`.
- Added focused positive and negative tests for known and unknown classes, missing source identities, no ambient discovery, disabled inclusion, untrusted context, stale context, oversized bodies, unsafe body markers, deterministic precedence conflicts, omitted-descriptor diagnostics, BLAKE3 body hashes, and safe source identity receipts.
- Documented the contract in `docs/zerostack-context-file-layering.md` and linked it from `README.md`.
- Added context-layering public exports to `cairn/runtime-facade-inventory.ncl`; a context-layering-specific grep of the inventory rail output found no context-layering issues.

## Passing checks after implementation

- `nix develop -c cargo fmt --all -- --check` passed.
- `nix develop -c cargo test -p animus-runtime context_layering` passed: 5 focused context-layering tests.
- `nix develop -c cargo test -p animus-runtime --lib` passed: 209 runtime unit tests.
- `nix develop -c cargo check -p animus-runtime --all-targets` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `scripts/cairn/check-roadmap --root .` passed.
- `scripts/cairn/check-evidence-safety --root .` passed.
- `nix run .#cairn -- validate --root .` passed.
- `nix run .#cairn -- gate proposal adapt-zerostack-context-file-layering --root .` passed.
- `nix run .#cairn -- gate design adapt-zerostack-context-file-layering --root .` passed.
- `nix run .#cairn -- gate tasks adapt-zerostack-context-file-layering --root .` passed.
- Scoped Clippy remains blocked by pre-existing runtime issues, but grep of `.pi/runtime-clippy.log` showed no context-layering findings after new-module fixes.

## Broader rail blockers

- `nix develop -c cargo clippy -p animus-runtime --lib --no-deps -- -D warnings` remains blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- Full workspace Clippy remains blocked by pre-existing adapter warnings-as-errors.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing adapter inventory drift, but a context-layering-specific grep found no new context-layering inventory issues.

## Requirement coverage

- `r[agent_harness_runtime.zerostack_context_layering.classes]`
- `r[agent_harness_runtime.zerostack_context_layering.classes.unknown]`
- `r[agent_harness_runtime.zerostack_context_layering.discovery]`
- `r[agent_harness_runtime.zerostack_context_layering.discovery.no_io]`
- `r[agent_harness_runtime.zerostack_context_layering.admission]`
- `r[agent_harness_runtime.zerostack_context_layering.admission.secret]`
- `r[agent_harness_runtime.zerostack_context_layering.precedence]`
- `r[agent_harness_runtime.zerostack_context_layering.precedence.conflict]`
- `r[agent_harness_runtime.zerostack_context_layering.receipts]`
- `r[agent_harness_runtime.zerostack_context_layering.receipts.omitted]`
