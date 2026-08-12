# Tasks

- [x] [serial] Define VCS-neutral review run requests for working copy changes, commits, branch comparisons, and externally supplied patches. r[agent_harness_runtime.openai_codex_diff_review_mode.review_request]
- [x] [depends:review_request] Add positive selector tests and negative missing-diff/unsupported-selector tests. r[agent_harness_runtime.openai_codex_diff_review_mode.review_request.selectors]
- [x] [serial] Add a read-only review policy profile that forbids mutations, write-capable tools, and approval escalation by default. r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy]
- [x] [depends:read_only_policy] Add negative tests proving review mode cannot edit files, run write-capable tools, or request write escalation. r[agent_harness_runtime.openai_codex_diff_review_mode.read_only_policy.no_mutation]
- [x] [serial] Define typed review finding DTOs with severity, category, affected span, evidence, rationale, confidence, and remediation guidance. r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema]
- [x] [depends:finding_schema] Add validation tests for valid findings and negative invalid-span/missing-evidence/duplicate-finding cases. r[agent_harness_runtime.openai_codex_diff_review_mode.finding_schema.validation]
- [x] [serial] Add bounded evidence and source-attribution rules for file spans, patch hunks, and external review inputs. r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds]
- [x] [depends:evidence_bounds] Add negative tests proving full files, unbounded logs, and unrelated context are not embedded in review reports. r[agent_harness_runtime.openai_codex_diff_review_mode.evidence_bounds.no_unbounded]
- [x] [serial] Define rerun comparison records for open, fixed, dismissed, and newly introduced findings. r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking]
- [x] [depends:rerun_tracking] Add rerun tests proving fixed findings are distinguished from newly introduced findings. r[agent_harness_runtime.openai_codex_diff_review_mode.rerun_tracking.state]
- [x] [depends:review_request.selectors,read_only_policy.no_mutation,finding_schema.validation,evidence_bounds.no_unbounded,rerun_tracking.state] Update docs and run focused review-mode tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.openai_codex_diff_review_mode.review_request]
