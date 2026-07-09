# Proposal: Adapt Hermes ROI recommendation contracts

## Why

Hermes' ROI command is a useful workflow: inspect current evidence, choose the highest-value next action, and offer begin or lifecycle-first routing. Animus should adapt the structured recommendation and action-routing contract without shipping a default `/roi` command or making automated prioritization part of the core agent loop.

As an SDK, Animus can validate ROI reports, safe scoring inputs, action descriptors, and receipts so host products can build prioritization tools on top.

## What Changes

- Define ROI request, score input, recommendation report, runner-up, missing-context, action descriptor, lifecycle-routing hint, and receipt DTOs.
- Parse/validate structured ROI reports with required fields and bounded text.
- Require action descriptors to pass host admission before starting implementation, Cairn planning, investigation, or any mutation.
- Keep repository inspection, scoring heuristics, model calls, UI action selection, Cairn change creation, and task execution host-owned.

## Impact

- **Files**: future workflow/report DTOs, parsers, tests, docs, and focused fixtures.
- **Testing**: positive report/action parsing plus negative malformed score, unsafe evidence, unknown action, auto-implementation, auto-Cairn, and receipt leak fixtures.
- **Non-claims**: this does not add a built-in ROI command, default prioritizer, automatic Cairn creation, automatic implementation, repository scan, or model-based scoring service.
