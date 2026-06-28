# Proposal: Adapt Clankers tool gateway delivery and outbox for Animus

## Why

Clankers' Tool Gateway shows a useful pattern for artifact delivery: validate the target and toolset policy first, call a selected delivery adapter only after approval, persist a bounded outbox attempt, and retry by attempt ID without exposing raw destinations or credentials. Animus should adopt that adapter boundary for future file/media/scheduled-output delivery while keeping platform-specific integrations outside reusable runtime crates.

## What Changes

- Define artifact delivery DTOs for artifact handle, artifact kind, target kind, active session context, policy decision, delivery attempt ID, retryability, and adapter outcome.
- Add Tool Gateway policy validation for local/session delivery, disabled toolsets, capability ceilings, unsupported platform targets, raw destination material, and missing active context.
- Execute approved delivery through injected delivery adapters and record safe attempt/outbox receipts.
- Add retry/status logic that references persisted attempt IDs and revalidates current policy before adapter execution.
- Add focused positive and negative fixtures for local delivery, unsupported target rejection, missing adapter, raw Matrix/webhook/cloud destination rejection, retryable/non-retryable failures, and receipt leak attempts.

## Impact

- **Files**: future gateway DTOs, delivery adapter traits, outbox receipt types, policy fixtures, tests, docs, and Cairn artifacts.
- **Testing**: positive local/session delivery fixtures and negative unsupported-target/missing-adapter/raw-destination/non-retryable/leak fixtures.
- **Non-claims**: this does not implement Matrix, webhook, cloud storage, or arbitrary platform delivery by default; those remain host adapter opt-ins.
