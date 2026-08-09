# Design: SecretSpec host credential resolution

## Context

SecretSpec separates declarations from provider-owned values. Profiles and scopes select which declarations an application can resolve. Its audit records omit values and redact provider credentials.

The assessed upstream release is `v0.17.0`, tag commit `a8794e46ec9664a0e1a3869cc3105d0853937e48`. Its native SOPS provider invokes the `sops` executable with `std::process::Command`. The upstream provider does not expose Animus process deadlines or output limits. Animus must therefore place any such resolution inside its own bounded process boundary.

## Goals

- Use the SecretSpec Rust SDK as the single host-side declaration and provider interface.
- Keep validation deterministic and provider access explicit.
- Bound provider processes, output, and teardown.
- Prevent secret material from entering durable evidence.
- Give downstream adapters one narrow credential API.

## Architecture

```text
host configuration
  -> pure request validation
  -> imperative SecretSpec resolver shell
  -> bounded resolver worker when provider execution is possible
  -> SecretSpec SDK and selected provider
  -> opaque credential material
  -> existing typed adapter input
```

### Functional core

The core accepts declaration metadata, an explicit profile, an explicit scope, a requested credential name, and caller policy. It returns either a validated resolution request or a typed rejection.

The core does not read files, environment variables, clocks, process state, or provider values. It rejects unknown declarations, undeclared scopes, implicit profiles, write requests, and disallowed provider schemes.

### Imperative shell

The shell loads the manifest, constructs the pinned SecretSpec SDK resolver, and obtains values from the selected provider. Provider paths that can launch commands run in a dedicated internal worker through `bounded-exec`.

The worker receives only the minimum manifest coordinates and provider credentials. It returns a bounded value through a private pipe. The parent closes the pipe after one read and clears temporary buffers where the Rust type permits it.

## Provider policy

- Environment providers are for explicit development and CI profiles.
- Systemd credential providers are the preferred service path.
- SOPS providers are allowed only by an explicit profile and scope.
- Provider fallback is disabled unless the manifest declares an ordered policy that the caller also allows.
- SecretSpec write operations are not exposed through the Animus adapter.
- SOPS stderr is mapped to a typed, redacted error. It is not copied verbatim into receipts.

## Data model

`CredentialRequirement` contains public metadata: logical name, required scope, allowed provider schemes, and purpose.

`ResolvedCredential` contains secret material plus public origin metadata. The origin records the manifest identity, profile name, scope name, provider scheme, and declaration name. It does not record a provider URI when that URI can contain credentials.

Secret values use secrecy-aware and zeroizing containers where supported. Pure adapter logic receives an opaque borrowed value and does not own provider access.

## Evidence and diagnostics

Receipts record the request identity, selected public profile and scope, provider scheme, policy outcome, timing class, and bounded failure category. They do not record values, value-derived hashes, provider credentials, decrypted file content, child stdout, or child stderr.

The manifest and dependency identities use BLAKE3 because they are public stack-owned artifacts. No digest is computed from secret material.

## Failure behavior

- Missing manifest, profile, scope, declaration, or provider fails closed.
- Missing SOPS executable fails with a stable installation category.
- Timeout, cancellation, oversized output, malformed output, and nonzero exit each have separate typed outcomes.
- A worker failure cannot fall back to an ambient environment value.
- Cancellation tears down the owned worker process tree.

## Compatibility and rollout

The existing explicit environment and file adapters remain available during migration. Callers opt into SecretSpec by configuration. After downstream migration, legacy paths can be deprecated in a separate change.

Lattice remains blocked on an archived Animus change and an immutable revision. A workspace-relative checkout is test input only, not a durable dependency.

## Validation

- Unit tests cover pure request validation and redaction.
- Integration fixtures cover environment, systemd credential, and SOPS reads.
- Negative fixtures cover missing scopes, disallowed providers, malformed encrypted files, missing executables, timeouts, oversized output, cancellation, and attempted writes.
- Repository checks include format, focused tests, clippy, Nickel configuration checks, and the relevant Nix flake checks.
