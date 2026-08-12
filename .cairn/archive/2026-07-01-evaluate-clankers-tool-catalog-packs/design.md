# Design: Optional tool catalog and capability packs

## Context

Clankers' embedded tool-kit manifest validates tool descriptors without starting runtimes. Animus can adopt this later as an embedder-facing contract once tool dispatch and effect services are stable.

## Decisions

### 1. Catalog descriptors are data, not execution

Tool catalog validation reads typed descriptors only. It must not execute tools, start plugin runtimes, load provider clients, read credentials, inherit environment, or contact network services.

### 2. Capability packs are explicit

Descriptors group capabilities such as observe/read, workspace mutation, shell, network, process, secret-adjacent, or host-custom. Dangerous capabilities require explicit approval policy and redaction.

### 3. Name collisions fail closed

Catalog construction rejects duplicate tool names, collisions with built-ins, unknown runtime kinds, invalid schemas, and unsafe capability/default-approval combinations.

### 4. Nickel is preferred for repo-owned manifests

If repo-owned policy is needed, use Nickel contracts and checked exports. Runtime Rust consumes typed data; it does not evaluate Nickel during tool dispatch.

### 5. Preserves remains a boundary option

If cross-language catalog exchange is required, Animus may project descriptors through Preserves without making Preserves the internal tool execution authority.

## Risks / Trade-offs

- Catalog manifests can be mistaken for tool execution authority. Keep the execution path behind injected tool/effect services.
- Capability taxonomy can grow too fast. Start with a minimal set required by Animus adapters.
- JSON schema validation can become complex. Use simple structural checks first.