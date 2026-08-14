# Agent Harness Runtime

## ADDED Requirements

### Requirement: Steel skill modules bind reviewed source and exports

r[agent_harness_runtime.steel_skill_modules.descriptor] Animus MUST represent an optional Steel skill module with exact manifest, frozen-source, language-cohort, source-body, export, dependency, capability, trust, bounds, and domain-separated BLAKE3 identities.

#### Scenario: A trusted module descriptor is valid

r[agent_harness_runtime.steel_skill_modules.descriptor.valid]
- GIVEN a selected skill binds one complete bounded Steel source unit, named exports, exact dependencies, and an admitted capability ceiling
- WHEN descriptor validation runs
- THEN it MUST return a stable module identity and safe summary
- AND the descriptor alone MUST NOT evaluate source or grant host authority.

### Requirement: Module admission precedes Steel evaluation

r[agent_harness_runtime.steel_skill_modules.admission] The host MUST validate source identity, trust, language cohort, syntax class, imports, exports, bounds, and effective policy before staged module evaluation.

#### Scenario: A module requests a native import

r[agent_harness_runtime.steel_skill_modules.admission.native_denied]
- GIVEN a module imports an unreviewed native module, dynamic library, arbitrary path, or package-manager reference
- WHEN module admission runs
- THEN admission MUST reject the module before evaluation
- AND it MUST NOT search the host, filesystem, network, environment, or package registry.

### Requirement: Module dependencies resolve deterministically

r[agent_harness_runtime.steel_skill_modules.dependencies] Module dependency resolution MUST be pure over supplied descriptors and MUST reject cycles, missing identities, duplicate modules, version conflicts, export conflicts, trust violations, capability widening, and cumulative bound overflow.

#### Scenario: Two modules export the same name

r[agent_harness_runtime.steel_skill_modules.dependencies.export_conflict]
- GIVEN two selected modules export one name without an explicit reviewed alias
- WHEN dependency resolution runs
- THEN resolution MUST fail with a deterministic conflict
- AND no module may enter the staged kernel.

### Requirement: Skill effects use the normal Steel bridge

r[agent_harness_runtime.steel_skill_modules.authority] Steel skill code MUST request host effects only through existing resumable bridge operations under the intersection of session, profile, skill, and current host capabilities.

#### Scenario: A skill requests an undeclared tool

r[agent_harness_runtime.steel_skill_modules.authority.tool_denied]
- GIVEN a loaded skill calls a named tool outside its declared or effective capability set
- WHEN bridge admission processes the call
- THEN it MUST deny the call before executor dispatch
- AND the skill MUST NOT register a new host operation or use an ambient fallback.

### Requirement: Module lifecycle uses kernel safe points

r[agent_harness_runtime.steel_skill_modules.lifecycle] Module loads and removals MUST occur only through explicit staged kernel transitions, and committed module identities MUST survive compaction and bind durable effect-disabled replay.

#### Scenario: Module initialization fails

r[agent_harness_runtime.steel_skill_modules.lifecycle.atomic]
- GIVEN a selected module fails during staged initialization
- WHEN the loader reaches the error
- THEN it MUST discard all staged definitions from that load operation
- AND the prior committed kernel and module set MUST remain authoritative.

### Requirement: Steel skill module evidence covers unsafe paths

r[agent_harness_runtime.steel_skill_modules.verification] Module support MUST include positive and negative production-entrypoint tests for descriptors, policy, dependency order, exports, bridge use, safe points, compaction, restore, untrusted source, drift, cycles, conflicts, native imports, capability widening, bounds, partial loads, and leaks.

#### Scenario: The module change is ready to archive

r[agent_harness_runtime.steel_skill_modules.verification.closeout]
- GIVEN maintainers intend to sync and archive Steel skill modules
- WHEN focused and lifecycle validation runs
- THEN skill, source, profile, Steel, bridge, restore, receipt, dependency, Cairn, and relevant Nix checks MUST pass
- AND native modules, package installation, automatic activation, skill correctness, task success, and production readiness MUST remain non-claims.
