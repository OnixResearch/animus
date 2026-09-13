# Qualify the live host crates under Octet

## Why

The broad Octet check omits `animus-std-openai` and `animus-live-steel-agent`. A focused check fails on the unchanged provider crate and prevents a complete downstream host result. This gap blocks the separate live-child control APIs.

The operator approved this repair on 2026-09-12. The work starts from fetched `origin/main` at `51d2683776ab73d4fd36d3d9e6ced2beadf9793d` in a separate worktree.

## What Changes

- Add independent and combined strict checks for both host crates. Retain the full pinned catalog, all targets, and all compatible features. r[agent_harness_runtime.live_host_quality.strict]
- Split mixed or oversized modules into coherent components. Keep public concept names and deterministic domain behavior. r[agent_harness_runtime.live_host_quality.structure]
- Replace recursion, unchecked arithmetic, and implicit invariants with bounded, checked control flow. Make public numeric bounds portable and record affected callers. r[agent_harness_runtime.live_host_quality.behavior]
- Retain positive and negative controls and exact local evidence. r[agent_harness_runtime.live_host_quality.evidence]
- Add explicit dependency construction and coherent runtime owner paths. Admit CLI arguments at the existing binary boundary. r[agent_harness_runtime.live_host_quality.structure]
- Refuse incomplete recovery, retain rejected commit observations, and prevent unproved continuation retries after effects. r[agent_harness_runtime.live_host_quality.recovery]

## Impact

- **Files**: both host crates, directly affected callers, `flake.nix`, related documentation, and this Cairn package. The approved expansion includes `animus-runtime` module owners, `animus-std` memory persistence construction, and their focused controls. Existing rails and inventories can receive direct source-reference migrations.
- **Authority**: no new ambient authority, live-provider campaign, worker scheduler, or consumer activation.
- **Compatibility**: current endpoint admission, parser results, error classes, persistence order, and constructor intent remain intact. Public integer-width changes require explicit caller migration and cannot count as source compatibility.
- **Gates**: unchanged baseline, focused positive and negative tests, feature checks, strict Clippy, independent and combined Octet checks, applicable host rails, and Cairn validation.

## Non-goals

The operator approved the owner, CLI, and recovery expansion after the retained `44a221ff` result. This change does not implement the pending companion APIs, complete the pi-steel child adapter, weaken the catalog, add lint exemptions, or repair unrelated workspace components. It does not change dependency pins or publish unverified work.
