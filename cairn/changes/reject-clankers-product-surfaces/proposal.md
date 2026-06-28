# Proposal: Reject Clankers product surfaces as Animus defaults

## Why

The useful Clankers transfer for Animus is architecture discipline: lifecycle ownership, policy, safe receipts, neutral ledgers, injected services, and focused rails. Clankers also has many product surfaces that would make Animus less embeddable if adopted wholesale: TUI, daemon/attach, Matrix/P2P, global state directories, provider router/OAuth, plugin subprocess runtime, Automerge desktop storage, self-evolution apply/rollback, live dogfood harnesses, and product tools. Animus needs an explicit guard that these remain out of scope unless a future Cairn change justifies one.

## What Changes

- Add an architecture note naming Clankers surfaces that Animus intentionally does not adopt by default.
- Add dependency and documentation checks that reusable Animus contracts do not require Clankers product crates, daemon/TUI concepts, global session directories, plugin runtimes, Matrix/P2P, Automerge storage, OAuth/provider router, or self-evolution workflows.
- Require future changes that open any rejected surface to document host ownership, injected service boundaries, safety policy, receipts, and non-default feature gating.

## Impact

- **Files**: architecture docs, dependency-isolation rail, Cairn artifacts.
- **Testing**: static dependency/marker checks and docs validation.
- **Non-claims**: this does not prevent hosts from building those features on top of Animus; it rejects them as reusable Animus defaults.