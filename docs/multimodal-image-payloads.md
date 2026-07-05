# Multimodal image payloads

Animus supports image input as provider-neutral model content descriptors. The reusable adapter contract validates safe metadata and ordering; host and provider adapters own bytes, media processing, and provider-native serialization.

## Boundary contract

- `animus-adapters` owns typed text/image content parts, validation errors, image receipt metadata, provider capability gates, and provider-native fixture DTOs.
- Reusable DTOs store safe labels, supported media type, byte count, BLAKE3 content identity, source trust, optional declared dimensions/detail, and safe adapter classes.
- Reusable DTOs never store raw image bytes, base64 payloads, data URLs, credentials, private absolute paths, raw provider payloads, or unbounded image/tool output.
- Host/provider adapters own image loading, MIME sniffing, decoding, resizing, transcoding, URL/file-handle resolution, upload tokens, provider wire formats, and live provider calls.
- Dispatch preparation fails closed when the selected model lacks image input metadata or the image adapter/service is unavailable; it must not switch providers or silently downgrade to text-only input.

## Prompt assembly

`assemble_model_content_parts` validates ordered text and image drafts, applies deterministic per-image byte budgets, and returns explicit image outcomes. Included images remain ordered with surrounding text. Over-budget images are represented by rejected outcomes instead of disappearing from review evidence.

## Provider-native fixtures

`serialize_model_provider_native_fixture` requires native fixture mode for image requests and an injected `ModelImagePayloadService`. Generic JSON conversion is rejected for image-bearing prompts because provider APIs differ across file IDs, approved handles, upload tokens, and detail settings.

## Safe receipts

`ModelImageReceipt` records only safe image provenance: modality, status, safe label, media type, byte count, optional dimensions, adapter class, redaction marker, and BLAKE3 identity/receipt hashes. Receipt validation rejects leak sentinels for raw bytes, base64/data URLs, full private paths, credentials, provider payloads, and unbounded metadata.

## Verification

Focused tests live in `crates/animus-adapters/src/model/multimodal.rs` and cover positive DTO validation, negative malformed descriptors, prompt ordering, over-budget rejection, provider capability gates, missing-service and lossy-conversion failures, provider-native fixtures, live-call-required failures, and receipt leak rejection.

Run:

```sh
cargo test -p animus-adapters multimodal
cargo test -p animus-adapters model
```
