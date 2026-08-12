# Evidence: adapt-clankers-confirmation-broker-kit

## Implementation evidence

- Added `crates/animus-adapters/src/effect/confirmation.rs` with pure typed confirmation request, binding, decision, evaluation, receipt, and leak-validation DTOs.
- Preserved the existing compatibility `ConfirmationRequest::new` entry point while adding explicit `ConfirmationRequestDraft`, `ConfirmationBindingDraft`, and `ConfirmationReceiptDraft` constructors for new protected-action boundaries.
- Extended generic service reasons with confirmation-specific no-decision, mismatch, and stale-confirmation labels.
- Added positive and negative confirmation tests for typed request identity, hash-only receipt payload behavior, approved/fresh dispatch, missing/default-deny handling, unavailable/cancelled/timed-out/malformed/no-decision outcomes, mismatched request/resource/policy bindings, stale expiry, injected broker use, and receipt leak rejection.
- Added `docs/confirmation-broker.md` and linked it from `README.md` to document host-owned interaction boundaries, safe receipts, non-claims, and focused validation commands.

## Verification evidence

- Baseline lifecycle validation before implementation: `nix run path:$PWD#cairn -- validate --root $PWD` passed with `valid: true`.
- Baseline Cairn gates before implementation: `gate proposal`, `gate design`, and `gate tasks` for `adapt-clankers-confirmation-broker-kit` passed.
- Baseline focused adapter tests before implementation: `nix develop -c cargo test -p animus-adapters effect` passed with 41 filtered effect-related tests.
- Baseline focused rail before implementation: `scripts/cairn/check-injected-services --root .` passed.
- Focused confirmation tests after implementation: `CARGO_INCREMENTAL=0 nix develop -c cargo test -p animus-adapters effect` passed with 50 filtered effect-related tests.
- Focused injected-services rail after implementation: `scripts/cairn/check-injected-services --root .` passed and included the confirmation leak, default-deny, binding, fail-closed, host-owned broker, and typed-request tests.
- Touched-file formatting check: `nix develop -c rustfmt --check --edition 2024 ...` passed for the confirmation-related files touched by this change.
- Full `cargo fmt --check` is blocked by pre-existing unrelated formatting drift in `crates/animus-adapters/src/workspace/receipt/mod.rs` and other non-confirmation files; touched confirmation files pass focused rustfmt check.
- `nix develop -c cargo clippy -p animus-adapters --lib -- -D warnings` remains blocked by pre-existing unrelated clippy diagnostics in non-confirmation modules; a grep of the captured output shows no remaining diagnostics in the touched confirmation files.
- One focused test rerun without `CARGO_INCREMENTAL=0` hit an incremental-cache filesystem race; rerunning with `CARGO_INCREMENTAL=0` passed.

## Non-claims

This evidence does not claim TUI approval prompts, browser/OAuth consent flows, remote approval services, durable approval stores, live provider behavior, daemon behavior, shell/network execution, or production readiness.
