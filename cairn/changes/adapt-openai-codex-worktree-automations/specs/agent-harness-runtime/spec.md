# agent-harness-runtime Change Spec: OpenAI Codex worktree automation adaptation

## Requirements

### Requirement: VCS-neutral isolated task workspaces

r[agent_harness_runtime.openai_codex_worktree_automations.isolated_workspace] Animus isolated task workspaces MUST be represented through VCS-neutral source identity, workspace identity, writable roots, snapshot identity, restore metadata, and adapter capability records.

#### Scenario: Workspace adapter is not Git-specific

r[agent_harness_runtime.openai_codex_worktree_automations.isolated_workspace.vcs_neutral]
- GIVEN a workspace adapter reports Jujutsu-oriented capabilities and no Git worktree capability
- WHEN Animus validates an isolated workspace run
- THEN validation uses the reported capabilities rather than requiring a Git worktree
- AND the run is rejected only if a required capability is missing.

### Requirement: Typed automation run contract

r[agent_harness_runtime.openai_codex_worktree_automations.automation_contract] Animus automations MUST define scheduled standalone runs and thread-bound wakeups with prompt, cadence, workspace policy, sandbox profile, and stop criteria.

#### Scenario: Unsafe automation configuration is rejected

r[agent_harness_runtime.openai_codex_worktree_automations.automation_contract.validation]
- GIVEN an unattended automation requests full-access sandboxing without an allowed policy profile
- WHEN Animus validates the automation definition
- THEN validation rejects the automation
- AND no scheduled run is registered.

### Requirement: Snapshot-based thread handoff

r[agent_harness_runtime.openai_codex_worktree_automations.handoff_snapshot] Moving a thread between foreground and isolated workspace contexts MUST create a snapshot record that identifies source state, target state, included files, and conflict status.

#### Scenario: Dirty-state conflict blocks handoff

r[agent_harness_runtime.openai_codex_worktree_automations.handoff_snapshot.conflict]
- GIVEN the foreground context and isolated workspace both changed the same tracked resource
- WHEN a handoff request attempts to merge the isolated context into the foreground
- THEN the handoff is blocked with a conflict outcome
- AND the snapshot record preserves enough evidence for manual review.

### Requirement: Explicit ignored-file inclusion

r[agent_harness_runtime.openai_codex_worktree_automations.ignored_file_include] Ignored or local-only files MUST be materialized into isolated workspaces only through explicit allowlist entries with provenance and redaction classification.

#### Scenario: Ignored secret is not copied implicitly

r[agent_harness_runtime.openai_codex_worktree_automations.ignored_file_include.secrets]
- GIVEN a local ignored file is classified as secret-bearing
- WHEN an isolated workspace is created without an allowlist entry and redaction approval for that file
- THEN the file is not copied
- AND the workspace manifest records a copy-denied entry without exposing the secret body.

### Requirement: Automation cleanup and restore evidence

r[agent_harness_runtime.openai_codex_worktree_automations.cleanup_reporting] Automation and isolated-workspace cleanup MUST record findings, no-op outcomes, failed runs, workspace cleanup, and restore-available states with bounded manifest evidence.

#### Scenario: Cleanup preserves restore manifest

r[agent_harness_runtime.openai_codex_worktree_automations.cleanup_reporting.restore]
- GIVEN an isolated workspace is eligible for cleanup
- WHEN cleanup removes the workspace materialization
- THEN Animus records manifest hashes, source identity, and restore metadata first
- AND later inspection can report that restore evidence is available.
