# agent-harness-runtime Change Spec: Agent status authority adaptation

## Requirements

### Requirement: Semantic agent status DTOs

r[agent_harness_runtime.agent_status_authority.semantic_state] Animus MUST define semantic agent status DTOs for `idle`, `working`, `blocked`, `done`, and `unknown` with source identity, agent label, optional native session reference, bounded message text, presentation metadata, sequence, and optional TTL fields.

#### Scenario: Lifecycle report is validated

r[agent_harness_runtime.agent_status_authority.semantic_state.validation]
- GIVEN a trusted host integration reports an agent label, source identifier, semantic state, target, and sequence
- WHEN Animus validates the status report
- THEN the report is accepted as a typed lifecycle observation
- AND malformed states, malformed source identifiers, missing targets, and unsupported session references are rejected with typed diagnostics.

### Requirement: Authority precedence is deterministic

r[agent_harness_runtime.agent_status_authority.authority_precedence] Status selection MUST choose one semantic authority for a target at a time, and a current trusted lifecycle authority MUST suppress heuristic fallback observations for that target.

#### Scenario: Lifecycle authority wins over heuristic fallback

r[agent_harness_runtime.agent_status_authority.authority_precedence.lifecycle_wins]
- GIVEN a current trusted lifecycle report marks an agent `working`
- AND a host heuristic observation would otherwise classify the same target as `idle`
- WHEN status selection runs
- THEN the selected semantic state is `working`
- AND the heuristic observation is recorded only as non-authoritative fallback evidence.

### Requirement: Presentation metadata is visual-only

r[agent_harness_runtime.agent_status_authority.presentation_only] Titles, display agent names, compact activity text, custom status labels, and state label overrides MUST be presentation-only metadata and MUST NOT change semantic lifecycle state, waits, notifications, rollups, or policy decisions.

#### Scenario: Custom label cannot escalate state

r[agent_harness_runtime.agent_status_authority.presentation_only.no_semantic_escalation]
- GIVEN a metadata report sets a custom visual label that says `needs input`
- AND the selected semantic state remains `working`
- WHEN waits, notifications, rollups, and policy decisions consume agent status
- THEN they use `working`
- AND the visual label is available only to presentation hosts.

### Requirement: Explainable status decisions are bounded

r[agent_harness_runtime.agent_status_authority.explain] Animus SHOULD expose bounded status-decision explanations that identify the selected authority, rejected candidates, freshness class, fallback reason, safe hashes, and redacted evidence summaries without embedding raw prompts, terminal snapshots, provider payloads, credentials, or unbounded detector output.

#### Scenario: Explanation rejects raw evidence leaks

r[agent_harness_runtime.agent_status_authority.explain.redacted]
- GIVEN a status explanation fixture contains raw terminal text, prompt text, provider JSON, a credential-looking value, or an unbounded detector transcript
- WHEN explanation validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the explanation is not accepted as durable evidence.

### Requirement: Detection is host-owned

r[agent_harness_runtime.agent_status_authority.host_owned_detection] Reusable Animus runtime crates MUST NOT inspect processes, read terminals, parse screen buffers, or load detector manifests; they MUST consume typed host observations supplied through adapter contracts.

#### Scenario: Pure status selection has no ambient I/O

r[agent_harness_runtime.agent_status_authority.host_owned_detection.no_ambient_io]
- GIVEN status selection receives typed lifecycle and heuristic observations in memory
- WHEN it selects the current semantic state
- THEN the selection result is deterministic over those inputs
- AND the pure status core does not read terminals, processes, files, clocks, environment variables, or network state.

### Requirement: Status authority verification

r[agent_harness_runtime.agent_status_authority.verification] The adaptation MUST include focused positive and negative tests for report validation, authority precedence, stale sequence and TTL handling, presentation-only metadata, bounded explanations, leak rejection, host-owned detection, and pure status selection.

#### Scenario: Closeout evidence covers status authority rails

r[agent_harness_runtime.agent_status_authority.verification.closeout]
- GIVEN maintainers intend to archive the agent status authority adaptation
- WHEN closeout validation runs
- THEN evidence names focused status DTO, authority precedence, presentation metadata, explanation redaction, host-owned detection, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a blocker and next-best validation.
