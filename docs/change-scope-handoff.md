# Change-scope handoff

`animus_runtime::change_scope_handoff` validates caller-supplied scope facts and binds each review decision to one scope identity.
The optional `animus-std` feature `change-scope-handoff` presents those facts to an injected review service.
Neither component observes files, changes VCS state, or grants product authority.

## Identity encoding

The scope digest uses BLAKE3 with the domain `animus.change-scope-handoff.scope.v2`.
The encoder writes these fields in order:

1. The digest domain.
2. The handoff contract identity.
3. The driving task.
4. The file count, then the ordered file labels.
5. The obligation count, then the ordered obligation labels.
6. The hunk count, then the ordered hunk records.

Each field starts with its byte length as an unsigned 64-bit little-endian integer.
Collection counts use canonical decimal UTF-8 text.
Each hunk contains its file label, content identity, old start, old line count, new start, and new line count.
Hunk coordinates use unsigned 32-bit little-endian integers.
The encoder preserves the supplied order and duplicates.
It does not normalize paths or compare file content.

The count prefixes separate adjacent collections.
A label cannot move from files to obligations without changing the encoded scope.
Individual length prefixes alone do not provide that property.

## Compatibility

The v2 digest replaces the ambiguous v1 encoding.
This correction changes scope identities, including identities for unchanged visible scope facts.
Callers must validate the scope again and obtain a fresh review decision.
A v1 scope identity cannot answer a newly validated scope.
There is no automatic legacy fallback.

Public Rust signatures, `animus.change-scope-handoff.v1`, and the receipt schema remain unchanged.
Receipt identities also change when their bound scope or decision identity changes.
Receipt validation checks supplied metadata consistency, not historical scope facts or external execution.
The fixed non-claims remain unchanged.

## Verification

The public-API regression target covers exact replay, completion, receipt projection, collection partitions, collection changes, all hunk fields, and legacy refusal.
Each rejected decision leaves the pending scope unchanged and cannot complete its handoff.
The original imported counterexample remains in the Cairn evidence directory.

Run these commands inside `nix develop`:

```sh
cargo test --locked -p animus-runtime --all-features --test handoff
cargo test --locked -p animus-runtime --all-features --lib change_scope_handoff
cargo test --locked -p animus-std --features change-scope-handoff --lib change_scope_handoff
```

These controls establish decision binding over admitted in-memory facts.
They do not prove file isolation, honest review, change correctness, policy enforcement, VCS completion, or release readiness.
