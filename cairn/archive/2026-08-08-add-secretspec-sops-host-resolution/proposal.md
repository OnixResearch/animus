# Change: Add SecretSpec host credential resolution

## Why

Animus host adapters need one explicit credential boundary for local tools, CI, and long-running services. The current environment and file paths do not provide a typed provider contract, profile selection, scope checks, or consistent redaction.

SecretSpec `v0.17.0` now includes a Rust SDK and a native SOPS provider. Animus can use that SDK without moving secret resolution into `no_std` cores or exposing secret values in receipts.

## What Changes

- Pin the SecretSpec Rust SDK at `v0.17.0` and record upstream tag commit `a8794e46ec9664a0e1a3869cc3105d0853937e48` in dependency evidence.
- Add an optional `std` host adapter that loads `secretspec.toml`, selects an explicit profile and scope, and resolves provider-owned values.
- Support the SecretSpec environment, systemd credential, and SOPS provider paths required by Animus hosts.
- Run SecretSpec resolution through a bounded helper when the selected provider can launch a child process, including SOPS.
- Give pure cores typed credential requirements and already-resolved opaque values. Keep provider I/O in the imperative shell.
- Keep plaintext, value-derived hashes, provider credentials, and decrypted files out of configuration, persistence, logs, diagnostics, transcripts, and lifecycle evidence.
- Add positive and negative fixtures for profile, scope, provider, SOPS, redaction, timeout, and output-bound behavior.

## Non-Goals

- Secret resolution in `animus-core`, `animus-loop`, or another `no_std` crate.
- A new secret store or key-management system.
- SecretSpec write operations from agent workflows.
- A claim that SOPS, SecretSpec, or a configured provider establishes caller authority.
- A second direct SOPS parser beside SecretSpec.

## Dependencies

- `bounded-exec` for bounded helper execution and owned teardown.
- A reviewed and immutable SecretSpec `v0.17.0` source pin.
- SOPS only when a selected SecretSpec provider requires the `sops` executable.

## Impact

- **Affected specs:** `credentials`, `agent-harness-runtime`
- **Affected code:** host adapter, runtime shell wiring, configuration loader, provider worker, and receipts
- **Downstream:** Lattice can consume the adapter after this change is validated, archived, and available at an immutable revision
