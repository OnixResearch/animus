# agent-harness-runtime Change Spec: TUI tool call and approval panels

## Requirements

### Requirement: Tool-call cards render from host bridge events

r[agent_harness_runtime.tui_tools.cards] Tool-call cards MUST be derived from `ToolHostEvent` started/progress/output/result/failure events with collapsible bounded output, and MUST NOT retain raw tool payloads or credentials in render state.

#### Scenario: Tool card shows bounded output

r[agent_harness_runtime.tui_tools.cards.bounded]
- GIVEN a tool emits started, progress, and result events through the host bridge
- WHEN the tool card renders from those events
- THEN it shows bounded model-visible output and safe size/truncation metadata
- AND no raw tool payload is retained in render state.

### Requirement: Diff views render from hashed-edit DTOs

r[agent_harness_runtime.tui_tools.diff_view] File-edit and diff views MUST render from hashed-edit-protocol DTOs and tool-gateway-delivery receipts host-only, and MUST NOT reconstruct edits from direct file reads in the TUI.

#### Scenario: Edit DTO renders as a diff

r[agent_harness_runtime.tui_tools.diff_view.from_dto]
- GIVEN a tool result carries a hashed-edit-protocol DTO with BLAKE3 snapshots and tagged spans
- WHEN the diff view renders it
- THEN it shows the tagged spans as a bounded diff with integrity metadata
- AND no direct file read occurs in the TUI.

#### Scenario: Malformed edit DTO fails safely

r[agent_harness_runtime.tui_tools.diff_view.malformed]
- GIVEN a hashed-edit DTO is missing a snapshot or has a span outside the snapshot bounds
- WHEN the diff view attempts to render it
- THEN it reports a typed malformed-edit diagnostic
- AND no partial or unsafe diff is rendered.

### Requirement: Progress rendering from host events

r[agent_harness_runtime.tui_tools.progress] Tool progress MUST render from existing tool host progress events without inventing progress sources or unbounded polling.

#### Scenario: Long-running tool shows bounded progress

r[agent_harness_runtime.tui_tools.progress.bounded]
- GIVEN a tool emits progress events with completed/total values
- WHEN the progress renderer applies them
- THEN it shows bounded progress metadata
- AND completed-exceeds-total or unknown-total cases render safe placeholders without panicking.

### Requirement: Approvals extend the finite correlated bridge

r[agent_harness_runtime.tui_tools.approval] Interactive approval prompts MUST extend the archived `ToolHostEvent`/`ToolHostCommand` bridge using finite `ToolCommandChoice` values and `ToolCorrelationId` identities, and MUST NOT add a new approval contract or allow unbounded free-form terminal input.

#### Scenario: Denied approval fails closed

r[agent_harness_runtime.tui_tools.approval.denied]
- GIVEN a guarded tool action requests approval through the host bridge
- WHEN the panel returns a denial for the matching correlation identity
- THEN runtime reports a typed denied or cancelled tool result
- AND the guarded action is not executed after the denial.

#### Scenario: Mismatched correlation is rejected

r[agent_harness_runtime.tui_tools.approval.mismatch]
- GIVEN the panel returns a decision for a different correlation identity
- WHEN runtime validates the command
- THEN the command is rejected as a mismatched-confirmation failure
- AND the guarded action remains unexecuted.

### Requirement: Tool panels verification

r[agent_harness_runtime.tui_tools.verification] The change MUST include focused positive and negative tests for card rendering, diff rendering from DTOs, malformed edit DTOs, progress bounding, approval denial, correlation mismatch, oversized output, and direct-draw leakage before closeout.

#### Scenario: Closeout evidence covers tool-panel fixtures

r[agent_harness_runtime.tui_tools.verification.closeout]
- GIVEN maintainers intend to archive the tool call and approval panels change
- WHEN closeout validation runs
- THEN evidence names focused card, diff, progress, approval, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
