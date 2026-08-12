# Proposal: Evaluate Clankers release evidence index for Animus

## Why

Clankers records readiness receipts and composes a current-head evidence index that separates pure gates, opt-in live/dogfood gates, dirty state, and non-claims. Animus is not ready for the full harness, but it should evaluate the evidence-index pattern for future release and archive claims once focused rails exist.

## What Changes

- Document a future Animus release-evidence index model that composes focused rail receipts, workspace checks, clippy, flake, Nickel drift, Cairn validation, and Cairn gates.
- Record current-head metadata such as commit, branch, dirty status, and receipt/head match when VCS support is available.
- Separate pure credential-free evidence from optional host-dependent checks.
- Require non-claims for live providers, TUI/daemon dogfood, VM checks, or production readiness unless explicitly covered.
- Keep the first step as an evaluation/documentation change, not a full harness.

## Impact

- **Files**: docs, possible evidence index schema, future script/check, Cairn artifacts.
- **Testing**: future fixture-based evidence index validation.
- **Non-claims**: this does not add Clankers' full test harness, dogfood tests, live model checks, VM gates, or release tagging workflow.