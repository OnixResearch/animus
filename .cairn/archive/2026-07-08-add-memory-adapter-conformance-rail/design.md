# Design: Add memory adapter conformance rail

## Context

Memory backends differ widely. A file-backed memory index emphasizes inspectability, SQL emphasizes persistence and queryability, vector stores emphasize semantic recall, MCP and remote services emphasize external capability boundaries, and null adapters prove memory can be disabled. A conformance rail lets Animus support those forms without treating any one backend as the reference implementation.

## Decisions

### 1. The matrix is backend-family oriented

**Choice:** Track conformance by backend family and adapter role, not by concrete product brand.

**Rationale:** The rail should cover markdown, SQL, vector, MCP, remote, extractor, and admission semantics while allowing hosts to pick implementations.

### 2. Every row has positive and negative coverage

**Choice:** Each applicable matrix row must name at least one positive behavior and one negative/failure behavior.

**Rationale:** Memory failures are safety-critical: unavailable stores, malformed records, path leaks, provider payload leaks, and denied admission must be tested as intentionally as happy paths.

### 3. Conformance inputs are normalized DTOs

**Choice:** Reusable conformance tests use in-memory normalized records, queries, adapter status values, and receipt drafts. Host-specific I/O is tested in shell-owned integration fixtures.

**Rationale:** The rail should validate the adapter contract without depending on a live database, MCP server, vector service, or provider.

### 4. Null memory is first-class

**Choice:** Disabled/no-op memory is an explicit backend family with expected unavailable or no-influence outcomes.

**Rationale:** Users and deployments must be able to turn memory off without disabling required guidance or causing ambient fallback discovery.

### 5. Backend internals stay out of receipts

**Choice:** Conformance rejects raw paths, raw provider payloads, remote auth material, vector-index internals, and raw memory bodies in model-visible ledgers or receipts.

**Rationale:** Adapter swapping should not widen the audit leak surface.

## Risks / Trade-offs

- A broad matrix can slow development. Keep initial fixtures minimal and require explicit not-applicable reasons.
- Backend families can overlap. The matrix should allow one concrete adapter to satisfy multiple rows when it proves the required behavior.
- Host integration tests may need temporary filesystem or database state. Keep those tests outside reusable core and clearly label their authority.
