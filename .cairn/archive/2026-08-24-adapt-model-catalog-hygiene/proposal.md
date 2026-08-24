## Why

Animus has a shell-owned provider registry, generated model profiles, an optional Models.dev projection, and hardened provider resolution. A durable user catalog still needs exact merge, secret, error, and fallback rules.

`digitallyinduced/haskell-agent` ships a versioned model catalog and merges an optional user catalog by stable model ID. It stores environment-variable names instead of secret values, reports malformed fields at startup, and excludes custom connections from automatic billing fallback. This change adapts those rules through typed Nickel contracts and checked runtime DTOs.

## What Changes

- Add a versioned typed catalog contract with shipped defaults and deterministic user replace-by-ID merge.
- Reserve built-in connection names and reject ambiguous duplicates.
- Store credential references as environment-variable names, never secret values.
- Report malformed catalogs at startup with the catalog and invalid field named.
- Keep custom connections manual-only and exclude them from automatic fallback.

## Impact

- **Files**: Nickel catalog contracts and projections, provider-registry DTOs and resolution, runtime parsing, fixtures, documentation, and this lifecycle package.
- **Testing**: focused Nickel-config, provider-registry, model-runtime-profile, and provider-retry rails, workspace tests, Clippy, Octet, formatting, Cairn validation and gates, and `nix flake check`.
- **Dependencies**: none new. Source evidence: `digitallyinduced/haskell-agent` at `master`.

## Non-goals

- Network discovery of models or credentials.
- Storing secret values in the catalog.
- Automatic fallback to operator-added endpoints.
- Claims that a catalog entry proves endpoint availability or model identity.
