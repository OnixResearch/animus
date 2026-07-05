# agent-harness-runtime Change Spec: Canonical model message contracts

## Requirements

### Requirement: Canonical model message request

r[agent_harness_runtime.model_compat.message_request] Animus MUST define a versioned provider-neutral model message request contract that carries ordered model messages, stable role labels, typed content parts, output controls, tool declarations, response-format intent, reasoning or thinking request metadata, contract identity, and safe provenance metadata without embedding raw provider payloads.

#### Scenario: Canonical request validation admits structured conversations

r[agent_harness_runtime.model_compat.message_request.validation]
- GIVEN a host or runtime shell prepares a model request with ordered system, developer, user, assistant, or tool messages and validated text or descriptor-backed content parts
- WHEN canonical model request validation runs
- THEN valid requests are admitted with stable contract identity and bounded metadata
- AND malformed roles, empty messages, unsupported content parts, oversized metadata, raw provider payloads, credentials, and missing required fields fail before provider dispatch.

### Requirement: Canonical model response envelope

r[agent_harness_runtime.model_compat.response_envelope] Animus MUST define a versioned provider-neutral model response envelope that can represent assistant-visible text, structured payloads, tool-call requests, refusal or safety blocks, usage metadata, finish reasons, and malformed-output diagnostics without exposing raw provider payloads.

#### Scenario: Response semantics survive provider normalization

r[agent_harness_runtime.model_compat.response_envelope.validation]
- GIVEN a provider or fixture adapter returns assistant text, a structured payload, a tool-call request, a refusal, usage metadata, or a terminal finish reason
- WHEN response normalization validates the canonical envelope
- THEN supported response semantics are preserved as typed data
- AND missing finish reasons, malformed usage counts, impossible response combinations, raw provider payloads, credentials, and unbounded hidden reasoning fail closed.

### Requirement: Legacy model DTO bridge

r[agent_harness_runtime.model_compat.legacy_bridge] Existing simple prompt/message model adapter behavior MUST remain available through pure compatibility projections between legacy DTOs and canonical model request/response contracts, and those projections MUST NOT add provider, filesystem, network, credential, process, or plugin authority.

#### Scenario: Legacy prompt path is equivalent under compatibility defaults

r[agent_harness_runtime.model_compat.legacy_bridge.equivalent]
- GIVEN one runtime path dispatches a legacy prompt/message DTO and another dispatches the equivalent canonical single-user-message request with compatibility defaults
- WHEN both receive deterministic assistant-text responses
- THEN both produce equivalent successful reports and events
- AND canonical responses carrying structured output, tool calls, refusal status, or other non-text semantics fail rather than being silently collapsed into legacy message text.

### Requirement: Canonical model contract verification

r[agent_harness_runtime.model_compat.verification] Canonical model contract work MUST include focused positive and negative tests for request validation, response validation, legacy bridge equivalence, lossy projection rejection, safe metadata redaction, boundary schema notes, runtime facade inventory, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers canonical model contracts

r[agent_harness_runtime.model_compat.verification.closeout]
- GIVEN maintainers intend to archive canonical model message contract work
- WHEN closeout validation runs
- THEN evidence names request validation, response validation, legacy bridge equivalence, lossy projection rejection, safe metadata redaction, docs, schema notes, facade inventory, workspace checks, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
