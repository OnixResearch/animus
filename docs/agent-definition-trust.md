# Agent definition trust

Animus treats reusable agent definitions as typed data at the adapter boundary. A definition can describe a persona, prompt fragments, preferred model role, and ceiling labels for capabilities, tools, memory, context references, and workflow abilities, but parsing the definition never grants authority.

The `animus-adapters` agent-definition core owns four pure checks:

- `AgentDefinitionManifest::validate` converts host-loaded manifest data into bounded typed data.
- `discover_agent_definitions` orders already-loaded definitions deterministically and rejects same-precedence duplicate names instead of relying on filesystem order.
- `activate_agent_definition` requires host admission for project definitions and clamps every model/tool/capability/memory/context/workflow request below active runtime policy.
- `AgentDefinitionReceipt` and `validate_agent_definition_receipt_text` keep activation evidence to safe metadata, hashes, labels, and redaction markers.

Project or workspace definitions are untrusted by default. They cannot influence prompt assembly, model selection, tool access, memory, context references, or workflow behavior unless the host admits the manifest identity. User/global and explicit host definitions still remain ceiling-only data: runtime policy and capability checks decide effective authority.

Receipts must not include raw prompt bodies, full project paths, credentials, provider payloads, or hidden memory bodies. Store manifest and policy identities as BLAKE3 hashes plus redaction markers instead.
