# Proposal: Add Preserves dataspace projections

## Why

Animus already has typed dataspace DTOs for bounded assertions, safe subscriptions, scoped orchestration events, and fake-backend evidence. Those contracts are useful beyond Rust hosts: a UI process, headless runner, policy auditor, or future Syndicate bridge needs a stable text/binary boundary representation without depending on live dataspace transports.

This change adds Preserves coverage for dataspace DTOs while keeping Preserves as an optional serialization bridge rather than a core state model.

## What Changes

- Add versioned Preserves schema families or record variants for dataspace scopes, assertions, subscriptions, and orchestration events.
- Project only safe dataspace material: stable IDs, scope labels, status classes, assertion kinds, redaction markers, BLAKE3 hashes, counts, lease/expiry metadata, and ownership-transfer labels.
- Decode Preserves records into validated `animus-adapters` dataspace DTOs before any host, runtime, policy, or future Syndicate behavior observes them.
- Add text and binary round-trip fixtures for valid dataspace records.
- Add malformed-input fixtures for unsupported versions, wrong families, cross-scope queries, missing required fields, unsafe metadata, and raw sensitive payload attempts.
- Update Preserves schema notes and facade inventory so embedders know the dataspace coverage tier and validation rail.

## Impact

- **Files**: `crates/animus-preserves`, `crates/animus-adapters` dataspace tests if helper constructors are needed, Preserves schema docs, facade inventory, Cairn evidence.
- **Testing**: focused Preserves round trips, malformed payload rejection, dataspace DTO validation, dependency isolation, workspace checks, and Cairn gates.
- **Non-claims**: this does not add live Syndicate transport, make Preserves required by `animus-core` or `animus-adapters`, or expose raw prompts, provider payloads, file bodies, tool input/output, model output, credentials, or full private paths.
