# agent-harness-runtime Change Spec: Preserve router boundary DTOs

## Requirements

### Requirement: Optional Preserves projection of router DTOs

r[agent_harness_runtime.model_router.preserves_projection] Animus MUST provide an optional, feature-gated `preserves` projection on `animus-router` for `Recipe`, `SelectedPlan`, `RouterExecutionPlan`, `RouteResult`, and `Receipt` that preserves identity, ordering metadata, status classes, redaction markers, and BLAKE3 hashes, and the default and Wasm builds MUST remain Preserves-free.

#### Scenario: Router receipt round-trips through Preserves

r[agent_harness_runtime.model_router.preserves_projection.round_trip]
- GIVEN an assembled leak-free `Receipt`
- WHEN it is projected to Preserves and back
- THEN the projected record preserves recipe ID, signal buckets, counts, quorum/fallback class, policy hash, duration buckets, redaction markers, and BLAKE3 identity
- AND no raw prompt, payload, worker output, or credential is embedded.

#### Scenario: Projection is opt-in

r[agent_harness_runtime.model_router.preserves_projection.opt_in]
- GIVEN an embedder builds `animus-router` without the `preserves` feature
- WHEN the build runs
- THEN no `animus-preserves` dependency is compiled in
- AND the Wasm surface rail still passes.

### Requirement: Preserves projection rejects leaks

r[agent_harness_runtime.model_router.preserves_projection.leak_validation] The projected `Receipt` MUST pass the same `validate_receipt` leak checks as the Rust receipt, and any raw-payload leakage, oversized field, or projection mismatch MUST fail validation.

#### Scenario: Raw worker output rejected in projection

r[agent_harness_runtime.model_router.preserves_projection.leak_validation.worker_output]
- GIVEN a Preserves receipt projection embeds a raw worker output or judge rationale
- WHEN `validate_receipt` runs on the round-tripped receipt
- THEN validation rejects the projection with a typed `ReceiptLeak`
- AND the projection cannot be used as Steel or dataspace evidence.

### Requirement: Preserves projection verification

r[agent_harness_runtime.model_router.preserves_projection.verification] The change MUST include focused positive and negative tests for each DTO projection, round-trip identity, leak rejection, oversized-field rejection, the `--features preserves` build, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers the Preserves projection

r[agent_harness_runtime.model_router.preserves_projection.verification.closeout]
- GIVEN maintainers intend to archive the Preserves projection change
- WHEN closeout validation runs
- THEN evidence names focused projection, round-trip, leak-validation, fail-closed, feature-build, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
