# Tasks

- [ ] [serial] Define corrective record, review metric record, promotion candidate, append request, promote request, and safe receipt DTOs. r[agent_harness_runtime.hermes_corrective_memory.records]
- [ ] [depends:records] Add explicit trust, scope, freshness, prompt-budget, and source admission for corrective records; reject always-read or untrusted prompt influence. r[agent_harness_runtime.hermes_corrective_memory.admission]
- [ ] [depends:admission] Add read-only repeated-finding aggregation and promotion candidate routing without automatic edits or persistence. r[agent_harness_runtime.hermes_corrective_memory.promotions]
- [ ] [depends:promotions] Add explicit append/promote action validation and receipts for host-owned stores. r[agent_harness_runtime.hermes_corrective_memory.host_actions]
- [ ] [depends:host_actions] Add positive and negative tests for records, metrics, admission, promotion grouping, explicit actions, no default napkin, and receipt leak rejection. r[agent_harness_runtime.hermes_corrective_memory.verification]
