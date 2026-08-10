# Tasks

- [x] [serial] Extend the real-check smoke receipt with exact selected-check and pinned-Cairn metadata. r[agent_harness_runtime.real_check_evidence_index.receipt]
- [x] [depends:agent_harness_runtime.real_check_evidence_index.receipt] Add the dedicated typed index record and bounded claim. r[agent_harness_runtime.real_check_evidence_index.claim]
- [x] [depends:agent_harness_runtime.real_check_evidence_index.claim] Parse one explicit real-check receipt in the current-head generator without running hidden checks. r[agent_harness_runtime.real_check_evidence_index.generator]
- [x] [depends:agent_harness_runtime.real_check_evidence_index.generator] Reject missing, stale, dirty, malformed, fixture, non-passing, source-unbound, wrong-check, wrong-Cairn, duplicate, and conflicting evidence. r[agent_harness_runtime.real_check_evidence_index.validation]
- [x] [depends:agent_harness_runtime.real_check_evidence_index.validation] Add positive and negative Nickel fixtures plus generated-index cases. r[agent_harness_runtime.real_check_evidence_index.verification]
- [x] [depends:agent_harness_runtime.real_check_evidence_index.verification] Update README, release-index documentation, roadmap, and focused rail commands. r[agent_harness_runtime.real_check_evidence_index.docs]
- [x] [serial] Run focused, dogfood, Nix, Cairn, clean-current-head, sync, and archive checks before publication. r[agent_harness_runtime.real_check_evidence_index.closeout]
