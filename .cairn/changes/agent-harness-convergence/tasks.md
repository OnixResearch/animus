## Phase 1: Typed results and renderings

- [ ] [serial] r[animus.convergence.typed_results] Make tool results typed values with declared projections.
- [ ] [parallel] r[animus.convergence.typed_results.projections] Add a fixture that requires all renderings to derive from one datum.
- [ ] [parallel] r[animus.convergence.typed_results.string] Add a fixture that must fail when a rendered string becomes the canonical datum.

## Phase 2: Size and handles

- [ ] [serial] r[animus.convergence.size] Implement harness-owned folding, paging, and truthful omission with stable handles.
- [ ] [parallel] r[animus.convergence.size.fold] Add a fixture that must fail on silent truncation.
- [ ] [serial] r[animus.convergence.handles] Enforce short, stable, store-typed handles.
- [ ] [parallel] r[animus.convergence.handles.resolve] Add a fixture that must fail closed on an unknown handle.

## Phase 3: History and silence

- [ ] [serial] r[animus.convergence.history] Record calls, results, and spans as structured append-only data.
- [ ] [parallel] r[animus.convergence.history.render] Add a fixture that must fail when a render re-runs a tool.
- [ ] [serial] r[animus.convergence.silence] Implement detach-with-handle bounds and the suspension dump.
- [ ] [parallel] r[animus.convergence.silence.detach] Add a fixture that must fail on a silent wait past the bound.
- [ ] [parallel] r[animus.convergence.silence.dump] Add a fixture that must name each suspended item.

## Phase 4: Supervised cycles and telemetry

- [ ] [serial] r[animus.convergence.cycle] Implement supervised cycles with a prepare/commit split and a park-versus-finish distinction.
- [ ] [parallel] r[animus.convergence.cycle.split] Add a fixture that requires commit after the guarded work completes.
- [ ] [parallel] r[animus.convergence.cycle.park] Add a fixture that must fail when a parked cycle reads as finished.
- [ ] [parallel] r[animus.convergence.cycle.guard] Add a fixture that must fail when side effects run inside the synchronization boundary.
- [ ] [serial] r[animus.convergence.telemetry] Ride bounded telemetry with each result.
- [ ] [parallel] r[animus.convergence.telemetry.visible] Add a fixture that names the telemetry bound.

## Phase 5: Fixtures and gates

- [ ] [serial] r[animus.convergence.verification] Add the focused positive, negative, and boundary matrix and run the rails.
