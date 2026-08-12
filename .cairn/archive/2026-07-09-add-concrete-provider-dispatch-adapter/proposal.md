# Proposal: Add concrete provider dispatch adapter

## Why

Animus has canonical model contracts, provider registry metadata, capability negotiation, provider projection fixtures, and streaming parser seams, but it still does not claim a concrete provider dispatch adapter. Provider execution is one of the dogfood readiness blockers: without a host-owned dispatch path, live-provider behavior cannot be tested or explicitly excluded beyond projection fixtures.

A bounded provider adapter should turn an already-admitted canonical model request into one provider-family dispatch plan while preserving fail-closed credential, service, capability, timeout, cancellation, and receipt behavior.

## What Changes

- Add a host-owned provider dispatch adapter contract and deterministic fixture implementation for one selected provider family.
- Require provider registry selection, model capability negotiation, request-shape projection, credential handle readiness, cancellation/deadline metadata, and receipt policy before dispatch.
- Add shared request-field parity and constructor inventory rails for concrete provider request construction.
- Preserve known-provider-prefix fail-closed behavior when credentials, entitlements, backend status, or service injection are absent.
- Keep live-provider smoke optional and separately indexed.

## Impact

- **Files**: std-shell provider dispatch DTOs, adapter trait or service shell, deterministic request/response fixtures, parity rails, docs, tests, and Cairn evidence.
- **Testing**: positive fixture dispatch and provider-native request literal tests plus negative missing credential, disabled backend, denied capability, unsupported feature, malformed response, timeout, cancellation, known-prefix fallback, and receipt leak cases.
- **Non-claims**: this does not add default provider discovery, OAuth, credential storage, router daemon behavior, broad provider parity, network fallback, or mandatory live-provider checks.
