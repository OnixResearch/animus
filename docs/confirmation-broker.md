# Confirmation broker contract

Animus confirmation is a host-owned safety boundary. Reusable adapter/runtime code emits typed protected-action requests and evaluates typed broker outcomes; hosts decide how to ask humans, policy engines, notification channels, or durable stores.

## Reusable contract

Confirmation requests carry:

- request identity;
- protected action kind;
- resource label or identity;
- ability label;
- safe prompt summary for host-local display;
- BLAKE3 policy hash;
- session or run identity;
- required confirmation level;
- optional host-provided expiry tick;
- redaction marker.

Approvals must echo the request identity, action kind, resource, ability, policy hash, and expiry. Mismatched or stale approvals fail closed. Missing, unavailable, timed-out, cancelled, malformed, denied, and no-decision broker outcomes also fail closed before protected action dispatch.

## Receipt safety

Confirmation receipts keep request, resource, ability, session, policy, and receipt identities as safe BLAKE3 hashes plus bounded status labels. Receipts do not include raw prompts, credentials, full paths, provider payloads, raw tool payloads, or secret summaries.

## Host-owned behavior

The reusable crates do not display prompts, read terminal input, send notifications, call external approval services, or persist approval records. Hosts inject a `ConfirmationBroker` implementation when a product surface needs those behaviors.

## Focused validation

Run the confirmation-focused adapter tests:

```sh
CARGO_INCREMENTAL=0 nix develop -c cargo test -p animus-adapters effect
```

Run touched-file formatting checks:

```sh
nix develop -c rustfmt --check --edition 2024 \
  crates/animus-adapters/src/effect/confirmation.rs \
  crates/animus-adapters/src/effect.rs \
  crates/animus-adapters/src/effect/input.rs \
  crates/animus-adapters/src/effect/result.rs \
  crates/animus-adapters/src/effect/tests.rs \
  crates/animus-adapters/src/lib.rs
```

Full workspace formatting and clippy can be blocked by unrelated pre-existing drift; record those diagnostics separately from confirmation-broker evidence.
