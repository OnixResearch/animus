# Design: TUI prompt input and keymap

## Context

The conversation render model shows the transcript; the streaming view shows live output. To close the loop, the harness needs a prompt where the user types, edits, recalls history, completes, and invokes slash commands. Reusable crates already define workflow-command manifests (`adapt-claude-code-workflow-commands`), skill manifests (`adapt-hermes-skill-manifests`), and plugin/tool manifests (`adapt-hermes-plugin-tool-manifests`); the TUI should populate menus from those without re-implementing dispatch semantics.

## Decisions

### 1. Input state is a pure reducer

**Choice:** The prompt editor state is advanced by applying key/command events to a pure reducer; terminal reads, raw-mode, and event polling stay in the shell.

**Rationale:** Input behavior is testable without a terminal and replayable for receipts.

### 2. Slash commands are detected, not executed as shell

**Choice:** The input reducer detects leading `/` invocations and resolves them against typed workflow-command and skill/plugin manifests. Dispatch is routed through the runtime's existing command/workflow path, never by executing prompt text as a shell script.

**Rationale:** Matches the bounded `adapt-claude-code-workflow-commands` scope that command text cannot bypass policy.

### 3. Completion and history are bounded

**Choice:** Completion candidates and input history are bounded collections with deterministic ordering; unbounded growth is rejected.

**Rationale:** Prevents memory blowups and keeps replay deterministic.

### 4. Keybindings are host-only

**Choice:** `rat-keymap` and `rat-leaderkey` types live only in `animus-tui`; reusable crates import no keybinding types.

**Rationale:** Preserves the archived dependency boundary.

### 5. Downstream-only composition

**Choice:** The input view consumes workflow-command and manifest contracts as-is and composes on the conversation render model. No new field is added to a reusable crate.

**Rationale:** Keeps the TUI strictly downstream.

## Risks / Trade-offs

- Slash detection could be confused by literal `/` text; the reducer must distinguish command invocations from prose.
- Keybinding complexity can leak into contracts if modes are over-modeled; keep modes in the shell.
- Completion sources can drift; populate from accepted manifest contracts only.
