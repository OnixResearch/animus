# Proposal: Harden Preserves boundary schemas

## Why

Preserves is already part of Animus' crate architecture and several planned changes mention future Preserves round trips. What is missing is a dedicated schema/versioning plan that says which boundary payloads are stable, how schema compatibility is tested, and how malformed payloads fail closed.

This change makes Preserves a first-class interoperability contract rather than an incidental serialization option.

## What Changes

- Define schema coverage tiers for adapter messages, runtime events, final reports, errors, session/provenance records, policy-visible DTOs, and dataset/export rows.
- Add versioned schema identities and compatibility rules for additive, breaking, deprecated, and unknown fields.
- Add canonical Preserves text/binary round-trip fixtures plus malformed payload fixtures.
- Require unknown or future payloads to be preserved only when safe, otherwise rejected with typed compatibility errors.
- Keep Preserves schema code out of core business decisions; core consumes typed Rust values after boundary validation.

## Impact

- **Files**: Preserves schema files or modules, fixture payloads, compatibility tests, docs, and Cairn artifacts.
- **Testing**: positive round trips and negative malformed/wrong-version/wrong-kind/missing-field fixtures.
- **Non-claims**: this does not make Preserves the only external wire format, replace Rust type validation, or permit raw private payloads in boundary fixtures.
