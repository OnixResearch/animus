# Evidence: add-composition-conformance-rail

schema: animus.change-evidence.v1
status: archived

## Baseline

- `nix develop -c cargo test -p animus-runtime --locked`: pass before core edits; 225 runtime tests passed.
- `nix run .#cairn -- gate proposal add-composition-conformance-rail --root .`: pass.
- `nix run .#cairn -- gate design add-composition-conformance-rail --root .`: pass.
- `nix run .#cairn -- gate tasks add-composition-conformance-rail --root .`: pass.
- `scripts/cairn/check-roadmap --root .`: pre-existing roadmap drift; no blocked dependents, active changes were unclassified and one stale roadmap change was no longer active.

## Implementation checks

- `nix develop -c cargo fmt --all -- --check`: pass.
- `nix develop -c cargo test -p animus-runtime --locked composition_conformance`: pass; 6 conformance tests passed.
- `nix develop -c scripts/cairn/check-composition-conformance --root .`: pass; generated bounded focused-rail receipt under `target/cairn/focused-rails/composition-conformance.md`.
- `nix build .#checks.x86_64-linux.composition-conformance --no-link -L`: pass.
- `nix develop -c cargo test -p animus-runtime --locked`: pass; 231 runtime tests passed.
- `nix develop -c cargo clippy -p animus-runtime --all-targets --locked -- -D warnings`: pass.
- `nix develop -c cargo check --workspace --all-targets --locked`: pass.
- `nix develop -c cargo test --workspace --locked`: pass.

## Lifecycle closeout

- `nix run .#cairn -- sync add-composition-conformance-rail --root .`: dry-run unblocked.
- `nix run .#cairn -- sync add-composition-conformance-rail --root . --execute`: pass; manual accepted-spec merge was required because the dry-run plan did not land the delta text.
- Accepted spec verification: `agent_harness_runtime.composability.*` requirement IDs found in `cairn/specs/agent-harness-runtime/spec.md`.
- `nix run .#cairn -- validate --root .`: pass after sync and after archive.
- `nix run .#cairn -- archive add-composition-conformance-rail --root .`: dry-run unblocked.
- `CAIRN_ARCHIVE_DATE=2026-07-04 nix run .#cairn -- archive add-composition-conformance-rail --root . --execute`: pass.
- `scripts/cairn/check-evidence-safety --root .`: pass after archive.

## Safe evidence summary

The conformance rail covers 5 matrix scenarios, 5 positive evidence records, and 9 negative fail-closed records. Focused receipt identity: BLAKE3:74db0f1d30c77ca7523ac4124921ab29708cf20802a501c1683600a4d34c57dc.

Evidence includes scenario names, classes, counts, labels, and BLAKE3 hashes only. It omits raw prompts, provider payloads, credentials, private paths, and unbounded tool output.

## Non-claims

This change does not claim live providers, real tool processes, network access, terminal UI, process execution, persistence backends, VM checks, production readiness, or performance results.
