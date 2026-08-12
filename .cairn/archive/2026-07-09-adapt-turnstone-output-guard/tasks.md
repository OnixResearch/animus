# Tasks

- [x] [serial] Define output assessment DTOs for risk level, detector flags, annotation labels, redaction state, bounded counts, safe output references, detector tiers, and BLAKE3 evidence hashes. r[agent_harness_runtime.turnstone_output_guard.dto]
- [x] [depends:dto] Add deterministic scanner rules for prompt-injection markers, credential leakage, encoded payloads, adversarial URLs, unsafe references, and over-budget output summaries. r[agent_harness_runtime.turnstone_output_guard.scanner]
- [x] [depends:scanner] Add merge logic where optional classifier assessments can add flags or annotations but cannot lower deterministic findings or override credential redaction. r[agent_harness_runtime.turnstone_output_guard.merge]
- [x] [depends:merge] Gate model-visible tool-output projection on assessment outcome, redaction policy, output bounds, and safe reference validation. r[agent_harness_runtime.turnstone_output_guard.projection]
- [x] [depends:projection] Add safe runtime events and receipts with flags, risk labels, counts, output hashes, redaction markers, and detector tier labels only. r[agent_harness_runtime.turnstone_output_guard.receipts]
- [x] [depends:receipts] Document Turnstone as a bounded reference and list product UI warning chips, live classifier calls, web fetching, shell execution, and raw output storage as non-claims. r[agent_harness_runtime.turnstone_output_guard.docs]
- [x] [depends:docs] Add positive clean-output and redacted-credential tests plus negative prompt-injection, encoded-payload, adversarial-URL, unsafe-reference, classifier-downgrade, over-budget, and receipt-leak tests. r[agent_harness_runtime.turnstone_output_guard.verification]
