# Proposal: Preserve router boundary DTOs

## Why

`animus-preserves` is the optional cross-language text/binary serialization bridge that keeps boundary encoding out of core contracts. The Steel fabric (`animus-steel`) and the Syndicate dataspace adapter consume Preserves records. Router DTOs (`Recipe`, `SelectedPlan`, `RouterExecutionPlan`, `RouteResult`, `Receipt`) currently have no Preserves projection, so a Steel-scripted turn loop or a dataspace observer cannot consume routing decisions portably. Adding an optional, feature-gated Preserves projection lets embedders script and observe routing without making Preserves a router core dependency.

## What Changes

- Add an optional `preserves` feature on `animus-router` that projects `Recipe`, `SelectedPlan`, `RouterExecutionPlan`, `RouteResult`, and `Receipt` into Preserves records via `animus-preserves`.
- Keep the projection in a `preserves` module gated behind the feature so the no-std/Wasm router core stays Preserves-free by default.
- Preserve event identity, ordering metadata, status classes, redaction markers, and BLAKE3 hashes without embedding raw payloads; the Preserves record must pass the same leak validation as the Rust `Receipt`.
- Add positive projection tests for each DTO and negative tests for raw-payload leakage and projection mismatch.

## Impact

- **Files**: `crates/animus-router` `preserves` feature module, `Cargo.toml` optional dep on `animus-preserves`, root facade feature, router tests, preserves docs, facade inventory, and Cairn evidence.
- **Testing**: positive projection fixtures per DTO; negative tests for raw-payload leak and field mismatch.
- **Non-claims**: this does not make Preserves a default router dependency, add it to the initial Wasm rail, change routing logic, or persist records; it is a feature-gated boundary projection.
