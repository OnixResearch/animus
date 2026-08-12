# Evidence

## Baseline

The change worktree started from clean `origin/main` commit `0dcc9fcde080fb8c6f41e16367eb5fd90b183668`.

Passing baseline checks covered the existing hashed-edit contract, conflicting-target mutation windows, capability-root context, safe receipts, and injected host services.

The existing `animus_adapters::tool::hashed` boundary validated hashes, tagged spans, overlap, replacement bounds, and metadata-only receipts. It had no capability-relative read, caller-visibility, resource lock, second read, publication, or durability behavior.

## Ownership inventory

- `animus-adapters` owns the existing no-std hashed-edit contract, dispatch gates, mutation windows, authority, approval, and safe effect receipts.
- `animus-std` owns existing capability-root wrappers and read-only context behavior. Those paths do not grant workspace mutation authority.
- The new `animus-std-workspace-edit` crate owns only the non-default concrete one-file edit host. Its pure functions own snapshot extension, visibility, byte application, freshness, publication reduction, and receipt projection. Its thin shell owns capability-relative reads and reviewed one-file publication calls.
- The caller retains root opening, authority, revision observations, metadata policy, writer-lock service, cancellation, deadlines, stage names, retries, recovery, semantic correctness, and release decisions.

## Reviewed mechanism cohorts

- Cap Root RID `rad:z3cWBCK7VPbEmbL5MAvKJWKjCE745`, revision `8991e2572578c16e590c2f1c0915db6c46c5d9cb`, license `AGPL-3.0-or-later`. Selected source BLAKE3: `60fbdb7eae53dff8c2c0d41bc15aa123cd3a362ec1611eb4bc892fd6be58f3cb`.
- Durable File Publication RID `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM`, revision `c57f01d9d80f27d4893961d75a31bf9c95d1f6c8`, license `MIT OR Apache-2.0`.
- `tools/sync-workspace-edit-mechanisms.rs` reproduces both selected vendor trees. `vendor/workspace-edit-mechanisms.ncl` binds their immutable Git revisions and selected source hashes.
- Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` is an ISC-licensed behavioral reference only. Animus has no runtime dependency on it.

Cap Root proves lexical admission only. The caller-opened `cap_std::fs::Dir` remains the authority boundary. Durable File Publication classifies one-file publication and durability observations. It does not prove payload meaning, multi-file transactions, rollback, filesystem correctness, or release eligibility.

## Implemented boundaries

- Exact snapshot binding includes root, resource, revision, complete content, line metadata, metadata policy, file metadata, visible spans, visible insertion boundaries, final-newline state, and writer generation.
- Gates run before filesystem access.
- The host acquires a resource-scoped writer lease, checks its exact generation, then re-observes and re-reads the target before staging.
- Only visible spans and explicit visible insertion boundaries can mutate.
- Byte application preserves CRLF, LF, mixed-ending policy, and final-newline state.
- Publication reduction distinguishes pre-publication failure, published visibility, durability completion, and uncertain post-publication outcomes.
- Receipts are metadata-only and retain `not-universal-writer-exclusion`.
- The optional root feature is `workspace-edit-host`. The crate is absent from the root default and no-default dependency graphs.

## Current focused results

- `cargo test -p animus-std-workspace-edit --locked`: 22 passing positive and negative tests.
- `nix develop -c cargo clippy -p animus-std-workspace-edit --all-targets -- -D warnings`: pass.
- `nix develop -c cargo octet check -p animus-std-workspace-edit`: clean; the newly included crate moved from 236 findings to 0.
- Root no-default build: pass.
- Root `workspace-edit-host` feature build: pass.
- Default dependency isolation: pass.
- Nickel provenance typecheck: pass.
- Mechanism import self-test: pass.
- Exact immutable import reproduction against both pinned sibling Git objects: pass.
- Component catalog, runtime facade inventory, adapter conformance index, and roadmap rails: pass.
- Pure model and planning source-tree BLAKE3: `bc26bf3b20a16c7339128afa6affd5565aab7d84d6eb9793287265028ca33c90`.
- Concrete host source-tree BLAKE3: `5248e0eb88e33d402ee217854c380546a2372fdb94dd7368f16c4b160beb6209`.
- Receipt projection source-tree BLAKE3: `06c6038ea110a8bc66c57ce3f36e73bac726b31939e62432285e314407d79e22`.
- Pure-core test-tree BLAKE3: `f805bdc44ecf4d241f36a500f43189f1f1acf37cf18687b4c0d8882c10361d6d`.
- Concrete-host test-tree BLAKE3: `ca7670659435e6fddc778c813b479a8f79f663c81d06dfe8ffa8bf6584762bc6`.
- Mechanism sync-script BLAKE3: `a8c3c5baa8621ffd122d37c5a1dedbfe855351d01acbd375711a15fb9f890158`.
- Focused-rail script BLAKE3: `938e50c6feb6ef5f31d7c4476b6c727c9b4a668ff2b979be2635b6a8b8d3783f`.
- Focused receipt BLAKE3: `d2c79982bba5e2ed296420b7018e55ed50e2550aba35cad9c910ee54a4587a98`.
- Focused provenance BLAKE3: `074a3f51ed6c506b29cf3c63a12e67081a65a9c6b76ded255ed4d56b27a0737b`.
- Workspace tests and strict workspace Clippy: pass.
- Repository-wide Tiger Style check: pass.
- `nix flake check -L`: pass on the fully staged implementation before requirement promotion.

## Lifecycle closeout

- Task gate receipt: `090dfa6c21743163c8fb081bd57dcfdfe1c353f33d290d5fb240e85e86cbe5fc` with all 26 tasks complete.
- Reviewed sync dry-run receipt: `c0225fa93c53a58d21b3f410becac5d23f4a24ce64954b069bf2f253a80af72c`.
- Sync execution receipt: `25e4b459553ec71fdfe1736ae0d89cb0c73c41f0dc49dbbfd3451f41e92b59ba`.
- Sync mutation-manifest BLAKE3: `0ab928964772e226f2833b157410489d2d2ad30e4c4bcf4b983e9328bd4773d2`.
- Accepted `agent-harness-runtime` spec BLAKE3: `2b01ca2c2a631aff488650965578477a408a9a37307a6769ee3a2d499eb76dd9`.
- Post-sync Cairn validation: pass.
- Scoped traceability for `agent_harness_runtime.hashed_edit_host.*`: zero missing and zero dangling requirements.
- Full traceability report BLAKE3: `1221f0bcb9ed0bc6c567388ac230e29c04bae96eb45a34cf39bb44aa5132974c`.
- The repository-wide traceability command still fails on unrelated accepted-requirement gaps. This change does not claim a global traceability pass.

## Adversarial review result

The focused rail covers owner and mode denial, cleanup failure, same-content metadata drift, stale writer generation, malformed visibility, stage-name collision, and unsupported parent-sync capability. It also checks invalid leases, second-observation failure, second-read failure, cancellation, timeout, invalid publication input, generation overflow, release failure, release drift, and every reviewed publication phase.

Every post-acquisition error fixture observes a release attempt. Receipt creation stops when release fails or returns the wrong generation. Post-rename parent-sync failure stays `uncertain-after-publication`; it never claims rollback or an unchanged target.

## Non-claims

This evidence does not claim universal exclusion of external writers, semantic correctness, root authorization from lexical checks, sandboxing, storage-device correctness, production readiness, or release eligibility.
