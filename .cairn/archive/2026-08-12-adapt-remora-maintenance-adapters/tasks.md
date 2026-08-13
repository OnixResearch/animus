## Phase 1: Baseline and ownership

- [x] [serial] B1 Record the current review-contract baseline and inventory existing schedule, queue, orchestration, review, quality, model-evidence, validation, workspace, VCS, and receipt owners. r[agent_harness_runtime.remora_adapters.composition] r[agent_harness_runtime.remora_adapters.verification]
- [x] [serial] B2 Record the exact Remora repository, revision, license declaration, imported scope, owner-grant basis, and rejected shell behavior. r[agent_harness_runtime.remora_adapters.provenance]

## Phase 2: Focused adapter crates

- [x] [depends:agent_harness_runtime.remora_adapters.change_request] C1 Add `animus-adapter-change-request` with forge-neutral repository, capability, change-request, diff, review, CI-failure, effect-plan, and receipt contracts. r[agent_harness_runtime.remora_adapters.change_request]
- [x] [depends:agent_harness_runtime.remora_adapters.maintenance] C2 Add `animus-adapter-maintenance` with dependency inventory, candidate, advisory, policy, bounds, grouping, update-plan, effect-plan, and receipt contracts. r[agent_harness_runtime.remora_adapters.maintenance]
- [x] [depends:agent_harness_runtime.remora_adapters.ports] C3 Add separate inventory, candidate, advisory, workspace, validation, change-request source, review analysis, change-request publication, and review publication ports. r[agent_harness_runtime.remora_adapters.ports]

## Phase 3: Deterministic planning and binding

- [x] [depends:agent_harness_runtime.remora_adapters.maintenance] I1 Add pure update planning with bounded groups, security and major isolation, ecosystem and manifest compatibility, human-review decisions, validation-profile requests, and neutral change-request drafts. r[agent_harness_runtime.remora_adapters.maintenance]
- [x] [depends:agent_harness_runtime.remora_adapters.change_request] I2 Add pure review planning with diff bounds, confidence policy, observed inline anchors, summary fallback, CI-failure triage, and typed escalation. r[agent_harness_runtime.remora_adapters.change_request]
- [x] [depends:agent_harness_runtime.remora_adapters.binding] I3 Add exact request and observation binding plus typed unavailable, denied, unsupported, cancelled, timed-out, malformed, stale, mismatch, overflow, and infrastructure-failure outcomes. r[agent_harness_runtime.remora_adapters.binding]

## Phase 4: Composition and evidence

- [x] [depends:agent_harness_runtime.remora_adapters.composition] E1 Add optional compatibility re-exports through `animus-adapters` and root `animus` without default dependency leakage. r[agent_harness_runtime.remora_adapters.composition]
- [x] [depends:agent_harness_runtime.remora_adapters.evidence] E2 Add BLAKE3-bound metadata-only receipts that reject raw dependency, diff, command, path, credential, provider, and unsupported claim content. r[agent_harness_runtime.remora_adapters.evidence]
- [x] [depends:agent_harness_runtime.remora_adapters.composition] E3 Update component, contract, facade, request-lifecycle, safety, README, and reference documentation with exact owner and non-claim boundaries. r[agent_harness_runtime.remora_adapters.composition] r[agent_harness_runtime.remora_adapters.provenance]

## Phase 5: Positive and negative verification

- [x] [parallel] T1 Add positive tests for independent port subsets, routine grouping, isolated security and major updates, exact service observations, anchored review comments, summary fallback, and safe receipts. r[agent_harness_runtime.remora_adapters.verification]
- [x] [parallel] T2 Add negative tests for missing services, zero bounds, mixed ecosystems or manifests, stale generations, cross-repository replay, unsafe validation commands, missing anchors, exhausted repair attempts, receipt leaks, and overclaims. r[agent_harness_runtime.remora_adapters.binding] r[agent_harness_runtime.remora_adapters.verification]
- [x] [parallel] T3 Add no-std, Wasm, dependency-isolation, facade-isolation, source-provenance, component-catalog, and static authority checks. r[agent_harness_runtime.remora_adapters.provenance] r[agent_harness_runtime.remora_adapters.verification]

## Phase 6: Closeout

- [x] [depends:agent_harness_runtime.remora_adapters.verification] D1 Add and run `scripts/cairn/check-remora-adapter-families --root .` plus focused Rust, formatting, Clippy, Wasm, facade, component, Cairn, and Nix checks. r[agent_harness_runtime.remora_adapters.verification]
- [x] [depends:agent_harness_runtime.remora_adapters.verification] D2 Review sync and archive plans, make sure that accepted requirements contain every new ID, archive the completed change, and record exact evidence and non-claims. r[agent_harness_runtime.remora_adapters.verification]
