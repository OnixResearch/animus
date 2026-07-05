# Design: Adapt Clankers process-job profiles for Animus

## Context

Background queue DTOs describe how long-running work is managed after a start request exists. Process-job profiles describe how projects name and constrain reusable jobs before dispatch. Animus should adapt this pure profile-resolution layer so queue tools can start known work without embedding unsafe command strings or host-specific backend policy in prompts.

## Decisions

### 1. Nickel owns profile policy

**Choice:** Process-job profile schemas, allowed backends, environment-variable prefixes, working-directory scopes, writable-path scopes, timeout/resource limits, and notification defaults are authored as Nickel contracts. Rust consumes checked exported data or test fixtures; runtime code does not evaluate Nickel during dispatch.

**Rationale:** Profiles are configuration with validation semantics. Nickel catches schema and merge errors before runtime.

### 2. Profile resolution is pure

**Choice:** A resolver accepts already-loaded profile sources and policy, then returns a backend-neutral `StartBackgroundWorkRequest` plus identity evidence or typed validation failures.

**Rationale:** The functional core can test profile behavior without filesystem reads, queues, pueue, systemd, or shell commands.

### 3. Source precedence is deterministic

**Choice:** Explicit sources override workspace sources, which override global sources. Duplicate profile names at the same precedence fail closed instead of depending on filesystem order.

**Rationale:** Deterministic source selection prevents surprising or malicious profile shadowing.

### 4. Secret and path policy fails before dispatch

**Choice:** Secret-like environment names, env keys outside allowlisted prefixes, cwd/writable path escapes, unsupported backends, conflicting command/program fields, and invalid args fail during resolution.

**Rationale:** Queue backends should receive only validated start requests.

### 5. Identity envelopes are safe

**Choice:** Resolved requests carry safe profile identity metadata: schema version, source label, policy hash, profile name, selected backend, and BLAKE3 profile/content identities. Raw environment values and backend-specific locators are excluded.

**Rationale:** Receipts need enough identity to replay/review profile choice without leaking secrets.

## Risks / Trade-offs

- Profile syntax can become a second shell DSL. Keep exactly one of command or program/args and validate strongly.
- Nickel export drift can hide policy changes. Add focused drift checks before claiming integration.
- Hosts may need richer backend-specific knobs. Keep extension points explicit and deny unknown fields by default.
