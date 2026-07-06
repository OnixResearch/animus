# Tasks

- [x] [depends:add-tui-conversation-render-model] Add a pure multiline prompt input reducer over key/command events with submit, undo, cursor movement, and bounded input history. r[agent_harness_runtime.tui_input.editor]
- [x] [depends:editor] Add a bounded, deterministically ordered completion menu driven by accepted manifest sources. r[agent_harness_runtime.tui_input.completion]
- [x] [depends:editor] Add slash-command detection and a dispatch surface populated from workflow-command and skill/plugin manifests that routes through runtime paths and never executes prompt text as shell. r[agent_harness_runtime.tui_input.slash_menu]
- [x] [depends:editor] Add a host-only `crossterm` key binding layer with default keybindings and no keybinding types in reusable crates. r[agent_harness_runtime.tui_input.keymap]
- [x] [depends:editor,completion,slash_menu,keymap] Add negative tests for malformed slash input, unbounded history/completion growth, unknown commands, stale completion, and keybinding-type leakage. r[agent_harness_runtime.tui_input.verification]
