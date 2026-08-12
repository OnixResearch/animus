# Proposal: Extend runtime facade inventory for composition metadata

## Why

The runtime facade inventory already distinguishes reusable contracts, host-injection surfaces, and shell adapters. To support stronger composability, it should also record how exports depend on each other, which authority they require, which feature gate exposes them, and which surfaces they are expected to compose with.

This change turns the inventory from a flat export classification into a lightweight composition map that can catch dependency and authority drift before it reaches embedders.

## What Changes

- Extend `cairn/runtime-facade-inventory.ncl` records with composition metadata such as `feature_gate`, `requires_authority`, `depends_on`, and `composes_with`.
- Update the facade inventory check to validate required fields, allowed authority values, feature references, and dependency direction.
- Add negative fixtures for reusable contracts depending on shell adapters, missing feature labels, hidden authority requirements, and invalid composition links.
- Update docs to explain how maintainers classify new exports.
- Emit safe inventory evidence with counts and BLAKE3 hashes only.

## Impact

- **Files**: inventory Nickel schema/data, check script, negative fixtures, docs, and Cairn artifacts.
- **Testing**: positive inventory validation and negative schema/authority/dependency/composition-link fixtures.
- **Non-claims**: this does not change runtime behavior, generate API docs automatically, or replace normal compile/tests.
