# Preserves dataspace projections

`animus-preserves` exposes optional versioned Preserves records for dataspace DTOs. The bridge covers scopes, bounded assertions, safe subscriptions, and host orchestration events as stable `dataspace_dto` schema-family records.

The bridge is a serialization boundary only. Decoding always reconstructs validated `animus-adapters` dataspace DTOs before runtime, host, policy, provenance, or future live-dataspace behavior can observe the payload.

## Safe payload set

Dataspace records carry only stable IDs, normalized scope labels, status classes, assertion kinds, redaction markers, BLAKE3 safe hashes, counts, lease/expiry metadata, and ownership-transfer labels. Records reject raw prompts, provider payloads, credentials, private paths, file bodies, tool input/output, model output, and unbounded logs.

## Validation

Focused rails:

```sh
cargo test -p animus-preserves dataspace --locked
cargo test -p animus-adapters dataspace --locked
```

Use `to_dataspace_record_text`, `from_dataspace_record_text`, `to_dataspace_record_bytes`, and `from_dataspace_record_bytes` for text/binary record fixtures. Live Syndicate transport is out of scope for this bridge.
