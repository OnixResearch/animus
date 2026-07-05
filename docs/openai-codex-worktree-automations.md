# OpenAI Codex worktree automation adaptation

Animus adapts Codex-style background worktree automations as VCS-neutral workspace contracts. The runtime does not assume Git worktrees, PR creation, or unattended full-access execution. Host adapters own concrete Jujutsu, Git, BranchFS, tempdir, or hosted filesystem behavior.

## Contract boundary

- `SpeculativeWorkspaceDescriptor` names the source identity, workspace identity, writable roots, operation policy, cleanup policy, and required adapter capabilities.
- `IsolatedTaskWorkspace` adds snapshot identity, restore metadata, and the exact adapter capability record used to validate the workspace.
- `WorkspaceAutomationRunContract` describes scheduled standalone runs and thread-bound wakeups with prompt, cadence, workspace target, sandbox profile, stop criteria, and an explicit policy profile when full access is requested.
- `WorkspaceSnapshotHandoffRecord` records thread movement between foreground and isolated contexts with source, target, snapshot identity, included resource labels, conflict status, and restore manifest hash.
- `WorkspaceIgnoredFileAllowlistEntry` and `plan_workspace_ignored_file_materialization` decide whether ignored/local-only file labels may be copied. The pure decision never copies data and denies secrets without explicit redaction approval.
- `WorkspaceAutomationEvent` records safe findings, no-op outcomes, failed runs, workspace cleanup, and restore-available states with manifest hashes and restore snapshot evidence.

## Safety rules

- Workspace validation uses advertised capabilities; Jujutsu-oriented adapters do not need to claim Git worktree support unless a specific operation requires it.
- Dirty-state handoff conflicts block automatic movement and preserve restore evidence for manual review.
- Ignored or local-only files are materialized only through explicit allowlist entries with provenance and redaction classification.
- Cleanup and restore events must preserve manifest hashes and restore snapshot identity before a host deletes workspace materialization.
- Full-access automations require an explicit admitted policy profile.

## Non-goals

This adaptation does not implement a scheduler, create or manage pull requests, copy ignored files itself, expose raw file bodies in receipts, or grant model/provider authority to bypass host workspace policy.
