# Tasks

- [x] [serial] Inventory large adapter modules, mechanical shard names, broad lint allowances, cross-family coupling, public exports, and conformance coverage. r[agent_harness_runtime.adapter_crate_maintainability.inventory]
- [x] [depends:inventory] Publish an adapter module ownership map that links semantic module names to adapter family, requirement IDs, public exports, and conformance cases. r[agent_harness_runtime.adapter_crate_maintainability.ownership_map]
- [x] [depends:ownership_map] Rename mechanical shards to semantic module names with public re-export compatibility or documented migration aliases. r[agent_harness_runtime.adapter_crate_maintainability.semantic_shards]
- [x] [depends:semantic_shards] Split large modules into pure validators/projectors/reducers/receipt builders and thin orchestration shells without changing behavior. r[agent_harness_runtime.adapter_crate_maintainability.functional_core]
- [x] [depends:functional_core] Remove broad lint allowances or narrow them to local justified allowances tied to the remaining exception. r[agent_harness_runtime.adapter_crate_maintainability.lint_scope]
- [x] [depends:lint_scope] Run adapter conformance, dogfood readiness, targeted unit/doc tests, and lint/check gates; document any pre-existing failures and migration aliases. r[agent_harness_runtime.adapter_crate_maintainability.verification]
