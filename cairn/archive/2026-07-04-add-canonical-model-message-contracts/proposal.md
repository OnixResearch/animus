# Proposal: Add canonical model message contracts

## Why

Animus currently exposes a simple `ModelRequest { prompt }` and `ModelResponse { message }` compatibility surface. That path is valuable, but it loses provider-relevant structure such as ordered roles, content parts, tool declarations, structured-output intent, sampling controls, usage metadata, refusal status, and finish reasons.

Model compatibility improves when adapters share one provider-neutral conversation contract before projecting to OpenAI Responses, OpenAI-compatible chat, Anthropic Messages, local HTTP, or future provider families.

## What Changes

- Define a versioned provider-neutral model conversation request contract with ordered messages, role labels, typed content parts, output controls, tool metadata, response-format intent, reasoning/thinking request, and safe provenance metadata.
- Define a versioned model response envelope that can represent assistant text, structured payloads, tool-call requests, refusal/safety blocks, usage metadata, and finish reasons without embedding provider-native raw payloads.
- Preserve the existing simple prompt/message path through an explicit compatibility bridge into and out of the canonical contract.
- Add positive and negative tests for valid conversation requests, malformed roles, empty messages, unsupported content parts, oversized metadata, unsafe provider payload leakage, and legacy equivalence.

## Impact

- **Files**: `animus-adapters` model DTOs, runtime dispatch compatibility defaults, Preserves/schema docs if exposed, provider-compatibility docs, and facade inventory.
- **Testing**: pure DTO validation tests plus legacy simple-model compatibility tests.
- **Non-claims**: this does not add live provider clients, network transport, OAuth, credential lookup, provider discovery, or a router daemon.
