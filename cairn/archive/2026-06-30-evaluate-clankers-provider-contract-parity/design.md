# Design: Future provider contract parity

## Context

Clankers' provider contract docs name the fields and tests that keep `clankers-provider`, `clanker-router`, and agent request construction aligned. Animus currently has only small adapter DTOs, so this change records future criteria without implementing providers.

## Decisions

### 1. Provider parity activates only when multiple request DTOs exist

If Animus adds provider/router-specific request DTOs alongside adapter DTOs, a parity rail must compare shared fields and serialization behavior.

### 2. Request constructors are inventoried

When a concrete request struct is constructed in multiple modules, a focused test should inventory constructors and require fields such as model, messages, system prompt, max tokens, temperature, tools, metadata, cache policy, and extra parameters as applicable.

### 3. Message conversion is provider-native

Adapters must not use lossy generic `serde_json::to_value`-style conversions when provider-native message shapes are required. Tests should use explicit fixtures.

### 4. Streaming claims use parser entrypoints

When Animus supports streaming provider responses, tests must feed raw stream/SSE bytes through the real parser entrypoint, not only helper state machines.

### 5. Known provider prefixes fail closed

If provider-qualified model IDs are adopted, known-but-unavailable providers must fail closed rather than silently falling back to another provider.

## Risks / Trade-offs

- Defining provider fields too early can overfit. Keep this as a future adoption matrix until provider DTOs exist.
- Constructor inventory tests need maintenance. They are useful once real provider surfaces grow.
- Live checks are out of scope; parity rails should be credential-free by default.