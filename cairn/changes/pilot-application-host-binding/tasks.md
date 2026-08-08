## Phase 1: Baseline and upstream admission

- [x] [serial] I1 Record baseline fake-backed headless, session authority profile, model runtime profile, tool admission, safe receipt, component catalog, and lifecycle results. r[agent_harness_runtime.application_host_binding_pilot.verification]
- [x] [depends:kamacite-application-host-binding-accepted] I2 Freeze the accepted Kamacite binding schema identity, producer revision, canonical fixture bytes, validation receipt, and required non-claims before implementation. r[agent_harness_runtime.application_host_binding_pilot.kamacite_contract]
- [x] [depends:agent_harness_runtime.application_host_binding_pilot.kamacite_contract] I3 Document the exact mapping from existing Animus application, entrypoint, schema, effect, session profile, adapter, resource, and policy identities into the Kamacite fields. r[agent_harness_runtime.application_host_binding_pilot.projection]

## Phase 2: Pure projection and pilot integration

- [x] [depends:agent_harness_runtime.application_host_binding_pilot.kamacite_contract] I4 Add authority-free projection DTOs and pure canonical validation helpers in the smallest reusable crate that preserves current dependency boundaries. r[agent_harness_runtime.application_host_binding_pilot.projection] r[agent_harness_runtime.application_host_binding_pilot.boundary]
- [x] [depends:agent_harness_runtime.application_host_binding_pilot.projection] I5 Add one closed symbolic effect summary for the deterministic fake-backed entrypoint and reject open, unknown, undeclared, or stale effects. r[agent_harness_runtime.application_host_binding_pilot.effects]
- [x] [depends:agent_harness_runtime.application_host_binding_pilot.projection] I6 Bind exactly one existing session runtime authority profile, adapter cohort, and resource profile without copying their policy bodies. r[agent_harness_runtime.application_host_binding_pilot.host_selection]
- [x] [depends:agent_harness_runtime.application_host_binding_pilot.host_selection] I7 Commit the pilot binding, consume the accepted runtime-authority-unit seal, and then permit dependent model or tool effects while retaining every current enforcement recheck. r[agent_harness_runtime.application_host_binding_pilot.pre_effect_binding] r[agent_harness_runtime.application_host_binding_pilot.current_enforcement]
- [x] [depends:agent_harness_runtime.application_host_binding_pilot.projection] I8 Add a read-only safe explanation projection for exact binding identities, effect classes, selected host, cohorts, compatibility state, and non-claims. r[agent_harness_runtime.application_host_binding_pilot.explain]

## Phase 3: Conformance and closeout

- [x] [parallel] V1 Add positive tests for canonical projection, one host, closed effects, metadata stability, pre-effect binding and seal order, current rechecks, and safe explanation output. r[agent_harness_runtime.application_host_binding_pilot.verification]
- [x] [parallel] V2 Add negative tests for missing or multiple hosts, unknown effects, undeclared operations, stale schema/profile/cohort identities, missing or widened seals, post-effect binding, unsafe output, and authority overclaims. r[agent_harness_runtime.application_host_binding_pilot.verification] r[agent_harness_runtime.application_host_binding_pilot.boundary]
- [ ] [depends:agent_harness_runtime.application_host_binding_pilot.kamacite_contract] V3 Add `scripts/cairn/check-application-host-binding --root .` to validate frozen Kamacite conformance, fixture freshness, safe output, and non-claims without live provider or network access. r[agent_harness_runtime.application_host_binding_pilot.kamacite_contract] r[agent_harness_runtime.application_host_binding_pilot.verification]
- [x] [serial] I9 Update contracts, component, facade, conformance, and README guidance with the Roc design reference and explicit non-adoption boundaries. r[agent_harness_runtime.application_host_binding_pilot.explain] r[agent_harness_runtime.application_host_binding_pilot.boundary]
- [ ] [depends:agent_harness_runtime.application_host_binding_pilot.verification] V4 Run focused baseline/post-change tests, the new conformance rail, Nickel freshness, Wasm and purity checks, workspace checks, Clippy, Cairn validation, gates, and relevant Nix checks. r[agent_harness_runtime.application_host_binding_pilot.verification]

## Coordination status

The Animus producer projection is implemented. V3 waits for Kamacite to freeze and validate the exact Animus producer revision. V4 waits for that cross-repository conformance receipt.
