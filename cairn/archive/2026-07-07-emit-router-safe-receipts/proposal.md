# Proposal: Emit safe router receipts

## Why

`animus-adapters::router::evidence` already defines `Receipt`, `ReceiptDraft`, `Redaction`, `ReceiptLeak`, `DurationBucket`, `QuorumClass`, `validate_receipt_text`, and BLAKE3 `hash_parts`. These are DTOs and helpers with no emission pipeline: nothing assembles a receipt from an executed plan and its results, nothing redacts raw payloads into safe metadata, and nothing validates that an assembled receipt is leak-free. The accepted `vllm_semantic_router_recipes.receipts` requirement demands safe metadata (recipe ID, signal buckets, selected labels, counts, quorum/fallback class, policy hash, duration buckets, redaction markers, BLAKE3 identity) with no raw prompts, provider payloads, worker outputs, judge rationales, or credentials. Animus needs the pure assembly that produces such receipts.

## What Changes

- Add a pure `assemble_receipt(plan: &RouterExecutionPlan, result: &RouteResult, signals: &Signals, policy_hash: &str, durations: &[DurationBucket]) -> Result<Receipt, ReceiptLeak>` that builds a safe receipt from executed-plan state and route result.
- Add redaction helpers that convert raw role outputs and synthesis artifacts into safe labels, counts, size classes, and BLAKE3 hashes, never embedding raw text.
- Add `validate_receipt(receipt: &Receipt) -> Result<(), ReceiptLeak>` that runs the existing `validate_receipt_text` plus structural leak checks (no raw output fields, no oversized payloads, no credential-looking values).
- Add positive tests for assembled receipts and negative tests for each leak class.

## Impact

- **Files**: `crates/animus-router` receipt-assembly module reusing `evidence/` DTOs, router tests, safe-receipts-artifacts fixtures, facade inventory, and Cairn evidence.
- **Testing**: positive assembly fixtures for each family; negative tests for raw-prompt leak, provider-payload leak, worker-output leak, judge-rationale leak, credential-looking field, oversized payload, and missing BLAKE3 identity.
- **Non-claims**: this does not persist receipts, write files, emit telemetry, or wire the runtime; assembly is pure over in-memory inputs.
