# Design: Dogfoodable host readiness roadmap

## Context

The current Animus architecture deliberately favors reusable contracts over product behavior. The accepted request lifecycle says provider discovery, credentials, receipt persistence, and ledger persistence remain outside reusable crates. The Clankers product-surface rejection note also prevents accidental adoption of daemon, TUI, plugin, router, OAuth, Matrix, global state, and self-evolution product surfaces.

The Clankers comparison exposed the practical downside of that stance: Animus has many accepted boundaries but no maintained path to prove an operator can run it end to end. Without a dogfoodability roadmap, future changes can either overreach by importing product behavior as defaults or underreach by adding more contracts without an executable shell.

## Decisions

### 1. Treat dogfoodability as optional host-owned readiness, not default product parity

**Choice:** The roadmap defines dogfoodability as evidence that a specific host shell can run a bounded prompt path through accepted Animus contracts. It does not define parity with Clankers and does not weaken the reusable-core/product-surface rejection rule.

**Rationale:** This gives Animus a concrete operational goal while preserving its core value: reusable, host-neutral contracts.

### 2. Inventory gaps as blockers, non-claims, or rejected surfaces

**Choice:** The maintained inventory classifies each Clankers gap as one of: actionable blocker for dogfoodability, intentional non-claim, rejected default product surface, or optional future host adapter.

**Rationale:** Clankers has working surfaces that Animus intentionally rejects by default. A classification prevents “missing” from becoming “must copy” while still naming real blockers such as no runnable binary, no provider execution shell, and no persistent session seam.

### 3. Land the first dogfood path as a non-default headless host shell

**Choice:** The first runnable target should be a minimal non-default headless or CLI host shell that uses fake/deterministic adapters before live providers. It should demonstrate prompt input, model dispatch through an injected adapter, tool admission through accepted descriptors, safe events, final output, and safe receipt metadata.

**Rationale:** A headless shell is easier to validate than a full TUI or daemon, exercises the request lifecycle, and avoids prematurely adopting Clankers product surfaces.

### 4. Sequence operational surfaces behind accepted contracts

**Choice:** Successor Cairns for live providers, built-in tools, persistence, TUI, daemon/attach, plugin/MCP runtime, and release harness work must declare prerequisite accepted requirement IDs and host-owned adapters.

**Rationale:** Product surfaces must not bypass the reusable runtime, injected service, receipt, ledger, policy, and adapter contracts that make Animus distinct from Clankers.

### 5. Require an evidence ladder for readiness claims

**Choice:** Readiness claims advance through deterministic unit/fixture checks, fake-backed integration, packaged headless smoke, optional live-provider smoke, optional TUI dogfood, and current-head evidence indexing. Every receipt must remain safe and bounded.

**Rationale:** Clankers has mature dogfood and release evidence. Animus needs comparable evidence discipline before claiming any operational surface, but each rung should be optional and scoped to the surface being claimed.

### 6. Add claim hygiene rails before parity language is used

**Choice:** Documentation and release notes must not claim Clankers parity, production readiness, live-provider support, daemon behavior, plugin runtime, or TUI dogfood unless the corresponding evidence ladder rung has passed and is indexed.

**Rationale:** Animus has many architecture notes with explicit non-claims. A focused rail prevents those non-claims from being accidentally contradicted as host shells arrive.

## Risks / Trade-offs

- A roadmap-only change can become shelfware; tasks therefore require successor Cairns and validation hooks, not just prose.
- Starting with headless/CLI dogfood leaves TUI and daemon gaps open longer, but reduces the chance of importing product assumptions into reusable crates.
- Fake-backed evidence does not prove live provider quality; live smoke remains a separate optional rung with explicit availability blockers.
- A parity inventory could be misread as a commitment to copy Clankers; the spec requires rejected surfaces and non-claims to stay explicit.
