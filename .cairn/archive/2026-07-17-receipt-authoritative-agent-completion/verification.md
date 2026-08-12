# Verification and dependency resolution

## Pre-change baseline

The following focused checks passed before core changes:

- `cargo test -p animus-adapters --locked validationplanner --no-fail-fast`: five planner/result tests.
- `cargo test -p animus-runtime --locked continuation --no-fail-fast`: matching continuation paths.
- `cargo test -p animus-steel --locked --no-fail-fast`: 37 tests, including the old substring-completion behavior replaced by this change.
- `scripts/cairn/check-steel-runtime-policy --root .`.
- Cairn proposal, design, and tasks gates.

## Resolved dependency

The former blocker is resolved. Cairn archived `make-agent-completion-receipts-authoritative` and Animus now pins revision `cf5c4a5c10361ca20a13addf14954407d6857028`, schema `cairn.agent-completion.v2`, and policy hash `b8f81a80716eb7427dacdcd4dac9c01c1599d2bee79cdeb0a6e444525377da59`.

The pinned Animus wrapper executed the real positive and stale-subject fixtures. The positive report was authoritative and valid with completion receipt `ea388ca71bc87663d36127a0dbcc2f1d5bcb60fea62921094f23bdc72946109b`; it truthfully recorded `checks_executed=false` because the fixture supplies saved host results. The negative report failed with `verification_contract.subject_stale`. Production hosts must supply current real execution evidence and may not replay fixture subjects.

## Adversarial review status

VibeThinker consultations failed to fetch or timed out and are not counted as evidence. Earlier malformed suggestions to treat `skipped` as no-follow-up, use YAML in place of Nickel, or accept ambiguous receipt shapes were rejected because they conflict with the pass-only contract. Repository tests, the real pinned Cairn CLI, deterministic fixtures, focused rails, Clippy, Tiger Style, and Cairn gates remain the validation authority.

## Non-claims

This change establishes bounded selection, pass-only evidence admission, completion-candidate handling, current-subject binding, injected host boundaries, and Cairn receipt parity for the selected contract. It does not prove check-oracle soundness, honest execution beyond supplied host evidence, model correctness, whole-product correctness, dependency satisfaction, production readiness, performance, deployment safety, or release eligibility outside the selected contract. It grants no release, push, pull-request, signing, shell, filesystem, network, clock, provider, or credential authority.
