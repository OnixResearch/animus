# Design: Adapt Ornith trust-boundary monitoring for Animus

## Context

The Ornith page separates the mutable inner scaffold from an immutable outer trust boundary. That distinction fits Animus: Steel scripts, scaffold profiles, and model decisions may shape orchestration, but they must not change capability policy, sandbox limits, verifier scripts, hidden paths, approval requirements, or evidence acceptance.

## Decisions

### 1. The outer boundary is explicit input data

**Choice:** Define a typed trust-boundary descriptor covering sanctioned tools, denied paths, verifier artifacts, sandbox profile identity, approval requirements, evidence acceptance policy, and BLAKE3 hashes for reviewed boundary data.

**Rationale:** Boundary enforcement cannot depend on ambient state or prompt text. The pure core needs explicit data to validate planned and observed actions deterministically.

### 2. Deterministic monitor denials are final

**Choice:** The monitor returns typed issue codes for exact violations such as withheld-path reads, verifier modification, unsanctioned tool calls, sandbox weakening, and approval suppression. A denial prevents the trajectory from being accepted as successful evidence.

**Rationale:** Exact policy violations should not be adjudicated by model intent. They are structural failures.

### 3. LLM judges are vetoes only

**Choice:** An optional frozen or host-provided judge may add suspicious-intent vetoes, but judge output cannot approve a monitor-denied trajectory, grant permissions, hide diagnostics, or mutate the trust boundary.

**Rationale:** Intent-level gaming may evade simple rules, but model judgment must not become a privilege-escalation mechanism.

### 4. Anti-gaming cases are fixtures, not hidden behavior

**Choice:** Reward-hacking patterns become explicit positive and negative fixtures. The fixtures validate monitor behavior without relying on hidden tests or real provider calls.

**Rationale:** Animus should make safety behavior reproducible and reviewable under normal CI.

### 5. Violation receipts remain redacted

**Choice:** Boundary violation receipts expose policy hash, issue code, sanitized labels, counts, and redaction status. They do not include hidden paths, raw prompts, verifier bodies, tool inputs, tool outputs, provider payloads, credentials, or full path dumps.

**Rationale:** Operators need diagnosis without leaking the protected material the boundary exists to defend.

## Risks / Trade-offs

- Exact monitor rules can miss allowed-surface gaming. Keep the optional judge as a veto layer and add new deterministic cases as patterns become precise.
- Overly broad rules can block legitimate evaluation fixtures. Require positive allowed-action tests next to every denial class.
- Boundary descriptors can leak sensitive paths if serialized. Receipts should use labels and hashes rather than raw path dumps.
- Host shells own sandbox enforcement. Reusable crates should validate descriptors and actions, not invoke OS sandbox APIs directly.
