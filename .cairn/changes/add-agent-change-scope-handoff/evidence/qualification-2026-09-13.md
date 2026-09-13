# Repaired handoff qualification

Status: the encoding repair passes its behavioral controls. Full acceptance remains blocked by the repository Octet gate.

## Subjects and scope

The repaired product is `71d7eb63547d1042fd2f71f31f446906dbcddb98`.
The red regression is `fdbed164e810ab6454602ebfa97ac9411b1fb4d6`.
Its product source equals the starting subject `0db64b651b677f26a73b939b4d6bf62b5190f3ee`.
The earlier probe and focused receipts remain unchanged.
This receipt supersedes their pending qualification status, not their historical results.

The repair count-frames each ordered collection under `animus.change-scope-handoff.scope.v2`.
It requires fresh validation and review, with no legacy fallback.
The public Rust signatures, receipt schema, dependency pins, gate policy, and authority boundaries remain unchanged.
All source-review passes are local and correlated.
No worker ran in this round.

## Observed checks

The persistent evidence root is `/home/brittonr/git/pi-steel-repl/.pi/animus-handoff-repair-20260913/`.
Each log in this table belongs to the repaired product unless the row names the red subject.

| Evidence | Result |
| --- | --- |
| `regression-final-red.log`, `regression-final-red-exit.txt` | On the red subject, four controls passed and two failed. Exit 101. |
| `owner-gates.log`, `owner-gates-exit.txt` | Package, feature, caller, Clippy, and boundary commands finished with exit 0. |
| `format-inventory.log`, `format-inventory-exit.txt` | Formatting and facade inventory controls passed. Exit 0. |
| `artifact-gates.log`, `artifact-gates-exit.txt` | Cairn validation and proposal/design/tasks gates passed for handoff and host qualification. Exit 0. |
| `full-flake.log`, `full-flake-exit.txt` | The full command selected 142 checks and failed `checks.x86_64-linux.octet`. Exit 1. |
| `supplemental.log`, `supplemental-exit.txt` | All three strict host selections and the driver check realized successfully. Five actual receipt-owner tests passed. Exit 0. |
| `preservation.log` | Primary equality and retained-worktree checks passed. Task 747 exited 0. |

The runtime package passed 657 unit tests, six handoff integration tests, and three facade integration tests.
The three facade cases are not doctests.
The std package passed 261 unit tests and its selected integration cases.
The live-agent package passed all selected targets, including 21 library cases.
The six handoff controls also passed without default features.
Default-off builds do not alone establish that `animus-runtime` is a `no_std` crate.

The controls cover exact replay, crossed collection partitions, collection counts and order, every hunk field, and legacy refusal.
Rejected decisions preserve the pending state and cannot complete the handoff.
The positive control preserves decision, completion, and receipt agreement.

## Exact full-gate blocker

Task 653 ran:

```sh
nix flake check --offline --keep-going "path:$ROOT" -L
```

Its before and after subject records both contain the repaired product commit.
The final failed attribute is `checks.x86_64-linux.octet`.
The Octet summary reports:

```text
Status: integration-failure
Findings: 214
Warnings: 0
Errors: 214
By crate:
  animus_runtime        214
```

| Lint | Reports |
| --- | ---: |
| `non_trait_imports` | 97 |
| `path_segment_repetition` | 74 |
| `usize_in_public_api` | 16 |
| `assertion_density` | 14 |
| `bool_naming` | 4 |
| `fallible_int_fallback` | 2 |
| `function_length` | 2 |
| `numeric_units` | 2 |
| `sentinel_fallback` | 2 |
| `raw_arithmetic_overflow` | 1 |

These are reported findings across compiler targets, not 214 distinct defects or a complete inventory of later blocked targets.
The independent provider, independent agent, and combined full pinned host selections each report zero findings, warnings, and errors.
Their green results do not override the runtime failure.
The supplemental command retrieved the same successful host outputs and the unchanged driver-control output.
It did not rerun every cached builder.

The scope contract says: “The public Rust signatures and receipt schema remain unchanged.”
The preserved `scope_file_bound() -> usize` and `approved_hunks() -> &[usize]` signatures fail `usize_in_public_api`.
`data.rs` is unchanged from the starting subject.
Passing this rule without exemptions requires a public API migration and caller qualification beyond the scoped encoding repair.
This scope judgment is the parent's interpretation of the recorded approval, not a claim that the user explicitly denied a later migration.
No API was removed, hidden, aliased, or changed to bypass the gate.

The safe-receipt wrapper also retains a stale `safe_receipt::` selector that selected zero tests.
Task 745 ran the actual `safe::receipt::` owner directly and passed one positive case and four negative cases.
That direct result does not repair the maintained wrapper or turn its zero-test selection into coverage.

## Preservation and lifecycle

The primary comparison covers HEAD, the index BLAKE3, status, staged and unstaged patches, changed paths, and the tracked/nonignored source tree.
The tree identity remains `fd79492b4b55caf4e6ff8e6b3115eb1ddb580e9b`.
This equality includes tracked modes and deletions, not ignored runtime data or shared Git object storage.
The retained host repair, companion, and audit worktrees remain clean at their recorded commits.
The delegation worktree and pi-steel primary remain clean at their recorded parents.

Handoff remains 10/11 tasks. Host qualification remains 13/14 tasks.
No accepted spec, archive, primary branch, consumer pin, companion composition, or published ref changed.
No worktree was removed.
A broader handoff qualification needs approval for the public API migration and fresh required gates.
No full completion, release readiness, external execution, or independent audit is claimed.
