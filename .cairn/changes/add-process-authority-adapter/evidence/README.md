# Blocked cohort qualification

Animus baseline `eff17736a6330cfe8f5d8fec2d986b65ff885f8d`: eight adapter tests passed.
Cairn proposal, design and task receipts have empty issues. No execution mapping
or policy admission change has been implemented.

Candidate Basalt `27547d7870fdb5ca21208d583b58a518ba820b72` was fetched from the
published owner. Source NAR: `sha256-6OAy8oTb9gmZ66ZRk9BVTwFSAvKGStVfbML87QiK0qQ=`.
Its execution-policy bytes have BLAKE3
`c90838cfc1f3c8f5f926f9ee98a05d203d2c0dc2199657514537c37bff3d8c7d`.
This observation is not execution authority.

The initial resolver rejected the locked zeroize 1.8.2 against UCAN's exact 1.9.0.
Targeted `cargo update -p zeroize` resolved normally to 1.9.0 and generated a candidate
lock with the new Basalt graph. No manual lock edit, precise-version override or
cryptographic downgrade occurred. This candidate lock is not qualified for adoption.

Actual `cargo test --locked` under Animus's pinned rustc 1.95.0 rejects four new
packages requiring Rust 1.96: bounded-http, bounded-http-core, replay-ledger-core,
and revocation-view-core. A Basalt owner feature-split candidate removes its own
HTTP/durable paths, but UCAN still introduces the durable packages unconditionally.
Its newer published main has the same dependency boundary. Upstream isolation is
required before this plan can preserve the consumer toolchain.

The rejected candidate manifest/lock patch is retained as `rejected-cohort.patch`
in private evidence. Only those three agent-owned edits were restored after capture;
the worktree now keeps its accepted old Cargo/Nix cohort. No mixed candidate was
published. All tasks remain unchecked. No real process or trainer ran.

Evidence: initiating Neural checkout `.pi/drains/2026-09-07-execution-adapter/` and
`.pi/drains/2026-09-07-basalt-minimal/`.
