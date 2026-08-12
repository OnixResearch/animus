# Design: Add an Animus component catalog

## Context

The existing architecture is split across crates and docs, but the relationship between "which block do I depend on?" and "what authority does it require?" is still manually reconstructed. `docs/runtime-facade-inventory.md` already uses the right classes, and `docs/contracts-index.md` already names maintained adapter families. The catalog should consolidate those as typed data without replacing the existing docs.

## Decisions

### 1. Nickel is the source of truth

**Choice:** Add a typed Nickel catalog under `cairn/` that emits deterministic raw lines or JSON for checkers.

**Rationale:** Nickel is already the repo-owned configuration language and can reject missing authority labels, unknown dependency links, and invalid feature labels before implementation claims are made.

### 2. Catalog entries classify blocks by authority

**Choice:** Each entry records component id, owner crate/module, component class, feature gate, required authority, dependency ids, composition ids, capability family, focused checks, and public import guidance.

**Rationale:** Lego-like composition depends less on file locations and more on knowing which blocks are reusable contracts, which are host-supplied ports, and which are shell adapters.

### 3. Generated docs are review aids

**Choice:** Generate or check a `docs/component-catalog.md` projection while keeping `README.md`, `docs/contracts-index.md`, and `docs/runtime-facade-inventory.md` as curated entry points.

**Rationale:** Reviewers need readable docs, but the validation source should be deterministic data. Generated projections must contain only safe labels, counts, and paths relative to the repo.

### 4. Catalog validation is a prerequisite for future crate splits

**Choice:** Future split changes must update this catalog and pass the catalog rail before claiming a new block boundary.

**Rationale:** The catalog prevents facade exports, features, or conformance rails from drifting away from newly extracted crates.

## Risks / Trade-offs

- The catalog can duplicate existing docs. Mitigate by making the Nickel data minimal and generating a concise projection.
- Strict dependency-link checks can slow exploratory work. Mitigate by allowing `planned` status entries with explicit non-claim labels.
- Catalog validation does not prove behavioral correctness. It only proves block ownership, authority posture, and validation coverage metadata.
