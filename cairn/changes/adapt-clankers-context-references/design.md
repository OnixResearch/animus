# Design: Adapt Clankers context references for Animus

## Context

Clankers expands documented `@` references before a prompt reaches the model and records metadata that is useful for replay/debugging without storing raw fetched content. Animus should adopt the pattern as a reusable boundary: prompt context is explicit input, expansion is bounded, host authority is injected, and receipts remain safe.

## Decisions

### 1. References are typed prompt inputs

**Choice:** Parse raw prompt references into typed `ContextReference` values with reference kind, raw label, normalized target, optional line range, and requested media/text mode.

**Rationale:** Raw prompt scanning is not a safe authority boundary. Typed references let policy, budgets, and resolvers all inspect the same normalized request.

### 2. Expansion requires injected resolver services

**Choice:** Filesystem, directory, diff, image, and URL expansion must call host-injected resolver services. Missing services, denied capabilities, unsupported reference kinds, and malformed targets produce typed failures before side effects are claimed.

**Rationale:** Animus reusable crates must not read the filesystem, run VCS commands, decode images, or fetch URLs by ambient default.

### 3. Bounds are evaluated before model-visible inclusion

**Choice:** Expansion returns candidate fragments plus metadata; pure budget logic decides whether to include, truncate, or reject each fragment.

**Rationale:** Hosts may produce large content. The runtime needs deterministic bounds and reviewable truncation behavior before model prompt construction.

### 4. Metadata is safe by construction

**Choice:** Ledger and receipt records store kind, status, safe target label or hash, byte/line counts, optional line range, truncation state, resolver class, and BLAKE3 content hash when needed. Raw expanded content, URL credentials, full absolute paths, provider payloads, and secret-looking values are excluded.

**Rationale:** Context-reference metadata should help replay and debugging without becoming a transcript or credential sink.

### 5. Embedders opt into discovery policy

**Choice:** A host may provide desktop-style project reference expansion, but the embeddable default is host-context-only with unsupported-reference receipts for ambient filesystem/network references.

**Rationale:** Animus should be safe for applications that supply their own context and do not want project directory discovery.

## Risks / Trade-offs

- Reference parsing can accidentally become permission granting. Tests must keep parser output separate from resolver authority.
- URL expansion overlaps retrieval budgets. This change should depend on retrieval-budget policy and reuse those stop/bound decisions.
- Diffs and images are host-specific. Keep VCS and media decoding behind adapters.
