# Tasks

- [x] [serial] Define review lifecycle request, evidence bundle, rail descriptor/result, reviewer attempt plan, lifecycle outcome, and safe receipt DTOs. r[agent_harness_runtime.hermes_done_review.lifecycle]
- [x] [depends:lifecycle] Add pure validation for bounded review evidence, safe subject metadata, rail status, reviewer attempt descriptors, and missing-service failure classes. r[agent_harness_runtime.hermes_done_review.evidence]
- [x] [depends:evidence] Add host-owned reviewer service and rail execution boundaries that fail closed without default reviewer, provider, command, VCS, UI, or metrics behavior. r[agent_harness_runtime.hermes_done_review.host_owned]
- [x] [depends:host_owned] Add outcome reducers that map PASS/WARN/FAIL and malformed or missing reviewer outputs to safe continuation intents without automatic mutation or retry. r[agent_harness_runtime.hermes_done_review.outcomes]
- [x] [depends:outcomes] Add positive and negative tests plus docs showing optional SDK use, no baked-in `done-review`, and safe receipt leak rejection. r[agent_harness_runtime.hermes_done_review.verification]
