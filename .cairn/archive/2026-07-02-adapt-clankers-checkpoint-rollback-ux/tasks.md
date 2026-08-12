# Tasks

- [x] [serial] Define VCS-neutral checkpoint and rollback DTOs with backend kind, opaque checkpoint ID, safe workspace identity, content hashes, changed-file counts, attribution, and status. r[agent_harness_runtime.clankers_checkpoint_rollback.dto]
- [x] [depends:dto] Add backend-neutral fixtures for jj-like, git-like, in-memory, and workspace-isolation checkpoint identities. r[agent_harness_runtime.clankers_checkpoint_rollback.dto.backend_neutral]
- [x] [serial] Define injected checkpoint service contracts and missing/unavailable/denied/unsupported failure classes. r[agent_harness_runtime.clankers_checkpoint_rollback.service]
- [x] [depends:service] Add strict-protected-mutation tests proving missing checkpoint service blocks dispatch before mutation. r[agent_harness_runtime.clankers_checkpoint_rollback.service.missing_strict]
- [x] [serial] Wire rollback validation through explicit host/operator confirmation decisions. r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation]
- [x] [depends:confirmation] Add unconfirmed rollback tests proving no checkpoint service restore call occurs. r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation.required]
- [x] [serial] Add rollback freshness precondition validation over current workspace identity and selected checkpoint evidence. r[agent_harness_runtime.clankers_checkpoint_rollback.freshness]
- [x] [depends:freshness] Add stale-workspace tests proving restore is rejected before backend mutation. r[agent_harness_runtime.clankers_checkpoint_rollback.freshness.stale]
- [x] [serial] Define safe checkpoint and rollback receipt DTOs with operation/status/backend/checkpoint/count/hash/error metadata. r[agent_harness_runtime.clankers_checkpoint_rollback.receipts]
- [x] [depends:receipts] Add leak fixtures for raw diffs, file bodies, credentials, environment dumps, and full path lists. r[agent_harness_runtime.clankers_checkpoint_rollback.receipts.rejects_leaks]
- [x] [depends:dto.backend_neutral,service.missing_strict,confirmation.required,freshness.stale,receipts.rejects_leaks] Update docs and run focused checkpoint tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_checkpoint_rollback.verification]
