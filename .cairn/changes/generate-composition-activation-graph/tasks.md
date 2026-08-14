# Tasks

## B. Baseline and source review

- [ ] [serial] B1: Record DeepSeek Harness revision `47f943859bef60e4160492346772ded9b24f765a`, MIT license, developer-preview status, capability-seam and event-map concepts, and rejected Cordis/plugin authority concepts. r[agent_harness_runtime.composition_activation_graph.catalog]
- [ ] [serial] B2: Run and record component-catalog, runtime-facade, request-lifecycle, injected-service, composition-conformance, and evidence-safety baselines. r[agent_harness_runtime.composition_activation_graph.verification]

## C. Catalog and projections

- [ ] [depends:B1] C1: Extend `.cairn/component-catalog.ncl` contracts with typed port, provider, consumer, scope, multiplicity, lifecycle, event, delivery, durability, activation, feature, authority, and focused-check records. r[agent_harness_runtime.composition_activation_graph.catalog]
- [ ] [depends:C1] C2: Extend catalog validation for unknown references, orphan required ports, undeclared consumers, invalid multiplicity, authority mismatch, missing lifecycle owners, unsupported event cycles, and unsafe evidence fields. r[agent_harness_runtime.composition_activation_graph.catalog]
- [ ] [depends:C2] C3: Generate component dependency, capability-seam, and event-ownership maps from the Nickel source and reject projection drift. r[agent_harness_runtime.composition_activation_graph.projections]

## P. Activation preflight

- [ ] [depends:C2] P1: Add pure selected-composition, readiness-observation, activation-decision, issue, graph-identity, and safe-snapshot DTOs. r[agent_harness_runtime.composition_activation_graph.preflight] r[agent_harness_runtime.composition_activation_graph.inspection]
- [ ] [depends:P1] P2: Implement deterministic `ready`, `degraded_optional`, and `blocked` preflight decisions with no host effects or fallback. r[agent_harness_runtime.composition_activation_graph.preflight]
- [ ] [depends:P2] P3: Integrate preflight at selected std composition roots before store, credential, provider, process, filesystem, network, terminal, daemon, or ready-endpoint effects. r[agent_harness_runtime.composition_activation_graph.fail_loud]
- [ ] [depends:P3] P4: Add bounded metadata-only activation inspection projection and a mapping fixture for the pending Host Control inspection envelope without taking an unpublished runtime dependency. r[agent_harness_runtime.composition_activation_graph.inspection]

## V. Verification and closeout

- [ ] [depends:P3] V1: Add positive fixtures for complete required closure, valid optional degradation, declared multiplicity, bounded event cycles, and ready composition roots. r[agent_harness_runtime.composition_activation_graph.verification]
- [ ] [depends:P3] V2: Add negative fixtures for missing providers, orphan ports, undeclared consumers, stale readiness, authority mismatch, missing lifecycle ownership, invalid event routes, hidden fallback, unsafe inspection, and effects before preflight. r[agent_harness_runtime.composition_activation_graph.fail_loud] r[agent_harness_runtime.composition_activation_graph.verification]
- [ ] [depends:V2] V3: Update component-catalog, composition-conformance, request-lifecycle, runtime-prelude, focused-rail, safety, live-host, and README reference documentation. r[agent_harness_runtime.composition_activation_graph.projections]
- [ ] [depends:V3] V4: Run Nickel export and drift checks, focused Rust tests, no-std and Wasm checks, formatting, Clippy, catalog and composition rails, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.composition_activation_graph.verification]
