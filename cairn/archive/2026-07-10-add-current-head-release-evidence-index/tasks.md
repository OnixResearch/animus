# Tasks

## Phase 1: Implementation

- [x] [serial] Add a shell-owned current-head release evidence index generator that records VCS head metadata and supplied check statuses. r[agent_harness_runtime.current_head_release_evidence_index.generator]
- [x] [depends:agent_harness_runtime.current_head_release_evidence_index.generator] Require explicit `--claim-current-head` before emitting current-head proof claims. r[agent_harness_runtime.current_head_release_evidence_index.claims]
- [x] [depends:agent_harness_runtime.current_head_release_evidence_index.generator] Recognize packaged headless dogfood smoke as a pure selected evidence mode. r[agent_harness_runtime.current_head_release_evidence_index.packaged_smoke]
- [x] [depends:agent_harness_runtime.current_head_release_evidence_index.claims] Add a focused rail covering generated positive, dirty, missing-mode, stale, and actual-head draft cases. r[agent_harness_runtime.current_head_release_evidence_index.verification]
- [x] [depends:agent_harness_runtime.current_head_release_evidence_index.verification] Wire docs, README commands, Nix check, roadmap classification, and lifecycle evidence. r[agent_harness_runtime.current_head_release_evidence_index.docs]

## Phase 2: Validation

- [x] [serial] Verify release-evidence fixtures, current-head generator rail, Nix checks, Cairn validation, and Cairn gates before archive. r[agent_harness_runtime.current_head_release_evidence_index.verification]
