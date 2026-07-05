# Proposal: TUI prompt input and keymap

## Why

A pi-style agent harness needs an interactive prompt: a multiline editor with submit, input history, completion, slash-command detection, and a keybinding/leader-menu layer. This change adds the prompt input and keymap surface to `animus-tui`, with a pure input-state reducer and a thin terminal shell, consuming existing workflow-command and skill/plugin manifest contracts downstream without executing slash text as shell or bypassing permission and capability policy.

## What Changes

- Add a multiline prompt editor with submit, undo, cursor movement, and bounded input history in `animus-tui`.
- Add a completion menu and a slash-command detection/dispatch surface that populates from Claude Code workflow-command manifests and Hermes skill/plugin manifests without executing command text as shell.
- Add a `rat-keymap`/`rat-leaderkey` binding layer and a leader menu, host-only, with no keybinding types in reusable crates.
- Keep input state as a pure reducer over key/command events; terminal reads stay in the shell.
- Compose on the conversation render model; add no new reusable-crate contracts.

## Impact

- **Files**: new `animus-tui` input, keymap, completion, and slash-menu modules; pure input reducer and tests.
- **Testing**: positive tests for edit/submit/history/completion/slash detection and keymap binding; negative tests for malformed slash input, unbound growth, unknown commands, stale completion, and keybinding-type leakage into reusable crates.
- **Non-claims**: this change does not execute slash text as shell, does not bypass permission/capability policy, does not own prompt assembly, and does not add a render loop.
