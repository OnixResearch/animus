# agent-harness-runtime Change Spec: Clankers request lifecycle ownership

## Requirements

### Requirement: Animus lifecycle map

r[agent_harness_runtime.clankers_lifecycle_doc.map] Animus MUST document the runtime request lifecycle from typed runtime input through deterministic core state, Steel loop decisions, adapter dispatch, event/report projection, safe receipt construction, and host ledger or persistence seams.

#### Scenario: Golden path is reviewable

r[agent_harness_runtime.clankers_lifecycle_doc.map.reviewable]
- GIVEN a maintainer needs to change runtime request behavior
- WHEN they inspect `docs/request-lifecycle.md`
- THEN the document describes the ordered Animus runtime path
- AND it distinguishes reusable core/adapters/Steel behavior from std-shell or host-owned behavior.

### Requirement: Boundary owners and source-of-truth types

r[agent_harness_runtime.clankers_lifecycle_doc.owners] Each lifecycle boundary MUST name the owning crate and source-of-truth type or trait for the state transition.

#### Scenario: Owner names prevent duplicate state machines

r[agent_harness_runtime.clankers_lifecycle_doc.owners.no_duplicate_state]
- GIVEN a change touches runtime input, Steel decisions, model/tool dispatch, reports, events, receipts, or ledger replay
- WHEN reviewers inspect the lifecycle document
- THEN the owning crate and source-of-truth type or trait are named
- AND the change does not introduce an undocumented duplicate owner for the same transition.

### Requirement: Forbidden dependency rules

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies] The lifecycle document MUST identify forbidden dependencies and side effects for reusable core, adapter, Steel, std-shell, receipt, and future ledger boundaries.

#### Scenario: Forbidden dependencies stay visible

r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies.visible]
- GIVEN a reusable Animus crate boundary is reviewed
- WHEN the lifecycle document is consulted
- THEN it identifies which boundaries must not read environment, filesystem, credentials, provider config, network, clocks, or concrete host persistence
- AND it points to the validation rail that checks those constraints when one exists.

### Requirement: Lifecycle links and marker rail

r[agent_harness_runtime.clankers_lifecycle_doc.links] Animus README or architecture docs MUST link to the lifecycle document so maintainers can find it before changing runtime behavior.

#### Scenario: Focused rail guards lifecycle drift

r[agent_harness_runtime.clankers_lifecycle_doc.rail]
- GIVEN the lifecycle document is part of the maintained architecture contract
- WHEN the focused marker rail runs
- THEN it fails if required boundary names, owner markers, forbidden dependency sections, or validation command references are missing
- AND it emits reviewable evidence without requiring live providers or host credentials.

#### Scenario: Lifecycle closeout is evidenced

r[agent_harness_runtime.clankers_lifecycle_doc.closeout]
- GIVEN the lifecycle change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes the focused lifecycle rail, relevant docs checks, Cairn validation, and Cairn proposal/design/tasks gates
- AND skipped checks record a blocker and next-best validation.
