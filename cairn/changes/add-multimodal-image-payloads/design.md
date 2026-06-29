# Design: Multimodal image payload support

## Context

Context-reference work gives Animus a way to identify images, but model calls need a separate provider-neutral payload boundary. The reusable runtime should understand validated content parts and safe provenance. Host/provider adapters should own every operation that requires bytes, media libraries, network URLs, or provider-specific wire formats.

## Decisions

### 1. Image inputs are typed content descriptors

**Choice:** Represent model input as ordered content parts. Image parts carry safe labels, media type, byte count, BLAKE3 content hash, source trust metadata, and optional host-supplied metadata such as dimensions or detail preference. Reusable crates never store raw image bytes, base64, data URLs, or private paths in the content part.

**Rationale:** Deterministic descriptors let prompt assembly, budgets, receipts, replay, and provider gating reason about image inputs without inheriting host filesystem or media-decoding authority.

### 2. Prompt assembly preserves multimodal order

**Choice:** Prompt assembly produces an ordered sequence of text and image parts after policy admission and budget checks. Any truncation, rejection, or image omission is explicit in the returned assembly outcome and receipt metadata.

**Rationale:** Multimodal providers are sensitive to part ordering. Silent text-only fallback would make replay and review misleading.

### 3. Provider capability gates image dispatch

**Choice:** The selected model/provider metadata must advertise image input support before dispatch. Unsupported image inputs fail closed with a typed diagnostic instead of falling back to another model or dropping image parts.

**Rationale:** Provider registries already model input modalities. Image support should use those capabilities rather than adding adapter-specific exceptions.

### 4. Host adapters own bytes and provider-native serialization

**Choice:** Host/provider adapters receive validated descriptors plus an injected image payload service that can resolve approved bytes or provider-ready handles. Provider adapters serialize to provider-native request shapes using explicit fixtures.

**Rationale:** Image loading, MIME sniffing, resizing, transcoding, remote URL handling, and provider wire formats are side-effectful and platform-specific. Keeping them in the shell preserves the functional-core / imperative-shell boundary.

### 5. Receipts record safe image provenance only

**Choice:** Receipts and session ledgers record modality, status, safe label or hash, media type, byte count, optional dimensions, truncation/rejection status, adapter class, and BLAKE3 identity. They reject raw image bytes, base64/data URLs, absolute path dumps, credentials, and unbounded provider payloads.

**Rationale:** Image requests can carry private screenshots and documents. Evidence must prove what was sent without committing sensitive pixels or transport payloads.

## Risks and Mitigations

- Provider APIs differ on URL handles, base64 blobs, file IDs, and detail settings. Use provider-native fixtures instead of a lossy generic JSON conversion.
- Image metadata can be spoofed by hosts. Treat host-supplied metadata as declared metadata and preserve content hashes for audit.
- Image parts can exceed context or provider limits. Budget decisions must reject or require host-side transformation before dispatch.
- Dataset export may need image references later. This change records safe identities and leaves raw artifact export to a separate admitted artifact workflow.
