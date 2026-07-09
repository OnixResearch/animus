# agent-harness-runtime Change Spec: Hermes host asset reconciliation contracts

## Requirements

### Requirement: Host asset reconciliation uses typed inventories

r[agent_harness_runtime.hermes_asset_reconcile.inventory] Animus MUST represent host asset reconciliation with typed manifests, installed observations, drift reports, reconciliation plans, mutation requests, and safe receipts for skills, resources, plugins, tool catalogs, policy profiles, prompt profiles, and other optional host assets.

#### Scenario: Non-Hermes host supplies asset observations

r[agent_harness_runtime.hermes_asset_reconcile.inventory.non_hermes]
- GIVEN a host supplies versioned asset manifests and installed observations from an application database or embedded bundle
- WHEN inventory validation runs
- THEN Animus accepts safe asset IDs, store labels, enablement state, trust state, and BLAKE3 content identities
- AND it does not require `~/.hermes`, `~/.pi`, symlinks, repository mirrors, or global config directories.

### Requirement: Drift classification is pure and store-neutral

r[agent_harness_runtime.hermes_asset_reconcile.drift] Reconciliation MUST classify supplied manifest and installed observations deterministically using safe drift classes such as installed-matches-manifest, installed-disabled, installed-drifted, host-only, manifest-only, identity-mismatch, duplicate-asset, stale-observation, and unsupported-store.

#### Scenario: Installed asset hash differs from manifest hash

r[agent_harness_runtime.hermes_asset_reconcile.drift.identity_mismatch]
- GIVEN a trusted manifest entry and a host-installed observation share an asset ID but have different content identities
- WHEN drift classification runs
- THEN the report records an identity-mismatch diagnostic with safe hashes and store labels
- AND it does not read files, resolve links, or choose which version is authoritative.

### Requirement: Reconciliation plans are dry-run unless host admits mutation

r[agent_harness_runtime.hermes_asset_reconcile.plan] Reconciliation planning MUST produce dry-run action descriptors by default, and any copy, link, unlink, enable, disable, install, remove, or manifest-write operation MUST require a separate host-owned mutation request with capability and freshness validation.

#### Scenario: Manifest-only asset yields advisory install action

r[agent_harness_runtime.hermes_asset_reconcile.plan.dry_run]
- GIVEN a trusted manifest contains an asset absent from installed observations
- WHEN reconciliation planning runs
- THEN the plan may include an advisory install action with safe asset identity and required capability label
- AND no file, symlink, plugin, global config, or host store is mutated by reusable logic.

### Requirement: Asset discovery and mutation are host-owned

r[agent_harness_runtime.hermes_asset_reconcile.host_owned] Filesystem traversal, path canonicalization, symlink resolution, live host-store reads, copy/link/unlink operations, enable/disable actions, plugin installation, manifest writes, UI prompts, clocks, and persistence MUST remain host-owned behavior outside reusable runtime and adapter-contract crates.

#### Scenario: Missing reconciliation host service blocks mutation

r[agent_harness_runtime.hermes_asset_reconcile.host_owned.missing_service]
- GIVEN a reconciliation mutation request asks to enable an asset
- AND no host asset service is injected
- WHEN mutation admission runs
- THEN Animus returns a service-unavailable diagnostic
- AND it does not attempt filesystem mutation, plugin installation, symlink resolution, or fallback discovery.

### Requirement: Asset reconciliation verification

r[agent_harness_runtime.hermes_asset_reconcile.verification] The asset reconciliation adaptation MUST include focused positive and negative tests for inventory validation, drift classes, duplicate assets, dry-run plans, explicit mutation requests, stale observations, unsupported stores, global-path leak rejection, no Hermes/Pi directory dependency, and safe receipt redaction.

#### Scenario: Closeout evidence states store-neutral SDK boundary

r[agent_harness_runtime.hermes_asset_reconcile.verification.closeout]
- GIVEN maintainers intend to archive the asset reconciliation adaptation
- WHEN closeout validation runs
- THEN evidence names inventory, drift, plan, host-owned boundary, receipt, docs, Cairn validation, and Cairn gate checks
- AND it records global skill management, symlink linking, plugin installation, and host-store canonicality as non-claims.
