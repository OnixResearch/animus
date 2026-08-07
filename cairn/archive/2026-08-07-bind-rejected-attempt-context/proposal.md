## Why

Animus has a bounded Ralph loop, fresh-context tasks, checkpoint rollback, safe receipts, and host-owned VCS effects. Cairn already defines revision handoff receipts for rejected review attempts. Animus does not yet bind those receipts and one VCS attempt identity into safe context for a later fresh iteration.

Riveter provides a useful reference: preserve a rejected `jj` revision, record the rejection reason, reopen the story, and show the next iteration what failed. Animus must adapt this behavior without making `jj` mandatory, copying raw diffs into prompts by default, or treating a rejection as task authority.

## What Changes

- Add transport-neutral rejected-attempt descriptors bound to task, run, attempt, VCS revision, workspace snapshot, and Cairn revision-handoff identities.
- Add pure admission and bounded summary planning for later fresh-context runs.
- Keep VCS discovery, diff reading, Cairn receipt loading, and prompt assembly in host shells.
- Require explicit redaction and byte budgets before any rejection context becomes model-visible.
- Preserve rejected attempts as evidence without granting completion, retry, mutation, or VCS authority.
- Add optional `jj` host fixtures and VCS-neutral compatibility cases.

## Impact

- **Files**: adapter DTOs, Ralph/fresh-context composition, host VCS service contracts, Nickel policy, safe receipts, fixtures, docs, and the agent runtime specification.
- **Dependencies**: Consume Cairn revision handoff receipts as external evidence. Cairn already owns rejection disposition semantics.
- **Testing**: Positive bounded handoff cases plus negative stale revision, wrong task, missing receipt, unsafe diff, oversized summary, ambiguous attempt, and completion-overclaim cases.
- **Boundary**: The descriptor records one rejected attempt and its reviewed handoff. It does not prove the rejection was correct or that the next attempt fixes it.
