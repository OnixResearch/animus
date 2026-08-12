# agent-harness-runtime Change Spec: Goose session branching controls

## Requirements

### Requirement: Session branch request DTOs

r[agent_harness_runtime.goose_session_branching.requests] Animus MUST define UI-neutral session branch request DTOs for edit-in-place, fork-from-message, duplicate-session, label updates, continuation target selection, source session identity, source ledger head, cut-point identity, copy-settings policy, and provenance.

#### Scenario: Fork request preserves source branch

r[agent_harness_runtime.goose_session_branching.requests.fork]
- GIVEN a user edits an earlier message and chooses to fork rather than overwrite the current session
- WHEN the branch request is validated
- THEN the request names the source session, source head, cut point, edited-message body class, requested branch label, and continuation target
- AND it does not require any UI widget, database row, provider session, or live process handle.

### Requirement: Pure branch planner

r[agent_harness_runtime.goose_session_branching.pure_planner] Session branch planning MUST be pure deterministic logic over in-memory ledger snapshots, active heads, cut points, branch labels, copy policies, policy admission, checkpoint evidence, and safe provenance, and MUST NOT read or write session stores, call providers, inspect credentials, open network connections, spawn processes, read files, or use clocks.

#### Scenario: Same snapshot yields same plan

r[agent_harness_runtime.goose_session_branching.pure_planner.deterministic]
- GIVEN two hosts provide the same validated ledger snapshot, request, policy state, and checkpoint evidence
- WHEN reusable branch planning runs
- THEN the selected operation, target parent, copied descriptor set, omitted descriptor set, diagnostics, and receipt metadata are equivalent
- AND host storage layout does not affect the plan.

### Requirement: Edit-in-place suffix discard

r[agent_harness_runtime.goose_session_branching.edit_in_place] Edit-in-place planning MUST treat all ledger entries after the edited cut point as an explicit suffix-discard span that requires policy admission, safe discarded-span metadata, and checkpoint or rollback evidence before a host may mutate durable session storage.

#### Scenario: Stale cut point is rejected

r[agent_harness_runtime.goose_session_branching.edit_in_place.stale]
- GIVEN an edit-in-place request references a cut point that is not reachable from the current source head or was superseded by a newer branch operation
- WHEN branch planning validates the request
- THEN planning fails with a stale-cutpoint diagnostic
- AND no suffix discard, ledger mutation, model replay, or storage update is planned.

### Requirement: Safe copy boundaries for fork and duplicate

r[agent_harness_runtime.goose_session_branching.copy_boundaries] Fork and duplicate planning MUST copy only admitted safe descriptors such as session labels, recipe references, extension labels, model profile labels, policy labels, and bounded ledger references, and MUST NOT copy credentials, live process handles, provider sessions, raw transcripts, raw tool outputs, or host database rows.

#### Scenario: Duplicate omits live provider state

r[agent_harness_runtime.goose_session_branching.copy_boundaries.omits_live_state]
- GIVEN a duplicate-session request asks to preserve settings from a source session that has live provider state and tool output artifacts
- WHEN duplicate planning runs
- THEN the plan copies only safe descriptor identities and marks live provider state and raw tool material omitted
- AND the duplicate session has no claimed process, provider, or credential continuity.

### Requirement: Safe branch receipts

r[agent_harness_runtime.goose_session_branching.receipts] Session branch operations MUST produce safe receipts with operation kind, source and target session hashes, source head hash, cut-point hash, suffix span counts, branch label status, copied descriptor classes, omitted descriptor classes, policy hash, checkpoint status, continuation target, diagnostics, and BLAKE3 receipt hash.

#### Scenario: Branch receipt rejects leaked transcript content

r[agent_harness_runtime.goose_session_branching.receipts.rejects_leaks]
- GIVEN a branch receipt candidate includes raw prompt text, raw transcript entries, provider payloads, credentials, full paths, raw tool outputs, or host database rows
- WHEN receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND no durable branch evidence echoes the raw material.

### Requirement: Session branching verification

r[agent_harness_runtime.goose_session_branching.verification] Session branching adoption MUST include positive fork, duplicate, and label tests plus negative stale-cutpoint, unsafe edit body, suffix-discard denial, duplicate branch identity, unsafe copy, no ambient storage, and receipt leak fixtures.

#### Scenario: Closeout evidence covers branch controls

r[agent_harness_runtime.goose_session_branching.verification.closeout]
- GIVEN maintainers intend to archive the Goose session branching adaptation
- WHEN closeout validation runs
- THEN evidence names branch DTO, pure planner, edit-in-place, fork/duplicate boundary, receipt, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
