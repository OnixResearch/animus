# Design: Animus request lifecycle ownership map

## Context

Clankers keeps a request-lifecycle page as the reviewable map for prompt execution, event translation, persistence, replay, and provider request shaping. Animus should use the same pattern but with Animus-owned crates and smaller scope. The document is not an implementation; it is the contract that tells maintainers where a state transition belongs and which rail proves it.

## Decisions

### 1. The lifecycle document is the golden path for runtime ownership

The first version names these boundaries: `animus-std` config/runtime input, `animus-core` deterministic state, `animus-steel` loop decision, `animus-adapters` model/tool DTOs, std-shell injected adapter dispatch, runtime event/report projection, safe receipt construction, and future host ledger persistence.

### 2. Each boundary names a source-of-truth type

Every section must name the type or trait that owns the transition. If a future change creates a new runtime DTO, the lifecycle document must be updated in the same change.

### 3. Forbidden dependency rules are part of ownership

The document must call out which crates may not read environment, filesystem, credentials, provider config, network, clocks, or host persistence. This keeps functional-core / imperative-shell boundaries visible in review.

### 4. A focused doc rail checks durable markers

A small rail should fail when the document stops mentioning required boundaries, source-of-truth types, forbidden dependency rules, or validation commands. The rail should be deterministic and not inspect live provider state.

## Risks / Trade-offs

- A lifecycle document can drift if it is prose only. The focused marker rail reduces that risk.
- Over-specifying internals can slow refactors. Keep ownership names precise but avoid duplicating full code behavior.
- This should not become a Clankers compatibility document; it must describe Animus-owned terms.