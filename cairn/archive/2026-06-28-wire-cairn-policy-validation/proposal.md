# Proposal: Wire Cairn policy validation for Animus

## Why

Animus uses native Cairn lifecycle artifacts, but default Cairn validation currently needs an explicit external policy path. That makes local validation less reproducible and easy to skip. The repository should either carry the generated policy JSON that Cairn runtime expects or provide a documented deterministic way to refresh and check it.

This change turns Cairn validation from a session-specific command into a repo-owned rail.

## What Changes

- Add repo-local Cairn policy wiring for the generated policy JSON consumed by Cairn validation commands.
- Document the source policy, refresh command, check command, and expected default validation invocation.
- Add a drift check that compares the checked-in generated policy against the canonical exported policy when the source is available.
- Ensure lifecycle validation and gates can run without hidden absolute paths in normal developer workflows.
- Record policy hashes or provenance labels in release/lifecycle evidence when available.

## Impact

- **Files**: `cairn-policy/` generated policy artifact and docs, optional check script or Nix check, Cairn artifacts.
- **Testing**: default Cairn validation, explicit policy validation, policy drift fixture/check, Cairn gates.
- **Non-claims**: this does not change Cairn's policy semantics, run Nickel during Cairn runtime validation, or make Animus depend on a live Cairn checkout at runtime.
