## Why

The repaired Octet Nix check reaches `animus-adapters` source linting and reports 348 findings across seven recently integrated contract families. This leaves the repository-wide Tiger Style gate red even though focused behavior tests pass.

## What Changes

- Move canonical implementations behind neutral searchable module names while retaining existing public compatibility paths.
- Remove non-trait imports and repair assertion, condition, naming, arithmetic, parameter, unwrap, and function-size findings.
- Keep existing contract behavior and authority boundaries unchanged.
- Add a focused zero-finding Octet regression rail for `animus-adapters`.

## Impact

- **Files**: affected `animus-adapters` modules, module declarations, compatibility tests, Nix checks, and lifecycle documentation
- **Testing**: pre-change focused tests, adapter tests, Clippy, no-default and Wasm checks, Octet zero-finding check, Cairn gates, and broad Nix validation
