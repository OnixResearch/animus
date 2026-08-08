## Context

The existing multimodal image contract separates safe descriptors from host-owned bytes and provider-native serialization. Video input has similar authority boundaries but adds duration, container, codec, frame geometry, optional audio, larger payloads, and provider-specific upload behavior. Reusing `DeliveryArtifactKind::Video` would confuse an output classification with an admitted model input. Extending the image variant would hide video-specific limits and capability checks.

## Decisions

### Decision: add a distinct canonical video content part

**Choice:** Add a video content part with safe label, admitted media type, byte count, BLAKE3 content identity, source trust, and optional declared duration, dimensions, frame rate class, audio-presence marker, and detail preference. Declared media facts remain metadata claims until a host service validates them.

**Rationale:** A distinct variant preserves ordered multimodal prompts and prevents image-only logic from silently accepting video.

### Decision: keep all media effects host-owned

**Choice:** Reusable contracts never load paths or URLs, inspect containers, decode frames, transcode media, create thumbnails, upload files, refresh tokens, call providers, or retain raw bytes. A host-injected video payload service resolves approved handles and returns provider-native safe handles plus metadata-only outcomes.

**Rationale:** Video processing has filesystem, network, process, codec, resource, and credential authority that does not belong in reusable adapter DTOs.

### Decision: negotiate video as an exact capability

**Choice:** Provider capability matrices gain a separate video content class and reviewed media constraints. Dispatch fails closed when video is required and the selected model, service, media type, size, duration, or provider projection is unsupported. Optional omission requires explicit policy and a visible downgrade record.

**Rationale:** Image support does not imply video support, and provider APIs differ materially.

### Decision: reject generic JSON conversion for video

**Choice:** Video-bearing requests require a provider-native fixture or live adapter lane with an injected payload service. Generic conversion returns a typed lossy or unsupported outcome.

**Rationale:** Providers use incompatible file IDs, upload sessions, handles, URLs, and content schemas.

### Decision: use named local and aggregate budgets

**Choice:** Typed Nickel policy owns descriptor, byte, duration, dimension, frame-rate, video-count, aggregate-media, provider-handle, and receipt limits. Pure assembly returns explicit included, rejected, omitted, or unsupported outcomes without silent clamping.

**Rationale:** Video payloads can exhaust memory, context, upload, or billing limits much faster than text or images.

## Risks / Trade-offs

- Provider video APIs and accepted media types can change; exact provider profiles and fixtures limit this risk.
- Declared metadata can differ from the media bytes; only host inspection can strengthen that claim.
- Adding a canonical variant affects exhaustive matches; focused compile and negative tests must expose missing handling.

## Non-Claims

This design does not establish media authenticity, codec safety, successful decoding, upload completion, provider acceptance, semantic understanding, playback, audio correctness, privacy, model correctness, production readiness, or release eligibility.
