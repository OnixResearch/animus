# Design: Adapt Goose ACP/MCP interoperability for Animus

Goose is useful as an interop reference because it treats MCP and ACP as first-class ways to connect tools, data, and other agents. Animus should adapt the descriptor and session-contract ideas while preserving strict adapter ownership of effects.

## Decisions

### 1. MCP descriptors are capability claims, not execution authority

**Choice:** MCP descriptors may declare transport kind, command label or endpoint label, tool/resource/prompt identities, schema IDs, timeout, environment key names, and available-tool ceilings. Validation is data-only.

**Rationale:** This matches Animus' manifest boundary: descriptors tell the host what might be needed, but they never launch processes or open network connections.

### 2. ACP descriptors separate agent identity from provider authority

**Choice:** ACP descriptors name an external agent adapter, model/profile preference, permission-mode mapping, tool pass-through policy, session correlation behavior, and known limitations.

**Rationale:** ACP agents can be providers or subagents, but they must remain explicit configured services rather than ambient fallbacks.

### 3. Tool pass-through is constrained by both sides

**Choice:** Passing Animus tools to ACP agents or MCP servers requires intersection of Animus capability ceilings, active permission mode, host admission, and the external agent's declared mode.

**Rationale:** Interop should not widen tool access merely because a bridge exists.

### 4. Secrets are key names only in portable manifests

**Choice:** Portable descriptors may request environment key names. Secret values are host-owned and never exported in recipes, manifests, receipts, or session ledgers.

**Rationale:** Goose's env-key prompting pattern is useful, but descriptor portability must not leak credentials.

### 5. Interop outcomes are receipt-backed

**Choice:** Discovery, session start, tool pass-through, denial, timeout, unavailable binary, malformed schema, and network-policy outcomes produce safe receipts with redacted command/endpoint identities.

**Rationale:** Standards bridges are hard to debug without evidence, but evidence must be safe to include in reviews.
