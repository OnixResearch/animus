## Phase 1: Cohort and adapter boundary

- [ ] [serial] Select and record one immutable Kamacite application-host binding, semantic operation, receipt, codec, and license cohort. r[agent_harness_runtime.kamacite_live_binding.cohort]
- [ ] [depends:kamacite-live-binding-cohort] Define Animus-owned nominal binding and operation DTOs plus a narrow canonical artifact adapter. r[agent_harness_runtime.kamacite_live_binding.adapter]
- [ ] [depends:kamacite-live-binding-cohort] Add positive and negative canonical artifact fixtures for host, effect, adapter, resource, policy, and schema identities. r[agent_harness_runtime.kamacite_live_binding.cohort]

## Phase 2: Real composition path

- [ ] [serial] Bind the exact `animus-live-steel-agent` application, entrypoint, input schema, output schema, host, adapter, resource, and policy identities. r[agent_harness_runtime.kamacite_live_binding.composition]
- [ ] [depends:kamacite-binding-composition] Map selected model and tool requirements to exact Kamacite `effect-operation` identities and reject name-only mappings. r[agent_harness_runtime.kamacite_live_binding.effects]
- [ ] [depends:kamacite-binding-composition] Commit binding admission before the existing runtime authority seal and preserve every current host recheck. r[agent_harness_runtime.kamacite_live_binding.preeffect]
- [ ] [depends:kamacite-binding-composition] Add typed non-default configuration with explicit legacy selection and no silent fallback. r[agent_harness_runtime.kamacite_live_binding.rollout]

## Phase 3: Evidence and validation

- [ ] [parallel] Add safe binding and operation identity fields to selected metadata-only runtime receipts. r[agent_harness_runtime.kamacite_live_binding.evidence]
- [ ] [parallel] Add negative stale host, adapter, resource, policy, operation, receipt, open-effect, secret-leak, and authority-bypass fixtures. r[agent_harness_runtime.kamacite_live_binding.verification]
- [ ] [depends:kamacite-binding-composition] Exercise the real packaged composition with controlled model and tool adapters through positive and denied paths. r[agent_harness_runtime.kamacite_live_binding.verification]

## Phase 4: Documentation and gates

- [ ] [serial] Document the composition root, rollout, removal, current authority gates, privacy boundary, and non-claims. r[agent_harness_runtime.kamacite_live_binding.rollout] r[agent_harness_runtime.kamacite_live_binding.evidence]
- [ ] [depends:kamacite-live-binding-verification] Run focused adapter, runtime, host, Preserves, Steel, Cairn, Clippy, Octet, and Nix checks. r[agent_harness_runtime.kamacite_live_binding.verification]
