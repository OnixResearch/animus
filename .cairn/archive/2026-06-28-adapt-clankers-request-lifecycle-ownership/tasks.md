# Tasks

- [x] [serial] Add `docs/request-lifecycle.md` with the Animus golden path from runtime input through core state, Steel decision, adapter dispatch, report/event projection, safe receipts, and host ledger seams. r[agent_harness_runtime.clankers_lifecycle_doc.map]
- [x] [depends:clankers_lifecycle_doc.map] Name the owning crate and source-of-truth type or trait for every documented transition. r[agent_harness_runtime.clankers_lifecycle_doc.owners]
- [x] [depends:clankers_lifecycle_doc.owners] Document forbidden dependencies and side effects for core, adapter, Steel, std-shell, receipt, and future ledger boundaries. r[agent_harness_runtime.clankers_lifecycle_doc.forbidden_dependencies]
- [x] [depends:clankers_lifecycle_doc.map] Link the lifecycle document from README and related runtime/provenance docs. r[agent_harness_runtime.clankers_lifecycle_doc.links]
- [x] [depends:clankers_lifecycle_doc.forbidden_dependencies] Add a focused lifecycle-doc marker rail that checks boundary names, owners, forbidden dependency sections, and validation command references. r[agent_harness_runtime.clankers_lifecycle_doc.rail]
- [x] [depends:clankers_lifecycle_doc.rail] Run the focused rail, docs checks if available, workspace tests if touched, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_lifecycle_doc.closeout]
