# Design: Adapt Zerostack permission kernel for Animus

## Context

Zerostack's permission checker is useful because it separates user-facing permission modes from per-tool glob/regex rules and records session allow decisions. Animus should adapt the interaction model while preserving its stronger rule: local permission allows are advisory and cannot replace authority, capability, sandbox, or redaction checks.

## Decisions

### 1. Permission modes are UX defaults

**Choice:** Represent modes as typed default-resolution profiles over guarded action classes.

**Rationale:** This keeps mode behavior deterministic and testable while preventing a mode from granting ambient authority.

### 2. Pattern rules are validated descriptors

**Choice:** Store glob-like and regex-like pattern descriptors with source identity, trust scope, precedence, action class, resource class, and outcome.

**Rationale:** Hosts need ergonomic matching, but invalid or untrusted patterns must fail before they can influence dispatch.

### 3. Session allowlists are scoped tokens

**Choice:** Model session allow entries as action/resource/policy/session scoped tokens with expiry or invalidation metadata.

**Rationale:** Zerostack's session allowlist reduces repeated prompts. Animus must also reject stale or mismatched grants.

### 4. Doom-loop detection is policy-visible

**Choice:** Detect repeated identical action identities in pure core and return a policy-selected outcome: coach, ask, deny, or ignore.

**Rationale:** Repetition is a safety and cost signal. It should be visible in receipts and policy, not hidden in a prompt-only warning.

### 5. Receipts remain bounded

**Choice:** Permission outcomes record safe action class, redacted resource identity or BLAKE3 hash, authority result, local match result, loop status, and prompt token status.

**Rationale:** Reviewers need auditability without raw prompts, credentials, full paths, provider payloads, or unbounded tool bodies.

## Risks

- Pattern matching can become a second policy language. Keep it subordinate to Nickel/authority checks and covered by negative tests.
- Repeated-call detection can false-positive valid polling. Make thresholds and action classes policy inputs.
