# Evidence

## Baseline

Before reusable logic changed, the focused baseline passed:

- `cargo test -p animus-adapters --locked recover::tests`: 9 positive and negative checkpoint/rollback cases passed.
- `cargo test -p animus-adapters --locked vcssnapshot`: 5 snapshot-ledger cases passed.
- `cargo test -p animus-adapters --locked workspace`: 31 matched workspace and connected recovery cases passed.
- `cargo test -p animus-runtime --locked provenance`: 29 provenance cases passed.
- `cargo test -p animus-adapter-safe --locked`: 2 receipt cases passed.
- `scripts/cairn/check-process-job-profiles --root .`: passed.

The baseline was recorded by Pueue task `14037` before implementation.

## Reviewed cohort

- CLI release: `1.0.0-beta.18`
- Nest source change: `QRYOAJLYA5KO7JAYU6LMYHPU3NSZCTUMYEPCK3HNZIRIDZDWSMBAC`
- Nest source state: `RISR562S7FZP3NNYDFVBQUMDEGB6YSSMSH2RGZDRYBIPGVQDSLXQC`
- crates.io SHA-256 checksum: `0500d14ab41adea4f0d71f03b9d1be895a0638e025204f5b6ab90e46c40684ff`
- Nix source hash: `sha256-vU41JiuxB6Bsi88st/tkt02054oN3HEN52pnLu5hMA4=`
- Cargo vendor hash: `sha256-Ach8wLBhZ3pA5+m910Gt+oftEaO3Mu/ii+bxgnla0ak=`
- Cohort BLAKE3 identity: `ed1fcd07852103dd8ea0b2e824aac6cad51406c987e54240077888ce796aee2b`
- Generated policy BLAKE3 identity: `ecb5dce6dc396514908584363b7613a52caa68553de8d7573240e8c2d3727273`
- License: `GPL-2.0-or-later`

The crates.io archive has no `.cargo_vcs_info.json`. The archive checksum and Nix source hash bind the release archive. The recorded Nest change and state are reviewed source context. This evidence does not prove that the archive bytes equal the Nest state.

Source review found that beta.18 rich log output can read repository and user identity files. The admitted shell therefore uses `log --hash-only --output-format json`. It rejects rich log objects. It reports state, author, timestamp, message, description, path, and signature facts as omitted or not reported.

## Existing owner map

- Checkpoint and rollback: `crates/animus-adapters/src/recover/`
- Snapshot ledger: `crates/animus-adapters/src/vcssnapshot.rs`
- Provenance facts and injected VCS backend: `crates/animus-runtime/src/provenance/` and `crates/animus-runtime/src/provenancegitai/backendpolicy.rs`
- Workspace descriptors and isolation: `crates/animus-adapters/src/workspace/`
- Process-job profiles: `policy/process-jobs/` and `scripts/cairn/check-process-job-profiles`
- Component catalog: `cairn/component-catalog.ncl`
- Runtime facade inventory: `cairn/runtime-facade-inventory.ncl`
- Request lifecycle: `docs/request-lifecycle.md`
- Safe receipts: `crates/animus-adapter-safe/src/lib.rs`

The adapter projects through these owners. It does not create a Pijul-specific ledger or persistence owner.

## Implementation evidence

- `animus-adapters::pijul` provides no-std DTOs, the bounded executor port, and the backend service port.
- `animus-runtime::pijul` validates the exact generated profile, request, output, hash, prefix, channel, status, exit, bounds, and metadata-only receipt.
- `animus-std::pijul` plans exact argv and parses only reviewed hash-log JSON, diff JSON, and channel lines through an injected executor.
- The root `pijul-vcs-adapter` feature is non-default.
- The Nix helper clears ambient environment inheritance, denies remote and mutation argument shapes, and invokes the exact pinned executable by store path.
- Protected mutations and unsupported surfaces return before executor dispatch.
- Validated log facts project into existing checkpoint, snapshot, provenance, and workspace records.

## Focused validation

The following checks passed after implementation:

- 15 pure runtime positive and negative Pijul tests.
- 11 injected std-shell positive and negative Pijul tests.
- Focused Clippy with `-D warnings` for touched adapter, runtime, and std surfaces.
- No-default adapter and root checks.
- `wasm32-unknown-unknown` adapter check.
- Checkpoint, snapshot, provenance, workspace, process-job, and safe-receipt regressions.
- Nickel export and positive/negative fixture checks.
- Component catalog and runtime facade inventory rails.
- Evidence-safety rail.
- `scripts/cairn/check-pijul-vcs-adapter --root .`.
- Cairn proposal, design, and tasks gates.
- Cairn repository validation.
- Exact Nix helper build and `pijul 1.0.0-beta.18` version check.
- Nix helper negative checks for remote and channel-mutation argument shapes.
- `nix build .#checks.x86_64-linux.pijul-vcs-adapter --no-link -L`.
- Cairn sync dry-run with every delta outcome `already_applied` and all Pijul requirement IDs present in the accepted spec.
- Cairn archive dry-run passed unblocked with plan `5e3bbec2e9c99019d2bf9116f5b46ba9a5b78639499d0a9e96d75b5a3c9f6ab9`.
- Archive execution passed with receipt `9c5c467f531714eb15040eea00dcf17e2353a379eda6a8fa1f3499d02b6538f8`.
- Post-archive Cairn validation, evidence safety, focused rail, and `git diff --check` passed.

The focused rail wrote `target/cairn/focused-rails/pijul-vcs-adapter.md`. This receipt is ignored local evidence. It contains cohort metadata, coverage classes, and non-claims only.

## Non-claims

This evidence does not establish repository integrity, semantic correctness, content correctness, merge safety, authorship authenticity, signature verification, key custody, remote correctness, conflict-resolution quality, persistence, durability, production readiness, or release eligibility.
