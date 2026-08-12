# Proposal: Add memory routing contracts

## Why

Animus has a store-agnostic retrieval planner, but it assumes candidate records have already been retrieved from the relevant adapters. Once memory grows across episodic, semantic, procedural, reflection, vector, graph, file, and remote backends, hosts need a typed way to decide which stores to query or update without embedding route logic in provider prompts or ambient storage behavior.

Animus should add a pure memory routing contract that consumes safe route signals and produces deterministic read/write route plans while keeping classifiers, extractors, stores, and clocks host-owned.

## What Changes

- Define memory route request, route candidate, route plan, and route receipt DTOs for read and write memory operations.
- Represent route signals such as task intent, record category, allowed scopes, adapter roles, match reasons, confidence, freshness, and prompt budget as safe typed data.
- Require routing to fail closed when classifier output is malformed, policy denies a route, or required adapters are unavailable.
- Add focused positive and negative fixtures for route selection, denied routes, unavailable adapters, unsafe classifier output, and no ambient fallback.

## Impact

- **Files**: future adapter DTOs, pure routing planner, memory docs, tests, and Cairn artifacts.
- **Testing**: positive route plans and negative malformed-route, denied-scope, unavailable-adapter, unsupported-role, and leak fixtures.
- **Non-claims**: this does not add an LLM router, vector search, graph traversal, provider extractor, database, MCP server, or automatic memory write behavior.
