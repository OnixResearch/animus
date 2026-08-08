# Session event replay evidence

## Baseline

Before reusable logic changed, these checks passed:

- `scripts/cairn/check-acp-nominal-identities --root .`
- `scripts/cairn/check-conversation-projection --root .`
- `scripts/cairn/check-coding-session-host --root .`
- `cargo test -p animus-runtime --locked session`
- `cargo test -p animus-acp-stdio-host --locked events`

The proposal, design, and tasks gates passed before implementation.

## Reviewed references

- `withastro/flue` revision `bf86b8726f5ba189844185fdbeca0e194344ded1`, Apache-2.0. The selected Sandbox Agent concept is durable reconnect from monotonic session events.
- `rivet-dev/agentos` revision `5b76f89ffd70817de303cf4654b1badd79c0442d`, Apache-2.0. The selected concept is the separation of completed semantic events from transient streaming deltas.

Animus does not adopt either storage model, API schema, remote server, transport, automatic reconnection, or sandbox behavior.

## Evidence boundary

The pure planner uses caller-loaded facts from the existing admitted ledger and runtime order. It returns initial, top-up, no-op, rebuild, gap, duplicate, stale-generation, branch-drift, truncation, and terminal outcomes.

Positive tests cover initial replay, exact top-up, no-op, terminal replay, live-delta omission, and equal ACP, headless, and TUI ordering. Negative tests cover duplicates, gaps, stale generations, branch drift, bounds, detached facts, unsafe identities, and receipt leaks.

## Validation

The following checks passed after implementation:

- `scripts/cairn/check-session-event-replay --root .`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`
- `nix run .#check-nickel-configs`
- focused runtime tests, Clippy, no-default-feature, Wasm, and root-facade checks from the replay rail
- composition conformance, safe-receipt, ACP nominal identity, ACP event, conversation projection, and coding-session-host rails
- `nix build .#checks.x86_64-linux.session-event-replay --no-link -L`
- `nix run .#cairn -- validate --root .`

`check-roadmap` reports only three unrelated pre-existing unclassified active changes: `add-execution-backend-selection-planner`, `add-optional-basalt-authority-adapter`, and `add-pijul-vcs-adapter`. This change is classified in the `session-ledger` tier.

The first sync preflight found the legacy `## Requirements` delta heading and blocked without mutation. The heading was corrected to `## ADDED Requirements`. The second dry run passed with receipt `2c7660d6b3d19912c10d494297dba778635aeccd9be969997ca97743bf96316d`. Executed sync receipt `4660bc07f344f9d00fd7c3f40612760001a0dc53a5b3aa3313e6c6c009d442bb` added all seven `agent_harness_runtime.session_event_replay.*` requirement IDs to the accepted specification.

Archive dry-run receipt: `48ee8387d7e9ddf3ce9cc05a9ed6a435293ebe08e1738cf3f52820c7523e98b2`.

Executed archive receipt: `9b22fd3ace9bfd11cb36802cb66da40b410040c4559355c596cf21404be28565`.

This evidence does not prove transcript truth, transport delivery, durable storage, or conversation continuity.
