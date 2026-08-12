# Proposal: Evaluate Goose MCP Apps surface patterns for Animus

## Why

Goose MCP Apps let MCP servers return interactive UI resources that render inside Goose Desktop with a sandbox, content security policy, permission policy, host theme sync, resize notifications, and chat message callbacks. This is a useful product-surface reference, but the Goose docs mark MCP Apps experimental and Animus does not yet have stable UI/headless capability boundaries for interactive extension views.

This change evaluates which MCP Apps ideas fit Animus' future TUI/headless surfaces without implementing iframe rendering, browser permissions, or persistent app windows.

## What Changes

- Document MCP Apps as a bounded reference for interactive tool-result UI metadata.
- Define an evaluation matrix for UI resource identity, MIME/profile tags, content security policy, permission requests, host-context sync, message callbacks, sizing, and lifecycle bounds.
- Identify which pieces belong in reusable DTOs versus optional TUI/headless host adapters.
- Add non-adoption constraints for iframe execution, browser device permissions, remote resource loading, and JavaScript bridge behavior.
- Add evaluation fixtures for safe metadata descriptors and unsafe descriptors that request broad domains, device permissions, unbounded callbacks, or ambient host control.

## Impact

- **Files**: evaluation note, optional metadata descriptor sketch, safety matrix, fixtures, docs, and focused validation checks if promoted later.
- **Testing**: positive safe-descriptor evaluation fixtures and negative unsafe CSP/permission/callback fixtures.
- **Non-claims**: this change does not implement MCP Apps, iframe rendering, browser permissions, persistent app windows, JavaScript bridges, or tool-result UI execution.
