# agent-harness-runtime Change Spec: TUI session context status rail

## Requirements

### Requirement: Session picker projects ledger DTOs

r[agent_harness_runtime.tui_status.session_picker] The session picker MUST project session-tree-ledger DTOs and issue typed switch commands to the host, and MUST NOT mutate session state directly in the TUI.

#### Scenario: Missing session reports diagnostic

r[agent_harness_runtime.tui_status.session_picker.missing]
- GIVEN the session-tree-ledger has no active sessions
- WHEN the session picker renders
- THEN it reports an empty-session diagnostic
- AND no session mutation is attempted.

### Requirement: Model picker projects registry without credentials

r[agent_harness_runtime.tui_status.model_picker] The model picker MUST project provider-registry/model-dispatch DTOs showing labels and capabilities only, and MUST NOT read or display credentials, tokens, or provider secret locators.

#### Scenario: Credential leakage is rejected

r[agent_harness_runtime.tui_status.model_picker.no_credentials]
- GIVEN a provider-registry DTO includes credential or secret-locator fields
- WHEN the model picker projects it
- THEN only labels and capabilities are shown
- AND credential or secret-locator values are not rendered.

#### Scenario: Unknown model is reported

r[agent_harness_runtime.tui_status.model_picker.unknown]
- GIVEN a selection targets a model absent from the registry
- WHEN the picker validates the selection
- THEN it reports an unknown-model diagnostic
- AND no model switch command is issued.

### Requirement: Bounded context gauge

r[agent_harness_runtime.tui_status.context_gauge] The context/retrieval-budget gauge MUST be a bounded, normalized projection of retrieval-budget DTOs and MUST render oversize contexts safely without panicking.

#### Scenario: Oversize context is normalized

r[agent_harness_runtime.tui_status.context_gauge.oversize]
- GIVEN a retrieval-budget DTO reports usage near or above budget
- WHEN the gauge renders it
- THEN it shows a normalized, bounded projection with overflow markers
- AND no numeric overflow or panic occurs.

### Requirement: Status bar and cost follow authority

r[agent_harness_runtime.tui_status.status_bar_cost] The status bar MUST follow status-authority precedence and the cost overlay MUST project cost-model-routing DTOs display-only with freshness metadata, and MUST NOT claim authoritative provider balances.

#### Scenario: Stale cost is marked display-only

r[agent_harness_runtime.tui_status.status_bar_cost.stale]
- GIVEN a cost DTO is older than the configured freshness window
- WHEN the cost overlay renders it
- THEN it marks the value as stale and display-only
- AND it does not claim an authoritative provider balance.

### Requirement: Read-only auxiliary panels

r[agent_harness_runtime.tui_status.aux_panels] Git/branch, environment, and todo panels MUST be read-only projections of process-job-profile and status surfaces, and any mutation MUST issue a typed command to the host rather than acting on filesystem, git, or process state directly.

#### Scenario: Git mutation issues a typed command

r[agent_harness_runtime.tui_status.aux_panels.typed_command]
- GIVEN a user requests a branch switch from the git/branch panel
- WHEN the panel handles the request
- THEN it issues a typed host command
- AND no direct git operation or filesystem mutation occurs in the TUI.

### Requirement: Status rail verification

r[agent_harness_runtime.tui_status.verification] The change MUST include focused positive and negative tests for session listing, model selection, credential rejection, context bounding, status precedence, cost freshness, auxiliary-panel command issuing, and reusable-crate contract isolation before closeout.

#### Scenario: Closeout evidence covers status-rail fixtures

r[agent_harness_runtime.tui_status.verification.closeout]
- GIVEN maintainers intend to archive the session context status rail change
- WHEN closeout validation runs
- THEN evidence names focused session, model, context, status, cost, auxiliary, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
