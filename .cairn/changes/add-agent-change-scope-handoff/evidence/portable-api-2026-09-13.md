# Portable handoff API qualification

## Result and exact subjects

The declared product gates pass on `147bbe03f429ec1810e7b4bb11698f9832e9ecec`.
All 142 `x86_64-linux` flake checks pass. Repository Octet reports zero findings, warnings, and errors.
The three strict host selections also report zero findings, warnings, and errors.
The primary comparison passes. This is local qualification, not publication or lifecycle completion.

The retained sequence is:

- Start: `433e4187a92e93f0143c5f5d8825cceb377e5427`.
- Pre-migration vector controls: `1cd3468b`.
- Portable API and private owners: `1f60bd20c9e34ce3f4bf32406dcbcb66a5e60a4a`.
- Explicit replay assertions and byte-unit fixture name: `147bbe03f429ec1810e7b4bb11698f9832e9ecec`.

Only two test files differ between the last two subjects. Production code is equal.
The final flake run records the same commit before and after the run.
Its source stayed frozen during evaluation and verification.
A later evidence-only commit does not establish a new full-flake execution.

## Authority and compatibility

The user approved the public integer migration after the encoding-only closeout.
Public bound accessors and approved hunk indexes now use `u32`.
The host review decision owns `Vec<u32>`. Collection and index conversions are checked.
This migration breaks Rust source compatibility. It does not change admitted v2 digest values.
Public facade names remain unchanged. Private owners separate facts, judgments, artifacts, framing, and fixtures.

The implementation reuses the existing pure handoff family and the pinned BLAKE3 dependency.
It adds no dependency, external port, clock, filesystem observation, or product authority.
Assertions cover internal invariants after admission. Public malformed inputs still return typed refusals.
The scope label limit still counts bytes. Receipt label validation still counts trimmed characters.
The legacy accessor name does not change that distinction.

## Digest and refusal evidence

Two controls ran on unchanged production code before the migration.
The second run locked these observations as assertions:

| Value | BLAKE3 digest |
| --- | --- |
| Scope | `0b9142a782580ca4e8d4e1bd3fa5e2760984b410752fc6f3f7d4a287cc07f88f` |
| Pending receipt | `f1e9385c6fc31860aeac18032e5b145029cff9d834aa22edab14b7672d72b2b8` |
| Decision | `f26a1865fbcdd0767beaa89204b9f1398fc212c2ca2dfc84b40fac9e977c2407` |
| Completed receipt | `f803a5ad391291f0ff145370abf5bac4615caf04c9686acc7bcf1be8719f87c2` |

The migrated controls retain all four values.
They cover the maximum admitted hunk scope and checked projection of its rejected remainder.
They reject duplicate, descending, one-past, and `u32::MAX` indexes without consuming the pending state.
Host controls retain the original out-of-range case and add `u32::MAX` refusal.
Collection controls accept each declared bound and reject one more item.
Artifact controls reject excessive counts and `u64::MAX` declared bytes.
These tests do not allocate collections larger than `u32::MAX`.

The ten original runtime cases remain under admission, ownership, and projection owners.
They cover scope admission, completion, continuation, missing decisions, crossed decisions, artifact rejection, non-claims, and receipt tampering.
The six public binding cases retain cross-field, replay, order, hunk-field, collection, and legacy-identity controls.
The final correction adds explicit positive replay assertions. It removes no negative case.
All source review is local and correlated. It is not an independent audit or formal proof.

## Verification ledger

Detailed commands, patches, complete logs, exits, and subject files remain in:

`/home/brittonr/git/pi-steel-repl/.pi/animus-handoff-qualification-20260913/`

