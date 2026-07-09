# Tasks

- [ ] [serial] Define ROI request, score input, recommendation report, runner-up, missing-context, action descriptor, lifecycle-routing hint, and receipt DTOs. r[agent_harness_runtime.hermes_roi.recommendation]
- [ ] [depends:recommendation] Add bounded parser/validator for structured ROI reports and safe score dimensions. r[agent_harness_runtime.hermes_roi.report_validation]
- [ ] [depends:report_validation] Add admission-gated action descriptors for begin, lifecycle-first, investigate, delegate, dismiss, and no-op outcomes. r[agent_harness_runtime.hermes_roi.action_descriptors]
- [ ] [depends:action_descriptors] Add host-owned boundaries for evidence collection, model scoring, repository inspection, UI selection, lifecycle mutation, and task execution. r[agent_harness_runtime.hermes_roi.host_owned]
- [ ] [depends:host_owned] Add positive and negative tests for reports, score fields, action routing, missing context, unsafe evidence, no default command, and receipt leak rejection. r[agent_harness_runtime.hermes_roi.verification]
