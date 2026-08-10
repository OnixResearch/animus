# Pending-input vault

Animus preserves accepted pending work across restart without automatic submission. The pure runtime core validates one explicit conversation boundary. The optional standard host calls only one injected store and transaction service.

## Scope and ownership

| Surface | Owner | Retained authority |
| --- | --- | --- |
| Active accepted input | Active-turn recovery | Acceptance, journal, provider gate, and body retention |
| In-flight steering | Active-turn recovery | Turn binding, durable append, and delivery gate |
| Queued steering and follow-up | Coding-session host | Queue class, current admission, transcript commit, and dispatch |
| Command input | Coding-session host | Command interpretation, policy, and execution |
| Attachment references | Existing context and host owners | Body access, path policy, content validation, and retention |
| Startup projection | `animus_runtime::pending_input_vault` | Pure identity, order, corruption, disposition, and receipt decisions only |
| Store and transactions | `animus_std::pending_input_vault_host` | Explicit load, commit, cleanup, quarantine, and admitted body access |

The recovery core does not read storage, discover conversations, inspect clocks, submit input, dispatch providers, mutate queues, create transcript messages, or delete state.

## Startup sequence

1. The host selects one explicit conversation, branch, store, authority profile, policy, and generation.
2. The injected service supplies durable entry facts and prior disposition evidence.
3. The pure core validates identities, entry classes, sequence, generation, conversation scope, body identities, attachments, and finite limits.
4. Empty state returns `NoVault`.
5. Complete non-empty state returns `ReviewRequired`.
6. Corrupt or uncertain state returns `BlockedCorrupt` and exposes no partial recoverable suffix.
7. Startup does not load an entry body or produce provider, queue, transcript, tool, or messaging work.

## Explicit disposition

Restore, discard, repair, and destructive discard bind the exact projection, boundary, generation, selected entries, disposition identity, and authorization observation.

A matching completed disposition is idempotent. Changed selection, generation, conversation, store, authority, or disposition facts fail before mutation.

For restore, a successful commit returns entry references only. Each reference must pass current coding-session queue, authority, policy, durable-commit, and provider gates. The standard host loads a body only after an exact current admission observation.

Disposition evidence commits before cleanup. A restore commit must retain each selected body under its admission reference. Cleanup must not remove that retained material. Cleanup failure retains completed evidence and returns `CleanupRequired`. Restart does not repeat admission or change the disposition.

## Corruption controls

Malformed, missing, reordered, duplicated, mixed-generation, over-budget, cross-conversation, and uncertain-commit facts block ordinary prompt, continue, steer, follow-up, and automatic dispatch.

A separately authorized request can inspect bounded metadata, export quarantine data, import an exact repair, or perform destructive discard. These plans grant no provider, transcript, tool, messaging, or unrelated-conversation authority.

## Evidence

Receipts contain bounded conversation, branch, store, authority, policy, generation, entry count, class count, state, disposition, cleanup, non-claim, and BLAKE3 identity metadata.

Receipts reject input bodies, attachment content, prompts, tool data, provider data, credentials, environment values, physical paths, and raw store errors.

The [transactional daemon handoff](transactional-daemon-handoff.md) binds the current vault generation before lease release. Replacement readiness must observe that exact generation. Handoff does not load entry bodies, choose a disposition, repeat a completed disposition, or bypass current admission.

## Autolith reference boundary

The reviewed behavior reference is Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`, under the ISC license. Animus adapted no-automatic-submit, exact-order, idempotent-import, visible-corruption, and conversation-isolation scenarios.

Animus did not adopt Autolith storage formats, S-expression configuration, Common Lisp runtime, saved images, Bash supervisor, global mutable state, or security claims. There is no runtime dependency on Autolith.

## Validation

```sh
cargo test -p animus-runtime pending_input_vault --locked
cargo test -p animus-std --features pending-input-vault --lib recovery_vault --locked
cargo clippy -p animus-runtime --all-targets --locked -- -D warnings
cargo clippy -p animus-std --features pending-input-vault --all-targets --locked -- -D warnings
scripts/cairn/check-pending-input-vault --root .
scripts/cairn/check-active-turn-recovery --root .
scripts/cairn/check-coding-session-host --root .
scripts/cairn/check-durable-session-store --root .
scripts/cairn/check-evidence-safety --root .
nix run .#cairn -- validate --root .
```

## Non-claims

This boundary does not prove power-loss durability, storage correctness, operator intent, provider acceptance, queue delivery, exactly-once effects, transcript completeness, encryption, production readiness, or release eligibility.
