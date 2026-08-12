# Design: Negotiate model capability matrix

## Context

Canonical model requests make compatibility requirements explicit, but a shell still needs a deterministic way to prove a selected model supports those requirements. The accepted provider registry and model dispatch plan already keep provider selection shell-owned and provider effects gated. This change adds the feature matrix and negotiation core that model dispatch can consume before execution.

## Decisions

### 1. Feature support is typed data, not provider probing

**Choice:** Store capability classes such as supported roles, content part kinds, tool-call behavior, structured-output modes, stream features, reasoning controls, budget limits, and privacy/locality posture in validated provider/model metadata.

**Rationale:** Runtime can decide compatibility from explicit inputs without live provider calls, credentials, network access, or ambient discovery.

### 2. Negotiation is a pure functional core

**Choice:** Implement negotiation as a deterministic function over canonical request requirements, selected model capability metadata, and host downgrade policy.

**Rationale:** Positive and negative compatibility behavior can be tested without standing up provider clients.

### 3. Required features fail closed

**Choice:** If a canonical request requires a feature the selected model does not support, negotiation returns a typed denial. Optional features may be downgraded only when policy allows it and the downgrade is recorded.

**Rationale:** Silent fallback or lossy downgrade hides compatibility bugs and can widen authority.

### 4. Nickel and Rust capability schemas stay aligned

**Choice:** The repo-owned provider registry Nickel contract includes every persisted capability field, and generated or exported examples are checked against Rust-side validation fixtures.

**Rationale:** Compatibility metadata should not drift between config authorship and runtime consumption.

## Risks / Trade-offs

- The matrix can become too broad. Start with features needed by canonical request dispatch and add future modalities under versioned contracts.
- Providers often have nuanced partial support. Represent unknown or conditional support explicitly and deny required unknowns by default.
- Downgrade policy can be abused. Keep downgrade decisions safe, explicit, and receipt-backed.
