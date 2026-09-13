# Verification plan

## Current evidence

The current documentation defines bounded control journals, explicit Lagged outcomes, replay cursor fences, and provider-neutral stream frames. This proposal does not assert an existing production defect without a failing fixture.

## Required evidence

Capture focused baseline tests before core changes. Use production entrypoints with deterministic clocks and injected services. Required positive cases include accepted admission, queued events before wait, valid reconnect, successful flush, and acknowledged cleanup. Required negative cases include full queues, overflowing budgets, stale generations, revocation during wait, retention gaps, deadline expiry, duplicate completion, and uncertain teardown.

Retain exact call-site coverage. A passing fixture for one host does not prove every Animus host. No evidence can infer peer delivery from enqueue or local flush. Product acceptance requires strict pinned Octet, Clippy, relevant Nix checks, and current source-bound conformance.

## Planning validation

Use the repository Cairn app with its pinned matching policy. Save validation and proposal/design/tasks gate outputs under `.pi/baz-cairn-planning/`. Existing unrelated changes and policy edits remain untouched. Structural gates do not establish implementation or deployment readiness.

## Planning checkpoint

The repository Cairn app with its pinned upstream policy returned `PASS` with empty issues for proposal, design, and tasks. Repository validation returned `valid: true`. Exact outputs are `.pi/baz-cairn-planning/proposal.json`, `design.json`, `tasks.json`, and `validate.json` in that directory. The author reviewed scope overlap, replay authority, resource bounds, and negative cases. This is not an independent review. No implementation task is complete.
