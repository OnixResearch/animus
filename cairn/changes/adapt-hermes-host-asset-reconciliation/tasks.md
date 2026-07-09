# Tasks

- [ ] [serial] Define host asset manifest, installed observation, drift report, reconciliation plan, mutation request, and safe receipt DTOs. r[agent_harness_runtime.hermes_asset_reconcile.inventory]
- [ ] [depends:inventory] Add pure drift classification over safe asset identities, store labels, trust, enablement, content hashes, and freshness metadata. r[agent_harness_runtime.hermes_asset_reconcile.drift]
- [ ] [depends:drift] Add dry-run reconciliation planning and explicit mutation request validation without filesystem traversal or symlink handling. r[agent_harness_runtime.hermes_asset_reconcile.plan]
- [ ] [depends:plan] Add host-owned boundaries for discovery, path resolution, copy/link/unlink, enable/disable, plugin install, and manifest writes. r[agent_harness_runtime.hermes_asset_reconcile.host_owned]
- [ ] [depends:host_owned] Add positive and negative tests for manifests, drift classes, dry-run plans, mutation requests, stale observations, no global paths, and receipt leak rejection. r[agent_harness_runtime.hermes_asset_reconcile.verification]
