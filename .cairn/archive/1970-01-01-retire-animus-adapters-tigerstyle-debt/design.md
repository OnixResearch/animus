## Context

The Octet derivation now uses the immutable Cargo vendor root correctly. Its first complete adapter scan reports duplicated lib and test-target diagnostics for 174 unique source locations. Most findings come from public type names repeating their module paths, followed by non-trait imports and local structural findings.

## Decisions

### Decision: Preserve public paths through compatibility modules

**Choice:** Compile each affected implementation under a neutral canonical module name and retain the current public module as a re-export-only compatibility surface.

**Rationale:** This removes repeated searchable path words without breaking downstream paths such as `animus_adapters::trajectory::TrajectoryPolicy`.

### Decision: Fix source findings without broad suppression

**Choice:** Qualify non-trait values, split conditions, extract pure helpers, add explicit assertions, use named unit-bearing values, and replace unchecked operations. Do not disable lints or add module-wide Tiger Style allows.

**Rationale:** The gate should prove improved source shape rather than hide accepted debt.

### Decision: Keep behavior and authority stable

**Choice:** Refactoring changes source organization and local validation shape only. Existing DTO fields, discriminants, validation outcomes, receipts, effect boundaries, and public compatibility paths remain stable.

**Rationale:** Tiger Style cleanup must not widen authority or alter admitted behavior.

## Risks / Trade-offs

- Moving canonical modules can expose missed internal paths. Workspace compile and public compatibility tests detect this.
- Added assertions can reveal invalid internal states earlier. They must not replace typed rejection of caller-controlled invalid input.
