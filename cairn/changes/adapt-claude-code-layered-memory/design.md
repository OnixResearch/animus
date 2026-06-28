# Design: Adapt Claude Code layered memory for Animus

## Context

Claude Code advertises project instructions, auto memory, and shared settings as a cross-surface customization layer. The transferable idea is not the exact file format; it is the operator-facing separation between durable project guidance, user/session preferences, and learned facts. Animus should express those inputs as typed data so prompt assembly remains deterministic, testable, and policy-gated.

Existing Animus work on policy profiles and resource discovery covers broad prompt posture and skill loading. This change focuses on memory-specific lifecycle, admission, freshness, and receipt behavior.

## Decisions

### 1. Instructions and learned memory are distinct descriptors

**Choice:** Model instructions as reviewed guidance descriptors and learned memory as admitted fact/preference descriptors with separate validation rules.

**Rationale:** Reviewed project guidance and learned session facts have different trust, freshness, and deletion expectations. Treating them as one prompt blob would hide authority and provenance.

### 2. Prompt influence is deterministic and attributable

**Choice:** Prompt assembly takes an in-memory set of descriptors, selected policy, and active trust decisions, then returns bounded prompt sections with source attribution.

**Rationale:** The core can test ordering, conflict handling, and redaction without reading files or knowing about a host's storage layout.

### 3. Learned memory requires host admission

**Choice:** Runtime may propose memory candidates, but persistence and future prompt influence require an injected host admission decision.

**Rationale:** Auto-memory is useful but dangerous when it silently stores secrets, transient debugging guesses, or user-private text.

### 4. Receipts prove influence without leaking bodies

**Choice:** Prompt-influence receipts record descriptor IDs, source labels, trust status, freshness state, inclusion decisions, redaction classes, and BLAKE3 body hashes where body identity is needed.

**Rationale:** Operators need to audit why guidance influenced a turn without exposing raw prompts, credentials, absolute paths, provider payloads, or large memory bodies.

## Risks / Trade-offs

- Too much memory can bloat prompts. Require budgets and stale-entry behavior in policy.
- File-format importers can become authority. Keep `CLAUDE.md` compatibility, if any, in std/adapters and convert into typed descriptors before use.
- Conflicting guidance can be subtle. Expose deterministic conflict diagnostics rather than silently merging incompatible rules.
