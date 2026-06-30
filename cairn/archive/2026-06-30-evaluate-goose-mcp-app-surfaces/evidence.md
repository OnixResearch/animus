# Evidence: evaluate Goose MCP Apps surfaces

## Scope

- Change: `evaluate-goose-mcp-app-surfaces`
- Requirement IDs: `r[agent_harness_runtime.goose_mcp_app_surfaces.*]`
- Reference: Goose MCP Apps are used as a bounded product-surface reference only.
- Non-claims: no iframe rendering, browser permissions, JavaScript bridge, persistent app window, remote resource loading, or runtime UI execution is implemented.

## Baseline

- `nix develop -c cargo test -p animus-adapters --locked`: PASS before MCP Apps evaluation edits; 96 adapter tests passed.

## Lifecycle gates

- `nix run .#cairn -- gate proposal evaluate-goose-mcp-app-surfaces --root .`: PASS before implementation.
- `nix run .#cairn -- gate design evaluate-goose-mcp-app-surfaces --root .`: PASS before implementation.
- `nix run .#cairn -- gate tasks evaluate-goose-mcp-app-surfaces --root .`: PASS before implementation.

## Focused implementation checks

- `nix develop -c cargo test -p animus-adapters --locked mcp_app::`: PASS; 5 tests cover safe metadata, policy-gated remote domains, broad-domain rejection, broad-permission rejection, unbounded-callback rejection, ambient-host-control rejection, direct-message rejection, runtime-execution claims, and malformed descriptor data.
- `nix develop -c cargo test -p animus-adapters --locked`: PASS; 101 adapter tests passed.
- `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: PASS.
- `nix develop -c cargo fmt --check`: PASS.
- `nix develop -c scripts/cairn/check-roadmap --root .`: PASS after removing stale active-roadmap references to the archived `adapt-vllm-semantic-router-recipes` package and correcting this evaluation-only package so product-surface prerequisites apply to future promotion rather than evaluation closeout.
- `nix run .#cairn -- validate --root .`: PASS.

## Evidence safety

Evidence contains only command/status summaries. Descriptor validation is data-only and records prohibited effects: no HTML rendering, JavaScript execution, remote resource loading, browser permission requests, chat callback dispatch, or host-control execution.
