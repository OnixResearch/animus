# Proposal: Adapt Clankers request lifecycle ownership for Animus

## Why

Animus has a small functional core, adapter contracts, Steel loop fabric, and std shell, but the durable request lifecycle is still implicit. Clankers has a useful golden-path lifecycle document that names the owner of each transition and the seams that need regression tests. Animus should adopt that documentation pattern before runtime behavior grows so future changes do not fork state machines across crates.

## What Changes

- Add `docs/request-lifecycle.md` for Animus' runtime path from typed runtime input through core state, Steel decisions, adapter dispatch, event/report construction, receipt creation, and host persistence boundaries.
- Document source-of-truth types and owning crates for each transition.
- Add forbidden dependency and forbidden side-effect notes per boundary.
- Add a focused validation rail that checks the lifecycle document names the maintained boundaries and validation commands.
- Link the lifecycle document from the README and relevant runtime/provenance docs.

## Impact

- **Files**: new lifecycle documentation, possible doc-check script, README links, Cairn artifacts.
- **Testing**: focused doc rail plus workspace docs/link validation when available.
- **Non-claims**: this does not add daemon, TUI, provider, persistence, or transport behavior.