# Zerostack provider registry hardening

Animus adapts Zerostack provider ergonomics as host-shell validation contracts. Runtime core receives injected model services only; credentials, HTTP clients, provider config, environment expansion, and client construction remain host-owned.

The reusable surface lives in `animus-runtime::provider_hardening` and is re-exported by `animus-runtime`.

## API style

`resolve_provider_api_style` resolves OpenAI-family providers to explicit Responses or Chat Completions metadata before client construction. Custom gateways may use policy defaults; ambiguous style fails closed.

## Headers and extra body

`validate_provider_header` validates safe custom header names and redacts environment-backed diagnostics. Missing environment sources report only the source label, not resolved values. `resolve_extra_body_descriptor` admits provider/model-scoped extra body descriptors only when selected provider/model scope matches.

## Model filtering, capability matrix, and receipts

`filter_agent_model` rejects embedding, image, audio, moderation, rerank, and other non-agent model classes for text agent turns unless a host override is explicit.

`ProviderModelCapabilityMatrix` records canonical message compatibility for a selected model: supported roles, content part classes, tool-call support, structured-output modes, stream channels, reasoning controls, context/output budgets, privacy posture, and schema identity. `negotiate_model_capability` is pure over a canonical request, selected matrix, and host downgrade policy. It admits fully supported requests, records explicit optional downgrades, and denies unsupported required roles, images, tool calls, structured output, streaming, reasoning, budget overflow, unknown metadata, privacy mismatch, or provider fallback before adapter dispatch.

`provider_selection_receipt` records provider/model IDs, endpoint identity or BLAKE3 hash, API style, capability class, credential source class, extra descriptor IDs, and diagnostics without raw credentials or headers.

## Focused verification

Run:

```sh
nix develop -c cargo test -p animus-runtime provider_hardening
```

The tests cover first-party style, custom-gateway defaults, ambiguous style, invalid/missing headers, extra body scope, non-agent model rejection, capability admission, optional downgrades, fail-closed unsupported features, bad metadata, budget overflow, denied fallback, privacy mismatch, and sensitive endpoint hashing.
