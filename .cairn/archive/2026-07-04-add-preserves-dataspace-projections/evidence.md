# Evidence: add-preserves-dataspace-projections

r[verify agent_harness_runtime.preserves_dataspace.closeout]

## Implementation evidence

- Added `animus-preserves` dataspace DTO records with `DataspaceRecord`, `DataspaceMessage`, `DataspacePayloadKind`, `DataspaceScopePayload`, and text/binary record encode/decode helpers.
- Extended the Preserves schema catalog with the stable `dataspace_dto` schema family.
- Dataspace decoding reconstructs validated `animus-adapters` `Scope`, `Assertion`, `Subscription`, and `OrchestrationEvent` DTOs before callers observe the payload.
- Safe-payload validation rejects raw prompt/provider/file/tool/model/log/credential markers, cross-workspace/raw-payload subscriptions, unsupported versions, wrong schema families, wrong payload kinds, and unbounded list payloads without echoing raw material.
- Added `docs/preserves-dataspace-projections.md` and facade inventory entries for the new bridge surface.

## Checks

- Baseline: focused crate suite initially exposed a Preserves coverage gap for `RuntimeEvent::AdapterOutcomeClassified`; fixed by adding versioned Preserves encode/decode for that event.
- `nix develop -c cargo test -p animus-preserves dataspace --locked` — passed.
- `nix develop -c cargo test -p animus-adapters dataspace --locked` — covered by focused crate suite.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime -p animus-preserves -p animus-std --locked` — passed.
- `nix develop -c cargo clippy -p animus-preserves --all-targets --locked --no-deps -- -D warnings` — passed.
- Dependency isolation: `cargo tree` checks for `animus-adapters` and `animus-runtime --no-default-features` showed no `preserves` dependency leak.
- Gates: `nix run path:$PWD#cairn -- gate proposal/design/tasks add-preserves-dataspace-projections --root .` — passed.

## Blocked rails

- Combined `animus-std` clippy remains blocked by existing `crates/animus-std/src/providers/capability/flow.rs` `too_many_arguments` lint; full log captured at `target/focused-clippy-preserves-std-nodeps.log`.
