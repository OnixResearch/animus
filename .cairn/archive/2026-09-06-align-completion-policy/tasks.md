## Implementation

- [x] [serial] Align reviewed current policy bindings and regenerate the checked registry. r[agent_harness_runtime.completion_policy.coherence]
- [x] [serial] Compare pinned core and current CLI reports with old-policy and stale-subject rejection. r[agent_harness_runtime.completion_policy.parity]

## Evidence

`docs/completion-policy-evidence.md` records source `92914e2`, the discriminating
pre-repair failure, five new controls, unchanged host tests, live completion,
Clippy, lifecycle tests, clean full Octet, and sandboxed Nix completion checks.
`evidence/live-receipt.md` and `evidence/real-check-receipt.md` retain scoped results.
The execution records for reviewed sync/archive plans accompany the completed archive.

## Verification

- [x] [serial] Record positive and negative evidence and run maintained gates before sync/archive plan review. r[agent_harness_runtime.completion_policy.coherence] r[agent_harness_runtime.completion_policy.parity]
