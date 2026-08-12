# Schedule reliability evidence

## Baseline

Before reusable logic changed, these checks passed:

- `cargo test -p animus-adapters --locked timed`
- `scripts/cairn/check-messaging-receipts --root .`
- `scripts/cairn/check-neutral-ledger --root .`
- `scripts/cairn/check-safe-receipts-artifacts --root .`
- `scripts/cairn/check-wasm-surface --root .`

The proposal, design, and tasks gates passed before implementation.

## Reviewed cohort and ownership

The implementation binds Prime Agent revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4` and its MIT license. The selected mechanisms are `/heartbeat`, `rlm_heartbeat`, `prime-agent schedule`, claim-before-delivery, and missed-tick coalescing.

Animus keeps the selected behavior in `animus-runtime::schedule_reliability`. Existing `animus-adapters` scheduled recipes own workflow descriptors and run admission. Messaging owns steer and follow-up routing. The neutral ledger and host storage own durable session facts. The new core only returns decisions over supplied state and logical ticks.

## Deterministic evidence

The focused tests cover these positive paths:

- on-time claim and delivery admission after a durable claim;
- crash recovery without replay of a claimed tick;
- one catch-up decision for each surface policy;
- stable recovery identity for reordered durable jobs;
- metadata-only receipt projection.

The focused tests cover these negative paths:

- delivery without an exact durable claim;
- agent mutation of the user heartbeat;
- a second visible user heartbeat;
- cross-session durable state;
- missed counts above policy bounds;
- stale or incomplete generated policy;
- prompt bodies and exactly-once claims in receipt text.

## Post-change checks

These checks passed after implementation:

- `cargo fmt --all -- --check`
- `cargo clippy -p animus-runtime --features schedule-reliability --all-targets -- -D warnings`
- `scripts/cairn/check-schedule-reliability --root .`
- `scripts/cairn/check-nickel-policy-contracts --root .`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`
- `scripts/cairn/check-composition-conformance --root .`
- `scripts/cairn/check-safe-receipts-artifacts --root .`
- `nix run .#cairn -- validate --root .`
- `scripts/cairn/check-evidence-safety --root .`
- `nix build .#checks.x86_64-linux.schedule-reliability --no-link -L`
- `cargo check --workspace --all-targets --locked`

The Nix remote builder was unavailable. Nix continued locally and the focused derivation passed.

## Known repository blocker

`scripts/cairn/check-roadmap --root .` now classifies this change. The rail still fails for four unrelated active changes:

- `add-execution-backend-selection-planner`
- `add-optional-basalt-authority-adapter`
- `add-pijul-vcs-adapter`
- `add-sequenced-session-event-replay`

The exact issue is `ROADMAP_UNCLASSIFIED_ACTIVE_CHANGE`. This failure existed outside the schedule change and does not weaken schedule tests.

## Non-claims

This evidence does not establish exactly-once execution, real-time delivery, timer accuracy, prompt outcome correctness, production readiness, or release eligibility.
