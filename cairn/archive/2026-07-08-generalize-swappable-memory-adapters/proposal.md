# Proposal: Generalize swappable memory adapters

## Why

Animus already has bounded Claude Code and Codex memory references, but the next memory iteration needs a generic adapter architecture that can support multiple agent patterns without choosing one storage model. Current systems split this problem differently: Claude Code and Goose use inspectable local files, Goose also exposes memory through MCP tools, Codex uses opt-in generated memories and background consolidation, LangGraph separates short-term checkpointers from long-term stores, AutoGen defines a small `Memory` protocol, Letta uses attachable memory blocks, CrewAI emphasizes scoped recall and scoring, and Mem0 provides an external memory layer.

Animus should absorb the common contract: memory is optional context carried through swappable adapters, not hidden authority, ambient storage, or provider-specific state.

## What Changes

- Define a generic memory adapter family for source/import, store, retrieval/search, extraction, embedding/search, admission, and instruction-source behavior.
- Normalize adapter outputs into evidence-backed memory record envelopes before reusable runtime or prompt assembly consumes them.
- Add pure store-agnostic retrieval planning over scopes, freshness, confidence, relevance scores, conflicts, and prompt budgets.
- Keep extraction, admission, storage, deletion, and prompt injection as separate steps so generated candidates cannot silently persist or influence prompts.
- Emit safe operation receipts and influence ledgers that record adapter outcomes without raw memory bodies, credentials, provider payloads, full paths, or unbounded prompt text.

## Impact

- **Files**: future adapter DTOs, memory planner core, safe receipts, policy fixtures, focused tests, docs, and Cairn artifacts.
- **Testing**: positive adapter-equivalence fixtures and negative missing-adapter, malformed-record, denied-candidate, stale/conflicting, budget, and leak fixtures.
- **Non-claims**: this does not add a mandatory memory database, require vector search, adopt a remote memory service, or make memories authoritative policy.
