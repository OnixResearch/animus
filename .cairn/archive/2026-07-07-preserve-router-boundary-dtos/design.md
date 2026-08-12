# Design: Preserve router boundary DTOs

## Context

`animus-preserves` exists to keep cross-language boundary encoding out of core contracts, and the Steel fabric plus Syndicate dataspace adapter consume Preserves records. Router DTOs are plain Rust structs with no projection. An optional projection lets embedders script and observe routing portably without polluting the no-std core.

## Decisions

### 1. Projection is feature-gated and lives in animus-router

**Choice:** Add a `preserves` Cargo feature on `animus-router` that optionally depends on `animus-preserves`. The projection lives in `crates/animus-router/src/preserves/` and is compiled only when the feature is enabled. The default and Wasm builds stay Preserves-free.

**Rationale:** Keeps the router core no-std and Wasm-eligible by default; embedders opt into the boundary projection. Mirrors how the root `animus` crate gates `preserves`.

### 2. Projection preserves identity, ordering, status, redaction, and hashes

**Choice:** Each projected record preserves the DTO's stable identity fields, ordering metadata (step index, role order), status classes, redaction markers, and BLAKE3 hashes. Raw role outputs, prompts, payloads, and credentials are never projected; the record carries only the same safe fields as the Rust `Receipt`.

**Rationale:** Connects to the accepted `claude_code_headless.serialization` and `vllm_semantic_router_recipes.receipts` requirements: projections preserve identity and redaction without making the format the core state model.

### 3. The projection reuses Receipt leak validation

**Choice:** The Preserves `Receipt` projection is round-tripped through `validate_receipt` so a projected receipt must pass the same leak checks as the Rust receipt. Any raw-payload leak in the projection fails validation.

**Rationale:** One leak-validation rail covers both representations, preventing drift between Rust and Preserves safety.

### 4. Steel and dataspace consumers are out of scope here

**Choice:** This change adds the projection only. Steel fabric bindings and Syndicate dataspace assertions that *consume* the projection belong to future steel/dataspace cairns; this change proves the projection is sound and leak-free.

**Rationale:** Keeps the change focused and avoids coupling router promotion to Steel/dataspace work.

## Risks / Trade-offs

- Preserves schema drift can break Steel consumers. Mitigate by versioning the record shapes via a stable symbol prefix and documenting the schema.
- Feature-gating can hide compile failures. Add a CI check that builds `animus-router --features preserves` alongside the Wasm rail check.
- Round-tripping every DTO through Preserves is verbose. Project only the DTOs Steel/dataspace consumers actually need (recipe, selected plan, route result, receipt) and defer the rest.
