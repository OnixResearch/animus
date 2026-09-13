# Design

## Context

The unchanged provider crate fails the pinned Octet catalog. Its failed dependency check hides the downstream host result. Independent package checks are necessary to expose both surfaces. A combined check remains necessary for the final integrated subject.

The repository already owns the provider core, SSE parser, tool declarations, explicit host adapters, and persistent session composition. The repair reuses those components at the selected immutable Animus revision. A new shared library or sibling dependency cannot correct these local invariant and layout findings.

## Decisions

### Decision: Preserve the strict subject

**Choice:** Add independent provider and live-agent checks beside the combined host check. Use the official `octet-deny-all` hook with all targets and all features. The immutable source is `OnixResearch/octet` at `f39ab155170f32ffc6408782d20c0d488137c995`.

**Rationale:** A passing dependency build is not a downstream lint result. The repo config does not deny every pinned lint. The official hook enables the full catalog and denies unknown lint names. Host checks use `DYLINT_RUSTFLAGS` for the selected packages. Other package checks retain their current invocation and config.

### Decision: Split by capability and effect ownership

**Choice:** Keep pure request admission, schema validation, parsing, and policy separate from transport, persistence, and composition. Use qualified paths and narrow modules. Preserve the public facade where its contract remains valid.

**Rationale:** Structural repairs must expose ownership, not hide a large module behind aliases or move effects into the core.

### Decision: Preserve decisions through checked transitions

**Choice:** Use bounded iterative schema traversal, checked arithmetic, explicit rejection paths, and local invariant assertions. Keep named policy bounds and existing error vocabulary.

**Rationale:** Refactoring cannot turn rejection into success, panic on untrusted data, or mutate state after refusal.

### Decision: Name portable count migrations

**Choice:** Replace public machine-width quantities with explicit portable bounds where the strict rule requires them. Inventory and migrate directly affected callers. Preserve const constructors where their revised signatures permit it.

**Rationale:** `usize` cannot remain a public portable contract. A changed integer type can break source compatibility. Evidence must name that change instead of hiding it with an alias or an exemption.

### Decision: Retain explicit host clock ownership

**Choice:** Acquire `cap_std::time::MonotonicClock` at the existing per-cell shell boundary. Pass the capability and start instant to the private checkpoint control. Keep the public four-input constructor unchanged.

**Rationale:** The direct dependency already pins `cap-std` 3.4.5 and `cap-primitives` 3.4.5 through Cargo checksums. Its clock separates authority-bearing time access from authority-free instant values. It preserves the underlying standard monotonic clock operations. This change adds no dependency, reusable clock authority, sampling policy, or preemptive deadline.

### Decision: Keep gate ownership limits explicit

**Choice:** Keep dependency findings visible. Do not relocate unrelated runtime sources or change a dependency API without a declared scope decision.

**Rationale:** The pinned module-file rule reads workspace paths from the compiler source map, including imported runtime files. The host also lacks an explicit empty constructor for `MemoryCodingSessionPersistence`. Hiding either finding cannot qualify the host.

## Parallel ownership

- Parser worker: provider `stream.rs` and `tool.rs`, with new `codec/` and `declarations/` subtrees. It does not edit the facade or other tests.
- Provider worker: remaining provider sources, binaries, tests, and the facade. It coordinates only through the declared module names and output manifests.
- Session worker: live-agent sources and tests. It leaves other crates and lifecycle files to the parent.
- Parent: cross-scope composition, direct callers, flake checks, documentation, lifecycle, and final verification. The parent also migrates direct path and marker references in the existing host rails.

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
