## Phase 1: Baseline and provider inventory

- [ ] [serial] Run and record current positive and negative multimodal image, canonical message, capability matrix, provider-hardening, no-std, Wasm, safe-receipt, and Cairn baselines. r[agent_harness_runtime.video_payloads.verification]
- [ ] [serial] Inventory selected provider video input APIs, accepted media and handle forms, limits, capability claims, provider-native fixture shapes, and exact non-claims without making live calls. r[agent_harness_runtime.video_payloads.capabilities] r[agent_harness_runtime.video_payloads.payload_service]
- [ ] [serial] Define a typed Nickel video profile with named descriptor, byte, duration, dimension, frame-rate, count, aggregate-media, provider-handle, and receipt bounds; export checked runtime data and Rust parity fixtures. r[agent_harness_runtime.video_payloads.typed_parts] r[agent_harness_runtime.video_payloads.assembly]

## Phase 2: Typed contracts and pure assembly

- [ ] [serial] Add video media, trust, detail, declared-metadata, source, outcome, error, safe handle, receipt, and draft DTOs without raw bytes, base64, paths, URLs, credentials, or provider payload bodies. r[agent_harness_runtime.video_payloads.typed_parts]
- [ ] [serial] Extend ordered multimodal prompt assembly with text, image, and video parts; enforce local and aggregate budgets and explicit included, rejected, omitted, and unsupported outcomes. r[agent_harness_runtime.video_payloads.assembly]
- [ ] [parallel] Add positive ordering and mixed-media assembly tests plus negative malformed metadata, unsupported media, zero or excessive dimensions, invalid duration, invalid frame-rate, duplicate identity, count overflow, and aggregate-budget tests. r[agent_harness_runtime.video_payloads.typed_parts] r[agent_harness_runtime.video_payloads.assembly] r[agent_harness_runtime.video_payloads.verification]

## Phase 3: Capability and provider-native seams

- [ ] [serial] Add a separate video content class to model capability negotiation and explicit required-video and optional-omission policy outcomes without silent provider switching. r[agent_harness_runtime.video_payloads.capabilities]
- [ ] [serial] Add an injected provider-video payload service for approved source handles, provider-native safe handles, and bounded metadata-only service outcomes. r[agent_harness_runtime.video_payloads.payload_service]
- [ ] [serial] Extend provider-native fixture serialization for each selected exact provider profile and reject generic JSON conversion for video-bearing requests. r[agent_harness_runtime.video_payloads.payload_service] r[agent_harness_runtime.video_payloads.capabilities]
- [ ] [parallel] Add positive provider fixture tests and negative unsupported model, media mismatch, missing service, expired or malformed safe handle, oversized upload, lossy conversion, implicit downgrade, and unauthorized live-call cases. r[agent_harness_runtime.video_payloads.capabilities] r[agent_harness_runtime.video_payloads.payload_service] r[agent_harness_runtime.video_payloads.verification]

## Phase 4: Evidence and closeout

- [ ] [parallel] Add deterministic metadata-only BLAKE3 receipts and leak negatives for bytes, base64, data URLs, private paths, credentials, upload tokens, provider payloads, frame bodies, audio bodies, and unbounded metadata. r[agent_harness_runtime.video_payloads.evidence] r[agent_harness_runtime.video_payloads.verification]
- [ ] [serial] Update the component catalog, facade inventory, request lifecycle, README, multimodal documentation, provider profile guidance, and focused Cairn/Nix rail. r[agent_harness_runtime.video_payloads.evidence]
- [ ] [serial] Run focused tests, exhaustive-match compile checks, dependency isolation, no-std and Wasm checks, Nickel checks, Cairn validation, and proposal/design/tasks gates; record skipped live-provider and media-processing checks. r[agent_harness_runtime.video_payloads.verification]
