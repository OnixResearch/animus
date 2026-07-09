# agent-harness-runtime Change Spec: Hermes VCS snapshot ledger contracts

## Requirements

### Requirement: VCS snapshots are backend-neutral records

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.records] Animus MUST represent recovery snapshots as backend-neutral records with snapshot intent, backend kind, opaque snapshot ID, optional parent ID, workspace label or hash, content identities, changed-artifact counts, actor/session labels, freshness metadata, and safe receipt identity.

#### Scenario: jj and Git snapshots share the same record shape

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.records.backend_neutral]
- GIVEN a host reports one snapshot from a jj backend and another from a Git backend
- WHEN snapshot records are validated
- THEN both records use the same backend-neutral shape with opaque backend IDs
- AND reusable logic does not require Git stash refs, shadow branch names, jj change IDs, or backend command syntax.

### Requirement: Snapshot operation plans validate before mutation

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.plans] Snapshot accept, restore, drop, squash, compare, and cleanup operations MUST be represented as validated plans with target snapshot IDs, freshness proofs, protected-action class, confirmation requirements, expected backend capabilities, and safe diagnostics before any host mutation is requested.

#### Scenario: Restore requires fresh snapshot proof

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.plans.restore_freshness]
- GIVEN a host requests a restore plan for a snapshot whose current workspace identity no longer matches the supplied freshness proof
- WHEN restore planning runs
- THEN planning returns a stale-snapshot diagnostic
- AND no checkpoint restore, VCS command, file mutation, or success receipt is requested.

### Requirement: Snapshot storage and VCS mutation are host-owned

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.host_owned] Snapshot creation, storage, VCS command execution, branch/ref/stash management, worktree mutation, cleanup, UI selection, timers, background scheduling, and automatic per-turn snapshot policy MUST remain host-owned behavior behind injected checkpoint or snapshot services.

#### Scenario: Runtime does not autocommit after a turn

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.host_owned.no_autocommit]
- GIVEN an Animus runtime turn completes with changed artifacts
- WHEN no host snapshot service or snapshot policy is injected
- THEN no snapshot, autocommit, shadow branch, stash, jj operation, or checkpoint mutation is created
- AND the runtime reports no snapshot claim.

### Requirement: Snapshot receipts are safe

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.receipts] Snapshot receipts MUST record safe metadata such as operation status, backend kind, snapshot ID hash, workspace label or hash, changed counts, freshness class, confirmation status, policy hash, and BLAKE3 receipt hash, and MUST NOT include raw diffs, file bodies, command lines, full private paths, provider payloads, credentials, raw tool output, or unsafe backend refs.

#### Scenario: Raw diff receipt is rejected

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.receipts.rejects_raw_diff]
- GIVEN a snapshot receipt fixture includes a raw unified diff, full private path, command line, or backend secret ref
- WHEN receipt validation runs
- THEN validation rejects the receipt with a typed leak diagnostic
- AND the receipt cannot be used as recovery or release evidence.

### Requirement: Snapshot ledger verification

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.verification] The VCS snapshot ledger adaptation MUST include focused positive and negative tests for backend-neutral records, lineage, freshness, protected restore/accept/drop/squash plans, missing checkpoint service, unsupported backend capabilities, confirmation denial, no automatic snapshot/autocommit behavior, backend command rejection, and receipt leak rejection.

#### Scenario: Closeout evidence states VCS mutation non-claims

r[agent_harness_runtime.hermes_vcs_snapshot_ledger.verification.closeout]
- GIVEN maintainers intend to archive the VCS snapshot ledger adaptation
- WHEN closeout validation runs
- THEN evidence names record, plan, host-owned service, receipt, docs, Cairn validation, and Cairn gate checks
- AND it records Git shadow branches, jj operations, automatic checkpoints, autocommit, restore UI, and VCS mutation as host-owned non-claims.
