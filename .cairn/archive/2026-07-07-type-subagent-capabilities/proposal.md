# Proposal: Type subagent capabilities

## Why

Recipe-backed subagents currently rely on normalized string capability labels at the adapter boundary. That is useful for interoperability, but internal runtime decisions need stronger typing to avoid ambiguous labels, wildcard-like authority, and lossy denial reporting.

This change introduces typed subagent capabilities and catalog references for reusable runtime logic while preserving stable string labels at external adapter boundaries.

## What Changes

- Add typed capability variants for context reads, search, tool use, mutation intents, workspace access, model use, and host-admitted external services.
- Resolve capability intersections over typed values and report which ceiling denied each request.
- Validate string labels against a capability catalog before converting them to typed runtime values.
- Preserve existing string labels for adapter DTOs, receipts, and compatibility fixtures.

## Impact

- **Files**: future capability types, catalog validation helpers, adapter conversion tests, denial diagnostics, docs, and fixture updates.
- **Testing**: positive typed capability conversion/intersection tests; negative duplicate, unknown, wildcard, malformed, unauthorized, and label-collision tests.
- **Non-claims**: this does not remove adapter string labels or grant new runtime authority.
