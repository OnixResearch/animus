# agent-harness-runtime Change Spec: Multimodal image payload support

## Requirements

### Requirement: Typed multimodal image parts

r[agent_harness_runtime.multimodal_image_payloads.typed_parts] Animus MUST represent image inputs as typed multimodal content parts with safe source labels, media type, byte count, BLAKE3 content identity, source trust metadata, and optional host-supplied image metadata, and MUST NOT place raw image bytes, base64 payloads, data URLs, credentials, or private absolute paths in reusable runtime content DTOs.

#### Scenario: Valid image descriptor is admitted

r[agent_harness_runtime.multimodal_image_payloads.typed_parts.validation]
- GIVEN a host-admitted image reference has a safe label, supported image media type, byte count, BLAKE3 content identity, and source trust metadata
- WHEN multimodal content-part validation runs
- THEN Animus accepts an image content part that can be ordered with text parts
- AND the reusable runtime DTO contains no raw image bytes, base64 payload, data URL, credential, or private absolute path.

#### Scenario: Malformed image descriptor is rejected

r[agent_harness_runtime.multimodal_image_payloads.typed_parts.invalid]
- GIVEN an image descriptor has an invalid media type, missing content identity, missing byte count, raw bytes, base64, a data URL, or a private absolute path
- WHEN multimodal content-part validation runs
- THEN Animus rejects the descriptor with a typed diagnostic
- AND no provider dispatch can be constructed from that descriptor.

### Requirement: Prompt assembly preserves image order and outcomes

r[agent_harness_runtime.multimodal_image_payloads.prompt_assembly] Prompt/model request assembly MUST preserve deterministic ordering of text and image content parts after policy admission and budget decisions, and MUST report explicit included, rejected, or unavailable outcomes for every requested image part before provider dispatch.

#### Scenario: Mixed text and image prompt keeps order

r[agent_harness_runtime.multimodal_image_payloads.prompt_assembly.order]
- GIVEN an admitted prompt contains text before and after an image part
- WHEN prompt assembly builds model-visible content
- THEN the assembled request preserves the relative text and image ordering
- AND any rejected or unavailable image part is represented by an explicit outcome instead of being silently dropped.

### Requirement: Image dispatch is capability gated

r[agent_harness_runtime.multimodal_image_payloads.capability_gate] Provider dispatch MUST require selected provider/model capability metadata that advertises image input support before sending image content parts, and MUST fail closed when image support is absent, disabled, or unavailable.

#### Scenario: Unsupported model rejects image request

r[agent_harness_runtime.multimodal_image_payloads.capability_gate.unsupported]
- GIVEN an assembled request contains an image content part
- AND the selected model metadata does not advertise image input support
- WHEN dispatch preparation runs
- THEN Animus returns a typed unsupported-modality diagnostic
- AND it does not silently switch providers, downgrade to text-only input, or call a provider adapter.

### Requirement: Host adapters own image bytes and native serialization

r[agent_harness_runtime.multimodal_image_payloads.adapter_serialization] Image byte loading, MIME sniffing, decoding, resizing, transcoding, remote handle resolution, and provider-specific URL/base64/file-handle serialization MUST live in host/provider adapters behind injected services, while reusable runtime crates validate descriptors and dispatch decisions only.

#### Scenario: Provider-native fixture serializes image request

r[agent_harness_runtime.multimodal_image_payloads.adapter_serialization.fixtures]
- GIVEN a provider adapter receives a validated image descriptor and an approved injected image payload service
- WHEN provider request serialization runs against a fixture
- THEN the adapter emits the provider-native image request shape expected by that fixture
- AND tests fail if serialization relies on lossy generic JSON conversion, requires a live provider call, or omits the image part.

### Requirement: Safe image receipts and ledger metadata

r[agent_harness_runtime.multimodal_image_payloads.safe_receipts] Multimodal receipts and session-ledger records MUST store safe image provenance only, including modality, status, safe label or hash, media type, byte count, optional dimensions, truncation or rejection status, adapter class, and BLAKE3 content identity, and MUST reject raw image bytes, base64/data URLs, full private paths, credentials, and unbounded provider payloads.

#### Scenario: Image metadata leak is rejected

r[agent_harness_runtime.multimodal_image_payloads.safe_receipts.rejects_leaks]
- GIVEN an image receipt fixture contains raw image bytes, base64, a data URL, a full private path, a credential-looking value, or an unbounded provider payload
- WHEN receipt metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic
- AND the invalid fixture is not accepted as durable lifecycle evidence.

### Requirement: Multimodal image verification

r[agent_harness_runtime.multimodal_image_payloads.verification] The adaptation MUST include focused positive and negative tests for image content-part validation, prompt ordering, provider capability gating, host image-service injection, provider-native serialization fixtures, unsupported models, malformed descriptors, oversized inputs, missing services, lossy conversions, and safe receipt validation.

#### Scenario: Closeout evidence covers image payload rails

r[agent_harness_runtime.multimodal_image_payloads.verification.closeout]
- GIVEN maintainers intend to archive the multimodal image payload adaptation
- WHEN closeout validation runs
- THEN evidence names focused content-part, prompt assembly, capability gate, adapter serialization, receipt leak, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
