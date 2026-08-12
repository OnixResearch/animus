# Proposal: Adapt Sandlock sandbox-enforcement patterns

## Why

Animus already treats sandbox descriptors as execution constraints around authorized runtime actions. Sandlock is a useful Linux reference for the std shell because it combines Landlock filesystem and network controls, seccomp-bpf syscall filtering, seccomp user notification, COW filesystem staging, resource supervision, and HTTP/network ACLs without requiring containers, cgroups, or root.

A native Cairn change is needed so future implementation can borrow Sandlock's enforcement patterns without turning Animus into a Sandlock wrapper or making Linux-specific kernel behavior part of `animus-core`. The change defines a portable sandbox descriptor boundary, a std-shell translation layer, fail-closed enforcement behavior, and focused positive and negative verification.

## What Changes

- Treat Sandlock as a bounded reference for Linux sandbox enforcement patterns only.
- Define a portable Animus sandbox descriptor contract covering filesystem, process, network, environment, determinism, and COW execution constraints.
- Keep descriptor validation pure in reusable crates while Linux enforcement remains in `animus-std` or an optional platform adapter.
- Require fail-closed dispatch when a required descriptor cannot be represented or enforced by the selected platform shell.
- Record sandbox enforcement evidence in redacted receipts with BLAKE3 descriptor and outcome hashes.
- Add positive and negative tests for descriptor normalization, platform translation, unsupported-kernel handling, malformed descriptors, and receipt redaction.

## Impact

- **Files**: New Cairn change artifacts under `cairn/changes/adapt-sandlock-sandbox-enforcement/`; later implementation may add sandbox descriptor DTOs, Nickel policy fixtures, std-shell executor adapters, docs, receipt fields, and focused tests.
- **Testing**: Future implementation must include positive tests for normalized descriptors, Linux translation, allowed filesystem/network constraints, COW/dry-run summaries, and redacted receipts plus negative tests for malformed descriptors, unsupported platform capabilities, policy-denied constraints, missing required enforcement, and receipt leak attempts.
- **Non-claims**: This change does not vendor Sandlock, require Sandlock as a runtime dependency, expose seccomp or Landlock from `animus-core`, promise cross-platform parity for Linux-only features, or treat sandbox enforcement as authorization proof.