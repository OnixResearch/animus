# Design: Adapt Zerostack provider registry hardening for Animus

## Context

Zerostack is valuable as a provider ergonomics reference because it treats OpenAI-compatible gateways differently from first-party OpenAI, supports custom headers with environment expansion, allows extra body parameters, and filters model listings for agent-suitable models. Animus should adapt these as host-shell validation contracts.

## Decisions

### 1. API style is explicit metadata

**Choice:** Provider entries declare the effective API style or an auto-detection policy that resolves to a typed API style before client construction.

**Rationale:** Responses and Chat Completions routes are not interchangeable, and runtime core should only see the selected model service.

### 2. Environment expansion is whole-value and redacted

**Choice:** Header descriptors may reference environment-backed values, but shell diagnostics name only the source label and never the resolved secret.

**Rationale:** Operators need gateway headers without leaking Cloudflare, bearer, or internal auth values into receipts.

### 3. Extra body parameters are provider-scoped

**Choice:** Extra request body descriptors are attached to provider/model selection metadata and validated against policy before dispatch.

**Rationale:** Provider-specific knobs can be useful, but global untyped JSON can break other providers or hide routing decisions.

### 4. Model listing normalizes capabilities

**Choice:** Dynamic model listing produces normalized model metadata and filters non-agent model classes before selection.

**Rationale:** Embedding, image, audio, moderation, or rerank models should not be selected as text agent models by accident.

## Risks

- Provider-specific body keys are hard to validate universally. Record them as policy-admitted opaque fields with source and provider scope.
- Model listing can be unavailable. Selection must degrade with explicit unavailable-listing diagnostics rather than guessing silently.
