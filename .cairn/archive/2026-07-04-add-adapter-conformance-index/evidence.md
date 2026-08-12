# Evidence: add-adapter-conformance-index

r[verify agent_harness_runtime.adapter_conformance.verification]

## Implementation evidence

- Added `docs/contracts-index.md` as the maintained contract-family map with owner modules, authority boundaries, safe receipt behavior, serialization posture, focused surfaces, and rails.
- Added `cairn/adapter-conformance-index.ncl` plus positive and negative fixtures under `cairn/adapter-conformance-index-fixtures/`.
- Added `scripts/cairn/check-adapter-conformance-index` and `scripts/cairn/check-adapter-conformance-index.rs` to validate family coverage, required positive/negative conformance cases, docs anchors, safe fields, and fixture failures.
- Added focused runtime prelude guidance in `docs/runtime-composition-preludes.md` and facade inventory entries for conformance-adjacent adapter outcomes, contract identities, and dispatch planning.

## Checks

- Baseline: `nix run path:$PWD#cairn -- validate --root .` — passed.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks add-adapter-conformance-index --root .` — passed.
- `nix develop -c scripts/cairn/check-adapter-conformance-index --root .` — passed.
- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix develop -c cargo fmt --check` — passed.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
