# Proposal: Ensure Wasm compile boundary

## Why

Animus is intended to be embeddable, with pure reusable crates separated from std-backed host shells. That shape should make a small Animus surface usable from Wasm, but today the repository does not name the supported Wasm target, the crates that must compile there, or the verification rail that catches future std/process/filesystem drift.

This change makes Wasm compilation an explicit boundary contract instead of an accidental property.

## What Changes

- Define the Wasm-supported Animus surface and target profile, initially focused on `wasm32-unknown-unknown` for deterministic reusable crates.
- Keep std, process, filesystem, terminal, native persistence, provider clients, and host orchestration outside the Wasm-supported surface unless a later target-specific adapter contract accepts them.
- Add a credential-free compile rail that checks the accepted Wasm surface and can be wired into Nix once deterministic.
- Document excluded crates/features so Wasm support does not imply browser APIs, WASI execution, plugin loading, or host shell behavior.

## Impact

- **Files**: Cargo feature wiring, README/safety-contract docs, optional check script or flake check, Cairn artifacts.
- **Testing**: positive `cargo check --target wasm32-unknown-unknown` coverage for the supported surface; negative/dependency drift coverage proving std shell crates are excluded from the Wasm rail.
- **Non-claims**: this does not require `animus-std`, `animus-steel`, `animus-tui`, native persistence, provider clients, plugin execution, browser bindings, or WASI host behavior to compile for Wasm in this change.
