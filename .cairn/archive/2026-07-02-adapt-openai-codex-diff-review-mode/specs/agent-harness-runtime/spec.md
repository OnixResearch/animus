# agent-harness-runtime Change Spec: OpenAI Codex diff review mode adaptation

## Requirements

### Requirement: VCS-neutral review run requests

r[agent_harness_runtime.openai_codex_diff_review_mode.review_request] Animus review mode MUST accept VCS-neutral change-set selectors for working copy changes, commits, branch comparisons, and externally supplied patches.

#### Scenario: Unsupported selector fails before review starts

r[agent_harness_runtime.openai_codex_diff_review_mode.review_request.selectors]
- GIVEN a review request references a change-set selector unsupported by the active VCS adapter
- WHEN Animus validates the review request
- THEN validation fails before model or tool calls begin
- AND the failure identifies the unsupported selector capability.

### Requirement: Review mode is read-only by default

r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy] Review mode MUST use a read-only policy profile that forbids file mutation, write-capable tools, and approval escalation by default.

#### Scenario: Review cannot mutate files

r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy.no_mutation]
- GIVEN a review-mode run is active
- WHEN the loop fabric requests a file edit or write-capable tool
- THEN runtime denies the request
- AND the review report records a read-only policy denial without changing workspace files.

### Requirement: Structured review findings

r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema] Review findings MUST include severity, category, affected span, evidence, rationale, confidence, and remediation guidance.

#### Scenario: Finding with invalid span is rejected

r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema.validation]
- GIVEN a review finding references a span outside the reviewed change set
- WHEN Animus validates the finding
- THEN the finding is rejected as invalid
- AND the report records a finding-validation failure.

### Requirement: Bounded review evidence

r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds] Review evidence MUST be source-attributed and bounded to relevant file spans, patch hunks, or external review inputs.

#### Scenario: Full file is not embedded as evidence

r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds.no_unbounded]
- GIVEN a finding only needs one patch hunk as evidence
- WHEN Animus renders the review report
- THEN the report includes the bounded hunk reference or snippet
- AND it does not embed the full file or unrelated logs.

### Requirement: Rerun finding-state tracking

r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking] Review mode SHOULD compare rerun results against prior review findings to classify findings as open, fixed, dismissed, or newly introduced when prior review state is available.

#### Scenario: Fixed finding is distinguished from new finding

r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking.state]
- GIVEN a prior review found one issue that has since been fixed and the rerun finds a different issue
- WHEN Animus compares the review reports
- THEN the old issue is marked fixed
- AND the different issue is marked newly introduced.
