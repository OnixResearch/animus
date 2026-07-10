# agent-harness-runtime Change Spec: Split the runtime kernel from feature packs

## Requirements

### Requirement: Minimal runtime kernel boundary

r[agent_harness_runtime.runtime_kernel_split.kernel] Animus MUST define a minimal runtime kernel boundary that owns typed runtime input/config, neutral loop-fabric orchestration, turn state transitions, runtime events, run reports, capability registry resolution, model dispatch planning, tool dispatch planning, and injected adapter orchestration without depending on Steel, std shell, provider clients, terminal UI, concrete storage, filesystem, process, network, credential, clock, or persistence authority.

#### Scenario: Kernel build excludes optional domains

r[agent_harness_runtime.runtime_kernel_split.kernel.minimal]
- GIVEN an embedder needs only the reusable agent-loop kernel
- WHEN they build the kernel-only profile or crate
- THEN the build excludes Steel, headless protocol, provenance packs, worker teams, batch, compaction, dataset export, research/eval, TUI, std shell, concrete providers, concrete tools, and persistence
- AND model/tool calls remain injected adapter requests rather than ambient host discovery.

### Requirement: Runtime feature packs are explicit Lego blocks

r[agent_harness_runtime.runtime_kernel_split.packs] Optional runtime domains MUST be represented as explicit feature packs or focused modules with cataloged owner, dependency, composition, feature, authority, and focused-check metadata.

#### Scenario: Embedder selects one runtime pack

r[agent_harness_runtime.runtime_kernel_split.packs.selective]
- GIVEN an embedder wants only headless protocol, provenance, worker-team delegation, hooks/policy, batch, compaction, dataset export, quality, research, scaffold, status, or orchestration behavior
- WHEN they inspect the component catalog and runtime composition docs
- THEN they can identify the smallest runtime pack, required dependencies, compatible preludes, feature gate, and focused validation rail
- AND unrelated runtime packs are not implied by that selection.

### Requirement: Runtime compatibility facade remains stable

r[agent_harness_runtime.runtime_kernel_split.compat_facade] `animus-runtime` MUST preserve existing flat facade exports during the kernel/pack migration and MUST provide migration guidance before removing or silently renaming any public runtime item.

#### Scenario: Existing runtime consumers remain source-compatible

r[agent_harness_runtime.runtime_kernel_split.compat_facade.existing]
- GIVEN downstream code imports existing `animus_runtime::*` facade items
- WHEN kernel and feature-pack boundaries are introduced
- THEN representative existing imports continue to compile through compatibility exports
- AND docs name the smaller direct imports preferred for new code.

### Requirement: Runtime kernel split verification

r[agent_harness_runtime.runtime_kernel_split.verification] The runtime kernel split MUST include focused positive and negative tests for kernel-only builds, feature-pack builds, facade compatibility, catalog metadata, composition conformance, and forbidden-authority leakage.

#### Scenario: Closeout evidence proves the split boundary

r[agent_harness_runtime.runtime_kernel_split.verification.closeout]
- GIVEN maintainers intend to archive the runtime kernel split
- WHEN closeout validation runs
- THEN evidence names kernel-only checks, feature-pack checks, facade compatibility, catalog checks, composition conformance, runtime facade inventory, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
