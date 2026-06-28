# Design: Future release evidence index

## Context

Clankers' readiness docs and harness receipts are useful because they connect local evidence to the exact payload and explicitly avoid overclaiming. Animus should adopt that evidence discipline after focused rails exist, not the full Clankers harness.

## Decisions

### 1. Evidence index composes existing receipts

The future index should collect outputs from focused rails, workspace tests, clippy, flake checks, Nickel drift checks, Cairn validation, and Cairn gates. It should not rerun missing modes unless explicitly requested.

### 2. Current-head matching is explicit

Where VCS information is available, an index records commit, branch, dirty status, and whether selected receipts match the current head. Dirty or stale receipts remain historical evidence, not current-head proof.

### 3. Pure and optional checks are separated

Credential-free deterministic checks are separated from optional host-dependent checks such as live providers, VM gates, TUI/dogfood, or remote services.

### 4. Non-claims are first-class

The index must state what it does not prove: production readiness, live provider behavior, daemon/TUI behavior, VM packaging, remote auth, plugin runtime, or performance unless those checks ran.

### 5. Start with fixtures

Before adding a real index generator, define fixture schemas and validation for stale receipts, dirty state, missing modes, matching current-head receipts, and overclaim rejection.

## Risks / Trade-offs

- Evidence indexes can become release theater. Require receipt/head matching and non-claims.
- VCS details differ between git and jj. Keep VCS collection in the shell.
- Avoid making optional live checks part of default validation.