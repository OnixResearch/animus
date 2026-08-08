# Provider video payload contracts

Animus has a provider-neutral video input contract in `animus-adapters`. The reusable code validates metadata and makes pure decisions. It does not read or process video bytes.

## Contract

`ModelContentPart::Video` keeps its position among text and image parts. A video descriptor contains these safe fields:

- a safe label;
- an admitted media type;
- a host-observed byte count;
- a lowercase BLAKE3 content identity;
- a source-trust class;
- optional declared duration, dimensions, frame rate, codec, container, audio presence, and detail preference.

Declared media facts are claims from the host. They are not results from media inspection.

The checked Nickel policy is `config/provider-video-payload-policy.ncl`. It owns the descriptor, prompt, aggregate-media, duration, dimension, frame-rate, video-count, handle, and receipt bounds. The generated runtime projection is `config/generated/provider-video-payload-policy.json`.

## Capability and downgrade rules

Video is a separate capability. Image support does not imply video support. Required video fails before a provider call if the selected model lacks one of these facts:

1. the video input class;
2. the exact media type and request limits;
3. an injected payload service;
4. an exact reviewed provider projection.

Animus does not switch providers or convert video to image or text. Optional omission requires `VideoCapabilityPolicy::OptionalOmissionAllowed`. The dispatch plan records that omission.

## Provider fixture inventory

The current checked cohort contains one exact fixture profile:

| Profile | Safe handle form | Fixture shape | Animus bounds | Claim |
| --- | --- | --- | --- | --- |
| `google-gemini-v1beta-files-fixture` | provider file identifier or approved remote handle label | ordered `fileData`-style part with media type and safe file handle | local Nickel policy | fixture serialization only |

This profile is a credential-free review fixture. It does not prove current API availability, upload success, provider acceptance, or production compatibility. A live adapter must review current provider documentation and bind its exact profile before dispatch.

Generic JSON conversion is not an accepted video lane. Provider-native serialization uses `VideoPayloadService`. The host owns file and URL resolution, media inspection, codecs, upload sessions, credentials, provider calls, cancellation, cleanup, and raw responses.

## Evidence

`VideoReceipt` contains metadata only. It binds safe modality, status, media type, bounded counts, service class, exact provider profile, redaction state, loss classes, non-claims, content identity, and a deterministic BLAKE3 receipt identity.

Receipt validation rejects raw bytes, base64, data URLs, remote URLs, private paths, credentials, provider payloads, frame bodies, audio bodies, and unbounded metadata.

## Verification

Run:

```text
scripts/cairn/check-provider-video-payloads --root .
```

The rail checks positive and negative Rust tests, Nickel policy drift, provider fixtures, Clippy, dependency isolation, no-default compilation, and Wasm compilation. It does not call a live provider or decode media.
