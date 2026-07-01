# Goose recipe-backed subagent adaptation

Animus adapts Goose recipe-backed subagents as explicit, policy-gated child-run DTOs. The reusable adapter layer validates delegation data only; it does not spawn subagents, discover external CLIs, inherit credentials, or merge child context into the parent.

## Runtime boundary

- `SubagentDelegation` validates child identity, role, task, optional admitted recipe/profile references, parameter binding labels, tool ceilings, workspace scope, model profile, timeout/turn ceilings, return mode, summary schema, delegation source, and optional external descriptor.
- Delegation starts only from a user, trusted skill, or admitted automation policy. Automatic and recursive starts fail before model, tool, external-agent, or host-effect dispatch.
- Recipe-backed requests require an admitted `RecipeTrustReceipt`; pending or missing recipe trust has no child-run influence.
- `resolve_subagent_capabilities` computes effective capabilities as the intersection of parent policy, recipe/profile ceilings, request ceilings, workspace scope, and host admission.
- `build_subagent_handoff` keeps child transcripts isolated by default. Summary-only and status-plus-summary modes do not import raw detail references; full detail requires explicit bounds.
- `ExternalSubagentDescriptor` accepts ACP/MCP descriptors only when an injected service label and declared tools are present. Ambient CLI discovery and inherited credentials are denied.
- `aggregate_subagent_outcomes` records success, failure, timeout, cancellation, malformed summary, denied tool, partial parallel success, and write conflicts without counting failed children as successful.

## Verification

Focused verification for this surface:

```sh
cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
nix run .#cairn -- gate proposal adapt-goose-recipe-backed-subagents --root .
nix run .#cairn -- gate design adapt-goose-recipe-backed-subagents --root .
nix run .#cairn -- gate tasks adapt-goose-recipe-backed-subagents --root .
```

The adapter tests include positive recipe-backed delegation, external descriptor, return-mode, capability-intersection, and aggregation cases plus negative implicit-spawn, recursive, untrusted recipe, unapproved automation, unauthorized tool, raw-detail bound, ambient external discovery, inherited credential, timeout, and write-conflict cases.
