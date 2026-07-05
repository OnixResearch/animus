# Proposal: Adapt Claude Code permission modes for Animus

## Why

Claude Code's operator-facing permission modes make agent actions understandable: some actions can proceed, some require confirmation, and some are denied. Animus already has UCAN-first capability work in flight, so it should adapt the UX shape as a local decision layer rather than as ambient authority.

This change captures permission modes as typed policy caveats around guarded actions so hosts can prompt users consistently while reusable runtime code stays deterministic and testable.

## What Changes

- Define typed permission-mode descriptors for allow, ask, and deny outcomes over guarded action classes.
- Resolve permission modes only after Animus capability requests, Nickel contracts, and UCAN authority checks are available.
- Add host-mediated confirmation requests for ask decisions with redacted action summaries and replay-safe decision tokens.
- Add safe receipts for permission decisions and prompt outcomes.
- Add positive and negative tests for allowed actions, denied actions, prompt-required actions, missing authority, stale prompt decisions, and receipt leak attempts.

## Impact

- **Files**: future permission DTOs, policy contracts, action-resolution core, host prompt adapter boundary, receipts, tests, and docs.
- **Testing**: happy-path allow/ask flows plus negative tests for local allow without authority, stale confirmation tokens, malformed permission policy, denied capabilities, unavailable prompt service, and leak attempts.
- **Non-claims**: this change does not copy Claude Code's permission implementation, does not treat local allow as UCAN authority, does not grant shell/network/filesystem access by default, and does not place interactive prompting in `no_std` crates.
