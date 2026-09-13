# Host qualification and complete-gate controls

This receipt records `c586725fd6607dd441ab71228f30a14bb4ccfe03` and the subsequent rail corrections.
It does not record archive, integration, or publication.

Done = the required gates are green and only the named repair files changed.

## Full pinned catalog

Task 6434 returned exit zero on the exact subject.
It selected both independent host checks, the combined check, and the driver controls.

| Check | Findings | Warnings | Errors |
| --- | ---: | ---: | ---: |
| `live-openai-octet` | 0 | 0 | 0 |
| `live-steel-octet` | 0 | 0 | 0 |
| `live-child-control-octet` | 0 | 0 | 0 |

The pinned Octet revision remains `f39ab155170f32ffc6408782d20c0d488137c995`.
The checks use its official `octet-deny-all` hook.
No catalog rule, target, feature, or warning budget was removed.
No new lint exemption was added.

The driver result is `/nix/store/q89p8jvvqx6hmmcjcwbn5qld49ab2ipq-animus-live-host-octet-driver`.
The authoritative log is `owner-octet-fourth.log` in the durable evidence directory.
Its three summaries record separate configuration and profile hashes.
The earlier task 6386 aggregate failure is not individual driver evidence.

## Rust and host coverage

Tasks 5996 and 6348 record the owner, restart, feature, caller, strict Clippy, and formatting controls.
Task 6327 executed the retained host roster with the two added positive cases.
Task 6408 passed the later qualified-import and blocked-state controls.
The roster comparison retains all 54 original agent cases and adds two positive cases.
`tool_negative` retains its Cargo target name after its file moves to `tests/tool.rs`.

Task 6440 repeated both host rails on `c586725f`.
Both rails passed with their maintained tests, strict Clippy, and feature checks.
Task 6441 also passed Cairn validation and the proposal, design, and tasks gates on that subject.
Those Cairn results describe structural validity, not complete acceptance.

The parent audit disposition is in `reconciliation-2026-09-13.md`.
It retains the generic completion and cleanup limits.
It does not convert advisory review into independent acceptance.

## Complete-gate baseline

Task 6440 ran the complete `x86_64-linux` flake gate with `--keep-going`.
It returned exit one.
Its final report identified these three failed attributes:

- `packaged-live-openai-smoke`
- `packaged-live-openrouter-smoke`
- `tool-execution-stages`

Both packaged checks use the same checker.
That checker still read the removed `src/smoke.rs`.
The tool-stage rail still read the removed live-agent `src/tool.rs` and `src/shell.rs`.
These failures required caller migrations. They did not justify smaller coverage.

## Rail corrections

`check-packaged-live-openai-smoke.rs` now reads `src/probe/mod.rs` for the exported smoke contract and reviewed model.
It reads `src/probe/report.rs` for the metadata-only output markers.
All existing input, model, credential, evidence, documentation, and denial checks remain.

Task 6499 passed the corrected packaged rail.
It ran the provider tests, all three binary suites, strict Clippy, release-evidence fixtures, and packaged denial controls.
The live request remained ignored. The rail reported no live requests.

`check-tool-execution-stages` now reads the actual `src/evaluation/mod.rs` and `src/assembly/stages.rs` owners.
Its required markers remain unchanged.
The runtime selector now names `toolexecutionstages`, not its public facade.

Task 6502 recorded zero tests for the old selector and seven passing owner tests for the new selector.
The owner tests include accepted handlers and zero-handler execution.
They also reject reordered phases, widened authority, output growth, commit mismatch, and unsafe receipt data.
The same task passed the Nix static subset, catalog checks, facade checks, and the whitespace check.
It did not run the entire non-static tool-stage rail.

The only subsequent rail sources are:

- `scripts/cairn/check-packaged-live-openai-smoke.rs`
- `scripts/cairn/check-tool-execution-stages`

The complete flake gate must pass again on the retained revision before completion.

## API and authority boundary

The portable count and bound changes require source migrations.
They are not source-compatible changes.
The maintained host guides name the affected APIs and their checked conversions.

`MemoryCodingSessionPersistence::empty()` keeps construction explicit without external authority.
`CodingSessionHost::resume_with_gate()` adds pre-effect admission and retains the later current-authority probe.
The coherent runtime relocations preserve their public module paths.
The CLI uses locked `clap_lex` 1.1.0 through explicit private parsers and binary-owned acquisition.
No package revision or `flake.lock` changed.

The recovery controls prove only their selected interruption and retry boundaries.
They do not prove arbitrary journal-pair equivalence, process-crash recovery, rollback, or atomicity.
The generic stage completion API remains fallible.
The fixed live-composition argument does not cover arbitrary labels or future injected handlers.

The companion head and cancellation APIs remain on their separate retained branch.
This repair does not silently import them.
No live-provider campaign, live-child completion, verifier execution, isolation, performance, or release claim follows from these checks.

## Durable records

The primary pi-steel checkout retains `.pi/animus-host-octet-20260912/`.
It contains the exact subject files, command logs, exit records, review reports, and preservation records.
This receipt uses these new records:

- `owner-octet-fourth-subject.txt`, `owner-octet-fourth.log`, `owner-octet-fourth-exit.txt`
- `owner-full-flake-subject.txt`, `owner-full-flake.log`, `owner-full-flake-exit.txt`
- `owner-artifacts-qualified-subject.txt` and `owner-artifacts-qualified-exit.txt`
- `owner-packaged-smoke-repair.log` and `owner-packaged-smoke-repair-exit.txt`
- `owner-stages-rail-repair.log` and `owner-stages-rail-repair-exit.txt`

A later complete-gate receipt must name its own immutable subject.
The pre-expansion `receipt.md` and `pueue-receipts.json` remain historical records for `44a221ff`.
