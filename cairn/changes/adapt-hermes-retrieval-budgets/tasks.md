# Tasks

- [ ] [serial] Define retrieval budget DTOs with purpose, first action, repeat conditions, stop conditions, missing-evidence behavior, citation/source expectations, progress guidance, provenance, and budget limits. r[agent_harness_runtime.hermes_retrieval_budgets.descriptor]
- [ ] [depends:descriptor] Add positive fixtures for Hermes-style retrieval budgets attached to skills/tools. r[agent_harness_runtime.hermes_retrieval_budgets.descriptor.valid]
- [ ] [serial] Add resource policy that can require retrieval budgets for retrieval-capable resources. r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy]
- [ ] [depends:resource_policy] Add missing-budget diagnostics for web-search, browser, documentation lookup, repo browsing, multi-lens, and property-discovery resources. r[agent_harness_runtime.hermes_retrieval_budgets.resource_policy.missing]
- [ ] [serial] Implement pure retrieval repeat/stop decision logic over budget descriptors and evidence summaries. r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules]
- [ ] [depends:repeat_rules] Add tests proving sufficient first evidence stops further retrieval. r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.stop_after_sufficient]
- [ ] [depends:repeat_rules] Add tests proving missing required facts authorize only targeted repeat retrieval or smallest-input requests. r[agent_harness_runtime.hermes_retrieval_budgets.repeat_rules.retrieve_missing]
- [ ] [serial] Add bounded retrieval evidence metadata for reports/receipts with safe references and BLAKE3 hashes. r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata]
- [ ] [depends:evidence_metadata] Add tests proving raw fetched bodies and unsafe paths are not embedded by default. r[agent_harness_runtime.hermes_retrieval_budgets.evidence_metadata.safe]
- [ ] [depends:descriptor.valid,resource_policy.missing,repeat_rules.stop_after_sufficient,repeat_rules.retrieve_missing,evidence_metadata.safe] Update docs and run focused retrieval-budget tests, prompt-hygiene checks, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.hermes_retrieval_budgets.verification]
