# Design: Emit safe router receipts

## Context

The receipt DTOs exist (`Receipt`, `ReceiptDraft`, `Redaction`, `ReceiptLeak`, `QuorumClass`, `DurationBucket`) and the safe-receipts-artifacts rail already validates receipt fixtures. What is missing is the *producer*: a pure function that takes an executed plan, its route result, the projected signals, a policy hash, and duration buckets, and returns a leak-free `Receipt`. Without it, every runtime consumer would assemble receipts inconsistently and the leak-validation rail would have nothing real to exercise.

## Decisions

### 1. Receipt assembly is pure over plan + result + signals

**Choice:** `assemble_receipt(plan, result, signals, policy_hash, durations)` is a pure function returning `Result<Receipt, ReceiptLeak>`. It reads only safe fields: recipe id, family, selected model labels, signal bucket labels, bounded counts, quorum class, fallback class, duration buckets, and a BLAKE3 identity over the hashed parts.

**Rationale:** Assembly is the fourth pure gate. It must be testable without persistence, telemetry, or clock side effects; durations arrive as already-classified `DurationBucket` values from the caller.

### 2. Redaction is structural, not string-scrubbing

**Choice:** Redaction happens at the field-selection boundary: `assemble_receipt` never reads raw role outputs. It consumes the bounded `RoleOutput` summaries (labels, size classes, BLAKE3 hashes) already produced by aggregation, and maps them to `Redaction` markers. The existing `validate_receipt_text` guards against any string field that looks like a prompt, payload, or credential.

**Rationale:** String-scrubbing raw payloads is fragile; never reading them is safe. This connects to the accepted `vllm_semantic_router_recipes.receipts` and `clankers_safe_receipts` requirements.

### 3. BLAKE3 identity covers the stable safe fields

**Choice:** The receipt's BLAKE3 identity hashes the stable safe parts (recipe id, family, signal buckets, counts, quorum/fallback class, policy hash, duration buckets) via the existing `hash_parts` helper, producing a deterministic receipt fingerprint.

**Rationale:** BLAKE3 is the repo default for content hashing and gives receipts a tamper-evident identity without embedding raw content.

### 4. Validation is idempotent and reusable

**Choice:** `validate_receipt` re-runs `validate_receipt_text` over all string fields plus structural checks (no raw output fields, no oversized payloads, no credential-looking values, BLAKE3 identity present and well-formed). The same validator backs the safe-receipts-artifacts rail.

**Rationale:** One validator, used at assembly time and by the artifacts rail, prevents drift between production and review.

## Risks / Trade-offs

- Duration buckets are caller-supplied; a malicious caller could lie. Mitigate by treating the receipt as evidence, not proof, and by having the runtime wiring cairn supply durations from a typed `DurationClass` source.
- BLAKE3 over evolving field sets can break receipt replay. Version the hashed-parts layout via a stable prefix and document it.
- Over-redaction can hide useful debugging signal. Keep redaction at the field-selection boundary and expose bounded counts/size classes so receipts stay useful while leak-free.
