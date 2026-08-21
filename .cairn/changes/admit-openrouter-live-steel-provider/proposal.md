## Why

The live Steel agent shell accepts only the official OpenAI Responses cohort. The lower-level Animus provider host already admits exact OpenAI and OpenRouter endpoint pairs, but the product configuration rejects OpenRouter before composition.

A product that selects OpenRouter must not bypass the existing shell, weaken endpoint checks, or add provider discovery.

## What Changes

- Admit either the exact OpenAI or exact OpenRouter Responses cohort in the live Steel agent configuration boundary.
- Bind each provider identity to its matching endpoint identity, endpoint URL, and injected credential reference.
- Keep crossed endpoints, arbitrary endpoints, credential drift, discovery, and fallback fail closed.
- Add positive and negative configuration fixtures for both admitted cohorts.

## Impact

- **Files**: `crates/animus-live-steel-agent`, its configuration tests, and live-agent documentation
- **Testing**: focused live-agent tests and Clippy, provider host rails, Cairn validation, and repository gates
