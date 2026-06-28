# agent-harness-runtime Change Spec: Clankers checkpoint rollback UX adaptation

## Requirements

### Requirement: VCS-neutral checkpoint DTOs

r[agent_harness_runtime.clankers_checkpoint_rollback.dto] Animus MUST represent checkpoints and rollback targets with VCS-neutral DTOs that include backend kind, opaque checkpoint ID, safe workspace label or hash, content identities, changed-file counts, session/tool attribution when available, and status metadata.

#### Scenario: Checkpoint identity is backend-neutral

r[agent_harness_runtime.clankers_checkpoint_rollback.dto.backend_neutral]
- GIVEN a host reports a checkpoint created by a jj, git, in-memory, or workspace-isolation backend
- WHEN Animus records checkpoint metadata
- THEN the DTO preserves backend kind and opaque checkpoint ID
- AND reusable runtime logic does not require backend-specific command syntax.

### Requirement: Injected checkpoint service

r[agent_harness_runtime.clankers_checkpoint_rollback.service] Checkpoint create, list, inspect, rollback, and cleanup operations MUST call an injected checkpoint service and MUST fail closed when the service is missing, unavailable, denied, or reports an unsupported operation.

#### Scenario: Missing checkpoint service blocks strict protected mutation

r[agent_harness_runtime.clankers_checkpoint_rollback.service.missing_strict]
- GIVEN checkpoint policy requires strict pre-mutation checkpoints
- AND no checkpoint service is injected
- WHEN a protected mutating tool action is requested
- THEN Animus returns a missing-checkpoint-service failure
- AND the mutation is not dispatched.

### Requirement: Explicit rollback confirmation

r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation] Rollback execution MUST require an explicit host/operator confirmation decision before workspace mutation occurs.

#### Scenario: Unconfirmed rollback is rejected

r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation.required]
- GIVEN a user or agent requests rollback to a checkpoint
- AND no approval decision authorizes that rollback target
- WHEN rollback validation runs
- THEN Animus rejects the rollback before invoking the checkpoint service
- AND records an unconfirmed rollback status.

### Requirement: Rollback freshness verification

r[agent_harness_runtime.clankers_checkpoint_rollback.freshness] Rollback execution MUST verify that the current workspace identity matches the expected rollback precondition before restoring checkpoint state.

#### Scenario: Stale workspace blocks rollback

r[agent_harness_runtime.clankers_checkpoint_rollback.freshness.stale]
- GIVEN the workspace changed after rollback preflight selected a checkpoint
- WHEN rollback execution validates current state
- THEN Animus rejects the rollback with a stale-workspace failure
- AND no restore operation is invoked.

### Requirement: Safe checkpoint receipts

r[agent_harness_runtime.clankers_checkpoint_rollback.receipts] Checkpoint and rollback receipts MUST include safe metadata such as operation, status, backend kind, safe workspace label or hash, checkpoint ID, changed-file counts, confirmation status, before/after BLAKE3 hashes, and error class, and MUST NOT include raw diffs, file contents, credentials, environment variables, or full path dumps.

#### Scenario: Receipt leak rejected

r[agent_harness_runtime.clankers_checkpoint_rollback.receipts.rejects_leaks]
- GIVEN a checkpoint receipt fixture contains a raw diff, file body, credential-looking value, environment dump, or full absolute path list
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Checkpoint rollback verification

r[agent_harness_runtime.clankers_checkpoint_rollback.verification] The adaptation MUST include focused positive and negative tests for checkpoint DTOs, strict and best-effort policy behavior, missing services, confirmation, freshness checks, unsupported backends, rollback receipts, and leak rejection.

#### Scenario: Closeout evidence covers checkpoint fixtures

r[agent_harness_runtime.clankers_checkpoint_rollback.verification.closeout]
- GIVEN maintainers intend to archive the checkpoint rollback adaptation
- WHEN closeout validation runs
- THEN evidence names focused DTO, service, confirmation, freshness, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
