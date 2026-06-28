# Proposal: Adapt BranchFS speculative-workspace patterns

## Why

BranchFS is a FUSE-based filesystem built for AI-agent speculative branching: it provides copy-on-write branches, nested branches, atomic commit to parent, zero-cost abort, and `@branch` virtual paths on top of ordinary filesystems. Those ideas fit Animus' need for isolated agent workspaces and parallel experiments, especially alongside sandbox-enforcement work inspired by Sandlock.

Animus should adapt the workspace semantics without making FUSE, macFUSE, a daemon, or BranchFS storage layout part of reusable core contracts. A native Cairn change is needed so future implementation can define an Animus-owned speculative workspace boundary, keep pure planning separate from platform mounts and filesystem mutation, and require fail-closed cleanup and receipt evidence.

## What Changes

- Treat BranchFS as a bounded reference for speculative workspace branching, nested workspaces, atomic commit/abort, virtual branch paths, and mmap/cache invalidation semantics.
- Define a VCS-neutral speculative workspace contract for source identity, branch identity, parentage, writable roots, commit/abort policy, conflict policy, cleanup policy, and adapter capabilities.
- Keep workspace planning, descriptor normalization, and conflict policy validation pure while `animus-std` or optional platform adapters own FUSE mounting, daemon lifecycle, storage directories, process cleanup, and host mutation.
- Require fail-closed behavior when a required workspace isolation backend is unavailable, partially enforceable, fails to commit, fails to abort, or leaves ambiguous cleanup state.
- Record redacted workspace receipts with BLAKE3 descriptor, branch, commit/abort, cleanup, and manifest hashes.
- Add positive and negative verification for workspace descriptor validation, nested branch planning, commit/abort outcomes, unsupported backend handling, conflict detection, cleanup, and receipt redaction.

## Impact

- **Files**: New Cairn change artifacts under `cairn/changes/adapt-branchfs-speculative-workspaces/`; later implementation may add speculative workspace DTOs, std-shell workspace adapters, optional BranchFS backend wiring, docs, receipts, and focused tests.
- **Testing**: Future implementation must include positive tests for descriptor normalization, nested workspace planning, commit/abort projections, adapter capability translation, cleanup records, and safe receipts plus negative tests for malformed descriptors, unsupported FUSE/macFUSE environments, partial enforcement, commit conflicts, failed cleanup, and receipt leak attempts.
- **Non-claims**: This change does not vendor BranchFS, require BranchFS or FUSE for all Animus users, replace VCS adapters, expose BranchFS storage internals through `animus-core`, or promise that filesystem isolation is authorization proof.
