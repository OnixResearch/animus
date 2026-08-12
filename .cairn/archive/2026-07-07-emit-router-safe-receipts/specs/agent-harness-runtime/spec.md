# agent-harness-runtime Change Spec: Emit safe router receipts

## Requirements

### Requirement: Pure router receipt assembly

r[agent_harness_runtime.model_router.receipt_emission] Animus MUST provide a pure `assemble_receipt` function that builds a safe `Receipt` from an executed `RouterExecutionPlan`, `RouteResult`, projected `Signals`, policy hash, and duration buckets, recording recipe ID, signal buckets, selected model labels, bounded counts, quorum class, fallback class, policy hash, duration buckets, redaction markers, and a BLAKE3 identity, and MUST NOT embed raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

#### Scenario: Collaborative route emits a leak-free receipt

r[agent_harness_runtime.model_router.receipt_emission.leak_free]
- GIVEN a ReMoM route completed with quorum satisfied and a validated final output
- WHEN `assemble_receipt` runs
- THEN the receipt records the recipe ID, signal buckets, selected labels, quorum class, fallback class, policy hash, duration buckets, and BLAKE3 identity
- AND no raw worker output, judge rationale, prompt, or credential is embedded.

### Requirement: Receipt validation rejects leaks

r[agent_harness_runtime.model_router.receipt_emission.validation] An idempotent `validate_receipt` MUST run text validation over all string fields plus structural checks for raw output fields, oversized payloads, credential-looking values, and missing or malformed BLAKE3 identity, and MUST reject any receipt that fails them.

#### Scenario: Worker-output leak rejected

r[agent_harness_runtime.model_router.receipt_emission.validation.worker_leak]
- GIVEN an assembled receipt fixture embeds a raw worker output or judge rationale
- WHEN `validate_receipt` runs
- THEN validation rejects the fixture with a typed `ReceiptLeak`
- AND the receipt cannot be used as review or release evidence.

#### Scenario: Credential-looking field rejected

r[agent_harness_runtime.model_router.receipt_emission.validation.credential]
- GIVEN a receipt string field looks like a token, key, or password
- WHEN `validate_receipt` runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Router receipt verification

r[agent_harness_runtime.model_router.receipt_emission.verification] The change MUST include focused positive and negative tests for each family's assembly, redaction, leak rejection, BLAKE3 identity, safe-receipts-artifacts wiring, workspace checks, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers receipt emission

r[agent_harness_runtime.model_router.receipt_emission.verification.closeout]
- GIVEN maintainers intend to archive the receipt-emission change
- WHEN closeout validation runs
- THEN evidence names focused assembly, redaction, validation, fail-closed, artifacts-rail, facade-inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
