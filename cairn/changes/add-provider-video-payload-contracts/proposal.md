## Why

Animus supports ordered text and image model input descriptors, but its canonical model content contract has no video part. The only current video type is a generic delivery-artifact classification. Treating that label as provider video support would omit media metadata, capability negotiation, payload services, budgets, provider-native serialization, and safe evidence. Provider video input needs its own bounded contract without adding decoding, URL resolution, file loading, or live-provider authority to reusable crates.

## What Changes

- Add provider-neutral typed video content descriptors with safe metadata, trust classification, BLAKE3 identity, and named bounds. r[agent_harness_runtime.video_payloads.typed_parts]
- Extend ordered prompt assembly and model capability negotiation with an explicit video content class and fail-closed downgrade policy. r[agent_harness_runtime.video_payloads.assembly] r[agent_harness_runtime.video_payloads.capabilities]
- Add an injected provider-video payload service for approved handles, uploads, and provider-native fixture serialization. r[agent_harness_runtime.video_payloads.payload_service]
- Add metadata-only video receipts, positive and negative provider fixtures, dependency isolation, and focused verification. r[agent_harness_runtime.video_payloads.evidence] r[agent_harness_runtime.video_payloads.verification]

## Impact

- **Files**: multimodal adapter contracts and tests, provider capability data and planning, runtime provider hardening, Nickel policy, provider-native fixtures, safe receipts, component and facade inventories, README links, and focused documentation and checks.
- **Dependencies**: reusable crates gain typed metadata only; provider clients, video codecs, media frameworks, file loaders, URL clients, and terminal packages remain outside the reusable surface.
- **Testing**: positive ordered video requests and provider fixtures; negative malformed, oversized, unsupported, missing-service, lossy-conversion, downgrade, and receipt-leak cases; no-std and Wasm checks; Cairn validation and gates.

## Non-Goals

- Video decoding, transcoding, thumbnail generation, playback, terminal graphics, audio extraction, persistence, or live-provider calls.
- A universal provider wire format or support for providers that lack an explicitly reviewed video input surface.
- Raw video bytes, base64, data URLs, private paths, upload credentials, or provider payloads in reusable DTOs or receipts.
