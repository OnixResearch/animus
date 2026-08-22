## Why

Animus Steel must use the reviewed DeepSeek model on the private `local-dspark` service. The existing live host admits only the official OpenAI and OpenRouter endpoint pairs, so it rejects this service before network contact.

## What Changes

- Admit one exact `local-dspark-responses` identity and private Responses URL pair.
- Permit cleartext HTTP only for that exact private endpoint.
- Project explicit disabled reasoning for this cohort so hidden-reasoning events do not enter the stream.
- Keep arbitrary endpoints, redirects, proxies, fallback, and endpoint discovery denied.
- Add positive, negative, and live smoke evidence for the exact DeepSeek model.

## Impact

- **Files**: `crates/animus-std-openai/`, provider-host docs, focused rails, and Cairn lifecycle artifacts
- **Testing**: focused provider tests, strict Clippy, formatting, Cairn gates, Octet, and one sanitized live `local-dspark` smoke
