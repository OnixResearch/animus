# Proposal: Adapt plugin/action manifests for Animus

## Why

Herdr's plugin surface separates manifest-declared actions, event hooks, pane entrypoints, link handlers, platform support, and minimum host version from runtime registration. Animus already has a runtime-neutral plugin descriptor boundary, but it needs a richer action/event manifest vocabulary before host tool gateways, TUI panes, and workflow plugins become concrete.

The useful adaptation is manifest role separation and host-version/platform gating, not Herdr's TOML format or command execution behavior.

## What Changes

- Extend Animus-owned plugin descriptor concepts to include action descriptors, event-hook descriptors, host-pane entrypoint descriptors, and link-handler descriptors as data-only declarations.
- Require manifest identity, version, minimum host capability/version, effective platform support, entrypoint IDs, titles, contexts, and command or artifact labels to validate before link or invocation.
- Define safe invocation context DTOs for active workspace, tab, pane, agent status, worktree/session provenance, request identity, plugin config/state directories, and source metadata.
- Treat link, enable, disable, list, action-list, and invocation planning as host-owned registry operations with warnings for unknown events or incomplete optional metadata.
- Ensure manifest validation and action planning never execute commands, open panes, read plugin storage, or weaken tool/policy gates.

## Impact

- **Files**: future manifest DTOs, Nickel schema candidates, plugin registry contracts, action invocation planner, host-pane entrypoint descriptors, tests, and docs.
- **Testing**: positive action/event/pane/link-handler manifest fixtures; negative missing version, unsupported platform, newer host version, unknown event, disabled plugin, duplicate action, unsafe context, and manifest-time execution tests.
- **Non-claims**: this does not adopt Herdr's TOML plugin format, install plugins from GitHub, run plugin commands from core, add managed plugin storage, or make plugin panes a required Animus product surface.
