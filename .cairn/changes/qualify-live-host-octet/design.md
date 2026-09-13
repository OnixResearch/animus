# Design

## Context

The unchanged provider crate fails the pinned Octet catalog. Its failed dependency check hides the downstream host result. Independent package checks are necessary to expose both surfaces. A combined check remains necessary for the final integrated subject.

The repository already owns the provider core, SSE parser, tool declarations, explicit host adapters, and persistent session composition. The repair reuses those components at the selected immutable Animus revision. A new shared library or sibling dependency cannot correct these local invariant and layout findings.

## Decisions

### Decision: Preserve the strict subject

**Choice:** Add independent provider and live-agent checks beside the combined host check. Use the existing pinned driver and deny catalog with all targets and all features.

**Rationale:** A passing upstream dependency build is not a downstream lint result. Separate observations expose the failure in each owner without reducing final acceptance.

### Decision: Split by capability and effect ownership

**Choice:** Keep pure request admission, schema validation, parsing, and policy separate from transport, persistence, and composition. Use qualified paths and narrow modules. Preserve the public facade where its contract remains valid.

**Rationale:** Structural repairs must expose ownership, not hide a large module behind aliases or move effects into the core.

### Decision: Preserve decisions through checked transitions

**Choice:** Use bounded iterative schema traversal, checked arithmetic, explicit rejection paths, and local invariant assertions. Keep named policy bounds and existing error vocabulary.

**Rationale:** Refactoring cannot turn rejection into success, panic on untrusted data, or mutate state after refusal.

### Decision: Name portable count migrations

**Choice:** Replace public machine-width quantities with explicit portable bounds where the strict rule requires them. Inventory and migrate directly affected callers. Preserve const constructors where their revised signatures permit it.

**Rationale:** `usize` cannot remain a public portable contract. A changed integer type can break source compatibility. Evidence must name that change instead of hiding it with an alias or an exemption.

## Parallel ownership

- Parser worker: provider `stream.rs` and `tool.rs`, with new `codec/` and `declarations/` subtrees. It does not edit the facade or other tests.
- Provider worker: remaining provider sources, binaries, tests, and the facade. It coordinates only through the declared module names and output manifests.
- Session worker: live-agent sources and tests. It leaves other crates and lifecycle files to the parent.
- Parent: cross-scope composition, direct callers, flake checks, documentation, lifecycle, and final verification.

Each worker has 160 tool calls and 40 minutes. Workers cannot delegate, run memo, publish, edit another scope, or run a workspace-wide formatter. The parent runs the shared baseline and final checks. A worker formats only its owned files after its writes finish.

## Risks / Trade-offs

- Public numeric widths can change caller source even when runtime outcomes stay the same.
- A module split can change private paths used by tests or source-shape checks.
- A new assertion must describe a real invariant. It cannot replace rejection for untrusted input.
- Recursive schema traversal needs explicit depth and work bounds with unchanged accepted cases.
- Separate package checks cannot substitute for the combined final check.
- The companion API branch can need a later explicit merge repair after these structural changes.

## Verification

Done = the required gates pass and only the declared repair files change.

The baseline runs both host test suites and strict Clippy before Rust edits. Independent Octet checks record each unchanged host result. Behavioral changes retain accepted and rejected fixtures, including malformed schemas, wrong identities, bounds, overflow, persistence refusal, and authority denial.

Final verification covers both crates, all targets, all compatible features, default-off behavior, directly affected callers, strict Clippy, the complete pinned catalog, and the applicable provider and live-shell rails. The parent verifies the final subject before lifecycle sync, archive, or publication. No live provider call is part of acceptance.
