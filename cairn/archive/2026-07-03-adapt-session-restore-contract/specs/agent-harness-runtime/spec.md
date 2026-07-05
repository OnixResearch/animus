# agent-harness-runtime Change Spec: Session restore contract adaptation

## Requirements

### Requirement: Explicit restore matrix

r[agent_harness_runtime.session_restore.restore_matrix] Animus MUST define typed restore modes and capability claims for live process persistence, snapshot layout restore, bounded history replay, native agent session resume, handoff-style transfer, and safe fallback.

#### Scenario: Restore claim states what survived

r[agent_harness_runtime.session_restore.restore_matrix.claims]
- GIVEN a host reports a restored session plan
- WHEN Animus validates the plan
- THEN each restored target explicitly states whether process state survived, layout was reconstructed, display history was replayed, or native conversation resume was requested
- AND no mode implies stronger survival than the host capability evidence supports.

### Requirement: Safe native session references

r[agent_harness_runtime.session_restore.native_session_reference] Native agent session references MUST be represented as safe typed records with source, agent, kind, bounded value identity, freshness, uniqueness scope, and resume capability metadata before any host attempts resume.

#### Scenario: Invalid reference degrades to fallback

r[agent_harness_runtime.session_restore.native_session_reference.invalid]
- GIVEN a restore snapshot contains a missing, malformed, duplicated, stale, unsupported, or sensitive native session reference
- WHEN restore planning evaluates that reference
- THEN Animus rejects native resume for that target with a typed diagnostic
- AND the plan degrades to an explicit safe fallback instead of claiming conversation continuity.

### Requirement: History replay is opt-in and display-only

r[agent_harness_runtime.session_restore.history_replay] Pane or transcript history replay MUST be disabled by default, and when enabled MUST be bounded, redacted, classified as display replay, and excluded from model-visible context unless separately admitted by policy.

#### Scenario: Replayed history cannot masquerade as live context

r[agent_harness_runtime.session_restore.history_replay.safe]
- GIVEN a restore snapshot includes recent display history
- WHEN history replay planning runs
- THEN the replay metadata records redaction and truncation status
- AND the replay is not treated as a live process, native conversation resume, or model-visible context by default.

### Requirement: Restore execution is host-owned

r[agent_harness_runtime.session_restore.host_owned_execution] Reusable Animus runtime crates MUST validate snapshots and produce restore plans without spawning processes, reading files, opening PTYs, constructing shell commands, transferring terminals, or inspecting environment state.

#### Scenario: Pure restore planning has no ambient I/O

r[agent_harness_runtime.session_restore.host_owned_execution.no_ambient_io]
- GIVEN restore planning receives typed snapshots, host capability declarations, session references, and policy inputs in memory
- WHEN planning runs
- THEN the returned plan is deterministic over those inputs
- AND no reusable runtime code performs filesystem, terminal, process, network, clock, or environment I/O.

### Requirement: Degraded restore outcomes are explicit

r[agent_harness_runtime.session_restore.degraded_outcomes] Unsupported agents, unavailable resume services, failed handoff claims, invalid snapshots, and denied history replay MUST produce explicit degraded-restore outcomes and safe receipts rather than silent blank-shell fallbacks.

#### Scenario: Fallback cannot claim successful resume

r[agent_harness_runtime.session_restore.degraded_outcomes.no_false_claims]
- GIVEN native resume is unavailable for a restored target
- WHEN the host creates a fallback shell or replacement process
- THEN the outcome records fallback status and reason
- AND receipts do not claim process survival, handoff success, native resume, or conversation continuity.

### Requirement: Restore verification

r[agent_harness_runtime.session_restore.verification] The adaptation MUST include focused positive and negative tests for restore matrix claims, native session references, history replay safety, pure restore planning, host-owned execution, degraded outcomes, and false-survival-claim rejection.

#### Scenario: Closeout evidence covers restore rails

r[agent_harness_runtime.session_restore.verification.closeout]
- GIVEN maintainers intend to archive the session restore adaptation
- WHEN closeout validation runs
- THEN evidence names focused restore matrix, native reference, history replay, host-owned execution, degraded outcome, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.
