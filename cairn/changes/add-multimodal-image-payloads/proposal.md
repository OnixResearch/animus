# Proposal: Add multimodal image payload support

## Why

`adapt-clankers-context-references` lets Animus identify images as bounded context references, but provider dispatch still needs an explicit multimodal payload contract. Without one, image support can become adapter-specific, silently drop image parts during prompt assembly, bypass model capability metadata, or leak raw image bytes, data URLs, base64 payloads, private paths, or credentials into receipts.

Animus needs provider-neutral image input descriptors that reusable crates can validate and order deterministically while host/provider adapters own image loading, decoding, transcoding, and provider-native request serialization.

## What Changes

- Define typed multimodal message parts for text and image inputs, including safe source labels, media type, byte count, BLAKE3 content identity, trust/source metadata, and optional host-supplied image metadata.
- Extend prompt/model request assembly so admitted image references become ordered model-visible content parts without embedding raw bytes in reusable runtime state or receipts.
- Gate image dispatch on provider/model input-modality metadata and fail closed when a selected model or provider adapter cannot accept images.
- Keep image bytes, MIME sniffing, decoding, resizing/transcoding, and provider-specific URL/base64 payload serialization in host/provider adapters behind injected services.
- Add provider-native fixture tests for image request serialization and negative tests for unsupported models, malformed media descriptors, missing image services, oversized images, lossy conversions, and receipt leak attempts.

## Impact

- **Files**: future multimodal DTOs, prompt assembly contracts, provider capability checks, provider adapter fixtures, receipt/ledger metadata, docs, and Cairn artifacts.
- **Testing**: positive text+image request fixtures and negative unsupported-model/malformed-descriptor/missing-service/oversize/lossy-conversion/leak fixtures.
- **Non-claims**: this does not add OCR, image generation, unrestricted image loading, live provider calls, browser screenshots, local vision models, or raw image persistence in session ledgers.
