# Agent Harness Runtime Specification

## ADDED Requirements

### Requirement: Stack drain supervisor uses workspace-local placement [r[agent_harness_runtime.ralph_stack_drain.workspace_placement]]

Animus MUST document the Pi-hosted stack drain supervisor as a workspace-local operator artifact at the sibling-workspace root rather than as a packaged Animus runtime or child-repository-owned stack authority.

#### Scenario: Canonical helper lives at the workspace root [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.root]]

- GIVEN a sibling workspace contains Animus and other independent native Cairn repositories
- WHEN an operator locates the canonical stack drain helper
- THEN the helper MUST live at the workspace root and default discovery to that current directory
- AND the current Animus repository MUST NOT retain a second canonical implementation copy.

#### Scenario: Root README documents operation [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.readme]]

- GIVEN an operator opens the workspace-root README
- WHEN they inspect stack drain guidance
- THEN it MUST show Pi Steel tool inputs for plan mode, one bounded run round, all-round mode, and explicit model override
- AND it MUST explain inherited model defaults, fresh Pi sessions, timeout behavior, blockers, validation, and no-push or no-PR constraints.

#### Scenario: Animus documentation points outward accurately [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.animus_docs]]

- GIVEN an operator reads current Animus Ralph documentation
- WHEN it references the stack drain supervisor
- THEN it MUST point to the workspace-root helper
- AND it MUST state that the helper is a local workspace operator surface rather than an embedded or packaged Animus host.

#### Scenario: Historical evidence remains immutable [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.history]]

- GIVEN prior archived Cairn evidence names the original Animus script path
- WHEN the canonical helper moves
- THEN historical archive content MUST remain unchanged
- AND current docs and accepted requirements MUST describe the new canonical placement without rewriting past evidence.

#### Scenario: Placement tests cover positive and negative behavior [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.tests]]

- GIVEN the helper has moved to the workspace root
- WHEN focused verification runs
- THEN its positive and negative Steel self-tests and default plan discovery MUST pass from that root
- AND a no-tools Pi smoke MUST produce a no-progress blocker without changing lifecycle state.

#### Scenario: Placement change closes cleanly [r[agent_harness_runtime.ralph_stack_drain.workspace_placement.closeout]]

- GIVEN implementation, documentation, and focused tests are complete
- WHEN Cairn gates, sync, archive, and validation run
- THEN the placement requirement MUST land in accepted specs and the change MUST archive successfully
- AND unrelated staged, unstaged, and untracked work MUST remain preserved.
