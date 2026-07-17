# Controller continuation policy

Animus keeps post-turn continuation selection in a pure runtime core. The core decides whether to replay queued user input, continue loop work, run validation or auto-test follow-up work, cancel, or finish; host shells perform provider calls, timers, queue waits, persistence, UI events, and validation command execution.

## State contract

Continuation state stores typed work IDs for queued prompts, active loop continuations, validation follow-ups, busy prompt work, cancellation, pending dispatch, completion candidates, required authoritative evidence, and completion acknowledgements. Work IDs are opaque safe labels; receipts and completion records expose BLAKE3 hashes instead of raw IDs. A completion candidate is an observation, not evidence that selected checks passed.

## Priority contract

The default priority is:

1. queued user prompt replay;
2. active loop continuation;
3. validation or auto-test follow-up;
4. completion only when no work remains and required authoritative evidence is present.

If no dispatchable work remains but authoritative evidence is required and absent, the controller returns `WaitForAuthoritativeEvidence` rather than completing. Candidate text, process exit, empty queues, task deltas, and archive appearance do not satisfy this state.

Callers may provide an explicit `ContinuationPriority` to reorder the three dispatchable work classes. Duplicate priority entries are rejected.

## Acknowledgement contract

Shells must acknowledge dispatch and completion with the exact pending work ID. Missing, stale, duplicate, or mismatched acknowledgements fail without clearing unrelated continuation state. Accepted dispatch removes only the selected candidate and marks it busy; rejected dispatch clears only the pending-dispatch marker and leaves the candidate pending.

## Receipt contract

Continuation receipts include prior state class, selected action, pending work ID hash, reason, optional rejection class, policy hash, and receipt hash. Receipt validation rejects raw prompt text, model output, tool output, provider payloads, full transcripts, credentials, full paths, and unbounded logs.

r[verify agent_harness_runtime.clankers_continuation_policy.verification.closeout]
