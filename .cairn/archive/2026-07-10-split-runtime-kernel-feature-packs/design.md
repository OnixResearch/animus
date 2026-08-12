# Design: Split the runtime kernel from feature packs

## Context

`docs/runtime-composition-preludes.md` already points new embedders toward focused preludes, but the crate still defaults to broad compatibility. The code has a pure transition core and many domain modules. This change makes that separation explicit and testable.

## Decisions

### 1. Define a kernel boundary before moving code

**Choice:** First define the runtime kernel API as typed input/config, turn state, loop fabric trait, model/tool dispatch plans, capability registry, runtime events, run reports, and adapter orchestration over injected services.

**Rationale:** Naming the kernel boundary gives later moves a stable target and prevents optional domains from being accidentally included in minimal harness builds.

### 2. Keep compatibility exports during migration

**Choice:** `animus-runtime` keeps existing flat root exports and `compat` behavior while adding a documented kernel path and feature-pack paths.

**Rationale:** This avoids breaking existing consumers and allows future changes to migrate imports gradually.

### 3. Feature packs declare dependencies in the catalog

**Choice:** Headless, provenance, worker-team, hooks/policy, batch, compaction, dataset export, quality, research, scaffold, status, and orchestration packs must declare dependencies and authority posture in the component catalog.

**Rationale:** Runtime packs are useful Lego blocks only if embedders know which other blocks they snap to and which host authority remains outside them.

### 4. Kernel remains Steel-neutral

**Choice:** The kernel depends on a neutral `LoopFabric` interface. Steel remains an optional loop-fabric adapter behind a feature or separate crate.

**Rationale:** Embedders should be able to use replay, test, Rust, or other fabric implementations without pulling Steel.

## Risks / Trade-offs

- Splitting runtime APIs can create duplicate import paths. Mitigate with facade parity tests and migration docs.
- Some domain modules may currently depend on root-level exports. Mitigate by moving internal imports toward kernel/prelude modules before crate extraction.
- Changing defaults would be breaking. This change records migration guidance and does not flip defaults without a later accepted change.
