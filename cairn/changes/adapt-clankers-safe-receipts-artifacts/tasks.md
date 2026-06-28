# Tasks

- [ ] [serial] Define minimal safe receipt DTOs with schema, status, issue code, labels, bounded counts, BLAKE3 hashes, effect or mutation kind, backend/interchange labels, and redaction markers. r[agent_harness_runtime.clankers_safe_receipts.dto]
- [ ] [depends:clankers_safe_receipts.dto] Add positive receipt tests for model, tool, authorization, runtime block, final response, adapter error, and provenance observations. r[agent_harness_runtime.clankers_safe_receipts.positive]
- [ ] [depends:clankers_safe_receipts.dto] Add negative leak fixtures for raw prompts, provider JSON, credentials, absolute paths, raw tool input/output, raw model output, and unbounded output. r[agent_harness_runtime.clankers_safe_receipts.rejects_leaks]
- [ ] [serial] Define BLAKE3 artifact hash types, canonical envelope versioning, artifact kinds, redaction classes, normalized payload handling, dependency sorting/deduplication, and parse/display errors. r[agent_harness_runtime.clankers_artifact_identity.envelope]
- [ ] [depends:clankers_artifact_identity.envelope] Add deterministic artifact tests proving dependency order does not affect hashes and BLAKE3 display/parse round trips. r[agent_harness_runtime.clankers_artifact_identity.deterministic]
- [ ] [depends:clankers_artifact_identity.envelope] Add redaction tests proving metadata-only, redacted-payload, and secret artifacts do not display payload bodies in receipts or review output. r[agent_harness_runtime.clankers_artifact_identity.redaction]
- [ ] [depends:clankers_safe_receipts.rejects_leaks,clankers_artifact_identity.redaction] Add focused receipt/artifact validation rails with reviewable evidence. r[agent_harness_runtime.clankers_safe_receipts.rails]
- [ ] [depends:clankers_safe_receipts.rails] Run focused rails, workspace tests if touched, dependency checks, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_safe_receipts.closeout]
