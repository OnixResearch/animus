# agent-harness-runtime Change Spec: Subagent UX hooks

## Requirements

### Requirement: Subagent status projection is UI-neutral

r[agent_harness_runtime.subagent_ux_hooks.status_projection] Animus MUST expose UI-neutral subagent status projections for queued, running, cancelling, completed, cancelled, failed, timed-out, blocked, and awaiting-parent states without depending on terminal widgets, headless transports, or host-control implementations.

#### Scenario: TUI and headless hosts render the same status

- GIVEN a child run is running with safe identity and budget metadata
- WHEN runtime builds a subagent status projection
- THEN TUI and headless hosts can render the same state from the projection
- AND the projection contains no terminal-specific widget state.

### Requirement: UX projections include safe metadata only

r[agent_harness_runtime.subagent_ux_hooks.safe_metadata] Subagent UX projections MUST include safe child identity, parent identity, child-run kind, budget usage, denial reason, handoff summary reference, evidence reference, and pending mutation-intent summaries when available, and MUST NOT include raw child logs, full transcripts, credentials, provider payloads, full path dumps, or unbounded tool output.

#### Scenario: Denied child run explains boundary safely

- GIVEN a child run is denied by capability policy
- WHEN the UX projection is built
- THEN the projection names the denial class and safe evidence reference
- AND it does not include raw prompt, credential, or provider payload material.

### Requirement: UX actions are command descriptors

r[agent_harness_runtime.subagent_ux_hooks.command_descriptors] Subagent UX controls MUST be represented as command descriptors for cancel, bounded detail import request, acknowledgement, and mutation-intent arbitration request rather than direct effects.

#### Scenario: Detail import request is bounded

- GIVEN a user asks to inspect child details from the UI
- WHEN the host constructs a detail import command descriptor
- THEN the descriptor includes requested bound, handoff mode, child id, and reason
- AND no detail is imported until policy and bounds checks accept the request.

### Requirement: UX commands route through normal policy

r[agent_harness_runtime.subagent_ux_hooks.policy_routing] UX command descriptors MUST route through coordinator state checks, capability policy, confirmation policy when required, and mutation arbitration policy before causing cancellation, detail import, acknowledgement, or mutation-intent follow-up effects.

#### Scenario: Unauthorized cancel is rejected

- GIVEN a UI sends a cancel command for a child run outside the user's admitted authority
- WHEN command routing evaluates the descriptor
- THEN routing returns an unauthorized-action diagnostic
- AND the child run is not cancelled.

### Requirement: UX hook failures fail closed

r[agent_harness_runtime.subagent_ux_hooks.fail_closed] Subagent UX projection and command routing MUST fail closed for raw-log exposure, unbounded detail import, unauthorized cancellation, mutation bypass attempts, stale handles, malformed command descriptors, hidden credential metadata, and lossy status projection.

#### Scenario: Raw-log projection is rejected

- GIVEN a status projection draft includes raw child log text
- WHEN projection validation runs
- THEN validation returns a leak diagnostic
- AND the projection is not emitted to TUI, headless, or host-control surfaces.

### Requirement: UX hook closeout is evidenced

r[agent_harness_runtime.subagent_ux_hooks.verification] Subagent UX hook adoption MUST include positive status and command descriptor tests, negative fail-closed tests, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: UX hook evidence is complete

- GIVEN the UX hook change is ready to archive
- WHEN closeout validation runs
- THEN evidence names status projection tests, command-routing tests, leak tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
