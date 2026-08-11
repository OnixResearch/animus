# Revision-gated workspace edit host

Animus provides concrete one-file text mutation only through the non-default `animus-std-workspace-edit` crate. The root facade is `animus::workspace_edit` behind the `workspace-edit-host` feature.

The default, no-std, Wasm, provider, TUI, and unrelated standard-library profiles do not depend on this crate.

## Authority boundary

The caller must open and authorize one `cap_std::fs::Dir`. The host accepts only a logical relative resource under that root.

`cap-root-core` checks lexical path shape. This check does not open or authorize the root. It does not prove no-follow filesystem behavior.

The caller also supplies:

- the exact root, resource, revision, policy, and writer-generation identities;
- complete tagged content through the existing hashed-edit snapshot;
- visible tagged spans and visible insertion boundaries;
- line-ending and final-newline facts;
- executor, capability, authority, approval, mutation, lifecycle, and output gates;
- current revision and writer-lock services;
- finite private-stage names; and
- owner, mode, collision, and durability policy.

The host does not discover roots, paths, locks, revisions, clocks, credentials, VCS state, or recovery services.

## Functional core

The pure core exports:

- `admit_snapshot` for exact snapshot, text-shape, and visibility admission;
- `plan_edit` for deterministic byte application;
- `validate_host_gates` for pre-I/O gate checks;
- `validate_freshness` for exact second-read comparison;
- `reduce_publication` for visibility and durability classification; and
- `project_receipt` for metadata-only evidence.

Replacement and deletion can select only fully visible tagged spans. Insertion can use only an explicit visible adjacent boundary. End insertion requires an explicit visible end boundary.

A complete content hash does not prove that the caller saw a line or boundary.

The core preserves LF, CRLF, admitted mixed endings, and the original final-newline state. Replacement text is logical LF-delimited text. An explicit policy selects the bytes for new boundaries.

## Host sequence

`WorkspaceEditHost::new(root, services, names)` binds the caller-opened root and injected services. `apply` uses the reviewed Linux or Android publication shell. `apply_with_publication_io` is the deterministic mechanism-test seam.

One operation runs in this order:

1. Check every host gate.
2. Admit the relative path and exact root/resource identity.
3. Observe the current revision, policy, and writer generation.
4. Read one bounded regular file through the capability root.
5. Bind content, file metadata, visibility, and text shape.
6. Build the exact after bytes with the pure core.
7. Acquire one resource-scoped Animus writer lease.
8. Validate the exact lease identity and generation.
9. Check cancellation and deadline state.
10. Observe and read the resource again under the lease.
11. Require exact revision, content, policy, file metadata, and generation agreement.
12. Check cancellation and deadline state again.
13. Stage and publish the exact planned bytes.
14. Classify target visibility separately from durability.
15. Release the lease with the retained or next writer generation.
16. Project a metadata-only receipt.

Every post-acquisition exit attempts lease release. A failed release blocks receipt creation.

## File and publication policy

The host denies absolute paths, parent escape, wrong root binding, symlink targets, directories, special files, owner mismatch, mode mismatch, and unsupported final modes.

Each read compares path metadata with metadata from the opened file before and after the bounded read. Unix identity includes device, inode, link count, mode, owner, group, size, modification time, and status-change time. A same-content file replacement therefore returns stale.

The publication shell uses Durable File Publication at revision `c57f01d9d80f27d4893961d75a31bf9c95d1f6c8`. It uses private exclusive stages, exact replacement, payload synchronization, and optional parent synchronization.

The host keeps these terminal classes separate:

- `failed-before-publication`: the target did not change;
- `published`: rename made the planned bytes visible;
- `durability-complete`: required parent synchronization completed; and
- `uncertain-after-publication`: visibility occurred, but later durability evidence is incomplete.

A failure after rename never creates a rollback or unchanged-target claim. A stage-cleanup failure can leave a private stage for operator review without claiming target publication.

## Reviewed source identity

The repository imports exact immutable Git objects:

- Cap Root RID `rad:z3cWBCK7VPbEmbL5MAvKJWKjCE745`, revision `8991e2572578c16e590c2f1c0915db6c46c5d9cb`, `AGPL-3.0-or-later`;
- Durable File Publication RID `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM`, revision `c57f01d9d80f27d4893961d75a31bf9c95d1f6c8`, `MIT OR Apache-2.0`.

`tools/sync-workspace-edit-mechanisms.rs` imports the selected trees. `vendor/workspace-edit-mechanisms.ncl` binds every imported file with a BLAKE3 identity.

The host crate is `AGPL-3.0-or-later` because it links Cap Root. It remains separate and non-default. The reusable Animus crates keep their existing license and authority boundaries.

Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871` supplies behavior scenarios only. Animus has no Autolith runtime dependency.

## Safe evidence

Receipts contain only bounded root, resource, revision, before, after, snapshot, plan, visibility, writer-generation, publication, durability, count, redaction, non-claim, and BLAKE3 identity metadata.

Receipts reject file bodies, replacement text, absolute paths, credentials, VCS data, process data, raw errors, universal exclusion, semantic correctness, atomic durability, authority, sandbox, and readiness claims.

Every receipt retains `not-universal-writer-exclusion`. The resource lease serializes Animus-owned writers only. An external writer can still race after the final read.

## Non-claims

This host does not prove:

- universal exclusion of external writers;
- semantic edit correctness;
- root authorization from lexical admission;
- sandboxing or general symlink safety without host policy;
- storage-device correctness;
- VCS consistency or multi-file atomicity;
- production readiness; or
- release eligibility.

## Verification

Run from the repository root:

```sh
scripts/cairn/check-revision-gated-workspace-edit-host --root .
cargo test -p animus-std-workspace-edit --locked
cargo clippy -p animus-std-workspace-edit --all-targets --locked -- -D warnings
cargo check -p animus --no-default-features --features workspace-edit-host --locked
nix build .#checks.x86_64-linux.revision-gated-workspace-edit-host --no-link -L
```

r[verify agent_harness_runtime.hashed_edit_host.verification] The focused receipt is `target/cairn/focused-rails/revision-gated-workspace-edit-host.md`. It is bounded mechanism evidence. It is not a production or release certificate.
