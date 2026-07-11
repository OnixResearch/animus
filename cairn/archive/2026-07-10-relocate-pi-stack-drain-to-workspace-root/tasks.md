## Phase 1: Relocate the operator surface

- [x] [serial] Move the canonical Steel supervisor to the workspace root and change its default root and usage text for root-local invocation. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.root]
- [x] [serial] Add a workspace-root README covering sibling repository boundaries, plan/run usage, inherited model defaults, explicit model overrides, safety behavior, and non-claims. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.readme]

## Phase 2: Repair Animus references

- [x] [serial] Update current Animus README, Ralph documentation, and agent guidance to reference the workspace-local helper and avoid packaged-runtime claims. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.animus_docs]
- [x] [serial] Preserve historical archive text while adding an accepted placement contract for the current workspace-local boundary. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.history]

## Phase 3: Verification and closeout

- [x] [serial] Run positive and negative Steel self-tests from the workspace root, verify default plan discovery, and run a no-tools Pi blocker smoke without lifecycle mutation. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.tests]
- [x] [serial] Run Cairn proposal/design/tasks gates and validation, sync the placement requirement, archive this package, and preserve unrelated worktree changes. r[agent_harness_runtime.ralph_stack_drain.workspace_placement.closeout]
