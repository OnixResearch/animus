# agent-harness-runtime Change Spec: Clankers neutral session ledger

## Requirements

### Requirement: Animus-owned neutral ledger DTOs

r[agent_harness_runtime.clankers_neutral_ledger.dto] Animus MUST define its own neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, unsupported entries, replay metadata, and typed replay errors without depending on Clankers session, daemon, TUI, provider, Automerge, or concrete storage types.

#### Scenario: DTOs stay storage-neutral

r[agent_harness_runtime.clankers_neutral_ledger.dto.storage_neutral]
- GIVEN reusable Animus ledger DTOs are reviewed
- WHEN dependency and field boundaries are inspected
- THEN the DTOs do not require concrete storage, filesystem paths, wall-clock construction, daemon events, TUI types, provider-native payloads, or Clankers transcript IDs
- AND host persistence can wrap them without changing pure replay semantics.

### Requirement: Pure deterministic replay

r[agent_harness_runtime.clankers_neutral_ledger.replay] Ledger replay MUST be a pure deterministic function over in-memory entries that returns model-visible context plus safe metadata counts or a typed error.

#### Scenario: Supported entries replay in order

r[agent_harness_runtime.clankers_neutral_ledger.replay_positive]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and safe receipt entries
- WHEN replay runs
- THEN model-visible user, assistant, tool, and summary context is returned in deterministic order
- AND usage and receipt entries contribute safe metadata counts without leaking raw storage details.

### Requirement: Unsupported entries fail closed

r[agent_harness_runtime.clankers_neutral_ledger.unsupported] Replay MUST fail closed on unsupported or adapter-owned ledger entries rather than silently dropping or exposing them.

#### Scenario: Unsupported payloads do not leak

r[agent_harness_runtime.clankers_neutral_ledger.unsupported.redacted]
- GIVEN replay encounters an unsupported or adapter-owned entry
- WHEN replay is required before a model request
- THEN replay returns a typed unsupported-entry error
- AND the error includes only a safe reason class, not the raw unsupported payload.

### Requirement: Resume requires an injected ledger service

r[agent_harness_runtime.clankers_neutral_ledger.missing_service] Runtime paths that require session resume MUST consume an injected ledger service and MUST fail before adapter dispatch when the service or requested session is unavailable.

#### Scenario: Missing ledger service blocks dispatch

r[agent_harness_runtime.clankers_neutral_ledger.missing_service.blocks]
- GIVEN a runtime request requires resume semantics
- WHEN no ledger service is injected or the requested session is missing
- THEN runtime returns a typed unavailable-service or missing-session failure
- AND no model or tool adapter call is made.

### Requirement: Pure branch query helpers when parent links exist

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries] If Animus ledger entries include parent/head relationships, branch query helpers MUST be pure and MUST detect duplicate IDs, missing parents, cycles, active branch ancestry, and abandoned/dead-end attempts.

#### Scenario: Malformed graphs fail without panic

r[agent_harness_runtime.clankers_neutral_ledger.branch_queries.invalid]
- GIVEN a ledger graph has duplicate IDs, a missing parent, a cycle, or reversed-order assumptions
- WHEN query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silent ancestry corruption occurs.

#### Scenario: Ledger closeout is evidenced

r[agent_harness_runtime.clankers_neutral_ledger.closeout]
- GIVEN the neutral ledger change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused replay tests, unsupported-entry negative tests, missing-service tests, dependency-isolation checks, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
