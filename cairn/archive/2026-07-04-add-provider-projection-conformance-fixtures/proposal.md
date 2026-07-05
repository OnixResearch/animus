# Proposal: Add provider projection conformance fixtures

## Why

Canonical model requests and negotiated capabilities are only useful if provider adapters preserve the same semantics when projecting to provider-native payloads. Provider families differ in message shape, tool-call representation, structured output, image references, reasoning controls, streaming chunks, stop reasons, and usage reporting.

Animus needs a conformance rail that proves each claimed provider projection preserves required canonical fields and fails closed when conversion would be lossy.

## What Changes

- Define a provider projection conformance harness for canonical request to provider-native request fixtures and provider-native response to canonical response fixtures.
- Add literal expected payload fixtures for claimed provider families instead of constructing expected values with the same builder under test.
- Cover OpenAI Responses, OpenAI-compatible chat/local HTTP, Anthropic Messages, and mock/local families according to explicit support claims.
- Add response normalization fixtures for assistant text, tool calls, structured payloads, refusals, usage, finish reasons, malformed payloads, and leak sentinels.
- Add streaming parser-entrypoint fixture coverage for any provider family that claims stream support.

## Impact

- **Files**: provider projection fixture harness, provider-family fixture data, conformance index/checker, docs, and closeout evidence.
- **Testing**: positive provider-native projection fixtures and negative lossy/malformed/leaky conversion fixtures.
- **Non-claims**: this does not add live provider clients, network calls, OAuth, credential lookup, provider discovery, or benchmark claims.
