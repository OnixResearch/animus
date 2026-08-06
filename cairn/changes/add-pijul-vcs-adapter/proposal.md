## Why

Animus already owns VCS-neutral checkpoint DTOs, a backend-neutral snapshot ledger, injected version-control provenance backends, isolated task workspaces, and safe receipts, but every concrete backend so far has been Git- or Jujutsu-shaped snapshot tooling. Pijul (https://pijul.org/) is a Rust patch-theory DVCS whose model differs in ways that matter to an agent harness: independent changes commute and apply associatively, conflicts are first-class data resolved by exactly one change, channels replace branches, and changes carry base32 BLAKE3 identities and optional key signatures.

Wiring Pijul into reusable Animus crates directly would require opening Sanakirja databases, discovering `.pijul` directories, and embedding database and filesystem authority — none of which Animus reusable surfaces may hold. It would also conflate Pijul's change-set identities with Animus' VCS-neutral records. Linking the GPL-2.0-family `libpijul` crates is not a licensing blocker: Animus accepts AGPL-compatible relicensing if a future in-process backend is chosen, so the boundary shape is decided on authority and stability grounds instead. Animus needs a pinned, non-default host adapter that treats the `pijul` CLI as an external host effect behind an injected bounded executor, with pure Rust validation and policy-controlled projection into the existing VCS-neutral owners.

## What Changes

- Add a non-default Pijul backend adapter profile pinned to an exact reviewed `pijul` CLI cohort (release line `1.0.0-beta.*`, upstream source revision, change-hash format, output-format version, and recorded GPL-2.0-family license cohort). r[agent_harness_runtime.pijul_adapter.boundary]
- Author typed Nickel policy for the exact cohort, read-only versus protected-mutation operation classes, named log/diff/record/path/channel bounds, channel allowlists, identity and signature handling, projection targets, and required non-claims. r[agent_harness_runtime.pijul_adapter.profile]
- Define an implementation-free VCS backend service port in `animus-adapters` and a thin non-default std host adapter over an explicitly injected bounded executor; reusable crates do not discover `pijul`, scan for `.pijul`, open Sanakirja databases, link `libpijul`, read identity keys, or spawn processes. r[agent_harness_runtime.pijul_adapter.service]
- Add pure Rust validation for machine outputs (`log`, `change`, `diff`, channel listing): base32 BLAKE3 change-hash syntax, channel names, state identifiers, record envelopes, conflict status, exit-status classes, and configured bounds. r[agent_harness_runtime.pijul_adapter.validation]
- Project validated observations into existing Animus checkpoint DTOs, snapshot-ledger records, provenance observations, and isolated-workspace descriptors without replacing their owners. r[agent_harness_runtime.pijul_adapter.projection]
- Emit metadata-only BLAKE3 receipts that bind cohort, policy, operation class, status, bounded counts, confirmation state, and required non-claims without diffs, file bodies, author details, key material, or private paths. r[agent_harness_runtime.pijul_adapter.evidence]
- Add positive and negative fixtures, a fake-`pijul` parity lane, dependency-isolation guards, a focused verification rail, integration docs, component/facade metadata, and a bounded README reference. r[agent_harness_runtime.pijul_adapter.verification]

## Impact

- **Reusable crates**: `animus-adapters` gains implementation-free DTOs and a backend service port; `animus-runtime` gains pure validation and projection only.
- **Host shell**: a dedicated non-default std adapter owns CLI protocol plumbing around a caller-injected bounded executor and an exact `pijul` executable identity.
- **Configuration**: Nickel remains the source of truth; generated JSON is checked and revalidated by Rust.
- **Existing owners**: checkpoint/rollback, snapshot ledger, provenance, workspace isolation, process-job profiles, and safe receipts remain authoritative; this adapter is one backend behind their existing ports.
- **Testing**: focused adapter/runtime/host tests, exact-cohort CLI parity fixtures, positive and negative fixtures, Nickel drift checks, source-shape guards, Cairn validation/gates, and a Nix check for the opt-in helper.

## Out of Scope

- Linking `libpijul`, `sanakirja`, or any Pijul crate into reusable or default Animus crates in this change, or opening `.pijul` databases from reusable code; an in-process backend remains a possible future change under AGPL-compatible relicensing.
- Network operations: `clone`, `pull`, `push`, remote management, and Nest hosting or account integration.
- Identity creation, key storage, key signing ceremonies, and signature verification beyond reporting signature status classes supplied by the CLI.
- Automatic recording, unrecording, applying, forking, resetting, or conflict resolution on behalf of an agent without explicit policy grant and confirmation.
- Treating Pijul's merge-correctness and commutation guarantees as evidence of semantic correctness, review quality, or build success of recorded content.
- Proving Pijul CLI stability across releases, change-format durability, repository integrity, authorship authenticity, or production readiness of a beta VCS.

## Affected Specs

- `agent-harness-runtime`: exact Pijul cohort, typed policy, injected service, pure validation, VCS-neutral projection, safe evidence, and verification.
