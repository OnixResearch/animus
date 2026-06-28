# Proposal: Evaluate Clankers tool catalog and capability packs for Animus

## Why

Clankers has a useful tool-catalog pattern: descriptors declare capability classes, side-effect levels, approval requirements, runtime neutrality, and redaction. Animus should evaluate this as a later adoption once its adapter/tool contracts and injected-service boundaries stabilize. The goal is not to import Clankers tools, but to define an Animus-owned catalog manifest for embedders.

## What Changes

- Design an optional Animus tool-catalog manifest with typed descriptors, capability classes, side-effect levels, approval policy, input/output schema references, and redaction policy.
- Keep catalog construction runtime-neutral: reading descriptors must not execute product tools, launch plugins, start providers, read secrets, or open network connections.
- Add collision and unsafe-capability validation.
- Add positive and negative manifest fixtures.
- Decide whether the catalog belongs in Nickel, Rust DTOs, Preserves, or a combination.

## Impact

- **Files**: possible `policy/tool-catalog/`, docs, manifest fixtures, validation script, Cairn artifacts.
- **Testing**: manifest validation positive/negative fixtures and runtime-neutrality rail.
- **Non-claims**: this does not add built-in product tools, plugin execution, shell/network tools, or provider integration.