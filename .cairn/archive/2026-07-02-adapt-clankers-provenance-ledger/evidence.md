# Evidence: adapt-clankers-provenance-ledger

## Baseline

- PASS: `cargo test -p animus-runtime` before provenance edits. Result: 80 runtime tests passed.
- PASS: Cairn gates before implementation: proposal, design, and tasks all returned PASS.
- INITIAL BLOCKER: `scripts/cairn/check-roadmap --root .` reported an unknown active change id for `adapt-clankers-process-job-profiles`. The roadmap entry referenced a non-active change, so the tracked roadmap classification was repaired by removing that stale active-change entry and its overlap reference. Re-run passed.

## Implementation checks

- PASS: `scripts/cairn/check-provenance-ledger --root .`. Result: 15 provenance tests passed and default Cargo manifests did not include forbidden Toolpath, Clankers session/message, Automerge, Pathbase, provider importer, Git/GitHub derivation, Matrix, plugin-runtime, or global session-directory dependencies.
- PASS: `cargo test -p animus-runtime`. Result: 95 runtime tests passed.
- PASS: `cargo test --workspace`. Result: workspace tests passed after adding Preserves boundary handling for model stream runtime events discovered by the workspace compile.
- PASS: `cargo check --workspace --all-targets`.
- PASS: `cargo fmt --all -- --check`.
- PASS: `nix run .#check-nickel-configs`.
- PASS: `nix build .#checks.x86_64-linux.provenance-ledger --no-link -L`.
- PASS: Cairn sync executed. Follow-up accepted-spec search initially did not find the promoted `agent_harness_runtime.provenance_ledger.*` IDs, so the requirement sections were manually merged into `cairn/specs/agent-harness-runtime/spec.md`; a second search found the promoted IDs and Cairn validation passed.
- PASS: Cairn archive executed to `cairn/archive/2026-07-02-adapt-clankers-provenance-ledger/`; post-archive validation passed with 33 active changes remaining.
- PASS: Post-archive roadmap validation passed after replacing archived change dependencies with the accepted provenance closeout requirement and moving the receipt-redaction overlap owner to an active change.

## Blocked or next-best checks

- BLOCKED: `cargo clippy --workspace --all-targets -- -D warnings` still fails before reaching all workspace crates because existing `animus-adapters` test/support code trips pedantic lints such as `needless_pass_by_value` in `crates/animus-adapters/src/tool/tests/support.rs`. Next best validation run: focused provenance rail, runtime tests, workspace tests, workspace check, and format check all passed.
- BLOCKED: `scripts/cairn/check-runtime-facade-inventory --root .` still reports pre-existing adapter inventory/public-export drift. Captured diagnostics had no `animus-runtime::Provenance*`, `animus-runtime::Toolpath*`, or provenance helper issues after registering the new exports. Next best validation run: the new provenance exports were added to `cairn/runtime-facade-inventory.ncl`, and the Nix provenance check passed.

## Safety notes

- Receipts and Toolpath projections use safe labels, bounded counts, BLAKE3 hashes, and redaction markers only.
- Raw prompts, provider payloads, credentials, full private paths, raw tool input/output, raw model output, raw unified diffs, and unbounded output are rejected or redacted by tests.
- Toolpath remains an owned redacted projection type; no default Toolpath or `toolpath-convo` dependency was introduced.
