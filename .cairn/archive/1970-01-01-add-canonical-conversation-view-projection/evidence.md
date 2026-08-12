## Scope and reference

This change adds one canonical redaction-first conversation projection over admitted ledger facts. It uses camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2` as an MIT-licensed ordering reference without a runtime dependency.

## Ownership

The ledger owns model context and tool correlation. The pure projection owns stream, cursor, item, action, snapshot, rebuild, heal, and receipt identities. Hosts own ledger reads, cache mutation, transport, clocks, UI rendering, persistence, and completion.

## Evidence

- Twelve runtime tests cover snapshot-first projection, exact identity, active-turn folding, steering markers, cursor no-op and rebuild, invalid source order, conservative healing, exact cache evidence, and receipt leaks.
- Three std tests cover generated policy parsing, malformed policy rejection, cache idempotence, and mutation-free partial failure.
- Positive and negative Nickel fixtures cover bounded policy and disposable-cache posture.
- Clippy passes for runtime and std feature surfaces.
- The focused rail checks no ambient authority in the pure source and no-default builds.
- Component and runtime facade catalogs classify the new reusable core and shell fixture.

## Non-claims

This evidence does not prove transcript truth, UI delivery, cache durability, transport order, renderer correctness, accessibility, task completion, production readiness, or release eligibility.
