## Phase 1: Baseline and mechanism review

- [x] [serial] B1 Run current hashed-edit, tool-executor, capability, mutation-serialization, and safe-receipt baselines. r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [serial] B2 Inventory current file read, capability-root, path admission, lock, staging, rename, durability, and receipt boundaries. r[agent_harness_runtime.hashed_edit_host.boundary]
- [x] [parallel] B3 Review exact Cap Root and Durable File Publication cohorts, licenses, replace semantics, durability states, and non-claims. r[agent_harness_runtime.hashed_edit_host.publication]
- [x] [parallel] B4 Record the pinned Autolith revision, ISC license, adapted visibility scenarios, rejected formats, and no-runtime-dependency boundary. r[agent_harness_runtime.hashed_edit_host.evidence]

## Phase 2: Snapshot and pure apply core

- [x] [serial] C1 Add nominal root, resource, revision, visibility, writer-generation, publication, and durability identities. r[agent_harness_runtime.hashed_edit_host.snapshot]
- [x] [serial] [depends:C1] C2 Extend host snapshots with visible tagged spans, line-ending mode, final-newline state, and capability-root identity. r[agent_harness_runtime.hashed_edit_host.snapshot]
- [x] [serial] [depends:C2] C3 Add visible replacement, deletion, insertion-boundary, and end-boundary admission. r[agent_harness_runtime.hashed_edit_host.visibility]
- [x] [serial] [depends:C3] C4 Add pure byte application that preserves admitted newline and final-newline state and computes exact after identity. r[agent_harness_runtime.hashed_edit_host.snapshot]
- [x] [serial] [depends:C4] C5 Add second-read freshness and writer-generation planning with typed stale outcomes. r[agent_harness_runtime.hashed_edit_host.freshness]
- [x] [serial] [depends:C5] C6 Add publication-state and durability-state reduction, including uncertain post-publication outcomes. r[agent_harness_runtime.hashed_edit_host.publication]

## Phase 3: Capability-relative shell

- [x] [serial] [depends:C6] H1 Create a separate non-default workspace-edit host or authority-scoped std block with no default filesystem dependency. r[agent_harness_runtime.hashed_edit_host.boundary]
- [x] [serial] [depends:H1] H2 Add explicit capability-root and logical-relative-resource admission before file I/O. r[agent_harness_runtime.hashed_edit_host.boundary]
- [x] [serial] [depends:H2] H3 Add first read, pure plan, resource lock, exact second read, and no-stage-on-drift ordering. r[agent_harness_runtime.hashed_edit_host.freshness]
- [x] [serial] [depends:H3] H4 Add bounded staging and exact one-file publication through the reviewed mechanism where contracts match. r[agent_harness_runtime.hashed_edit_host.publication]
- [x] [serial] [depends:H4] H5 Add file and directory durability observations without claiming rollback after visibility changes. r[agent_harness_runtime.hashed_edit_host.publication]
- [x] [serial] [depends:H5] H6 Bind the host to existing tool executor, authority, approval, mutation, cancellation, timeout, and output gates. r[agent_harness_runtime.hashed_edit_host.boundary]

## Phase 4: Filesystem and adversarial evidence

- [x] [parallel] [depends:H6] T1 Add positive fixtures for visible replacement, deletion, insertion, end insertion, supported line endings, final newline, and complete durability. r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [parallel] [depends:H6] T2 Add hidden span, partial visibility, stale revision, stale second read, writer-generation drift, overlapping edit, and unsupported newline fixtures. r[agent_harness_runtime.hashed_edit_host.visibility] r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [parallel] [depends:H6] T3 Add absolute path, parent escape, wrong root, symlink-policy, file-kind, ownership, permission, and missing-authority fixtures. r[agent_harness_runtime.hashed_edit_host.boundary] r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [parallel] [depends:H5] T4 Add staging failure, rename failure, post-publication sync failure, uncertain durability, cancellation, timeout, and external-writer race fixtures. r[agent_harness_runtime.hashed_edit_host.publication] r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [parallel] [depends:H6] T5 Add receipt leak and overclaim tests for file bodies, replacement text, absolute paths, VCS data, universal exclusion, atomic durability, and semantic correctness. r[agent_harness_runtime.hashed_edit_host.evidence] r[agent_harness_runtime.hashed_edit_host.verification]

## Phase 5: Documentation and closeout

- [x] [serial] [depends:T1] [depends:T2] [depends:T3] [depends:T4] [depends:T5] D1 Document the host sequence, visibility rule, capability boundary, publication states, race boundary, and non-claims. r[agent_harness_runtime.hashed_edit_host.boundary] r[agent_harness_runtime.hashed_edit_host.evidence]
- [x] [serial] [depends:D1] D2 Update hashed-edit, tool-executor, request-lifecycle, component, std-host, facade, roadmap, and README indexes. r[agent_harness_runtime.hashed_edit_host.boundary]
- [x] [serial] [depends:D2] D3 Add a focused rail and Nix check for dependencies, pure apply, capability-relative shell, filesystem failures, and safe receipts. r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [serial] [depends:D3] D4 Run focused tests, formatting, Clippy, feature isolation, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.hashed_edit_host.verification]
- [x] [serial] [depends:D4] D5 Review dry-run sync and archive plans, validate accepted requirement promotion, and archive only with current evidence. r[agent_harness_runtime.hashed_edit_host.verification]
