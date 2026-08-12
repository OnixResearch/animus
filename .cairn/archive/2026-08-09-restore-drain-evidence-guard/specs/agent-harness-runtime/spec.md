## ADDED Requirements

### Requirement: Drain teardown preserves ignored evidence

r[agent_harness_runtime.drain_evidence_guard.teardown] Animus SHALL provide a pre-teardown guard that inspects explicit or registered linked Git worktrees for ignored `.pi/` evidence without reading evidence contents.

#### Scenario: Empty linked worktree is safe to remove

r[agent_harness_runtime.drain_evidence_guard.teardown.empty]
- GIVEN a registered linked worktree has no `.pi/` entries
- WHEN the operator runs the drain evidence guard before removal
- THEN the guard reports that worktree as safe
- AND the guard exits successfully.

#### Scenario: Persistent primary worktree is not a teardown target

r[agent_harness_runtime.drain_evidence_guard.teardown.primary]
- GIVEN a repository primary worktree uses a `.git` directory
- WHEN the guard scans registered worktrees
- THEN the primary worktree is classified as persistent
- AND its `.pi/` contents do not authorize or block linked-worktree teardown.

### Requirement: Ambiguous or live evidence fails closed

r[agent_harness_runtime.drain_evidence_guard.fail_closed] The guard MUST block teardown when a linked worktree contains `.pi/` entries or when the target, Git marker, evidence root, traversal, or symlink disposition cannot be classified safely.

#### Scenario: Live evidence blocks teardown

r[agent_harness_runtime.drain_evidence_guard.fail_closed.live]
- GIVEN a linked worktree contains at least one `.pi/` entry
- WHEN the guard scans that worktree
- THEN the guard reports a bounded blocking count
- AND it exits unsuccessfully before worktree removal.

#### Scenario: Internal evidence symlink blocks teardown

r[agent_harness_runtime.drain_evidence_guard.fail_closed.internal_symlink]
- GIVEN a linked worktree `.pi` symlink resolves inside that worktree
- WHEN the guard scans the worktree
- THEN the guard treats the target as removable evidence
- AND it exits unsuccessfully.

#### Scenario: Invalid explicit target fails closed

r[agent_harness_runtime.drain_evidence_guard.fail_closed.invalid_target]
- GIVEN an explicit target is missing, not a directory, or lacks a supported Git marker
- WHEN the guard scans the target
- THEN it returns a typed safe diagnostic
- AND it exits unsuccessfully.

### Requirement: Guard output is bounded and metadata-only

r[agent_harness_runtime.drain_evidence_guard.evidence] Guard output MUST omit evidence filenames, contents, WIDs, receipt bodies, credentials, private evidence paths, and content-derived identities while retaining bounded counts, target labels, classifications, and verdicts.

#### Scenario: Blocking output omits evidence details

r[agent_harness_runtime.drain_evidence_guard.evidence.redacted]
- GIVEN a linked worktree contains sensitive `.pi/` filenames and contents
- WHEN the guard blocks teardown
- THEN output includes only the worktree label, bounded entry count, classification, and verdict
- AND output does not include evidence names or contents.

### Requirement: Drain guard has positive and negative verification

r[agent_harness_runtime.drain_evidence_guard.verification] The guard SHALL include deterministic positive and negative tests for empty, primary, non-empty, malformed, unreadable, bounded, and symlink cases, and SHALL be exercised by a focused Nix check.

#### Scenario: Focused guard verification passes

r[agent_harness_runtime.drain_evidence_guard.verification.focused]
- GIVEN the checked guard source and supported host prerequisites
- WHEN the focused guard check runs
- THEN positive fixtures pass
- AND negative fixtures fail with their expected safe classifications.
