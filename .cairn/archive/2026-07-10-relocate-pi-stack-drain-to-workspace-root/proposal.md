## Why

The Pi-hosted stack drain supervisor coordinates sibling repositories and is not an Animus-only runtime surface. Keeping its canonical script under `animus/scripts/` makes invocation depend on one child repository, obscures the workspace-wide ownership boundary, and gives the misleading impression that an Animus checkout packages the whole stack operator.

The sibling-workspace root is the correct local operator location. A root README can explain that each child remains an independent repository while documenting one consistent Pi invocation for the whole stack.

## What Changes

- Move the canonical `ralph-drain-stack.scm` helper from the Animus repository to the OnixResearch workspace root.
- Change its default workspace root from the Animus parent to the current directory.
- Add a workspace-root `README.md` describing repository independence, plan/run usage, model selection, safety boundaries, and verification.
- Update Animus guidance to point at the workspace-local helper without presenting it as a packaged Animus runtime artifact.

## Impact

- **Workspace files**: `../ralph-drain-stack.scm` and `../README.md` relative to Animus.
- **Animus files**: removal of `scripts/ralph-drain-stack.scm`, documentation path updates, accepted lifecycle placement contract, and this Cairn package.
- **Testing**: Steel self-test from the workspace root, plan-mode discovery with default arguments, positive Pi no-tools blocker smoke, Cairn gates, and lifecycle validation.
- **Non-claims**: The workspace root remains a sibling checkout container, not a monorepo, shared VCS boundary, shared release unit, or durable package source.
