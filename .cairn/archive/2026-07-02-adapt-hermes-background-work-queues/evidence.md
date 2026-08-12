# Evidence: Hermes background work queue adaptation

## Baseline and gates

- Baseline broad test before queue DTO edits: `nix develop -c cargo test --workspace --locked` passed during the drain session before this change's core edits.
- PASS: `nix run path:<repo>#cairn -- validate --root <repo>` before implementation and after task completion.
- PASS: `nix run path:<repo>#cairn -- gate proposal adapt-hermes-background-work-queues --root <repo>`.
- PASS: `nix run path:<repo>#cairn -- gate design adapt-hermes-background-work-queues --root <repo>`.
- PASS: `nix run path:<repo>#cairn -- gate tasks adapt-hermes-background-work-queues --root <repo>` after all task markers were checked.

## Implementation evidence

- Added pure no-std queue adapter DTOs and `BackgroundQueueService` in `crates/animus-adapters/src/queue.rs`.
- Added positive tests for enqueue/status through an injected service, explicit all-groups destructive controls, group/parallelism status, bounded log viewports, and send-input with a safe hash.
- Added negative tests for ambiguous destructive controls, zero parallelism, oversized visible logs, partial logs without truncation markers, unsupported send-input, and send-input without an input hash.
- Added `docs/background-work-queues.md` and linked it from `README.md`.

## Checks

- PASS: `nix develop -c cargo fmt --check`.
- PASS: `nix develop -c cargo test -p animus-adapters --locked queue` with 13 focused queue/timed tests.
- PASS: `nix develop -c cargo test -p animus-adapters --locked` with 359 adapter tests.
- PASS: `nix develop -c cargo test --workspace --locked`.
- BLOCKED: `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings` remains blocked by existing adapter lint debt beginning with duplicated attributes in `crates/animus-adapters/src/front/guidance/assembly.rs`; the captured log contains no `queue.rs` diagnostics after moving test-only constants into the test module. Scratch log: `.pi/cairn-drain/background-queue-clippy.log`.

## Non-claims

This change does not make pueue mandatory, does not spawn processes from reusable runtime crates, does not add a concrete queue backend, and does not expose raw full logs or raw send-input bodies.
