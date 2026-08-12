## Context

`/home/brittonr/git/OnixResearch` contains independent repositories with separate VCS state, build systems, lifecycle trees, and commits. The stack drain supervisor discovers direct child native Cairn roots and launches Pi inside one selected child at a time. Its behavior is therefore workspace orchestration even though its first implementation was committed in Animus.

The workspace root is intentionally not a shared Git repository. Root-level operator files are local coordination assets; they do not become part of every child release or establish cross-repository dependency evidence.

## Decisions

### 1. Make the workspace root canonical

**Choice:** Move the implementation to `../ralph-drain-stack.scm` relative to Animus and remove the repository-owned script path.

**Rationale:** The path now matches the scope being orchestrated and can be invoked without selecting an arbitrary child as owner.

### 2. Default to the current directory

**Choice:** The relocated script uses `.` as its default workspace root. Operators running Pi from the OnixResearch root need no path override; callers elsewhere may still pass `--workspace-root` explicitly.

**Rationale:** Safe defaults should match the canonical documented location.

### 3. Keep Animus as a contract reference, not the file owner

**Choice:** Animus retains the bounded behavior and safety contract in its accepted spec and documentation, but references the workspace-local path and states that it is not a packaged Animus runtime surface.

**Rationale:** Historical implementation evidence remains valid while current ownership becomes accurate.

### 4. Add root documentation

**Choice:** The root README explains workspace shape, plan-only and run invocations through Pi's Steel tool, inherited model selection and overrides, round limits, timeout behavior, completion evidence, and cross-repository non-claims.

**Rationale:** The script must be understandable where operators encounter it.

### 5. Do not create a root VCS or lifecycle

**Choice:** This change does not initialize Git, Jujutsu, Cairn, a flake, or a shared build at the workspace root.

**Rationale:** Root orchestration must not collapse independent repository boundaries.

## Risks / Trade-offs

- Root files are local workspace assets and are not preserved by an Animus clone alone.
- Existing references to the committed Animus path must be updated together with the move.
- Historical Cairn archives continue to name the original path and remain unchanged.
- A future durable stack-operator repository may supersede this local placement.
