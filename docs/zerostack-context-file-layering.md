# Zerostack context file layering

Animus adapts Zerostack's context file layering as explicit, host-owned discovery feeding pure runtime admission. Runtime core never walks ancestors, reads files, consults environment variables, or canonicalizes paths; hosts provide already-loaded descriptors.

The reusable surface lives in `animus-runtime::context_layering` and is re-exported by `animus-runtime`. In the adapter index this is one `family:context` role, distinct from file, directory, diff, image, URL, and external-handle resolvers. Context-layer descriptor sources provide already-loaded descriptors; they do not imply filesystem discovery or authority for any other context role.

## Descriptors and discovery boundary

`ContextLayerDescriptor` records class, source identity, source kind, trust state, freshness, declared size, redaction state, inclusion policy, precedence, prompt section, and body text. Unknown classes and missing source identities are withheld with typed diagnostics.

`ContextLayerDiscoveryBoundary` records no-I/O proof states for ancestor walks, file reads, environment reads, and path canonicalization.

## Admission and precedence

`assemble_context_layers` applies policy for discovery enablement, trust, freshness, body size, redaction, and prompt inclusion before any descriptor can influence model-visible prompt sections. Unsafe markers, stale files, disabled discovery, untrusted sources, and oversized bodies are omitted with diagnostics.

When multiple admitted descriptors target the same prompt section, higher precedence wins deterministically and the omitted descriptor is recorded with a conflict reason when diagnostics are enabled.

## Receipts

`ContextLayerReceipt` records included and omitted descriptor IDs, classes, source kinds, safe source identities, BLAKE3 body hashes, and omission/inclusion reasons without embedding raw file bodies or private paths. The same safe-evidence posture applies to context adapter role records and std-shell resolver outcomes: durable evidence names safe labels, roles, counts, hashes, truncation or omission reasons, and redaction state rather than expanded bodies, credentials, URLs, headers, private paths, environment values, or hidden transcripts.

## Focused verification

Run:

```sh
nix develop -c cargo test -p animus-runtime context_layering
```

The tests cover known/unknown classes, missing source identity, no ambient discovery, disabled/untrusted/stale/oversized/unsafe admissions, deterministic precedence conflicts, and safe included/omitted receipts.
