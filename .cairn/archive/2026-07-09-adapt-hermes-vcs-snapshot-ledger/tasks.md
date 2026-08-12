# Tasks

- [x] [serial] Define snapshot intent, snapshot record, lineage, restore/accept/drop/squash/cleanup plan, freshness proof, confirmation binding, and receipt DTOs. r[agent_harness_runtime.hermes_vcs_snapshot_ledger.records]
- [x] [depends:records] Add pure lineage, freshness, compare, and protected-operation plan validation over VCS-neutral snapshot data. r[agent_harness_runtime.hermes_vcs_snapshot_ledger.plans]
- [x] [depends:plans] Add injected checkpoint/snapshot service boundaries and fail-closed outcomes for missing, unavailable, unsupported, denied, stale, or malformed backend operations. r[agent_harness_runtime.hermes_vcs_snapshot_ledger.host_owned]
- [x] [depends:host_owned] Add receipt validation that rejects raw diffs, file bodies, command lines, full paths, provider payloads, credentials, and unsafe backend refs. r[agent_harness_runtime.hermes_vcs_snapshot_ledger.receipts]
- [x] [depends:receipts] Add positive and negative tests for VCS-neutral records, lineage, plans, confirmation, missing service, no autocommit, backend command rejection, and receipt leaks. r[agent_harness_runtime.hermes_vcs_snapshot_ledger.verification]
