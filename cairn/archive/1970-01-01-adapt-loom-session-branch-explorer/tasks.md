## Phase 1: Baseline and reference boundary

- [x] [serial] B1 Record current session-tree, branch-control, redacted-export, conversation-projection, provenance, TUI reducer, and receipt baselines. r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [serial] B2 Inventory ledger, annotation, active-leaf, comparison, render-cache, persistence, and effect-state owners. r[agent_harness_runtime.session_branch_explorer.boundary]
- [x] [parallel] B3 Record the Loom article and `universal-weave` revision, useful concepts, rejected mechanisms, license posture, and no-runtime-dependency boundary. r[agent_harness_runtime.session_branch_explorer.boundary] r[agent_harness_runtime.session_branch_explorer.evidence]

## Phase 2: Policy and pure projection core

- [x] [serial] P1 Add typed Nickel policy for schema versions, node/depth/label/order/comparison bounds, redaction, annotations, external-state posture, receipts, and non-claims. r[agent_harness_runtime.session_branch_explorer.projection] r[agent_harness_runtime.session_branch_explorer.evidence]
- [x] [serial] P2 Export policy data and add strict Rust revalidation with domain-separated BLAKE3 policy identity. r[agent_harness_runtime.session_branch_explorer.projection]
- [x] [serial] C1 Add pure validated branch snapshot, node, path, bound, truncation, and diagnostic types over existing ledger identities. r[agent_harness_runtime.session_branch_explorer.boundary] r[agent_harness_runtime.session_branch_explorer.projection]
- [x] [serial] C2 Add deterministic topology projection with one-parent validation, leaf classification, active-path marking, storage sequence, child counts, and redaction-first output. r[agent_harness_runtime.session_branch_explorer.projection]
- [x] [serial] C3 Add typed bookmark, safe-label, and sibling-order validation without replay or storage-order mutation. r[agent_harness_runtime.session_branch_explorer.annotations]
- [x] [serial] C4 Add pure inspect, activate-leaf, and fork-from-node planning with exact revision fences and external-state posture. r[agent_harness_runtime.session_branch_explorer.selection]
- [x] [serial] C5 Add common-ancestor and ordered-suffix comparison with bounded safe metadata and explicit unavailable facts. r[agent_harness_runtime.session_branch_explorer.comparison]
- [x] [serial] C6 Add exact sibling-equivalence validation over complete generation identities without deletion or merge actions. r[agent_harness_runtime.session_branch_explorer.equivalence]

## Phase 3: Adapter and host boundaries

- [x] [serial] A1 Add implementation-free projection, annotation, selection, comparison, and evidence ports in the smallest reusable crate. r[agent_harness_runtime.session_branch_explorer.boundary]
- [x] [serial] A2 Compose fork requests with accepted append-only session branch controls and reject destructive rewrite operations. r[agent_harness_runtime.session_branch_explorer.selection]
- [x] [serial] A3 Compose selected read mode with the canonical conversation-view projection without reverse cache authority. r[agent_harness_runtime.session_branch_explorer.projection]
- [x] [serial] H1 Add a non-default std host shell or fixture host for annotation persistence and active-leaf compare-and-set writes. r[agent_harness_runtime.session_branch_explorer.annotations] r[agent_harness_runtime.session_branch_explorer.selection]
- [x] [serial] H2 Require separate admitted isolation and effect evidence before a host labels mutating continuations independent. r[agent_harness_runtime.session_branch_explorer.selection]

## Phase 4: Reference TUI

- [x] [serial] U1 Add pure explorer state for map/read mode, focus, collapse, bookmark intent, comparison targets, refresh, and diagnostics. r[agent_harness_runtime.session_branch_explorer.tui]
- [x] [serial] U2 Render bounded deterministic text snapshots with active-path, leaf, bookmark, truncation, stale, and external-state indicators. r[agent_harness_runtime.session_branch_explorer.tui]
- [x] [serial] U3 Dispatch typed inspect, activate, fork, bookmark, reorder, and compare requests through host ports without direct ledger writes. r[agent_harness_runtime.session_branch_explorer.tui]
- [x] [serial] U4 Keep Tile, browser, remote, and accessibility integration outside this change and record the consumer handoff. r[agent_harness_runtime.session_branch_explorer.boundary]

## Phase 5: Positive and negative evidence

- [x] [parallel] T1 Add positive tests for one root, multiple leaves, active paths, internal-node inspection, leaf activation, append-only fork, bookmarks, partial sibling order, and common-ancestor comparison. r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [parallel] T2 Add negative topology tests for empty trees, duplicate IDs, missing parents, cycles, multiple-parent attempts, false leaves, stale revisions, and overbound projections. r[agent_harness_runtime.session_branch_explorer.projection] r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [parallel] T3 Add negative annotation tests for unknown targets, repeated children, cross-parent order, unsafe labels, stale annotations, and attempts to alter replay order. r[agent_harness_runtime.session_branch_explorer.annotations] r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [parallel] T4 Add comparison and equivalence tests for equal text with different effects, cross-session targets, missing ancestors, incomplete metadata, invalid generation identities, and deletion attempts. r[agent_harness_runtime.session_branch_explorer.comparison] r[agent_harness_runtime.session_branch_explorer.equivalence] r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [parallel] T5 Add TUI tests for collapse, refresh, stale selection, blocked activation, branch comparison, deterministic render snapshots, and host-port failure. r[agent_harness_runtime.session_branch_explorer.tui] r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [parallel] T6 Add redaction and receipt-leak tests for transcript text, reasoning, tool bodies, provider payloads, credentials, paths, raw UI state, rollback claims, and semantic-quality claims. r[agent_harness_runtime.session_branch_explorer.evidence] r[agent_harness_runtime.session_branch_explorer.verification]

## Phase 6: Documentation and closeout

- [x] [serial] D1 Document topology ownership, annotation sidecars, selection and fork semantics, comparison, equivalence, external-state posture, TUI composition, and non-claims. r[agent_harness_runtime.session_branch_explorer.boundary] r[agent_harness_runtime.session_branch_explorer.evidence]
- [x] [serial] D2 Update ledger, branch-control, conversation-projection, provenance, redacted-export, TUI, component, facade, safety, request-lifecycle, roadmap, and README indexes. r[agent_harness_runtime.session_branch_explorer.boundary]
- [x] [serial] D3 Add a focused branch-explorer rail and Nix check for policy drift, pure reducers, host fakes, redaction, receipts, TUI snapshots, and dependency isolation. r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [serial] D4 Run focused tests, formatting, Clippy, no-std and Wasm checks where applicable, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.session_branch_explorer.verification]
- [x] [serial] D5 Review dry-run sync and archive plans. Confirm that every requirement reaches the accepted spec before archive execution. r[agent_harness_runtime.session_branch_explorer.verification]
