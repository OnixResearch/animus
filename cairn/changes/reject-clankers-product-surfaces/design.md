# Design: Clankers product-surface non-adoption guard

## Context

Animus is a modular embeddable harness. Clankers is a large application agent. Several active changes adapt Clankers boundary patterns, but without an explicit guard, product-edge assumptions can slip into reusable crates.

## Decisions

### 1. Rejected surfaces are documented

Animus docs will list Clankers surfaces not adopted by default: TUI, daemon/attach protocol, Matrix/P2P, global `~/.clankers` state, provider router/OAuth, plugin subprocess runtime, Wasm/Extism loading, Automerge desktop storage, self-evolution apply/rollback, live dogfood harnesses, product tool implementations, and release tagging workflow.

### 2. Reusable crates stay product-surface free

`animus-core`, `animus-adapters`, `animus-steel`, `animus-preserves`, and default std-shell contracts must not require Clankers product crates or concepts. Optional host adapters may be proposed later behind explicit features.

### 3. Future exceptions require a Cairn change

Any future adoption of a rejected surface must create a focused Cairn change that names host ownership, injected services, permission/policy boundaries, receipts, redaction, feature gating, and validation rails.

### 4. Static checks guard the boundary

A dependency/marker rail should fail when reusable crates mention or depend on rejected product-surface markers unless the mention appears in docs explicitly listing non-adoption.

## Risks / Trade-offs

- A guard can become too broad and block legitimate docs. Allow references in bounded-reference docs and README references.
- Some future hosts may want a daemon or TUI. They can build those as host-owned layers without making them Animus defaults.
- Static marker checks need careful allowlists to avoid false positives.