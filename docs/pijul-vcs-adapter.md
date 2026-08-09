# Pijul VCS adapter

## Status

The Pijul adapter is optional and is disabled by default. It is an injected CLI boundary. It does not link `libpijul` or `sanakirja`.

The reviewed cohort is:

- Pijul CLI release: `1.0.0-beta.18`
- Nest repository: `https://nest.pijul.com/pijul/pijul`
- Release change: `QRYOAJLYA5KO7JAYU6LMYHPU3NSZCTUMYEPCK3HNZIRIDZDWSMBAC`
- Release state: `RISR562S7FZP3NNYDFVBQUMDEGB6YSSMSH2RGZDRYBIPGVQDSLXQC`
- crates.io SHA-256 checksum: `0500d14ab41adea4f0d71f03b9d1be895a0638e025204f5b6ab90e46c40684ff`
- Nix source hash: `sha256-vU41JiuxB6Bsi88st/tkt02054oN3HEN52pnLu5hMA4=`
- Cohort BLAKE3 identity: `ed1fcd07852103dd8ea0b2e824aac6cad51406c987e54240077888ce796aee2b`
- License: `GPL-2.0-or-later`

The published crates.io archive does not contain `.cargo_vcs_info.json`. Therefore, the archive cannot prove a Git or Pijul source revision by itself. The adapter binds the archive checksum and Nix source hash. It also records the observed Nest release change and state. These values are source-cohort evidence, not a proof that the archive bytes equal the Nest state.

Adoption requires an explicit GPL license review. This adapter does not grant a license compatibility exception.

## Authority boundary

`animus-adapters` owns implementation-free DTOs and service ports. `animus-runtime` owns pure validation, projection, and receipt logic. `animus-std` owns the optional CLI shell.

The reusable boundary cannot:

- discover repositories or executables;
- search `PATH`;
- open `.pijul` or Sanakirja databases;
- spawn a process;
- read the environment;
- read identity or key files;
- access a network or remote;
- persist a result;
- apply, record, unrecord, reset, fork, or mutate a channel.

The host must inject a bounded executor. Each request contains an absolute executable path, exact executable identity, absolute repository path, argv, working directory, empty environment, empty standard input, byte limits, and a deadline. Ambient environment inheritance is false.

The Nix package `.#pijul-cli-adapter` supplies an optional helper named `animus-pijul-cli`. The helper admits only `--version` and the exact reviewed log, diff, and channel query argument shapes. It denies remote and mutation argument shapes before Pijul starts. It uses the exact Nix Pijul package through `env -i` and sets only inert `HOME`, `XDG_CONFIG_HOME`, and `RUST_BACKTRACE` values. The Rust adapter does not invoke this helper by itself.

## Admitted queries

The initial CLI service admits three query forms:

1. `pijul log --hash-only --output-format json --limit <bound>`
2. `pijul diff --json`
3. `pijul channel`

Each invocation also receives an explicit `--repository` value. Log and diff can receive an allowlisted `--channel` value.

The beta.18 rich log path reads repository identity files and can load user identities. The initial adapter does not admit that path. It uses hash-only JSON. Therefore, the CLI shell reports no state, author, timestamp, message, description, path, or signature facts from log output.

The contracts still distinguish full change hashes, state identities, descriptive author metadata, descriptive timestamps, and signature status classes. The reviewed CLI shell accepts only full change hashes. It rejects rich log objects because they exceed its identity-file authority.

`diff --json` has a reviewed bounded parser. The parser accepts the beta.18 path-to-status map. It rejects unknown status fields and unknown operation classes. Pure validation checks relative paths, counts records, and then removes paths from the validated summary.

`channel` is human-oriented output. The adapter does not claim that it is a stable upstream machine schema. The exact beta.18 parser accepts only `* <name>` and `  <name>` lines. The cohort identity binds this parser. Any format change fails closed.

## Hashes and prefixes

A canonical Pijul change hash and channel state identity has 53 uppercase base32 characters. The accepted alphabet is `A-Z` and `2-7`.

A full hash and a prefix are different input classes. A prefix must meet the configured minimum length. A loaded result must contain exactly one matching full hash. Zero matches and multiple matches fail with `ambiguous_prefix`. No prefix is stored as a canonical change identity.

## Protected and unsupported operations

The profile classifies these operations as protected mutations:

- `record`
- `unrecord`
- `apply`
- `fork`
- `reset`
- channel creation, deletion, rename, or switch

The initial service does not dispatch these mutations, even after confirmation. It returns a typed denial before it calls the executor.

The `change` command is unsupported because beta.18 emits human-oriented change text. Clone, pull, push, remote management, Nest operations, identity creation, identity proving, signing, key custody, and automatic conflict resolution are also unsupported.

## Existing owner projections

A validated log observation can project into existing Animus owners:

- `CheckpointRecord` with backend kind `other`;
- `SnapshotRecord` with backend kind `other`;
- `ProvenanceFact::Opaque` with sanitized counts and classes;
- `SpeculativeWorkspaceDescriptor` with the neutral `vcs_worktree` capability.

The adapter does not create a second checkpoint, snapshot, provenance, workspace, or receipt ledger. Projection is pure. The host remains responsible for any durable commit.

## Evidence

The receipt contains only:

- cohort, policy, executor, projection, and receipt identities;
- operation and status classes;
- bounded record and conflict counts;
- confirmation and redaction classes;
- existing owner labels.

It excludes repository paths, file paths, raw diffs, file bodies, change messages, descriptions, author details, timestamps, identity documents, credentials, keys, signatures, prompts, process output, and unrestricted diagnostics.

The receipt does not establish repository integrity, semantic correctness, content correctness, merge safety, authorship authenticity, signature verification, key custody, network correctness, conflict-resolution quality, durability, production readiness, or release eligibility.

## Verification

Run:

```sh
scripts/cairn/check-pijul-vcs-adapter --root .
```

The rail checks the exact cohort, Nickel parity, positive and negative fixtures, pure validation, CLI parsing, owner projections, metadata-only receipts, protected-operation denials, no-default builds, Wasm compatibility, dependency isolation, and connected owner regressions.
