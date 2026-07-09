# Design: Adapt Hermes VCS snapshot ledger contracts

## Context

Animus already has VCS-neutral checkpoint rollback DTOs, injected checkpoint services, provenance branch queries, and safe receipts. Hermes adds two useful recovery concepts: per-turn checkpoints and shadow histories. This change generalizes those ideas into optional snapshot ledger contracts.

## Decisions

### 1. Snapshot records are backend-neutral

**Choice:** Snapshot records contain backend kind, opaque snapshot ID, parent snapshot ID, workspace label, content/artifact identities, changed-file counts, actor/session labels, freshness metadata, and safe receipt hashes.

**Rationale:** The same contract can represent jj, Git, BranchFS, in-memory, or custom host backends.

### 2. Protected operations are plans first

**Choice:** Accept, drop, squash, restore, cleanup, and compare operations are action plans that require checkpoint service support, freshness validation, and confirmation when protected.

**Rationale:** Recovery operations can destroy work; the SDK should not execute them.

### 3. Automatic snapshots are not default

**Choice:** The contract can describe host policies for when snapshots are requested, but no runtime crate schedules, creates, or persists snapshots automatically.

**Rationale:** Snapshot frequency and storage are host product decisions.

### 4. Receipts reference hashes and labels only

**Choice:** Snapshot receipts omit raw diffs, file bodies, branch names when unsafe, full paths, command lines, and VCS-specific private refs unless hosts provide safe labels.

**Rationale:** Recovery evidence is durable and must avoid leaking workspace details.

## Risks / Trade-offs

- VCS-neutral plans may not expose every backend capability. Hosts can attach bounded backend capability labels.
- Confirmation requirements can make recovery slower. Protected mutation safety is more important.
- Snapshot identity collisions must be avoided through domain-separated BLAKE3 receipt/content hashes plus host opaque IDs.
