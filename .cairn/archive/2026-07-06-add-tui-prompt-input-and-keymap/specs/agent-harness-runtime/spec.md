# agent-harness-runtime Change Spec: TUI prompt input and keymap

## Requirements

### Requirement: Pure prompt input reducer

r[agent_harness_runtime.tui_input.editor] The prompt editor state MUST be a pure reducer over key/command events with submit, undo, cursor movement, and bounded input history, and terminal reads MUST stay in the shell.

#### Scenario: Multiline edit and submit produce a prompt message

r[agent_harness_runtime.tui_input.editor.submit]
- GIVEN a user types, edits across lines, and submits through key events
- WHEN the input reducer applies those events
- THEN it produces a typed prompt message ready for runtime dispatch
- AND no terminal read occurs inside the reducer.

#### Scenario: Input history is bounded

r[agent_harness_runtime.tui_input.editor.bounded_history]
- GIVEN the user submits more prompts than the configured history bound
- WHEN history is recorded
- THEN older entries are evicted with safe metadata
- AND history growth is bounded.

### Requirement: Bounded completion menu

r[agent_harness_runtime.tui_input.completion] The completion menu MUST be a bounded, deterministically ordered collection populated from accepted manifest sources, and unbounded or unordered completion growth MUST be rejected.

#### Scenario: Unknown completion source is rejected

r[agent_harness_runtime.tui_input.completion.unknown_source]
- GIVEN a completion source is not an accepted manifest contract
- WHEN completion population runs
- THEN the source is rejected with a typed diagnostic
- AND no candidates from that source are offered.

### Requirement: Slash commands are detected, not executed as shell

r[agent_harness_runtime.tui_input.slash_menu] The input reducer MUST detect leading slash invocations and resolve them against typed workflow-command and skill/plugin manifests by routing through runtime paths, and MUST NOT execute prompt text as a shell script or bypass permission and capability policy.

#### Scenario: Slash command routes through runtime

r[agent_harness_runtime.tui_input.slash_menu.runtime_route]
- GIVEN a user submits `/review` matching a registered workflow command
- WHEN the input reducer resolves it
- THEN it produces a typed workflow invocation for the runtime
- AND no shell process is spawned from the prompt text.

#### Scenario: Literal slash text is not treated as a command

r[agent_harness_runtime.tui_input.slash_menu.literal]
- GIVEN a prompt begins with `/` but does not match any registered command
- WHEN the input reducer evaluates it
- THEN it is treated as ordinary prompt text
- AND no command dispatch occurs.

### Requirement: Host-only keybindings

r[agent_harness_runtime.tui_input.keymap] Keybinding and leader-menu types from `rat-keymap` and `rat-leaderkey` MUST live only in `animus-tui`, and reusable crates MUST NOT import keybinding types.

#### Scenario: Keybinding types stay in the TUI crate

r[agent_harness_runtime.tui_input.keymap.host_only]
- GIVEN maintainers inspect reusable crate dependencies and public exports
- WHEN the keymap and leader menu are added
- THEN `rat-keymap` and `rat-leaderkey` types appear only in `animus-tui`
- AND reusable crates import no keybinding types.

### Requirement: Prompt input and keymap verification

r[agent_harness_runtime.tui_input.verification] The change MUST include focused positive and negative tests for edit/submit, bounded history, completion ordering and source rejection, slash detection and runtime routing, literal slash handling, host-only keybindings, and keybinding-type isolation before closeout.

#### Scenario: Closeout evidence covers input fixtures

r[agent_harness_runtime.tui_input.verification.closeout]
- GIVEN maintainers intend to archive the prompt input and keymap change
- WHEN closeout validation runs
- THEN evidence names focused editor, completion, slash-menu, keymap, dependency-isolation, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
