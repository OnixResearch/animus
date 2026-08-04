# Tasks: Add a universal-weave branching adapter

## Phase 1: Baseline and boundary

- [ ] [serial] B1 Record the upstream cohort (`universal-weave` `0.4.1`, Unlicense, default features disabled) and run a workspace baseline check before wiring the new crate. r[agent_harness_runtime.universal_weave.boundary]
- [ ] [depends:agent_harness_runtime.universal_weave.boundary] B2 Add `crates/animus-adapter-weave` as a non-default workspace member with the exact pin, `default-features = false`, no_std and unsafe-forbid markers, and the root `weave-adapter` feature facade. r[agent_harness_runtime.universal_weave.boundary]

## Phase 2: Functional core

- [ ] [depends:agent_harness_runtime.universal_weave.boundary] C1 Define the Animus-owned content type, policy, request, error, snapshot, and receipt DTOs with named bounds. r[agent_harness_runtime.universal_weave.bounds] r[agent_harness_runtime.universal_weave.evidence]
- [ ] [depends:agent_harness_runtime.universal_weave.bounds] C2 Implement the pure adapter core over the DAG weave: construction, add, edit, split, merge-with-parent, move, bookmark, activation, removal, active-path, and consistency check. r[agent_harness_runtime.universal_weave.adapter_core]
- [ ] [depends:agent_harness_runtime.universal_weave.adapter_core] C3 Implement metadata-only receipt and redacted snapshot projection with domain-separated BLAKE3 identities. r[agent_harness_runtime.universal_weave.evidence]

## Phase 3: Positive and negative evidence

- [ ] [parallel] T1 Add positive tests for policy admission, every operation class, active-path reads, consistency checks, deterministic receipts, and redacted snapshots. r[agent_harness_runtime.universal_weave.adapter_core] r[agent_harness_runtime.universal_weave.evidence]
- [ ] [parallel] T2 Add negative tests for zero bounds, oversized content, fan-in overflow, node-limit exhaustion, missing nodes, duplicate identifiers, missing parents, out-of-range splits, failed merges, failed moves, and unchanged-state proofs after rejection. r[agent_harness_runtime.universal_weave.bounds] r[agent_harness_runtime.universal_weave.adapter_core]
- [ ] [parallel] T3 Add evidence negatives proving receipts and snapshots exclude raw node text, prompts, provider payloads, credentials, and paths. r[agent_harness_runtime.universal_weave.evidence]

## Phase 4: Documentation and closeout

- [ ] [depends:agent_harness_runtime.universal_weave.evidence] D1 Add `docs/universal-weave-adapter.md`, register the component catalog row, update the README architecture, verification, and reference sections, and record explicit non-claims. r[agent_harness_runtime.universal_weave.verification]
- [ ] [depends:agent_harness_runtime.universal_weave.verification] V1 Add `scripts/cairn/check-universal-weave-adapter` covering the exact pin, disabled features, no_std and unsafe-forbid markers, forbidden-authority absence, catalog and docs registration, and fail-closed drift detection. r[agent_harness_runtime.universal_weave.verification]
- [ ] [depends:agent_harness_runtime.universal_weave.verification] V2 Run focused tests, formatting, Clippy, workspace checks, the opt-in crate Wasm compile check, the component catalog rail, Cairn validation and gates, and record concrete blockers. r[agent_harness_runtime.universal_weave.verification]
- [ ] [depends:agent_harness_runtime.universal_weave.verification] V3 Record evidence in the change package and verify that receipts remain metadata-only before proposing archive. r[agent_harness_runtime.universal_weave.evidence] r[agent_harness_runtime.universal_weave.verification]
