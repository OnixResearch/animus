# Proposal: Adapt Clankers confirmation broker kit for Animus

## Why

Animus already plans injected services and default-deny confirmation behavior, but protected runtime actions need a reusable confirmation broker contract that hosts can implement consistently. Clankers' embedding boundary treats confirmations as host-owned typed requests with explicit deny/unavailable/timeout/cancel outcomes. Animus should make that a first-class brick before tool mutation, rollback, delivery, remote admission, and workflow surfaces depend on approvals.

## What Changes

- Define typed confirmation request, decision, challenge, timeout, cancellation, and receipt DTOs for protected actions.
- Require the default broker to deny, and require absent, unavailable, timed-out, cancelled, stale, or mismatched confirmations to fail closed.
- Bind confirmations to action identity, resource/ability, policy hash, session/run identity, and optional expiry so approvals cannot be replayed onto different actions.
- Keep UI prompts, human approval collection, notification channels, and persistence in host adapters.
- Add positive and negative fixtures for approved actions, denied actions, missing brokers, stale confirmations, mismatched action IDs, timeout/cancel outcomes, and receipt leak attempts.

## Impact

- **Files**: future confirmation DTOs, broker traits, validation core, receipt fixtures, docs, tests, and Cairn artifacts.
- **Testing**: positive approval fixtures and negative default-deny/missing/stale/mismatch/timeout/cancel/leak fixtures.
- **Non-claims**: this does not implement a TUI approval prompt, browser/OAuth consent flow, remote approval service, or durable approval store by default.
