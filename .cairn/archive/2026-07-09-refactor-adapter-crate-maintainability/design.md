# Design: Adapter crate maintainability refactor

## Context

Adapter crate growth has produced large modules and broad allowances in areas such as tool contracts, provider fixture/projection, front memory, cost routing, and prompt assembly. The goal is not new behavior; it is to make future behavior changes safer by narrowing ownership and keeping conformance evidence discoverable.

## Decisions

### 1. Behavior-preserving refactor only

**Choice:** Refactor module boundaries, naming, exports, docs, and lint scopes without changing adapter semantics.

**Rationale:** This work should reduce review risk before larger host-owned execution changes land.

### 2. Semantic ownership map drives splits

**Choice:** Before moving code, record a module ownership map with adapter family, requirement IDs, conformance cases, and public exports.

**Rationale:** Large modules can hide behavior ownership. A map prevents accidental orphaning.

### 3. Functional core plus imperative shell

**Choice:** Split pure validators, projectors, reducers, and receipt builders from shells that perform orchestration or host/service wiring.

**Rationale:** Pure cores are easier to test and keep aligned with adapter conformance fixtures.

### 4. Lint allowances become local and justified

**Choice:** Blanket crate/module allowances are removed or replaced with narrow local allowances that name why the exception remains.

**Rationale:** Broad allowances hide drift and make new warnings harder to evaluate.

### 5. Compatibility is explicit

**Choice:** Public re-exports are preserved where possible; any required migration aliases are documented.

**Rationale:** Maintainability work should not surprise downstream code.

## Risks / Trade-offs

- Module moves can create noisy diffs. Keep commits/review batches aligned by ownership family.
- Alias preservation can temporarily duplicate names. Add removal criteria if aliases are temporary.
- Lint tightening can expose unrelated warnings. Treat unrelated failures as blockers or document pre-existing status.
