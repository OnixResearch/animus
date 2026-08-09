# SecretSpec host credential resolution

This optional host adapter resolves one declared credential through SecretSpec. It keeps provider access out of reusable `no_std` crates.

## Reviewed dependency cohort

| Dependency | Reviewed identity | License | Enabled features |
|---|---|---|---|
| SecretSpec | `v0.17.0`, commit `a8794e46ec9664a0e1a3869cc3105d0853937e48`, crates.io checksum `68498f9695bb3662c157b8fd4b4665a594f1157de022ff5b0f891af4c7ec75d2` | `Apache-2.0` | `sops` only |
| bounded-exec | commit `29dac88ecded94457572db3fdfaaaab95fa91525` | `AGPL-3.0-or-later` | default library surface |

Environment and systemd credential providers are unconditional SecretSpec library providers. Animus disables SecretSpec default features. It enables only the extra `sops` feature.

The upstream SOPS provider starts `sops` with `std::process::Command`. It has no Animus deadline or output limit. Animus therefore uses two owned boundaries:

1. `animus-secretspec-worker` bounds the complete SecretSpec resolution.
2. `animus-secretspec-sops-guard`, installed as the worker's private `sops`, admits only the reviewed decrypt argument shapes.
3. The guard runs the explicit real SOPS executable through `bounded-exec`.

The outer worker uses an owned process group. Timeout or cancellation tears down the worker, guard, and SOPS process tree. The guard bounds SOPS stdout before SecretSpec can retain it.

## Functional boundary

`animus-adapters::secret_resolution` owns the pure contract. It accepts only public metadata:

- manifest BLAKE3 identity;
- explicit profile and scope;
- exact declaration;
- provider family;
- purpose and destination;
- policy BLAKE3 identity;
- positive worker limits.

The pure contract rejects an implicit profile, an undeclared scope, an unknown declaration, provider fallback, a disallowed provider, and every write operation. It does not read files, variables, clocks, or processes.

`animus-std-secretspec` owns the imperative shell. It reads the explicit manifest, verifies its identity, creates an audit-disabled private SecretSpec config, clears ambient environment variables, and starts the internal worker.

## Provider profiles

The checked policy is `config/secret-resolution-policy.ncl`. Its generated runtime input is `config/generated/secret-resolution-policy.json`.

- `environment-host` accepts one required environment value for development or CI.
- `systemd-host` accepts one absolute `CREDENTIALS_DIRECTORY`. This is the preferred service path.
- `sops-host` accepts one explicit real SOPS executable and a private guard directory.

A direct provider override collapses SecretSpec provider chains. Animus does not admit cache providers or fallback providers. The adapter does not expose SecretSpec create, update, generate, import, cache-write, or delete operations.

## Deployment procedure

1. Declare the credential in one SecretSpec profile.
2. Put that declaration in one explicit scope.
3. Select one admitted provider URI. Do not put provider credentials in the URI.
4. Compute the BLAKE3 identity of the exact manifest bytes.
5. Update the matching profile in `config/secret-resolution-policy.ncl`.
6. Run `scripts/config/export-nickel-configs`.
7. Install `animus-secretspec-worker` as an absolute executable.
8. For SOPS, install `animus-secretspec-sops-guard` as `sops` in a private directory. Supply the real SOPS executable as an absolute path.
9. Run `scripts/cairn/check-secretspec-host-resolution --root .`.

Do not use ambient `SECRETSPEC_PROFILE`, `SECRETSPEC_SCOPE`, `SECRETSPEC_PROVIDER`, `PATH`, or user SecretSpec configuration. The worker clears the environment and supplies exact inputs.

## Rotation procedure

1. Rotate the value in the selected provider.
2. Do not change the declaration, profile, or scope unless policy must change.
3. Restart the consuming one-shot host flow.
4. Run the focused rail with a redacted fixture.
5. Confirm only metadata appears in receipts and diagnostics.

Animus does not hash the old or new value. A provider success does not prove freshness or correctness.

## Rollback procedure

1. Stop new credential deliveries.
2. Restore the last reviewed manifest and generated policy as one unit.
3. Restore the prior provider value through provider-owned tooling.
4. Restart the consuming host.
5. Run the focused rail again.

Do not place decrypted files, old values, provider credentials, or raw provider diagnostics in rollback evidence.

## Material and evidence rules

`ResolvedCredential` is non-cloneable, non-debuggable, non-serializable, and non-hashable. Its bytes are held in a zeroizing container. A caller can borrow the bytes only during one delivery call. The representative provider sink does not change pure provider-planning logic.

Receipts contain request identity, profile, scope, declaration, provider family, bounded outcome, omission markers, and non-claims. They omit:

- secret values and value-derived hashes;
- decrypted files;
- provider credentials;
- provider URIs that can contain credentials;
- full host paths;
- child stdout and stderr;
- raw diagnostics;
- transcript and snapshot content.

SecretSpec and SOPS resolve supplied values. They do not establish caller authority, credential correctness, freshness, provider integrity, runtime safety, production readiness, or release eligibility. Existing authority gates remain mandatory before provider dispatch.

## Downstream publication

Lattice can consume this adapter only after this Cairn change is archived and the Animus commit is published as an immutable revision. A sibling checkout is test input. It is not a durable dependency.
