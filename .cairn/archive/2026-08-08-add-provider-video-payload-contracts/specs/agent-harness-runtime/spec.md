# Agent Harness Runtime

## ADDED Requirements

### Requirement: Canonical model content includes typed video parts

r[agent_harness_runtime.video_payloads.typed_parts] Animus MUST represent provider-neutral video model input through a distinct typed content part with safe metadata, source trust, admitted media type, byte count, BLAKE3 identity, named bounds, and explicit validation outcomes.

#### Scenario: Valid video descriptor is admitted

r[agent_harness_runtime.video_payloads.typed_parts.valid]
- GIVEN a video draft has a safe label, admitted media type, bounded byte count, valid BLAKE3 identity, source trust, and valid optional declared metadata
- WHEN reusable validation runs
- THEN it MUST return a typed video content part with the admitted safe fields
- AND declared duration, dimensions, frame rate, codec, container, and audio presence MUST remain metadata claims unless a host validates them.

#### Scenario: Unsafe video descriptor is supplied

r[agent_harness_runtime.video_payloads.typed_parts.invalid]
- GIVEN a video draft contains raw bytes, base64, a data URL, private path, credential, provider payload, malformed identity, unsupported media type, invalid declared metadata, or an exceeded bound
- WHEN reusable validation runs
- THEN it MUST return a typed rejection
- AND the unsafe material MUST NOT enter canonical content, receipts, or diagnostics.

### Requirement: Multimodal assembly preserves ordered video outcomes

r[agent_harness_runtime.video_payloads.assembly] Animus MUST assemble ordered text, image, and video content under named local and aggregate budgets and MUST preserve explicit included, rejected, omitted, and unsupported outcomes for every video draft.

#### Scenario: Mixed prompt is assembled

r[agent_harness_runtime.video_payloads.assembly.ordered]
- GIVEN ordered text, image, and video drafts fit all selected policy bounds
- WHEN pure multimodal assembly runs
- THEN it MUST preserve the original admitted order in canonical content
- AND it MUST emit one ordered video outcome for each video draft.

#### Scenario: Video exceeds aggregate policy

r[agent_harness_runtime.video_payloads.assembly.over_budget]
- GIVEN a video or mixed-media prompt exceeds a named local or aggregate bound
- WHEN pure assembly runs
- THEN the affected video MUST receive an explicit rejected or omitted outcome according to policy
- AND it MUST NOT disappear, clamp silently, or become text.

### Requirement: Video capability negotiation fails closed

r[agent_harness_runtime.video_payloads.capabilities] Model capability matrices MUST represent video input separately from image and audio classes and MUST fail closed when required video media, limits, payload service, or provider projection is unavailable.

#### Scenario: Selected model supports the request

r[agent_harness_runtime.video_payloads.capabilities.supported]
- GIVEN the selected model profile admits video input, media type, request limits, provider projection, and payload service
- WHEN dispatch preparation runs
- THEN it MUST preserve the video part and bind the exact selected capability facts
- AND image support alone MUST NOT satisfy video capability.

#### Scenario: Required video is unsupported

r[agent_harness_runtime.video_payloads.capabilities.unsupported]
- GIVEN a request requires video and the selected model or provider lacks an admitted video capability
- WHEN dispatch preparation runs
- THEN it MUST return a typed unsupported-capability outcome before a provider call
- AND it MUST NOT switch providers, omit the video, convert it to text, or treat it as an image unless explicit reviewed policy authorizes that separate outcome.

### Requirement: Provider video payload service is injected

r[agent_harness_runtime.video_payloads.payload_service] Video bytes, media inspection, path or URL resolution, upload sessions, provider handles, credentials, transcoding, thumbnails, and live calls MUST remain behind a host-injected provider-video payload service.

#### Scenario: Provider-native fixture uses a safe handle

r[agent_harness_runtime.video_payloads.payload_service.fixture]
- GIVEN a validated video part, exact provider profile, and injected fixture service
- WHEN provider-native fixture serialization runs
- THEN it MUST use an approved bounded safe handle and selected provider schema
- AND reusable DTOs MUST retain no bytes, upload token, credential, private path, URL body, or provider response body.

#### Scenario: Service is missing

r[agent_harness_runtime.video_payloads.payload_service.missing]
- GIVEN a video-bearing request has no injected payload service
- WHEN provider serialization or live dispatch preparation runs
- THEN it MUST fail with a typed missing-service outcome
- AND generic JSON conversion or ambient media resolution MUST NOT occur.

### Requirement: Video conversion loss stays visible

r[agent_harness_runtime.video_payloads.loss] Provider projection MUST report unsupported, omitted, transformed, unresolved, or lossy video fields and MUST reject a successful lossless claim when the selected provider shape cannot represent admitted canonical intent.

#### Scenario: Provider cannot represent a video field

r[agent_harness_runtime.video_payloads.loss.provider]
- GIVEN canonical video intent includes a field or ordering constraint that the selected provider profile cannot represent
- WHEN provider projection runs
- THEN it MUST return a typed loss report or fail according to policy
- AND it MUST NOT silently remove or rewrite the intent.

### Requirement: Video receipts are safe and deterministic

r[agent_harness_runtime.video_payloads.evidence] Video receipts MUST bind safe modality, media, byte and declared-metadata counts, status, service class, provider profile, redaction state, BLAKE3 identities, loss classes, and explicit non-claims without retaining content or authority-bearing values.

#### Scenario: Safe video receipt is produced

r[agent_harness_runtime.video_payloads.evidence.safe]
- GIVEN a video draft reaches an assembly or provider-projection outcome
- WHEN receipt projection runs
- THEN it MUST emit only admitted metadata, bounded counts, safe labels, status, loss classes, and BLAKE3 identities
- AND the receipt identity MUST be deterministic over those typed fields.

#### Scenario: Receipt leaks video material

r[agent_harness_runtime.video_payloads.evidence.leak]
- GIVEN a receipt candidate contains video bytes, base64, data URLs, private paths, credentials, upload tokens, provider payloads, frame or audio bodies, or unbounded metadata
- WHEN receipt validation runs
- THEN it MUST reject the candidate with a typed leak diagnostic
- AND the candidate MUST NOT become durable evidence.

### Requirement: Video payload verification includes failure paths

r[agent_harness_runtime.video_payloads.verification] Provider video payload support MUST include positive and negative DTO, assembly, capability, service, provider-fixture, loss, bounds, receipt, dependency-isolation, no-std, Wasm, and lifecycle checks before archive.

#### Scenario: Change reaches closeout

r[agent_harness_runtime.video_payloads.verification.closeout]
- GIVEN implementation and fixtures are complete
- WHEN closeout runs
- THEN focused tests, dependency checks, Nickel checks, no-std and Wasm checks, Cairn validation, and all Cairn gates MUST pass
- AND skipped live-provider, upload, decoding, playback, audio, production, and release checks MUST remain explicit non-claims.
