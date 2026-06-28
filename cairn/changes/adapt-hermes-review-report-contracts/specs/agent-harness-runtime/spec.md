# agent-harness-runtime Change Spec: Hermes review report contract adaptation

## Requirements

### Requirement: Structured review report DTO

r[agent_harness_runtime.hermes_review_reports.report] Animus MUST define a structured review report contract with a required `VERDICT: PASS | WARN | FAIL` line, parsed sections, metric-bearing findings, diagnostics, source, stage, subject, and safe metadata.

#### Scenario: Valid report parses deterministically

r[agent_harness_runtime.hermes_review_reports.report.valid]
- GIVEN a markdown review report with `VERDICT: WARN`, a `## Findings` section, tagged finding bullets, and optional `## Gaps` or `## Next Steps` sections
- WHEN report parsing runs over the in-memory text
- THEN the parsed report contains verdict `WARN`, normalized sections, parsed findings, and no diagnostics
- AND parsing does not read files, clocks, environment, process state, or network resources.

### Requirement: Findings are parsed only from Findings section

r[agent_harness_runtime.hermes_review_reports.findings_scope] Animus MUST parse metric-bearing finding bullets only from an explicit `## Findings` section.

#### Scenario: Checklist bullets outside Findings are ignored

r[agent_harness_runtime.hermes_review_reports.findings_scope.ignore_checklists]
- GIVEN a report contains bracketed checklist bullets in `## OpenSpec Check`, `## Gaps`, or `## Next Steps`
- WHEN metric findings are extracted
- THEN only valid bullets inside `## Findings` become findings
- AND checklist bullets outside that section remain human-readable prose.

### Requirement: Finding tags are validated

r[agent_harness_runtime.hermes_review_reports.finding_tags] Metric-bearing finding bullets MUST include `class`, `scope`, `route`, and `promoted` tags, and `promoted` MUST be either `yes` or `no`.

#### Scenario: Malformed finding tags reject metrics record

r[agent_harness_runtime.hermes_review_reports.finding_tags.invalid]
- GIVEN a review report has a finding bullet missing `route` or using `promoted=maybe`
- WHEN a metrics record action validates the report
- THEN validation fails with typed diagnostics naming the malformed tags
- AND no durable metrics record is appended.

### Requirement: Metrics read/write behavior is explicit

r[agent_harness_runtime.hermes_review_reports.metrics_boundary] Animus MUST separate read-only report parsing, summary, and promotion-candidate analysis from explicit append-only metrics recording.

#### Scenario: Summary does not write metrics

r[agent_harness_runtime.hermes_review_reports.metrics_boundary.summary_readonly]
- GIVEN durable review metric records already exist
- WHEN a host requests a summary or promotion-candidate report
- THEN the operation reads supplied records and returns bounded aggregate output
- AND it does not append, rewrite, compact, delete, or promote records.

#### Scenario: Explicit record action appends valid report

r[agent_harness_runtime.hermes_review_reports.metrics_boundary.record_append]
- GIVEN a valid parsed report, source, stage, subject, and host-supplied timestamp or sequence metadata
- WHEN an explicit metrics record action runs
- THEN exactly one append-only record is produced through the host storage adapter
- AND the record excludes raw prompts, provider payloads, credentials, full host paths, and unbounded tool/model output.

### Requirement: Repeated finding promotion summaries

r[agent_harness_runtime.hermes_review_reports.promotions] Animus SHOULD support read-only repeated-finding grouping that suggests durable prevention routes such as tests, deterministic checks, prompt rails, spec rules, or human checkpoints without mutating tasks or specs automatically.

#### Scenario: Repeated finding becomes promotion candidate

r[agent_harness_runtime.hermes_review_reports.promotions.candidate]
- GIVEN multiple metric records contain unpromoted findings with the same class, scope, and route tags
- WHEN promotion-candidate analysis runs with a configured minimum count
- THEN the output groups those findings, includes representative examples, and suggests a prevention route
- AND the analysis does not mark anything promoted or edit lifecycle artifacts.

### Requirement: Review report adaptation verification

r[agent_harness_runtime.hermes_review_reports.verification] The review report adaptation MUST include positive and negative tests for verdict parsing, section parsing, finding extraction, malformed tag rejection, metrics append validation, read-only summaries, promotion candidate grouping, and safe metadata redaction.

#### Scenario: Closeout evidence covers report fixtures

r[agent_harness_runtime.hermes_review_reports.verification.closeout]
- GIVEN maintainers intend to archive the Hermes review report adaptation
- WHEN closeout validation runs
- THEN evidence names focused report parser tests, metrics boundary tests, promotion grouping tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
