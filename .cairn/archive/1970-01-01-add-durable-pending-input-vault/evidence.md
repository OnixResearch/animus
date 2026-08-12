# Durable pending-input vault evidence

## Result

Animus now has one public pending-input recovery boundary for an explicit conversation. The pure runtime core validates supplied durable facts. The non-default standard host uses one injected service and does not discover a store.

Startup returns no-vault, review-required, cleanup-required, completed, or blocked-corrupt state. It does not load bodies, submit input, dispatch providers, create transcript messages, mutate queues, call tools, or send messages.

## Baseline

Before implementation, the focused coding-session host, durable session store, messaging receipt, and active-turn recovery rails passed. The prior `animus-runtime` suite contained 617 passing tests.

The ownership and restart inventory is in `docs/pending-input-vault.md`. It records active accepted input, in-flight steering, queued steering, follow-up, command, attachment-reference, transaction, cleanup, and body ownership.

## Reference boundary

The reviewed behavior reference is Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`, under the ISC license.

Animus adapted no-automatic-submit, exact-order, idempotent-import, visible-corruption, and conversation-isolation scenarios. It rejected Autolith storage formats, S-expression configuration, Common Lisp runtime, saved images, Bash supervisor, global mutable process state, and security claims. This change adds no Autolith runtime dependency.

## Implementation evidence

- `animus_runtime::pending_input_vault` is the public pure and injected-service boundary.
- `animus_std::pending_input_vault_host::Host` is the non-default standard shell.
- Startup binds one explicit conversation, branch, store, authority profile, policy, and generation.
- Mixed pending classes retain original identity, sequence, provenance, body identity, attachment references, and order.
- Restore and discard plans bind the exact projection, boundary, generation, selection, kind, and disposition identity.
- A repeated disposition resolves from retained completion evidence after restart without a second commit.
- Restore commit evidence confirms that selected body material remains available under admission references before cleanup.
- Cleanup failure retains completed evidence and permits cleanup retry without another disposition commit.
- Restored references require exact current admission before body access.
- Corruption exposes no partial recoverable suffix and blocks ordinary ingress.
- Recovery controls bind the exact blocked projection and full boundary before a service effect.
- Receipts reject bodies, attachment content, prompts, tool data, provider data, credentials, environment values, paths, raw store errors, malformed counts, and unsupported claims.

The focused receipt is `target/cairn/focused-rails/pending-input-vault.md`:

- core BLAKE3: `187c66be4198d2a57630d0cd43cfa675e7be1b1969a9aa789a711fe7c7505e0d`
- receipt core BLAKE3: `b29ce7b1067941176a454f54d9b42ee3458fd79aefb22e66f797f20987e77e77`
- host BLAKE3: `40d896a439a3c5c8b043ed9118418169209bf1313b42120eddbec85adf1d5919`
- documentation BLAKE3: `70b8c2385337b703e6af19a2f6ccdafc8b7e12b91ffe68f305def4debfec7fc3`

## Verification

The final validation set includes these passing checks:

- `cargo fmt --all -- --check`
- `cargo test -p animus-runtime --locked`: 632 passed
- `cargo test -p animus-std --features pending-input-vault --locked`: 86 unit tests and two restart integration tests passed
- `cargo check --workspace --all-targets --locked`
- `cargo clippy -p animus-runtime --all-targets --locked -- -D warnings`
- `cargo clippy -p animus-std --features pending-input-vault --all-targets --locked -- -D warnings`
- `cargo check -p animus-runtime --no-default-features --target wasm32-unknown-unknown --locked`
- `cargo check -p animus-std --no-default-features --features pending-input-vault --locked`
- `scripts/cairn/check-pending-input-vault --root .`: 15 runtime tests and three standard-host tests passed
- `scripts/cairn/check-active-turn-recovery --root .`
- `scripts/cairn/check-coding-session-host --root .`
- `scripts/cairn/check-durable-session-store --root .`
- `scripts/cairn/check-messaging-receipts --root .`
- `scripts/cairn/check-evidence-safety --root .`
- `scripts/cairn/check-request-lifecycle --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-roadmap --root . --review-markdown`
- `nix build .#checks.x86_64-linux.pending-input-vault --no-link -L`
- `nix build .#checks.x86_64-linux.component-catalog --no-link -L`
- `nix run .#cargo-tigerstyle -- check`: zero findings
- `nix run .#cairn -- validate --root .`
- proposal, design, and tasks gates: pass
- scoped traceability audit: no `agent_harness_runtime.pending_input_vault` requirement is missing

The global default traceability profile still fails because unrelated accepted requirements lack markers. This pre-existing global gap does not include the pending-input requirements.

The configured remote Nix builder was unavailable over SSH. Nix used the local builder and completed the relevant derivations.

## Lifecycle closeout

- Sync dry-run was unblocked.
- Sync execution applied all seven requirements with receipt `3717c30f5dce45e588bdb20c4ccfc588d1544f07437bed2dc92210f1ed91f371`.
- Accepted-spec after-hash: `a0356822f767bc968ad4742affd45f47d41ea5a7a954de5dd705088cc950f842`.
- Sync mutation manifest: `9fd61f94141df45a11389286ab5d089271c5f240a530b48ef25a9ab8121ceec6`.
- Archive dry-run was unblocked with receipt `b887a96c69528daf682c4433077bbc48198725eee5ab0ffafabed7481edfd1f5` and plan `10eb6db55e3d98179bf4aebf4c3c89c242532290cee39992cfe080e66ef4ea58`.
- Archive execution completed with receipt `d36bcfafb63c449bbcb7f3702dd0a7f2bc8f7117de0d8cc48cbd2356ffa85ca4`.
- Archive execution plan: `a7a9dd8871abddb13a9c59107e76439e1e3f3d2177d20bc272c227fae7ad122d`.
- Archive mutation manifest: `0eb6d596288688e30d7c55de54a982adf3842b96db31e17d4e6da3d8cacc7630`.

## Non-claims

This evidence does not prove power-loss durability, storage correctness, operator intent, provider acceptance, queue delivery, exactly-once effects, transcript completeness, encryption, production readiness, or release eligibility.
