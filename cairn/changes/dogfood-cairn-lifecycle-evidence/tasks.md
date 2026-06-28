# Tasks

- [ ] [serial] Document `cairn/specs/` as accepted baseline, `cairn/changes/` as active deltas, and `cairn/archive/` as completed history. r[agent_harness_runtime.cairn_dogfood.lifecycle_layout]
- [ ] [depends:lifecycle_layout] Define change-threshold guidance for when Animus work needs a Cairn package versus direct safe edits. r[agent_harness_runtime.cairn_dogfood.change_threshold]
- [ ] [depends:lifecycle_layout] Add validation and gate workflow docs for proposal, design, tasks, validate, sync dry-run, archive dry-run, and executed archive closeout. r[agent_harness_runtime.cairn_dogfood.gate_workflow]
- [ ] [depends:gate_workflow] Define bounded evidence conventions for validation results, policy identity/hash, sync/archive receipts, release-readiness summaries, and unavailable-check blockers. r[agent_harness_runtime.cairn_dogfood.evidence_contract]
- [ ] [depends:evidence_contract] Add focused checks or fixtures proving evidence summaries omit raw prompts, credentials, provider payloads, full private paths, and unbounded output. r[agent_harness_runtime.cairn_dogfood.safe_evidence]
- [ ] [depends:gate_workflow,evidence_contract] Run validation, gates for this change, and sync/archive dry runs with reviewable evidence. r[agent_harness_runtime.cairn_dogfood.closeout]
