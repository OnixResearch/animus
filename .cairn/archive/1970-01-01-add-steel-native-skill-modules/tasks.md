# Tasks

## B. Descriptor and policy

- [x] [serial] B1: Inventory existing skill manifest, source lifecycle, runtime profile, and Steel module boundaries. r[agent_harness_runtime.steel_skill_modules.verification]
- [x] [serial] B2: Define the module descriptor, source identity, exports, dependencies, capability requests, trust, bounds, and safe receipt DTOs. r[agent_harness_runtime.steel_skill_modules.descriptor]
- [x] [depends:B2] B3: Add typed Nickel policy and deterministic Rust revalidation for the reviewed module cohort. r[agent_harness_runtime.steel_skill_modules.admission]

## M. Resolution and loading

- [x] [depends:B3] M1: Implement pure dependency ordering with cycle, duplicate, version, export, trust, capability, and bound rejection. r[agent_harness_runtime.steel_skill_modules.dependencies]
- [x] [depends:M1] M2: Add staged Steel source validation and module evaluation without native or ambient imports. r[agent_harness_runtime.steel_skill_modules.admission]
- [x] [depends:M2] M3: Bind declared host operations to the resumable bridge under effective session and skill capability intersections. r[agent_harness_runtime.steel_skill_modules.authority]
- [x] [depends:M3] M4: Commit modules only at safe points and bind module identities into compaction and durable restore. r[agent_harness_runtime.steel_skill_modules.lifecycle]
- [x] [depends:M4] M5: Integrate explicit module selection with model runtime profiles and progressive disclosure. r[agent_harness_runtime.steel_skill_modules.lifecycle]

## T. Tests and closeout

- [x] [depends:M5] T1: Add positive tests for selection, dependency order, exports, helper reuse, bridge calls, compaction, and restore. r[agent_harness_runtime.steel_skill_modules.verification]
- [x] [depends:M5] T2: Add negative tests for untrusted source, identity drift, cycles, conflicts, native imports, ambient paths, capability widening, automatic activation, bound overflow, partial load, restore drift, and receipt leaks. r[agent_harness_runtime.steel_skill_modules.verification]
- [x] [depends:T2] T3: Prove default, no-std, and Wasm dependency isolation for the descriptor surface. r[agent_harness_runtime.steel_skill_modules.authority]
- [x] [depends:T3] T4: Update docs and component metadata, add a focused rail, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.steel_skill_modules.verification]