| Record | Subject and observed result |
| --- | --- |
| Task 811, `baseline.log` | Starting subject: runtime handoff 10, public binding 6, host handoff 5 passed. |
| Tasks 846 and 849, `vector-before.log`, `vector-locked.log` | Two pre-migration controls passed. The second run asserted all four observed digests. |
| Task 869, `focused.log` | First candidate: runtime 10, binding 6, portable 4, boundary 3, host 5 passed. |
| Task 909, `qualified-rewrite.log` | Two helper tests passed. Formatting then refused the missing `ownership.rs` module. |
| Task 914, `qualified-controls.log` | Owner renames, focused controls, and strict runtime/std Clippy passed. |
| Task 917, `corrected-controls.log` | Final production changes: focused controls and strict runtime/std Clippy passed. |
| Task 954, `frozen-campaign.log` | Exact `1f60bd20`: package, feature, caller, Wasm, Clippy, receipt, boundary, and Cairn campaign passed. |
| Task 953, `full-flake.log` | Exact `1f60bd20`: 142 checks selected. Repository Octet rejected three test diagnostics. |
| Task 1024, `final-controls.log` | Exact final test correction: binding 6, boundary 3, portable 4 passed with all features and default features off. Strict runtime Clippy passed. |
| Task 1024, `final-flake.log` | Exact `147bbe03`: all 142 checks passed, exit 0. |

The package campaign passed 657 runtime unit tests and 261 std unit tests.
It also passed the selected integration targets and live-agent caller suite, including 21 library cases.
Runtime facade integration selected three cases with all features and two with default features off.
Feature repetitions are not distinct tests. Zero-test selections establish no behavioral coverage.

The corrected safe-receipt wrapper ran five `artifact::` cases and five `safe::receipt::` cases.
The receipt selection includes one positive case and four rejection cases.
The facade inventory passed its current-source check, seven negative fixtures, and one positive fixture.
The child-service, tool-stage, provider-host, and live-agent shell rails passed.
Cairn validation and proposal/design/tasks gates passed for both handoff and host qualification packages.
A structural Cairn gate does not prove product behavior or authorize archive.

The campaign ran inside `nix develop`, with locked, offline Cargo inputs.
The root check was `cargo check --locked --offline -p animus --no-default-features --target wasm32-unknown-unknown`.
This root check and runtime default-off tests are separate claims. The runtime crate does not declare `no_std`.

The complete final gate was:

```sh
nix flake check --offline --keep-going path:/home/brittonr/git/worktrees/animus-main-reconcile-20260913 -L
```

The gate retained repository package coverage, all three full pinned host selections, and the host driver controls.
The change removes no lint, package, target, or feature from the gates.
Dependency pins, warning policy, and generated policy remain unchanged.

## Wider diagnostics and process limits

Separate direct Octet selections are not green:

| Saved log | Selection | Findings/errors | Warnings |
| --- | --- | --- | --- |
| `octet-first.log` | Initial runtime/std, all targets and features | 1016 | 0 |
| `octet-handoff.log` | Corrected runtime/std, all targets, std handoff feature | 62 | 0 |
| `octet-expanded.log` | Corrected runtime/std, all targets and features | 979 | 0 |
| `octet-std.log` | Std alone, all targets and features | 2250 | 0 |

Earlier crate errors can prevent later targets from running.
Absence of a handoff diagnostic is not a clean direct gate.
These selections expose untouched runtime or std code beyond this change's authority.
Their counts are compiler reports, not distinct defects. Different selections are not comparable regression totals.
The final green repository selection does not establish green all-feature lint coverage for every runtime or std family.
Any repair of that wider code needs a separate scope decision.

The contract allowed 200 tool calls, one implementation round, and up to three correction iterations.
The compacted context supplied no trustworthy call counter.
A search of the pi-steel session directory found no record for this round.
Budget compliance remains unverified.
No worker round occurred. The rewrite helper is a bounded text tool, not a general Rust parser.
Compilation, tests, and the final gate validate the resulting source within their stated selections.

## Preservation and non-claims

`preservation-detail.log` records equal primary HEAD, patches, index BLAKE3, changed paths, and private-index source tree.
The first status comparison failed because it used condensed untracked-directory listings.
The repeat with `--untracked-files=all` matches the original status exactly.
That correction changed no primary file or index.
The comparison includes tracked changes, nonignored untracked files, modes, and deletions.
It excludes ignored runtime data and shared Git object storage.

The retained repair, companion, and audit worktrees keep their recorded commits and clean status.
The pi-steel primary and delegation observations also match the retained summaries.
No primary integration, push, pin change, accepted-spec sync, archive, companion composition, or teardown occurred.
Host qualification remains a separate lifecycle package.

All fixed handoff non-claims remain unchanged.
Receipts describe bounded metadata, not file observations, correctness, VCS completion, policy enforcement, production readiness, or release eligibility.
No live provider contact, live-child qualification, performance gain, or `aarch64-linux` qualification is claimed.
