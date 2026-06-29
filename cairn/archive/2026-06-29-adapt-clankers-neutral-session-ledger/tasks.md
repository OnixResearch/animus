# Tasks

- [x] [serial] Define Animus-owned neutral ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, unsupported entries, replay metadata, and typed replay errors. r[agent_harness_runtime.clankers_neutral_ledger.dto]
- [x] [depends:clankers_neutral_ledger.dto] Implement pure in-memory replay from supported entries to ordered model-visible context plus safe metadata counts. r[agent_harness_runtime.clankers_neutral_ledger.replay]
- [x] [depends:clankers_neutral_ledger.replay] Add positive replay tests for user, assistant, tool, summary, usage, and receipt entries. r[agent_harness_runtime.clankers_neutral_ledger.replay_positive]
- [x] [depends:clankers_neutral_ledger.dto] Add negative replay tests for unsupported entries and adapter-owned payload markers that fail closed without raw payload leakage. r[agent_harness_runtime.clankers_neutral_ledger.unsupported]
- [x] [depends:clankers_neutral_ledger.dto] Add missing-session and missing-ledger-service runtime tests that fail before model or tool adapter calls. r[agent_harness_runtime.clankers_neutral_ledger.missing_service]
- [x] [depends:clankers_neutral_ledger.replay] If branch parents are included, add pure duplicate-id, missing-parent, cycle, active-branch, and dead-end tests. r[agent_harness_runtime.clankers_neutral_ledger.branch_queries]
- [x] [depends:clankers_neutral_ledger.replay_positive,clankers_neutral_ledger.unsupported,clankers_neutral_ledger.missing_service] Run focused replay tests, workspace tests if touched, dependency-isolation checks, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_neutral_ledger.closeout]
