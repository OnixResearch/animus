# Tasks

- [x] [serial] Add roadmap documentation that classifies active changes as foundational, core-runtime, host-adapter, optional-reference, product-surface, or evaluation-only work. r[agent_harness_runtime.roadmap_sequence.taxonomy]
- [x] [depends:taxonomy] Define prerequisite tiers for baseline specs, policy wiring, runtime loop, injected services, receipts/provenance, session ledger, events/hooks, policy/guidance, concrete adapters, UI/headless, and export surfaces. r[agent_harness_runtime.roadmap_sequence.dependency_order]
- [x] [depends:dependency_order] Add consolidation guidance for overlapping ledger, receipt, provider, hook, manifest, and tool-catalog changes. r[agent_harness_runtime.roadmap_sequence.overlap_resolution]
- [x] [depends:dependency_order] Add a focused roadmap fixture or static check that reports unknown change IDs, unknown prerequisite IDs, cycles, and unclassified active changes. r[agent_harness_runtime.roadmap_sequence.validation]
- [x] [depends:validation] Add negative fixtures for cycle detection, missing prerequisite IDs, and product-surface changes that bypass foundational safety contracts. r[agent_harness_runtime.roadmap_sequence.validation.negative]
- [x] [depends:taxonomy,dependency_order,overlap_resolution,validation.negative] Run lifecycle validation, roadmap checks, and Cairn proposal/design/tasks gates with reviewable evidence. r[agent_harness_runtime.roadmap_sequence.closeout]
