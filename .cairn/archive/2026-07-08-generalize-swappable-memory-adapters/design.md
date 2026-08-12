# Design: Generalize swappable memory adapters

## Context

Memory needs to be broader than one product reference. Existing agent ecosystems converge on a few durable ideas: local files are useful for inspectability, persistent stores/checkpointers are useful for continuity, vector or hybrid search is optional retrieval infrastructure, extraction may be model-assisted, and users need review and deletion controls. Animus should model those ideas through adapter contracts while preserving the existing functional-core / imperative-shell split.

## Decisions

### 1. Memory behavior is a family of ports

**Choice:** Represent memory as distinct host-injected adapter roles: instruction/source import, durable store, retrieval/search, extraction, embedding or ranking support, admission, and deletion/update.

**Rationale:** A markdown directory, SQLite store, MCP server, vector database, or Mem0-like service should be replaceable without changing reusable runtime logic.

### 2. Reusable logic consumes normalized records

**Choice:** Every backend result is converted to a `MemoryRecordEnvelope` before planning or prompt assembly. The envelope carries safe identity, kind, scope, evidence, trust/admission, freshness, confidence, sensitivity, body hash, budget, and conflict/supersession metadata.

**Rationale:** Backends can store richer data, but reusable code needs one testable contract.

### 3. Retrieval planning is pure and store-agnostic

**Choice:** Search adapters return candidate records plus declared scores or match reasons. A pure planner applies policy-defined scope filters, freshness rules, conflict handling, score weights, and prompt budgets.

**Rationale:** Ranking behavior must be deterministic, explainable, and testable with in-memory fixtures.

### 4. Extraction is not admission

**Choice:** Extraction adapters may propose candidate memories. Admission policy and host decisions decide whether those candidates can persist, update existing records, or influence prompts.

**Rationale:** Model-generated memories are fallible and privacy-sensitive; extraction should never imply storage.

### 5. Receipts are safe by construction

**Choice:** Operation receipts contain adapter family labels, safe record IDs, counts, status, policy hash, redaction class, and BLAKE3 body hashes. Raw bodies, provider payloads, credentials, full paths, and hidden memory bodies are rejected.

**Rationale:** Operators need explainability without turning receipts into a secret or transcript store.

## Risks / Trade-offs

- Too many adapter roles can feel heavy. Start with the common DTOs and no-op/manual adapters, then add concrete shells incrementally.
- Store-specific ranking can leak into the core. Keep backend scores as inputs and require the planner to make the final deterministic inclusion decision.
- Local file memories are easy to inspect but easy to over-trust. Preserve non-authority and admission rules.
