# Proposal: Adapt Clankers focused verification rails for Animus

## Why

Clankers uses small checked-brick rails that validate one contract and emit reviewable evidence. Animus currently has broad cargo/Nix checks, but the Clankers patterns being adopted need focused rails for policy drift, lifecycle markers, receipt redaction, ledger replay, injected-service dispatch, artifact identity, and dependency isolation. Animus should add narrow rails before claiming these patterns are complete.

## What Changes

- Add a lightweight focused-rail convention for scripts or checks that validate one boundary and emit deterministic JSON or Markdown evidence.
- Add initial rails for lifecycle documentation, Steel policy drift/activation, safe receipts/artifacts, neutral ledger replay, injected-service boundaries, and dependency isolation.
- Wire stable rails into Nix/flake checks once they are deterministic.
- Document how closeout evidence names successful rails and explains skipped checks.

## Impact

- **Files**: scripts/check-* rails, target receipt paths, docs, flake check wiring when stable, Cairn artifacts.
- **Testing**: each rail has positive and negative fixtures where applicable.
- **Non-claims**: this does not introduce Clankers' full test harness, live model dogfood, TUI dogfood, VM gates, or release tag process.