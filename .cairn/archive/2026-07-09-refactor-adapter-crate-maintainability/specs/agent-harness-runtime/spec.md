# agent-harness-runtime Change Spec: Adapter crate maintainability

## Requirements

### Requirement: Adapter maintainability hotspots are inventoried

r[agent_harness_runtime.adapter_crate_maintainability.inventory] Animus MUST maintain an inventory of large adapter modules, mechanical shard names, broad lint allowances, cross-family coupling hotspots, public exports, and conformance coverage before behavior-preserving adapter refactors.

#### Scenario: Large module appears without owner

r[agent_harness_runtime.adapter_crate_maintainability.inventory.large_module]
- GIVEN an adapter module exceeds the locally documented size or ownership threshold
- WHEN maintainability validation runs
- THEN the inventory names its adapter family, public exports, requirement IDs, conformance cases, and refactor disposition
- AND unowned hotspots block maintainability closeout.

### Requirement: Adapter modules have a semantic ownership map

r[agent_harness_runtime.adapter_crate_maintainability.ownership_map] Adapter crate module boundaries MUST be documented by semantic adapter family with links to requirement IDs, public exports, conformance cases, and known host-owned/non-default surfaces.

#### Scenario: New adapter family boundary is introduced

r[agent_harness_runtime.adapter_crate_maintainability.ownership_map.new_family]
- GIVEN a refactor introduces a new adapter-family module boundary
- WHEN the ownership map is updated
- THEN it records owned requirement IDs, conformance index entries, public exports, and non-claims
- AND reviewers can find the family without following mechanical shard names.

### Requirement: Mechanical shards use semantic names or compatibility aliases

r[agent_harness_runtime.adapter_crate_maintainability.semantic_shards] Mechanically named adapter shards SHOULD be renamed to semantic module names when behavior-preserving migration is possible, and public compatibility aliases or migration notes MUST be provided when downstream imports may be affected.

#### Scenario: Mechanical shard is renamed

r[agent_harness_runtime.adapter_crate_maintainability.semantic_shards.rename]
- GIVEN a mechanical module shard is renamed to a semantic adapter-family name
- WHEN public exports are checked
- THEN existing public import paths remain available through aliases or documented migration
- AND conformance indexes continue to point at the semantic owner.

### Requirement: Large adapter modules split functional core from shell

r[agent_harness_runtime.adapter_crate_maintainability.functional_core] Non-trivial adapter modules SHOULD split pure validation, projection, reduction, and receipt-building logic from orchestration shells that perform host or service wiring.

#### Scenario: Receipt builder is extracted

r[agent_harness_runtime.adapter_crate_maintainability.functional_core.receipt_builder]
- GIVEN a large module contains both receipt-building rules and shell orchestration
- WHEN the module is refactored
- THEN receipt construction can be tested as a pure function over typed inputs
- AND orchestration code remains a thin shell that supplies inputs and propagates typed outcomes.

### Requirement: Lint allowances are narrow and justified

r[agent_harness_runtime.adapter_crate_maintainability.lint_scope] Adapter crate lint allowances MUST be removed when unused or narrowed to local scopes with comments that justify the remaining exception.

#### Scenario: Broad allow hides new warning

r[agent_harness_runtime.adapter_crate_maintainability.lint_scope.broad_allow]
- GIVEN a crate-level or module-level broad lint allowance suppresses warnings across unrelated adapter families
- WHEN maintainability refactor runs
- THEN the allowance is removed or replaced with local justified allowances
- AND unrelated warning suppression no longer applies.

### Requirement: Adapter maintainability verification preserves behavior

r[agent_harness_runtime.adapter_crate_maintainability.verification] Adapter maintainability refactors MUST run adapter conformance, dogfood readiness, targeted unit/doc tests, public export checks, and relevant lint/check gates before archive, and MUST document any pre-existing failures or compatibility aliases.

#### Scenario: Closeout records no behavior claim

r[agent_harness_runtime.adapter_crate_maintainability.verification.closeout]
- GIVEN maintainers prepare to archive adapter maintainability work
- WHEN closeout evidence is collected
- THEN it names inventory, ownership map, semantic shard changes, functional-core splits, lint-scope changes, compatibility aliases, conformance evidence, dogfood readiness, and Cairn gates
- AND it states that no new provider client, tool executor, persistence backend, dogfood shell behavior, or product feature was claimed.
