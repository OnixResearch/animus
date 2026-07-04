# Design: Add adapter conformance rail and contracts index

## Context

The runtime facade already has focused domain preludes, and adapter modules contain many positive and negative tests. The missing piece is a maintained index and rail that makes adapter expectations discoverable across modules and prevents new public adapter families from shipping with only happy-path tests or unclear authority boundaries.

## Decisions

### 1. Contracts index is the map of record

**Choice:** Add a bounded `docs/contracts-index.md` that maps contract families to owner modules, stability/version status, authority posture, safe receipt behavior, serialization bridge status, and focused checks.

**Rationale:** Maintainers and embedders need one place to discover the contract surface without reading the giant flat facade.

### 2. Conformance cases are explicit and reusable

**Choice:** Define a shared conformance matrix with required positive and negative cases for each public adapter family. Families can mark non-applicable cases with a reason, but cannot omit negative coverage silently.

**Rationale:** The repo rule requires positive and negative tests; adapter contracts need the same expectation at family level.

### 3. Focused adapter preludes guide new code

**Choice:** Add or document focused adapter preludes/module-level export sets for model, tool, host/status, workspace, security/policy, and receipt surfaces while preserving root compatibility exports.

**Rationale:** New embedders should depend on the smallest surface, and the flat facade should remain compatibility-only.

### 4. Inventory rail avoids brittle API counting

**Choice:** The conformance rail should inspect maintained family declarations or a small checked-in manifest rather than scrape every Rust export. It fails when a declared public family lacks index, positive coverage, and negative coverage.

**Rationale:** Explicit inventory is more reviewable and less noisy than broad export scraping.

## Risks / Trade-offs

- A manual manifest can drift. Keep the check small and require updates when adding public adapter families.
- Preludes may duplicate exports. Treat them as guidance surfaces and preserve flat exports until migration is planned.
- Some families have specialized failure modes. Allow documented non-applicability but require a safe reason.
