# Proposal: Decouple runtime loop fabric contracts from Steel

## Why

`animus-runtime` currently exposes the reusable loop fabric boundary through Steel-specific turn and decision types. That makes Steel the default fabric and also a structural dependency for embedders that want to bring their own deterministic loop fabric. Animus should keep Steel as a canonical scripting adapter while making the runtime loop contract provider-neutral and fabric-neutral.

This change introduces neutral runtime loop DTOs and moves Steel-specific adaptation behind an optional integration surface so custom fabrics can compose with runtime transitions without pulling Steel.

## What Changes

- Define runtime-owned neutral loop turn and loop decision DTOs for fabric input/output.
- Change the `LoopFabric` trait to use neutral runtime DTOs rather than Steel DTOs.
- Add a Steel adapter layer that translates between Steel fabric values and neutral runtime values.
- Gate Steel convenience runners behind an explicit Steel feature or integration crate boundary.
- Add fake non-Steel fabric tests proving `animus-runtime` can run with injected model/tool adapters and no Steel dependency.
- Preserve compatibility reexports or migration shims where needed, with deprecation notes rather than silent behavior changes.

## Impact

- **Files**: future runtime fabric DTOs, `LoopFabric` trait, Steel integration module, feature metadata, runtime shell tests, docs, facade inventory, and Cairn artifacts.
- **Testing**: positive fake-fabric loop execution and Steel-adapter execution; negative malformed decision, missing adapter, and Steel feature-disabled compile-surface checks.
- **Non-claims**: this does not remove Steel, change Steel script policy, add async execution, or introduce live provider/tool behavior into reusable runtime code.
