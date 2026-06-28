# Design: Runtime facade export inventory

## Context

Clankers' `split-runtime-facade-contracts` change separates green reusable contracts, yellow host-injection surfaces, and desktop adapter shells. Animus has a cleaner starting point: `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, `animus-std`, `animus-tui`, and the root compatibility crate already express a functional-core / imperative-shell split. The risk is future drift: new public exports can bypass that split unless a deterministic inventory names what each export is allowed to own.

This change adapts the Clankers inventory pattern in Animus-owned terms. It should remain lightweight, data-first, and credential-free.

## Decisions

### 1. Inventory classes are architectural, not branding

Animus classifies public runtime-facing exports into these classes:

- **Reusable contract**: deterministic DTOs, pure validation helpers, no-ambient traits, and no-side-effect boundary types suitable for embedders.
- **Host-injection surface**: traits, builders, service handles, or capability brokers that let a host provide authority without the reusable crate owning that authority.
- **Shell adapter**: std, TUI, provider, process, filesystem, credential, clock, network, plugin, or product integration code that may perform side effects only in the owning shell crate.

The inventory must not imply that classification grants authority. Runtime authorization still belongs to policy, capability checks, and injected services.

### 2. Every public export needs an owner and authority class

The inventory records at least the export path, owning crate, source module or marker, class, side-effect allowance, stability intent, and review note. A public export that appears in a runtime-facing crate without classification fails the rail.

### 3. Reusable contracts reject ambient authority

Reusable-contract exports must not require `std`, process environment, filesystem traversal, credentials, clocks, randomness, network, provider discovery, TUI state, daemon frames, plugin launch, or concrete persistence. If such behavior is needed, it belongs behind a host-injection surface or shell adapter.

### 4. Host-injection surfaces fail closed by default

Default host-injection implementations should be explicit no-op, unavailable, or deny-by-default adapters. A missing provider, tool, ledger, confirmation, queue, or capability service must not silently create a shell adapter, read ambient config, or contact a backend.

### 5. Inventory evidence is safe and reviewable

The focused rail emits safe metadata: schema name, status, checked crates, export counts, unclassified export names, class counts, fixture names, and BLAKE3 hashes for evidence artifacts when content identity is needed. It must not include raw prompts, provider payloads, credentials, full host paths, environment values, or tool output.

### 6. Generated docs are consumers, not authority

A generated or maintained inventory summary may appear in docs, but the rail and source markers are the authority. Documentation must explain that shell adapters are optional and that embedders should depend on the smallest crate/class surface they need.

## Risks / Trade-offs

- A manual inventory can go stale. The focused rail should either derive exports mechanically or fail when marker coverage is incomplete.
- Over-classification can slow early development. Start with runtime-facing public exports and grow the inventory as crates expose more facade APIs.
- Shell adapters are legitimate but easy to overexpose. Keep them feature-gated or crate-local where possible and document non-default status.
