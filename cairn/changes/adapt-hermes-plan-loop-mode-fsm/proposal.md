# Proposal: Adapt Hermes plan/loop mode FSM contracts

## Why

Hermes' mode extension contains useful patterns: explicit planning markers, selected-step execution, loop breakout signaling, and transient/rate-limit continuation handling. Animus should adapt those as mode-state and continuation contracts without baking in Hermes shortcuts, UI, commands, subagent names, or tool restrictions.

As a framework/SDK, Animus should let embedders define their own modes and host shells while reusable crates validate state transitions, selected work, and safe receipts.

## What Changes

- Define typed plan/loop mode descriptors, mode state, plan extraction results, selected-step sets, loop conditions, breakout acknowledgements, retry classifications, and mode receipts.
- Parse explicit plan markers and selected-step updates as pure logic over supplied text, not as hidden prompt state.
- Integrate loop and validation follow-up work with continuation policy without automatic model turns or provider probing.
- Keep shortcuts, commands, UI pickers, tool activation, read-only enforcement, delegation, timers, provider probing, and persistence host-owned.

## Impact

- **Files**: future prompt/continuation DTOs, pure reducers, tests, docs, and focused fixtures.
- **Testing**: positive marker extraction and FSM transitions plus negative malformed marker, stale step, unsafe loop prompt, provider-probe, hidden-tool-change, and receipt leak fixtures.
- **Non-claims**: this does not ship `/mode`, `/plan`, `/loop`, keyboard shortcuts, a built-in plan executor, automatic loop turns, or read-only mode enforcement by prompt alone.
