# Evidence

## Baseline

Before credential core changes, the focused baseline passed:

- `cargo test -p animus-std --locked providers`: 28 provider tests passed.
- `cargo test -p animus-std --locked adapters`: 11 adapter tests passed.
- `cargo test -p animus-adapter-safe --locked`: 2 receipt tests passed.
- `scripts/cairn/check-injected-services --root .`: passed.
- `scripts/cairn/check-process-job-profiles --root .`: passed.

Pueue task `14080` recorded this baseline before implementation.

## Reviewed cohorts

### SecretSpec

- Release: `v0.17.0`
- Tag commit: `a8794e46ec9664a0e1a3869cc3105d0853937e48`
- crates.io SHA-256 checksum: `68498f9695bb3662c157b8fd4b4665a594f1157de022ff5b0f891af4c7ec75d2`
- Nix source hash: `sha256-s8LXSo6veXTQsd2GlhHS/W3xhDz9v9fjE/5fgsk/edc=`
- License: `Apache-2.0`
- Enabled optional features: `sops`
- Disabled features: SecretSpec defaults, CLI, keyring, KDBX, cloud providers, Vault, OpenBao, Bitwarden, Azure Key Vault, Infisical, age, and Scaleway

Environment and systemd credential providers are unconditional library providers in this release. The SOPS provider is the only enabled optional provider.

Source review found that `execute_sops_command_with_stdin` uses `Command::new("sops")`, inherits or applies provider environment, captures complete child output, and provides no Animus deadline, output limit, cancellation, or teardown interface. Animus therefore contains the SDK in an outer worker and places a read-only bounded guard in the worker's private `PATH`. The guard admits only reviewed decrypt shapes and bounds the real SOPS child before the SDK can retain its output.

### bounded-exec

- Commit: `29dac88ecded94457572db3fdfaaaab95fa91525`
- Nix source hash: `sha256-BVmqyUYyoNpY6LfOABxwPO3DY88ZtXeKNg3TPoGCcL0=`
- License: `AGPL-3.0-or-later`

Pueue task `14084` checked out and verified the exact bounded-exec commit. Cargo and Nix both pin these exact source identities.

## Policy identities

- Generated policy BLAKE3: `638874d5602a7aab7dfcd9e5c317ff52f3d44dd7afb0d45e6a218138ae1e643f`
- Fixture manifest BLAKE3: `0a647707075f5c8db3e7c3d395ea2ad04416003ee8d988d135551fffc4ef74d4`

The typed Nickel policy fixes the resolver kind, dependency cohorts, disabled ambient discovery and environment inheritance, disabled non-authoritative audit, read-only operation class, required non-claims, one-value count, positive maximum bytes, deadlines, polling, teardown, and exact environment, systemd credential, and SOPS profiles.

## Implementation evidence

- `animus-adapters::secret_resolution` is `no_std` plus `alloc`. It contains public metadata, pure admission, stable issues, a bounded plan, and an injected service port.
- `animus-std-secretspec` owns manifest reads, policy parsing, temporary audit-disabled configuration, explicit environment construction, worker execution, and SDK access.
- `animus-secretspec-worker` resolves one exact declaration through the pinned SDK and emits one private framed value.
- `animus-secretspec-sops-guard` permits read-only decrypt arguments and bounds the explicit real SOPS executable.
- The outer worker uses an owned process group. Timeout or cancellation tears down the worker, guard, and SOPS descendants.
- Provider fallback, generation, import, migration, login, writes, cache writes, whole-profile injection, and ambient manifest, profile, scope, provider, environment, or executable discovery are not exposed.
- `ResolvedCredential` is non-cloneable, non-debuggable, non-serializable, non-hashable, and zeroizing. It exposes only borrowed delivery to an injected sink.
- Receipts bind public request metadata and record one bounded outcome, count, capability, omission class, and non-claims. They omit values, value-derived hashes, provider credentials, decrypted documents, full paths, child output, and raw diagnostics.

The SOPS success fixture verifies the reviewed SDK-to-provider invocation and both bounded process layers with an explicit provider credential. It does not prove SOPS cryptography, encrypted-file correctness, KMS correctness, or key correctness.

## Focused validation

The following checks passed after implementation:

- 3 positive and negative pure admission tests in `animus-adapters`.
- 2 private protocol tests and 9 provider and failure integration tests in `animus-std-secretspec`.
- Positive environment, systemd credential, and guarded SOPS delivery.
- Negative manifest, profile, scope, declaration, provider, fallback, write, provider-credential, executable, malformed output, nonzero exit, timeout, cancellation, overflow, unreviewed cohort, ambient policy, audit policy, excessive bound, plaintext policy, and receipt-leak cases.
- Focused Clippy with `-D warnings` for `animus-adapters` and `animus-std-secretspec`.
- `cargo check --workspace --all-targets`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- Default, no-default, root-feature, and `wasm32-unknown-unknown` isolation checks.
- Nickel export freshness and positive and negative Nickel fixtures.
- Component catalog, runtime facade inventory, injected-services, process-job, and evidence-safety rails.
- `scripts/cairn/check-secretspec-host-resolution --root .` in Pueue task `14060`.
- `git diff --check`.
- Cairn proposal gate receipt `a3027c6c03f2f8eb29cced65be11a94cfa7e0ab86906d4c944dd1e5a8f489ff3`.
- Cairn design gate receipt `d611ce163c4cfc745c1735658f1f002c0267bdba90ac42acda60decbb94577e0`.
- Cairn tasks gate receipt `289f6c7b13ed5307cb5c6fdb4381b6c66f75ccb25c3a312ec1e6f1e9f1f865ec`.

The focused rail writes `target/cairn/focused-rails/secretspec-host-resolution.md`. This ignored local receipt contains cohort metadata, coverage classes, and non-claims only.

## Nix validation

`nix build .#checks.x86_64-linux.secretspec-host-resolution --no-link -L` passed in Pueue task `14072`. The check used pinned flake sources to vendor the private bounded-exec cohort without sandbox SSH authority. It reran Nickel parity, pure and host tests, dependency isolation, Wasm checks, root-feature checks, Clippy, and the complete focused rail.

## Accepted-spec sync

The unblocked dry-run had plan `1b2707042da4d1e04501ec09e60ca6139ea356f5f6234fddc7c136143c4f21bd`. Sync execution applied all 9 `agent_harness_runtime.secret_resolution.*` requirements and all 6 `credentials.secretspec.*` requirements. The execution plan was `274f595cefc26ae813bc89e3618427adae4b6588dda677b692c356b4378617cb`, and its receipt was `efcea5ce44aa197f1aa65c8e2c49d9e73b1b2522167b2e0b0a55b0690fc2f34a`.

## Downstream publication

The validated implementation revision `3eac1c6` was published on the immutable branch `origin/agent/drain-add-secretspec-sops-host-resolution`. Lattice may consume only the final archived completion revision after it is verified on `origin/main`. A workspace-relative checkout remains test input only.

## Archive

Archive execution moved the validated change to `cairn/archive/2026-08-08-add-secretspec-sops-host-resolution/`. Its plan was `cbeb4b85bb7ff7e815075832e9c40983b37a855c8a6f709ab0e7bde38df0bbe8`, and its receipt was `966a3a239f80f6623ebfbe19d05d42ced4537217939e411afff4aa9a590f7e29`.

## Non-claims

This evidence does not establish caller authority, credential correctness or freshness, provider integrity, SOPS correctness, key-service correctness, audit completeness, arbitrary confidentiality, host-compromise resistance, downstream effect uniqueness, persistence, production readiness, or release eligibility.
