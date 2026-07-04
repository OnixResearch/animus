# Proposal: Add adapter conformance rail and contracts index

## Why

Animus has a broad and growing adapter surface: model, tool, streaming, capability, workspace, queue, status, policy, receipts, resources, prompts, and host-control contracts. Tests exist in many modules, but embedders need a concise conformance story: which failures every adapter family must handle, which docs define the boundary, and which focused prelude exposes the smallest useful surface.

This change adds a repo-owned conformance rail and contracts index for adapter families.

## What Changes

- Add `docs/contracts-index.md` listing maintained contract families, owner modules, stability/version posture, authority boundary, receipt behavior, and focused validation rails.
- Add focused adapter preludes or module-level export guidance for model, tool, host/status, workspace, security/policy, and receipt surfaces while retaining the flat compatibility facade.
- Add an adapter conformance matrix covering happy path plus missing, denied, unavailable, timeout, cancellation, malformed, oversized, unsafe-reference, schema-mismatch, duplicate, unknown-field, and leak cases.
- Add a focused check or test inventory that fails when new public adapter families lack positive and negative conformance coverage.

## Impact

- **Files**: docs, adapter preludes/exports if needed, conformance matrix tests or scripts, facade inventory, README link, and Cairn evidence.
- **Testing**: positive and negative conformance inventory for maintained adapter families plus docs/index checks.
- **Non-claims**: this does not remove flat exports, force a public API break, or add concrete host adapters.
