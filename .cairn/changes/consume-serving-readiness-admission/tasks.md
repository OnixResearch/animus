## Phase 1: Pure core

- [x] [serial] 1.1 Add bounded snapshots, tokens, references, and identities in `crates/animus-adapters/src/serving/{basis,facts}.rs` with positive and negative tests. r[agent_harness_runtime.serving_admission.snapshot]
- [x] [serial] 1.2 Add the profile and eligibility screening before preference, with readiness, freshness, identity, privacy, capability, envelope, capacity, hosted-quota, rotation, and preference tests. r[agent_harness_runtime.serving_admission.eligibility]
- [x] [serial] 1.3 Add the pre-dispatch gate with last-slot, stale and revoked grant, unavailable service, replacement, substitution, and replica tests. r[agent_harness_runtime.serving_admission.gate]
- [x] [serial] 1.4 Add metadata-only receipts with consistency checks and usage provenance tests. r[agent_harness_runtime.serving_admission.receipt]
- [x] [serial] 1.5 Add opt-in rollout facts, requirements, gate binding, and response provenance tests. r[agent_harness_runtime.serving_admission.rollout]

## Phase 2: Host path and configuration

- [x] [serial] 2.1 Add the `serving-admission` feature, `OwnerPort`, `UnavailableOwner`, `FixtureOwner`, JSON codec, root facade, and Mesh `ResponseFacts`, with host-path tests. r[agent_harness_runtime.serving_admission.host_path]
- [x] [serial] 2.2 Add `config/serving-capacity-profile.ncl`, the generated JSON, positive and negative fixtures, and export/check wiring. r[agent_harness_runtime.serving_admission.host_path]
- [x] [serial] 2.3 Register the public modules in `.cairn/runtime-facade-inventory.ncl`. r[agent_harness_runtime.serving_admission.host_path]

## Phase 3: Remaining

- [x] [serial] 3.1 Add a focused `scripts/cairn/check-serving-admission` rail, its flake check, component-catalog entries, `docs/serving-admission.md`, and README rail lines. r[agent_harness_runtime.serving_admission.host_path]
- [ ] [serial] 3.2 Run the full repository gates, including `nix flake check -L`, and record results in `evidence/`. Run on top of `c5fe338e`; see `evidence/fixture-qualification.md`. Left unchecked because host-toolchain workspace clippy fails in files outside this change, and the full flake result is recorded in the integration commit. `check-roadmap` fails only on the accepted pre-existing `add-acp-coding-agent` issue. r[agent_harness_runtime.serving_admission.receipt]
- [ ] [serial] 3.3 Qualify one selected live local endpoint separately, labelled `live-endpoint`. Not performed; requires a live serving owner. r[agent_harness_runtime.serving_admission.host_path]
