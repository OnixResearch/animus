# Proposal: Add subagent fan-out/fan-in orchestration contracts

## Why

Subagents become more useful when a parent can safely ask several children to investigate, verify, or review in parallel. Animus already models partial parallel success and write conflicts for recipe-backed subagents, but it lacks a first-class fan-out/fan-in plan that coordinates budgets, cancellation, aggregation, and mutation arbitration across child-run kinds.

This change adds typed fan-out/fan-in contracts that preserve fail-closed behavior and keep mutation intents as data until policy arbitration accepts them.

## What Changes

- Add a fan-out plan with child-run set, concurrency policy, shared budget, cancellation policy, aggregation schema, and mutation-arbitration policy.
- Add deterministic fan-in aggregation that distinguishes success, partial success, failure, timeout, cancellation, malformed output, denied capability, and write conflict.
- Add cancellation and budget exhaustion semantics that do not count failed or cancelled children as success.
- Preserve mutation intents as unresolved or denied data until normal policy and coordination accept a later action.

## Impact

- **Files**: future fan-out DTOs, aggregation reducers, mutation-intent arbitration helpers, tests, docs, and coordinator integration.
- **Testing**: positive all-success and partial-success aggregation; negative timeout, cancellation, malformed summary, denied capability, write conflict, budget exhaustion, and mutation bypass tests.
- **Non-claims**: this does not execute children concurrently inside reusable runtime code or apply mutations during aggregation.
