# Tasks

- [x] [serial] Define parsed review report DTOs for verdict, sections, findings, diagnostics, source, stage, subject, and safe metadata. r[agent_harness_runtime.hermes_review_reports.report]
- [x] [depends:report] Add positive parser fixtures for PASS, WARN, and FAIL reports with optional sections. r[agent_harness_runtime.hermes_review_reports.report.valid]
- [x] [serial] Scope metric finding extraction to `## Findings` only. r[agent_harness_runtime.hermes_review_reports.findings_scope]
- [x] [depends:findings_scope] Add tests proving checklist bullets outside `## Findings` are ignored. r[agent_harness_runtime.hermes_review_reports.findings_scope.ignore_checklists]
- [x] [serial] Validate `class`, `scope`, `route`, and `promoted` tags for metric-bearing findings. r[agent_harness_runtime.hermes_review_reports.finding_tags]
- [x] [depends:finding_tags] Add negative tests for missing tags, duplicate tags, unknown promoted values, and malformed verdicts. r[agent_harness_runtime.hermes_review_reports.finding_tags.invalid]
- [x] [serial] Define metrics summary and record boundaries with read-only summary and explicit append-only record actions. r[agent_harness_runtime.hermes_review_reports.metrics_boundary]
- [x] [depends:metrics_boundary] Add tests proving summaries do not write and record actions append only valid safe records. r[agent_harness_runtime.hermes_review_reports.metrics_boundary.summary_readonly]
- [x] [depends:metrics_boundary] Add safe metadata/redaction tests for explicit record appends. r[agent_harness_runtime.hermes_review_reports.metrics_boundary.record_append]
- [x] [serial] Add repeated-finding promotion candidate grouping without automatic lifecycle mutation. r[agent_harness_runtime.hermes_review_reports.promotions]
- [x] [depends:promotions] Add grouping tests for repeated class/scope/route findings and already-promoted records. r[agent_harness_runtime.hermes_review_reports.promotions.candidate]
- [x] [depends:report.valid,findings_scope.ignore_checklists,finding_tags.invalid,metrics_boundary.summary_readonly,metrics_boundary.record_append,promotions.candidate] Update docs and run focused review-report tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_review_reports.verification]
