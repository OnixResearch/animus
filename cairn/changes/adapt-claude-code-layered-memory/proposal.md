# Proposal: Adapt Claude Code layered memory for Animus

## Why

Claude Code's project instructions and memory model show a useful ergonomic pattern: teams need durable project guidance, session-scoped notes, and learned facts that can influence future turns without forcing every prompt to restate them. Animus already has policy/profile and resource-discovery work, but it needs a narrower memory contract that separates trusted project instructions from learned memory and keeps every prompt influence attributable.

A native Cairn change is needed so Animus can adapt the UX benefit without adopting `CLAUDE.md` as a canonical format, hidden auto-memory writes, or vendor-specific prompt semantics.

## What Changes

- Define typed instruction and memory descriptors with source identity, trust scope, precedence, freshness, and prompt-inclusion policy.
- Add deterministic prompt assembly for layered instructions and admitted memory entries with compact attribution.
- Require host-mediated admission before learned memory can persist or influence a future prompt.
- Add redacted memory receipts using BLAKE3 hashes for bodies that are not safe to embed.
- Add positive and negative tests for trusted guidance, conflicting layers, untrusted project files, stale memory, and leak attempts.

## Impact

- **Files**: future memory DTOs, prompt assembly core, std-owned loaders/importers, Nickel policy contracts, receipts, tests, and docs.
- **Testing**: valid layer composition plus negative tests for untrusted project memory, conflicting precedence, raw secret-looking body leakage, stale entries, and malformed descriptors.
- **Non-claims**: this change does not copy Claude Code's prompt text, does not make `CLAUDE.md` the Animus source of truth, does not write hidden memory without host admission, and does not add provider-specific memory behavior to reusable core crates.
