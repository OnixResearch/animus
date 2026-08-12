# Proposal: Sequence the agent-harness Cairn roadmap

## Why

Animus has many active Cairn changes that describe useful runtime, policy, provenance, UI, dataset, and reference-adaptation work, but the changes are currently flat. Without an explicit sequence, foundational contracts can land after dependents, optional reference adapters can appear before safety boundaries, and duplicate ledger/receipt/provider work can drift.

This change adds a roadmap package that makes the implementation order reviewable without merging unrelated product decisions into one oversized change.

## What Changes

- Define roadmap tiers for baseline lifecycle/spec wiring, core runtime loop, injected services, receipts/provenance/session state, policy/guidance, concrete host adapters, UI/headless surfaces, dataset/export surfaces, and optional reference adapters.
- Record dependency rules for which active changes should land before others.
- Require each planned change to identify whether it is foundational, core-runtime, host-adapter, optional-reference, product-surface, or evaluation-only work.
- Add closeout criteria that prevent archiving dependent changes until their prerequisite contracts are accepted or explicitly waived.
- Document consolidation rules for overlapping ledger, receipt, hook, provider, and manifest changes.

## Impact

- **Files**: roadmap documentation, possible dependency matrix fixtures, Cairn artifacts.
- **Testing**: focused lifecycle validation, roadmap fixture checks, and Cairn proposal/design/tasks gates.
- **Non-claims**: this does not implement runtime behavior, reorder Git history, archive existing changes, or decide every optional adapter's final product status.
