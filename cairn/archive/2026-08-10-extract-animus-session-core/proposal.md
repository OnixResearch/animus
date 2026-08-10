## Why

`animus-runtime` owns a coherent set of pure session-state and evidence contracts, but its crate boundary does not prevent those contracts from acquiring runtime host authority. A focused `animus-session-core` crate gives the compiler an enforceable no-std boundary without moving repository, license, policy, storage, or release authority.

## What Changes

- Add one internal `animus-session-core` crate for pure session authority, replay, projection, recovery, repair, branch, and redacted export decisions.
- Keep every existing `animus_runtime::*` module path and feature name through explicit compatibility re-exports.
- Keep serialized forms, BLAKE3 domains, receipt schemas, and public type names unchanged.
- Add positive and negative manifest checks, no-std and Wasm checks, catalog ownership, and facade compatibility evidence.

## Impact

- **Files**: workspace manifests, selected `animus-runtime` session modules, `animus-session-core`, component catalog, runtime facade inventory, Wasm rail, README, Cairn lifecycle evidence, and boundary fixtures.
- **Testing**: baseline and post-move session tests, direct-crate tests, facade tests, forbidden-dependency fixtures, no-std and Wasm checks, workspace checks, Clippy, Octet, Cairn, and Nix.
- **Non-claims**: this change does not add storage, provider, process, filesystem, network, clock, terminal, credential, or release authority. It does not prove whole-session correctness.
