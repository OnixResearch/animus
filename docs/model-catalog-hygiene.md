# Versioned model catalog hygiene

Animus keeps model catalog policy in the host shell. Reusable runtime code receives an already selected model service and does not read catalog files or credentials.

The design adapts bounded catalog rules from [`digitallyinduced/haskell-agent`](https://github.com/digitallyinduced/haskell-agent). Animus keeps its Rust, Nickel, authority, BLAKE3, provider, and receipt owners.

## Typed source and runtime projection

`config/model-catalog.ncl` is the typed shipped source. `config/generated/model-catalog.json` is the checked runtime projection. Runtime code never evaluates Nickel.

The schema is `animus.model-catalog.v1`. Each model has one stable ID, one connection ID, one provider ID, and one provider-native model ID. Each connection is `built_in` or `custom`.

`parse_model_catalog_json` rejects malformed JSON, unsupported versions, duplicate IDs, missing connections, unsafe labels, and literal credential values. Its diagnostic names only the safe source label, field class, and issue class.

## Deterministic merge

`merge_model_catalogs` applies replace-by-ID semantics. A user model with an existing stable ID replaces the complete shipped entry at that position. A new stable ID appends in validated user order.

User connections must be custom. A user catalog cannot redefine a shipped built-in connection. Duplicate connections within one source fail closed.

## Credentials

Catalogs contain an environment-variable name or no credential reference. They never contain resolved credential values. Literal values fail admission and do not appear in diagnostics.

The shell reads environment variables only after catalog admission. Catalog acceptance does not prove that a credential exists or is valid.

## Selection policy

Custom connections are manual-only. Exact manual selection can return a custom entry. Automatic retry, routing, or billing fallback cannot select a custom connection.

## Verification

Run:

```sh
nix develop -c scripts/cairn/check-model-catalog-hygiene --root .
```

The rail runs typed Nickel fixtures, Rust positive and negative tests, root feature compilation, marker checks, and a metadata-only receipt.

Passing evidence proves the declared catalog shape, deterministic merge, bounded diagnostics, and selection policy for tested inputs. It does not prove endpoint availability, provider identity, model identity, credential validity, billing correctness, transport safety, or provider behavior.
