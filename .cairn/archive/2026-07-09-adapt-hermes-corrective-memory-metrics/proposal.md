# Proposal: Adapt Hermes corrective memory and review-metrics contracts

## Why

Hermes combines a repo-local napkin, structured review metrics, and repeated-finding promotion to turn mistakes into durable prevention. Animus should adapt the useful loop without making `.agent/napkin.md`, always-read memory, or automatic task/spec mutation part of the framework.

Animus needs optional corrective-memory and review-metric contracts that embedders can store wherever they choose, admit explicitly, and promote only through host-approved actions.

## What Changes

- Define safe corrective record and metric record descriptors for mistakes, preferences, working patterns, failing patterns, domain notes, and tagged review findings.
- Add admission rules so corrective records influence prompts only when trusted, scoped, fresh, and explicitly selected by host policy.
- Add repeated-finding promotion candidate grouping that recommends deterministic checks, policy guidance, spec rules, or human checkpoints without mutating files automatically.
- Add explicit append/promote request DTOs and safe receipts for hosts that choose to persist records.

## Impact

- **Files**: future quality/memory adapter DTOs, pure grouping/admission logic, tests, docs, and focused fixtures.
- **Testing**: positive corrective record, metrics summary, and promotion grouping tests plus negative untrusted, oversized, stale, unsafe, automatic-write, and leak fixtures.
- **Non-claims**: this does not add mandatory napkin loading, require `.agent/napkin.md`, create global memory, persist records by default, edit Cairn tasks, or make review metrics a core scoring authority.
