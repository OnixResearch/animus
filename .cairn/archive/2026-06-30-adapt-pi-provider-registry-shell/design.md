# Design: Adapt Pi provider registry shell for Animus

## Context

Pi maintains a provider/model registry that combines built-in model metadata, custom provider configuration, credential lookup, thinking-level control, model cycling, and dynamic provider registration. Animus should adapt the registry as a shell convenience and boundary adapter, not as runtime core logic.

## Decisions

### 1. Registry belongs in `animus-std` or host shells

**Choice:** Provider/model registry types and default config loading live in the standard-library shell or host integrations, while reusable runtime crates consume selected injected model services.

**Rationale:** Provider discovery requires files, environment variables, credentials, and network-capable clients, which are imperative-shell concerns.

### 2. Nickel owns repo-defined provider config

**Choice:** When Animus owns provider registry config, it is authored in Nickel with typed contracts and exported to checked runtime data for shell consumption.

**Rationale:** This matches repo configuration policy and catches malformed provider data before runtime use.

### 3. Capabilities are explicit metadata

**Choice:** Model entries declare input modalities, context budget, output budget, reasoning support, allowed thinking levels, API kind, and cost metadata.

**Rationale:** Prompt construction, queueing, compaction, and UI/headless state need capability facts without probing providers at runtime.

### 4. Credential resolution is redacted shell behavior

**Choice:** The shell resolves credentials through explicit sources and returns either a usable provider service or a typed unavailable/missing-credential failure with redacted diagnostics.

**Rationale:** Runtime reports need to explain missing providers without leaking tokens or environment values.

### 5. Runtime receives only injected services

**Choice:** After registry selection, runtime dispatch sees only a validated model/provider service and provider-neutral request DTOs.

**Rationale:** The runtime must not read config, inspect auth stores, call OAuth, or construct HTTP clients.

## Risks / Trade-offs

- Provider config schemas can churn as APIs evolve. Keep v1 metadata minimal and extensible.
- Cost metadata can become stale. Treat it as shell/UI metadata, not a billing authority.
- Dynamic provider registration can bypass Nickel. Host-provided runtime overrides need the same validation and redacted diagnostics as file-backed config.
