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

**Choice:** The operator approved the owner expansion after `44a221ff`. Permit coherent `animus-runtime` source relocation, an explicit empty `MemoryCodingSessionPersistence` constructor, direct caller migration, and owner controls. Preserve public module paths and `Default` behavior. Keep unrelated runtime logic unchanged.

**Rationale:** The pinned module-file rule reads workspace paths from the compiler source map, including imported runtime files. The host lacks an explicit empty constructor for its memory persistence owner. Owner repairs must not hide either finding or grant new external authority.

### Decision: Admit CLI arguments at the binary root

**Choice:** Preserve each existing positional grammar, safe usage error, activation order, and bounded credential stdin. Select a reviewed argument-acquisition component with actual parsing or authority semantics. Keep its process effects at the binary root and pass admitted values inward.

**Rationale:** Import aliases, unlinted shims, and renamed ambient calls do not create a capability boundary. Existing dependency revisions stay pinned. A new dependency revision or incompatible grammar requires a separate decision.

### Decision: Refuse unproved recovery and retry

**Choice:** Validate correlated commit observations before consuming protected pending state. Preserve the successful kernel-before-session journal order. Refuse a reopened kernel/session pair when its binding cannot prove safe continuation. Retain a terminal blocked state after a continuation attempt can have external effects; do not restore that attempt as retryable.

**Rationale:** Separate journals are not one transaction. Restoring a pending value cannot undo a provider call or persistence effect. Positive and negative controls must show exact state retention or safe refusal, not assume rollback.

**Imported source layout:** The first contract-module move did not reduce the eight runtime source files in the host compiler source map. The child-session owner now lives in `childsession/mod.rs`. Its contents, internal module name, and public `child_session_service` facade remain unchanged. The source rail names the new path and selects the actual `childsession` test module. Runtime controls and the full host catalog must verify this move.

**CLI component:** Reuse the locked `clap_lex` 1.1.0 package from crates.io. Its Cargo checksum is `c8d4a3bb8b1e0c1050499d1815f5ab16d04f0959b233085fb31653fbfc9d98f9`. Argument acquisition stays at each binary root. The lexer owns the argument buffer, cursor traversal, flag classification, and UTF-8 decoding. The binary adapter admits the existing grammar and safe errors. It rejects attached values and unknown flags. It preserves flag-like positional values and the activation-before-acquisition order. No library gains an argument reader. The optional direct edges use the existing opt-in features and do not change a dependency revision. This is a lexer contract, not a new authority token or an alias for an environment call.

**Owner API:** Add `CodingSessionHost::resume_with_gate` over the existing additive `CodingSessionEffectGate`. Keep `resume` behavior and its current-authority probe unchanged. The live root selects read-only recovery and refuses an automatic repair commit. Checking `RepairRequired` after `resume` is too late: `resume` can already commit the repair. This additive method prevents that write without skipping authority admission.

## Parallel ownership

- Parser worker: provider `stream.rs` and `tool.rs`, with new `codec/` and `declarations/` subtrees. It does not edit the facade or other tests.
- Provider worker: remaining provider sources, binaries, tests, and the facade. It coordinates only through the declared module names and output manifests.
- Session worker: live-agent sources and tests. It leaves other crates and lifecycle files to the parent.
- Parent: cross-scope composition, direct callers, flake checks, documentation, lifecycle, and final verification. The parent also migrates direct path and marker references in the existing host rails.

The initial workers each had 160 tool calls and 40 minutes. Those rounds have ended. The approved owner round gives CLI research 35 calls and eight minutes, and recovery repair 100 calls and 25 minutes. Recovery starts after the unchanged baseline and updated artifact gates pass. Consolidation starts before its final 20 calls. A separate final immutable-subject audit has 40 calls and ten minutes.

Workers cannot delegate, run memo, publish, edit another scope, or run shared checks or formatting. The parent owns runtime layout, persistence construction, CLI implementation, baseline checks, and final verification. Each worker writes its own report. Reports do not establish acceptance.

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
