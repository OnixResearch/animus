# Proposal: Adapt Zerostack provider registry hardening for Animus

## Why

Zerostack's provider layer handles several practical provider issues: OpenAI Responses versus Chat Completions routing, custom OpenAI-compatible gateways, provider-specific request body parameters, safe header expansion, model listing, and filtering non-agent models. Animus already has a provider-registry shell change; this Cairn narrows the Zerostack-specific hardening patterns that should be adapted there.

The goal is robust host-owned provider construction without moving credentials, HTTP clients, or provider config into reusable runtime core crates.

## Scope

- Add API-style metadata for OpenAI Responses and Chat Completions compatible routes.
- Add safe custom header and environment-reference descriptors with redacted diagnostics.
- Add provider-specific extra request body policy that is tied to selected provider/model metadata.
- Add dynamic model listing normalization and non-agent model filtering.
- Add provider selection receipts that include safe endpoint identity, API style, capability metadata, and redacted credential status.

## Impact

- **Files**: provider registry DTOs, std-shell provider adapters, Nickel registry contracts, tests, and docs.
- **Testing**: positive OpenAI-compatible gateway and metadata tests; negative missing credential, malformed header, invalid extra body, and non-agent model selection tests.
- **Non-claims**: this does not copy Zerostack's Rig-based implementation, require Rig, expose raw credentials, or construct provider clients in reusable runtime crates.
