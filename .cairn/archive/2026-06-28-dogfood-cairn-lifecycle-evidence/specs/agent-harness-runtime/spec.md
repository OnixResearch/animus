# agent-harness-runtime Change Spec: Cairn lifecycle dogfooding

## Requirements

### Requirement: Lifecycle layout

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout] Animus MUST treat `cairn/specs/` as the accepted requirements baseline, `cairn/changes/` as active change deltas, and `cairn/archive/` as completed lifecycle history once changes are accepted and archived.

#### Scenario: Accepted spec is source of current requirements

r[agent_harness_runtime.cairn_dogfood.lifecycle_layout.accepted_baseline]
- GIVEN a requirement has been accepted and synced
- WHEN maintainers inspect current lifecycle requirements
- THEN the requirement appears under `cairn/specs/`
- AND the completed change package is archived rather than left as active planning state.

### Requirement: Change threshold guidance

r[agent_harness_runtime.cairn_dogfood.change_threshold] Animus MUST document when work requires a Cairn change package, including public behavior, compatibility semantics, mutation behavior, architecture, receipt/schema contracts, release/evidence contracts, roadmap scope, and safety-boundary changes.

#### Scenario: Small safe edits can stay direct

r[agent_harness_runtime.cairn_dogfood.change_threshold.direct_safe_edit]
- GIVEN a change is limited to typo repair, fixture hardening, or narrow documentation cleanup
- WHEN it does not affect public behavior, architecture, compatibility, mutation, schema, receipt, or roadmap scope
- THEN maintainers may apply it directly
- AND ordinary repository checks still provide evidence.

### Requirement: Gate workflow

r[agent_harness_runtime.cairn_dogfood.gate_workflow] Animus MUST define a lifecycle workflow for proposal, design, tasks, validation, sync dry-run, archive dry-run, and executed archive closeout using Cairn commands or pinned equivalents.

#### Scenario: Change cannot archive without gates

r[agent_harness_runtime.cairn_dogfood.gate_workflow.blocks_incomplete]
- GIVEN an active Cairn change lacks passing proposal, design, tasks, validation, or sync/archive dry-run evidence
- WHEN maintainers attempt closeout
- THEN the change is not considered archive-ready
- AND blockers or waivers are recorded explicitly.

### Requirement: Bounded evidence contract

r[agent_harness_runtime.cairn_dogfood.evidence_contract] Cairn lifecycle evidence SHOULD record command identity, status, issue codes, requirement/change IDs, policy source identity, policy hash when available, output artifact hashes, and unavailable-check blockers while keeping evidence bounded and reviewable.

#### Scenario: Policy identity is visible in evidence

r[agent_harness_runtime.cairn_dogfood.evidence_contract.policy]
- GIVEN Cairn validation or gates run under a policy artifact
- WHEN evidence is recorded
- THEN the evidence identifies the policy source or artifact and hash when available
- AND reviewers can tell which policy accepted the lifecycle content.

### Requirement: Safe evidence summaries

r[agent_harness_runtime.cairn_dogfood.safe_evidence] Durable Cairn evidence summaries MUST omit raw prompts, credentials, provider payloads, full private paths, raw private file bodies, and unbounded command output by default.

#### Scenario: Evidence summary redacts private material

r[agent_harness_runtime.cairn_dogfood.safe_evidence.redacted]
- GIVEN validation, gate, sync, archive, release-readiness, or workspace-check output includes private or unbounded material
- WHEN durable evidence is summarized
- THEN it includes bounded status, counts, issue codes, hashes, and safe relative labels
- AND raw sensitive material is omitted or replaced by approved redaction markers.

### Requirement: Cairn dogfood closeout

r[agent_harness_runtime.cairn_dogfood.closeout] Cairn dogfood work MUST run validation, proposal/design/tasks gates, and sync/archive dry runs for the change itself before archive.

#### Scenario: Dogfood evidence is reviewable

r[agent_harness_runtime.cairn_dogfood.closeout.evidence]
- GIVEN Cairn dogfood work is ready to archive
- WHEN reviewers inspect evidence
- THEN validation, gates, sync dry-run, archive dry-run, and evidence-safety checks are named with status
- AND unavailable checks include a blocker and next best validation path.
