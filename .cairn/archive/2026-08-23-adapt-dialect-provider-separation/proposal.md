## Why

Animus composes exact per-model prompt, skill, memory, and tool profiles while provider hosts own wire protocols and transport effects. One OpenAI-compatible or routing transport can serve models from several behavioral families. A transport name alone does not define the model-facing prompt, schema, project-instruction format, or subagent protocol.

`digitallyinduced/haskell-agent` separates provider transports from model dialects. Providers own authentication, billing, transport, and provider continuation. Dialects own the model-facing contract. This change makes that separation explicit in Animus model-target resolution.

## What Changes

- Resolve every model target to an independent provider transport and model-facing dialect.
- Assign authentication, billing, transport, and provider continuation to provider hosts.
- Assign prompts, tool surface, schema conventions, project-instruction formatting, and subagent protocol to dialect profiles.
- Bind both identities into the provider-neutral request snapshot.
- Reject unknown, incompatible, or cross-family combinations before provider contact.

## Impact

- **Files**: adapter DTOs, model-runtime profiles, provider registry, request snapshots, provider hosts, fixtures, documentation, and this lifecycle package.
- **Testing**: focused model-runtime-profile, provider-registry, request-snapshot, and provider-host rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- A new provider wire protocol.
- Claims that different providers or model families are semantically equivalent.
- Automatic dialect inference from model output.
- Provider-side rewriting of Animus prompt or tool semantics.
