# Proposal: Add Steel-native skill modules

## Why

Animus skills are validated manifests and progressively disclosed instruction bodies. They do not provide callable program modules for the model-facing Steel kernel.

Prime Agent skills can supply programmatic Python helpers. A Steel agent needs the equivalent mechanism without arbitrary native module loading or ambient host authority.

## What Changes

- Add a skill-module descriptor that binds one validated skill manifest to versioned Steel source, exports, dependencies, capabilities, and BLAKE3 identity. r[agent_harness_runtime.steel_skill_modules.descriptor]
- Validate and compile selected modules before they enter a model-facing kernel. r[agent_harness_runtime.steel_skill_modules.admission]
- Resolve module dependencies deterministically and reject cycles, conflicts, unreviewed natives, and authority widening. r[agent_harness_runtime.steel_skill_modules.dependencies]
- Permit host effects only through the resumable Steel bridge and declared capability intersections. r[agent_harness_runtime.steel_skill_modules.authority]
- Load modules at explicit kernel safe points and preserve their identities through compaction and durable restore. r[agent_harness_runtime.steel_skill_modules.lifecycle]
- Add positive and negative module, source, dependency, authority, restore, and leak tests. r[agent_harness_runtime.steel_skill_modules.verification]

## Prerequisites

- `add-resumable-steel-host-bridge`
- `add-durable-steel-kernel-restore`
- `agent_harness_runtime.hermes_skill_manifest.descriptor`
- `agent_harness_runtime.hermes_skill_manifest.progressive_disclosure`
- `agent_harness_runtime.skill_source_lifecycle.frozen`
- `agent_harness_runtime.model_runtime_profiles.dispatch_binding.retry`

## Impact

- **Crates**: a focused no-authority skill-module contract plus `animus-steel` loader support and host source services.
- **Policy**: typed Nickel owns allowed source forms, imports, exports, bounds, trust, capability labels, and native-module denial.
- **Kernel**: selected modules become reviewed helper definitions. They do not become host plugins.
- **Evidence**: receipts bind module identities and safe counts only.

## Out of Scope

- Arbitrary Rust FFI, dynamic libraries, package-manager installation, network fetch during load, unreviewed native Steel modules, or automatic skill activation.
- Skill correctness, host-effect safety beyond admitted components, task success, or production readiness.

## Affected Specs

- `agent-harness-runtime`: module descriptors, admission, dependencies, authority, lifecycle, and verification.
