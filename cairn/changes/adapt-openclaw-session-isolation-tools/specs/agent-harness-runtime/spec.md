# agent-harness-runtime Change Spec: OpenClaw session isolation and tools

## Requirements

### Requirement: Session isolation is host-store neutral

r[agent_harness_runtime.openclaw_sessions.scope] Animus MUST adapt OpenClaw session isolation as host-store-neutral planning over explicit metadata, and MUST NOT depend on OpenClaw's global state directory, JSONL transcript layout, gateway-owned persistence, channel connectors, cleanup scheduler, filesystem traversal, clocks, or concrete delivery behavior in reusable crates.

#### Scenario: Missing session store does not trigger discovery

- GIVEN session planning requires loaded session metadata
- WHEN no host session service or metadata snapshot is supplied
- THEN planning returns a missing-session-service diagnostic
- AND reusable runtime does not inspect global directories, transcript files, environment variables, or channel state.

### Requirement: Session key planning is pure and privacy-aware

r[agent_harness_runtime.openclaw_sessions.key_planning] Session key planning MUST be pure over agent identity, source kind, direct-message scope, channel, account, peer, group, thread/topic, cron, hook, node, subagent, and identity-link metadata, and MUST produce deterministic keys plus privacy diagnostics for shared direct-message scopes in multi-sender contexts.

#### Scenario: Per-channel peer isolates direct messages

- GIVEN direct-message scope is per-channel-peer and two different senders message through the same channel
- WHEN session key planning runs for each sender
- THEN it produces distinct keys containing the agent, channel, and normalized peer identity
- AND no prior sender's context is selected for the other sender by key derivation.

### Requirement: Session maintenance is a deterministic plan

r[agent_harness_runtime.openclaw_sessions.maintenance_policy] Session maintenance MUST produce deterministic plans for stale-entry pruning, entry-count caps, transcript rotation thresholds, archive retention, and disk-budget pressure over host-loaded metadata without deleting files, reading directories, using clocks, or mutating stores from reusable crates.

#### Scenario: Cleanup preview has no side effects

- GIVEN a host supplies session metadata, logical age classes, transcript size summaries, and a maintenance policy
- WHEN maintenance planning runs in preview mode
- THEN it returns ordered proposed actions and safe counts
- AND no transcript, archive, store, or filesystem mutation occurs.

### Requirement: Session tool visibility is clamped

r[agent_harness_runtime.openclaw_sessions.tools_visibility] Session tool planning MUST enforce self, tree, agent, and all visibility modes, MUST clamp sandboxed or delegated sessions to host policy ceilings, and MUST reject visibility widening, cross-agent access, or unbounded history previews before transcript or ledger data is exposed.

#### Scenario: Sandboxed session cannot widen to all sessions

- GIVEN a sandboxed session has a host clamp of tree visibility
- WHEN a session-list or history request asks for all sessions
- THEN planning returns only the current session and spawned descendants or a visibility-denied diagnostic
- AND no unrelated session metadata or transcript preview is exposed.

### Requirement: Inter-session influence is provenance-bearing

r[agent_harness_runtime.openclaw_sessions.inter_session_provenance] Inter-session send and spawn planning MUST record source session, target session, source tool, source channel when available, child lineage, handoff mode, reply/announce bounds, and safe influence metadata before routed agent instructions can affect another session's context.

#### Scenario: Routed agent message is not external user input

- GIVEN one session sends a task to another session through a session tool
- WHEN the target ledger or transcript projection is built
- THEN the routed message is marked as inter-session provenance with source metadata
- AND replay does not treat it as unauthenticated external user input.

### Requirement: Session adaptation verification

r[agent_harness_runtime.openclaw_sessions.verification] The OpenClaw session adaptation MUST include positive key-planning, identity-link, maintenance-plan, visibility, inter-session provenance, and lineage tests plus negative shared-DM leakage, ambiguous metadata, sandbox widening, cross-agent access, missing provenance, unbounded history, and ambient store discovery tests before archive.

#### Scenario: Closeout evidence covers privacy and delegation

- GIVEN maintainers intend to archive the OpenClaw session adaptation
- WHEN closeout validation runs
- THEN evidence names key planner, maintenance, visibility, provenance, lineage, docs, Cairn validation, and Cairn gates
- AND omitted concrete store, cleanup scheduler, channel connector, gateway persistence, and live delivery checks are listed as non-claims.
