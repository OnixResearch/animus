# Design: Add context adapter family

## Context

Current context support is split across several safe surfaces:

- `animus-adapters::prompt` parses and validates `ContextReference` values, plans resolver dispatch, applies deterministic budgets, and emits safe provenance metadata.
- `ContextResolverSet` groups host-injected file, directory, diff, image, and URL resolvers without giving reusable crates fallback authority.
- `animus-runtime::context_layering` evaluates already-loaded context descriptors and emits safe receipts without walking directories, reading files, checking environment variables, or canonicalizing paths.
- `animus-runtime::fresh_context_tasks` models isolated child-task context and citations without inheriting hidden parent authority.
- Prompt assembly can admit `HostContext` and `ContextReference` sources, but the maintained adapter-family index does not yet list context as a first-class family.

The implementation gap is therefore not raw capability; it is family-level packaging, conformance, std-shell implementations, and documentation.

## Decisions

### 1. Context is a formal adapter family

**Choice:** Add `family:context` to the public contracts index and adapter conformance index. The family covers context-reference resolvers, context-layer descriptor source/admission boundaries, external context handles, and safe prompt-source projection.

**Rationale:** Context crosses high-risk host authority boundaries. Treating it as a maintained family makes ownership, non-claims, tests, and safe evidence visible to embedders.

### 2. Keep role-specific resolver seams

**Choice:** Retain narrow resolver traits for file, directory, diff, image, and URL context, and introduce only small registry/outcome DTOs when needed to describe availability, capabilities, policy decisions, and typed failures. Do not add a single broad adapter that owns filesystem, VCS, image, network, and discovery behavior at once.

**Rationale:** Separate roles keep authority explicit and testable. A monolithic adapter would blur policy, make least-authority composition harder, and encourage ambient fallback.

### 3. Discovery remains host-owned

**Choice:** Local project/global context-file discovery, ignore handling, path canonicalization, VCS diff collection, image loading, URL fetching, and external handle resolution remain std-shell or host responsibilities. Reusable crates receive already-loaded descriptors or resolver outputs and validate them before prompt influence.

**Rationale:** Reusable crates must remain deterministic functional cores. Concrete host authority belongs in thin shells and adapter implementations.

### 4. Context outcomes use safe provenance only

**Choice:** Context adapter outcomes and receipts store safe target labels, resolver role, source kind, policy decision, status, bounded counts, truncation state, BLAKE3 content/body hashes, redaction markers, and omission reasons. They do not store raw prompt text, raw resolver bodies in durable evidence, credentials, provider payloads, absolute private paths, headers, environment values, hidden transcripts, or unbounded logs.

**Rationale:** Context frequently contains private source code and credentials. Durable evidence must prove decisions without becoming a data exfiltration path.

### 5. Std implementations are explicit opt-in shells

**Choice:** `animus-std` may provide no-op/null context services and policy-gated local file, directory, and diff resolvers. URL fetching, image decoding, repository-wide crawlers, and credentialed remote handles remain absent by default or downstream/feature-gated with their own policy checks.

**Rationale:** Embedders need deterministic local defaults, but network and rich media authority should never appear implicitly.

### 6. Conformance covers both existing and new context surfaces

**Choice:** Conformance evidence maps existing parser/planner/budget/provenance/layering tests and adds new tests for family registry, std resolvers, and adapter outcome classes.

**Rationale:** Existing tests already cover many context invariants. The change should consolidate them under the adapter family and fill gaps instead of duplicating logic.

## Risks / Trade-offs

- A formal family may look like a request for a broad `ContextAdapter`. Avoid this by documenting the role-specific seam as the success criterion.
- Std local resolvers could tempt ambient discovery. Keep them policy-gated and explicit: no resolver object means no context access.
- URL/image roles may remain interface-only for a while. Record unsupported or not-applicable cases with safe reasons until concrete adapters exist.
- Adding conformance rows before every std resolver exists requires precise evidence labels and non-claim wording.