# Isolated integration checkpoint

Status: histories and local edits reconciled. Acceptance is blocked by a defect in the imported handoff implementation.

## Subjects and authority

- Fetched `origin/main`: `51d2683776ab73d4fd36d3d9e6ced2beadf9793d`
- Preserved primary `main`: `ce968741e2761827eb5d5046854ff991af0fb9f9`
- Preserved host repair: `9b02be4367192ad62179b5abc082ce3164c9b0b4`
- Primary source snapshot: Git tree `fd79492b4b55caf4e6ff8e6b3115eb1ddb580e9b`
- History merge: `473f8649c6c043ba4400a538c74d6d6ba480a78d`
- Composed product subject: `57e3d057a26dacb0b0dda7175de630bfa1be71d1`
- Branch: `change/main-reconcile-20260913`
- Worktree: `/home/brittonr/git/worktrees/animus-main-reconcile-20260913`

The user approved a separate-worktree reconciliation with the primary unchanged.
This worktree started at the freshly fetched `origin/main`.
The history merge retains both exact parents without a rebase or history rewrite.
No primary commit, index update, stash, push, archive, consumer-pin change, or worktree removal occurred.
No worker ran in this round.

The repair design states: “Keep unrelated runtime logic unchanged.”
This round interprets that limit as permission to compose inspected edits and resolve owner paths, not to redesign the unrelated handoff identity contract.
The failed control requires that separate core decision before acceptance.

## Reconciliation decisions

The six primary commits contain six plans and one README reference.
Four plans remain active and unchanged.
The other two plans already have completed successors on `origin/main`:

- `.cairn/archive/2026-08-24-add-bounded-tool-result-cache/`, introduced by `c267b2b2`
- `.cairn/archive/2026-08-24-agent-harness-convergence/`, introduced by `854c1ec8`

The primary already deletes both obsolete active packages.
The merge preserves those deletions and leaves the accepted specs and archives unchanged.
It retains both README reference lists.

The composed product retains these inspected local edits:

- The unfinished `change_scope_handoff` core, injected std review service, feature, inventory entries, documentation, and existing task evidence.
- The `bound-stream-host-lifecycle`, `host-wasm-component-plugins`, and `receipt-live-agent-actions` planning packages.
- The Baz README reference.

The runtime conflict adds the handoff export without replacing the repaired `toolexecutionstages` owner or its public facade.
Every imported handoff source file equals the captured primary blob.
The three uncommitted planning packages also equal their captured inputs.
Historical task markers and review receipts remain historical inputs, not new acceptance evidence.

The primary policy edits already exist in the repair policy.
Replacing that policy with the primary file also removes the newer `outcome-machine` profile.
The composition therefore retains the current generated policy without a manual edit.
No dependency version, Cargo lock, flake input, or `flake.lock` changed.

## Checks and limits

Task 475 ran baseline checks on the history-only merge:

- All 21 live-agent library cases passed.
- All 647 runtime library cases passed with all features.
- The initial `codingsession` filter selected zero std cases. It is not test evidence for that owner.

Task 482 validated the history-only Cairn tree.
Task 501 checked the composed product subject:

- All 657 runtime library cases passed with all features, including the ten imported handoff cases.
- All five imported std handoff cases passed.
- All 21 live-agent library cases passed.
- Strict Clippy passed for runtime, std, and live-agent packages with all targets and all features.
- Workspace formatting passed.

Task 546 used the corrected `coding_session::` selector and passed all 20 std owner cases.
Its runtime-facade rail passed, including seven negative fixtures and the positive fixture.
Task 502 validated all eight active Cairn packages without issues.
These results do not establish the missing scope-binding property.

The new public-API control in task 530 returned exit 101: two controls passed and the cross-field refusal failed.
The exact scope accepts its own decision.
A changed obligation rejects the prior decision.
However, a label moved between file and obligation fields does not reject that decision.
The unchanged imported implementation records it as `Approved` for the different scope.

`scope_identity` length-frames each value but does not encode the file and obligation group boundaries.
These admitted facts therefore produce the same input bytes for BLAKE3:

```text
files = ["src/alpha.rs", "src/beta.rs"]
obligations = ["gate.alpha"]

files = ["src/alpha.rs"]
obligations = ["src/beta.rs", "gate.alpha"]
```

The task and hunk facts remain equal.
This is ambiguous input encoding, not a cryptographic collision in BLAKE3.
The control demonstrates a contract-level decision-binding error, not a live filesystem or provider exploit.
The companion API branch remains separate.

The regression source is retained in the handoff change evidence and in the persistent evidence directory.
The temporary Cargo test file was moved to persistent evidence after the run.
The product source remained unchanged.
No full-flake or full pinned host-catalog result is claimed for this composition.
The failed binding control already blocks acceptance, so this round did not broaden to those gates.
The earlier complete gate remains bound to `9c91c884422bae6ef0220d41c03b9e8d366b9ee8` only.

## Preservation and next decision

Task 549 compared primary HEAD, status, staged and unstaged patches, index BLAKE3, and a complete Git source snapshot.
All comparisons passed.
The source snapshot covers tracked and nonignored files, file modes, and deletions.
It does not claim equality for ignored runtime data or shared Git object storage.

The host repair remains active at 13/14 tasks.
The imported handoff remains active at 7/8 tasks.
No task was checked to bypass the failed control.
A repair of the handoff identity encoding must retain the captured baseline and add the cross-field refusal to its maintained tests.
Fresh owner, host, strict catalog, and full-flake checks remain necessary before integration or publication.

Durable evidence: `/home/brittonr/git/pi-steel-repl/.pi/animus-main-reconcile-20260913/`.
The manifests, exact subjects, commands, logs, exit records, and preservation comparisons in that directory own this checkpoint.
