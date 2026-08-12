## Tasks

- [x] [serial] r[model-evidence-adapters.report-dto] Define model-evidence adapter DTOs for bounded verification, conversion, kernel-proof, and benchmark report profiles.
- [x] [depends:model-evidence-dto] r[model-evidence-adapters.core-validation] Add pure validation for evidence kind, model identity, property scope, verifier identity, freshness, diagnostics, capability labels, and non-claims.
- [x] [depends:model-evidence-validation] r[model-evidence-adapters.host-shell] Add a host-owned ingestion seam for already-produced model reports without adding model-runtime authority to reusable crates.
- [x] [depends:model-evidence-validation] r[model-evidence-adapters.positive] Add positive fixtures for bounded verification, conversion, and kernel-proof summary reports.
- [x] [depends:model-evidence-validation] r[model-evidence-adapters.negative] Add negative fixtures for stale, mismatched-model, unsupported-verifier, missing-property, benchmark-only, and weakened-non-claim reports.
- [x] [depends:model-evidence-fixtures] r[model-evidence-adapters.validation] Run DTO tests, model-router/admission fixture checks, safe-receipts checks, Cairn gates, and focused Animus validation.