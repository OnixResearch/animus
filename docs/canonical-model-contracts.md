# Canonical model message contracts

Animus keeps the legacy `ModelRequest { prompt }` and `ModelResponse { message }` DTOs for compatibility, and adds provider-neutral canonical model contracts for adapters that need structured conversation semantics before provider projection.

## Request boundary

`CanonicalModelRequest` validates `animus.model-message-request.v1` drafts with ordered `system`, `developer`, `user`, `assistant`, and `tool` messages. Content parts are typed as safe text or descriptor-backed image references; raw provider payloads, credentials, hidden reasoning, empty messages, unsupported content kinds, oversized metadata, and missing contract identities fail before provider dispatch.

Requests also carry tool declarations, response-format intent, output controls, reasoning/thinking request metadata, and safe provenance key/value metadata. The contract is reusable adapter data only: it does not discover providers, read credentials, open files, call networks, load plugins, or choose fallback routes.

## Response boundary

`CanonicalModelResponseEnvelope` validates `animus.model-response-envelope.v1` drafts with assistant text, structured payload labels, tool-call requests, refusal or safety blocks, usage metadata, finish reasons, and malformed-output diagnostics. Missing finish reasons, malformed usage, impossible success/refusal/malformed combinations, raw provider payloads, credentials, and hidden reasoning fail closed.

## Legacy bridge

`CanonicalModelRequest::from_legacy_request` and `CanonicalModelResponseEnvelope::from_legacy_response` preserve the simple prompt/message path under compatibility defaults. Reverse projection fails closed when canonical data contains tools, structured output, refusal/safety/malformed semantics, non-default output controls, reasoning requests, usage, provenance, or other fields the legacy DTOs cannot represent.

## Provider projection conformance

Claimed provider/API-style projections use `ProviderProjectionFamily`, `ProviderProjectionSupport`, `project_model_provider_request_fixture`, `normalize_model_provider_response_fixture`, `assert_model_provider_stream_fixture`, and `validate_model_provider_projection_index`. The conformance index is `.cairn/provider-projection-conformance-index.ncl`. Literal fixtures prove canonical roles, content parts, tool declarations, structured-output intent, reasoning controls, stream options, and safe provenance survive provider-family projection without generating expected payloads from the builder under test.

## Verification

Focused evidence:

```sh
nix develop -c cargo test -p animus-adapters model
nix develop -c cargo test -p animus-adapters provider_conformance
```

The focused tests cover valid structured requests, malformed roles, empty messages, unsupported content parts, oversized metadata, raw provider payload rejection, valid response variants, missing finish reasons, malformed usage, impossible combinations, hidden reasoning rejection, legacy equivalence/lossy projection rejection, provider literal projection fixtures, provider response normalization, provider leak sentinels, streaming parser-entrypoint fixtures, and provider conformance index validation.
