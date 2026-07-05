# Evidence: version-normalize-boundary-dtos

r[verify agent_harness_runtime.boundary_versions.verification]

## Implementation evidence

- Added stable boundary contract identity constants for model requests/responses, tool requests/responses, model stream frames, safe receipts, and adapter-call outcomes.
- Added `BoundaryContractIdentity` and `parse_boundary_contract_identity` for supported-version admission and fail-closed unsupported identity handling.
- Audited representative boundary constructors so safe labels normalize while raw/body retention remains explicit and bounded.
- Added positive and negative tests for identity availability, whitespace normalization, NUL/overlong rejection, unsupported versions, and unsafe raw text leakage.
- Updated Preserves schema notes, runtime composition docs, and facade inventory entries.

## Checks

- `nix develop -c cargo test -p animus-adapters versioning --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters tool --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-preserves --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks version-normalize-boundary-dtos --root .` — passed.

## Blocked rails

- Combined `animus-std` clippy remains blocked by existing `crates/animus-std/src/providers/capability/flow.rs` `too_many_arguments` lint; full log captured at `target/focused-clippy-preserves-std-nodeps.log`.
