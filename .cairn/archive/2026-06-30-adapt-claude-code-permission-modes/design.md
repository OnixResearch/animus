# Design: Adapt Claude Code permission modes for Animus

## Context

Claude Code exposes practical permission modes to keep tool calls and shell actions understandable. Animus should preserve that clarity while aligning with existing plans for Nickel-authored contracts, Basalt/UCAN authorization, typed tool contracts, and safe receipts.

The key adaptation is to separate authority from UX: UCAN and contracts answer whether an action can be authorized; permission modes answer whether local policy should allow, ask, or deny within that authority.

## Decisions

### 1. Permission modes are local caveats, not authority

**Choice:** Represent allow, ask, and deny as local permission outcomes that can narrow or pause an authorized action but cannot expand authority.

**Rationale:** This preserves Claude Code's usability without creating a second authority system that conflicts with UCAN grants.

### 2. Guarded actions resolve through a pure decision core

**Choice:** A pure resolver consumes action class, resource labels, authority status, selected permission policy, session state, and prompt-decision state, then returns execute, prompt, or deny.

**Rationale:** Tests can cover policy precedence and stale decisions without invoking tools, shells, network calls, or clocks.

### 3. Ask decisions are shell-owned interactions

**Choice:** The reusable runtime emits a host prompt request containing redacted action summaries and allowed response choices. The shell or embedder obtains user input and returns a signed or otherwise replay-safe decision token.

**Rationale:** `no_std` crates cannot own TUI, desktop, or web prompts, and confirmation text must not leak raw secrets.

### 4. Permission receipts remain safe and replayable

**Choice:** Receipts record action class, redacted resource identity, authority status, local caveat outcome, prompt status, policy hash, and BLAKE3 receipt hash.

**Rationale:** Operators need to understand why an action ran or stopped without storing raw prompts, tool bodies, credentials, provider payloads, or full paths.

## Risks / Trade-offs

- Permission UX can become confused with security authority. Tests must prove local allow without UCAN authority denies.
- Prompt tokens can become ambient approvals. Scope them to action identity, policy hash, session, and expiry metadata supplied by the host.
- Deny-by-default can frustrate users. Hosts can provide profiles, but core resolution must stay explicit and auditable.
