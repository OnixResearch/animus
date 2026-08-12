# Proposal: Add packaged live OpenAI smoke

## Why

Animus has a bounded OpenAI host and an ignored live test. It does not have a packaged operator path that accepts an explicit model and credential channel.

The release evidence checker also accepts a live-provider claim from a stale optional receipt. A live result must match the indexed head before it supports that claim.

## What Changes

- Add a non-default `animus-live-openai-smoke` binary in `animus-std-openai`.
- Require the model as an exact argument and the API key through bounded standard input.
- Keep the endpoint, timeout, response limit, prompt, and credential label fixed and reviewed.
- Emit metadata-only evidence with BLAKE3 request and response identities.
- Reject missing, duplicate, unknown, oversized, or malformed input before provider contact.
- Require a passing current-head optional receipt before the release index can claim `live-providers`.
- Keep an absent live run as an explicit non-claim.

## Impact

- **Crate**: `animus-std-openai` gains a packaged binary and a smoke composition module.
- **Evidence**: the release evidence validator rejects stale live-provider receipts.
- **Docs**: the live host guide and dogfood roadmap name the packaged command and current-head indexing rule.
- **Testing**: positive metadata projection and negative input, leak, stale-receipt, and no-credential cases.
- **Non-claims**: no provider reliability, model quality, billing, production, retry, fallback, streaming, tool use, or credential storage claim.
