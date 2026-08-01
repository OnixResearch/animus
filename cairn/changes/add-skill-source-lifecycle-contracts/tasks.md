# Tasks

## 1. Baseline and policy shape

- [ ] [serial] Run focused host asset, skill manifest, safe receipt, no-std, and Wasm checks before core changes and record existing failures. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Record the QM revision, reviewed source-lifecycle mechanisms, rejected fetch and activation behavior, license facts, and claim boundary. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Define typed Nickel contracts for source kinds, modes, bounds profiles, canonical identity profiles, promotion policy, diagnostics, and mandatory non-claims. r[agent_harness_runtime.skill_source_lifecycle.policy]
- [ ] [depends:source-policy] Export the deterministic runtime policy and add positive and negative freshness fixtures. r[agent_harness_runtime.skill_source_lifecycle.policy]

## 2. Build the pure adapter contract

- [ ] [serial] Add bounded nominal source, revision, tree, manifest, observation, candidate, review, promotion, action, capability, and receipt references in a focused `no_std` adapter module. r[agent_harness_runtime.skill_source_lifecycle.contract]
- [ ] [depends:source-contract] Add frozen descriptor validation and exact revision, tree, manifest, policy, trust, provenance, and bounds matching. r[agent_harness_runtime.skill_source_lifecycle.frozen]
- [ ] [depends:source-contract] Add tracked-candidate classification for unchanged, changed, stale, denied, and conflicting observations without activation. r[agent_harness_runtime.skill_source_lifecycle.tracked_candidate]
- [ ] [depends:frozen-source] [depends:tracked-candidate] Add exact candidate-to-frozen promotion admission and preserve the existing host mutation service, dry-run plan, action, and capability checks. r[agent_harness_runtime.skill_source_lifecycle.promotion]
- [ ] [depends:source-contract] Add duplicate asset, source collision, unsafe metadata, unsupported mode, missing bound, and invalid identity rejection. r[agent_harness_runtime.skill_source_lifecycle.contract]

## 3. Preserve host authority boundaries

- [ ] [serial] Keep source discovery, raw locations, credentials, VCS, transport, archive handling, filesystem traversal, symlink resolution, extraction, storage, clocks, review, and mutation outside reusable Animus crates. r[agent_harness_runtime.skill_source_lifecycle.host_authority]
- [ ] [parallel] Add host conformance inputs for Bounded Exec process outcomes, Cap Root path admission, entry-type policy, and Mantle-style frozen materialization without importing those effects into the core. r[agent_harness_runtime.skill_source_lifecycle.host_authority]
- [ ] [parallel] Prove default, core, adapter, runtime, Steel, TUI, no-std, Wasm, and unrelated std profiles gain no VCS, transport, archive, credential, filesystem, process, network, clock, or store implementation. r[agent_harness_runtime.skill_source_lifecycle.host_authority]

## 4. Add safe reconciliation and evidence

- [ ] [serial] Integrate source lifecycle with host asset reconciliation while preserving compatibility for manifests without a source-lifecycle claim. r[agent_harness_runtime.skill_source_lifecycle.contract] r[agent_harness_runtime.skill_source_lifecycle.promotion]
- [ ] [serial] Add metadata-only source observation, candidate, promotion, and receipt projections with BLAKE3 only over public canonical bytes. r[agent_harness_runtime.skill_source_lifecycle.evidence]
- [ ] [parallel] Reject raw URLs, full paths, credentials, tokens, environment values, source bytes, file bodies, scripts, VCS arguments, stdout, stderr, tool output, and unrestricted diagnostics in safe metadata. r[agent_harness_runtime.skill_source_lifecycle.evidence]
- [ ] [parallel] Document frozen acquisition, tracked candidate observation, explicit promotion, consumer pinning, rollback, and non-claims. r[agent_harness_runtime.skill_source_lifecycle.host_authority] r[agent_harness_runtime.skill_source_lifecycle.evidence]

## 5. Verify positive and negative behavior

- [ ] [serial] Add positive tests for exact frozen identity, unchanged tracked observation, changed inactive candidate, explicit promotion, legacy compatibility, and safe receipts. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Add negative tests for moving tags without exact revisions, tree mismatch, stale observation, duplicate asset, source collision, missing bounds, unsupported modes, and unsafe metadata. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Add negative tests for denied symlink or submodule facts, automatic activation, conflicting promotion, missing service, plan drift, partial host results, receipt leakage, and feature contamination. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [parallel] Add assertions that a content-identity match cannot establish source authenticity, trust, script safety, authority, confinement, or release readiness. r[agent_harness_runtime.skill_source_lifecycle.evidence] r[agent_harness_runtime.skill_source_lifecycle.verification]

## 6. Validate and close out

- [ ] [serial] Run format, focused adapter tests, Clippy, Nickel freshness, safe-receipt, component, contract, no-std, Wasm, Cairn, traceability, and relevant Nix rails. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Record exact Animus, policy, schema, fixture, and consumer identities plus concrete blockers for every skipped rail. r[agent_harness_runtime.skill_source_lifecycle.verification]
- [ ] [serial] Verify that downstream adoption requires an immutable Animus revision and preserves host-owned acquisition, review, mutation, and runtime authority. r[agent_harness_runtime.skill_source_lifecycle.host_authority]
