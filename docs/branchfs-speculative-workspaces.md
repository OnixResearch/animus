# BranchFS-inspired speculative workspaces

BranchFS is a bounded reference for Animus speculative workspace semantics: copy-on-write branch creation, nested workspace parentage, leaf-only commit and abort behavior, commit-to-parent promotion, zero-cost discard intent, virtual branch addressing, and cache-invalidation awareness.

Animus adopts the portable behavior, not the BranchFS implementation surface. Reusable contracts must not require FUSE or macFUSE, start a daemon, expose BranchFS CLI flags, depend on a storage layout, serialize mount paths, copy ioctls or tombstone details, or move host filesystem authority into reusable crates.

`animus-adapters` owns pure DTOs and deterministic planning for workspace intent, adapter capabilities, fail-closed diagnostics, explicit adapter outcomes, and safe receipts. Host shells or optional platform adapters remain responsible for mounts, worktree creation, temporary directories, process cleanup, VCS operations, and all filesystem mutation.
