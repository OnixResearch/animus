# Ornith scaffold profiles

Animus adapts Ornith-style scaffolds as route-neutral reviewed strategy data, not executable harness code. A scaffold profile may describe bounded strategy preferences such as retry posture, validation checklist use, tool-ordering preference, compaction preference, memory inclusion, and task-category hints. It must not construct providers, execute commands, mutate verifiers, or bypass the Rust/Steel adapter boundary.

The reusable runtime surface is pure and lives in `animus-runtime::scaffold`, re-exported by `animus-runtime`. Hosts own profile discovery, storage, prompt assembly, Steel input construction, admission UI, and any future evaluation or training loops.

## Runtime contract

A scaffold profile records:

- stable profile and revision identities;
- task-class labels used for deterministic selection;
- route capability constraints expressed as reusable labels, not provider-specific clients;
- reviewed strategy knobs;
- provenance and review status;
- BLAKE3 content hashes for reviewed descriptors;
- an enabled/disabled policy state.

Selection happens before task execution over in-memory profile descriptors, task class, host policy, and route labels. The selected profile may influence prompt or Steel inputs only after validation, and every resulting model, tool, memory, summary, or response action still crosses the existing Rust boundary checks.

## Admission boundary

Runtime or evaluation flows may propose scaffold revisions, but proposals are inert until an injected host admission outcome admits them. Denied, unavailable, or stale-base outcomes are record-only: they cannot persist, become active, or influence future prompt/Steel inputs.

## Receipt safety

Scaffold influence receipts carry safe metadata only: profile ID, revision ID, task class, review status, route labels, validation outcome, and BLAKE3 hashes. Receipts reject raw prompts, scaffold bodies, provider payloads, credentials, full paths, raw tool bodies, and unbounded outputs.

## Focused verification

Run the focused tests with:

```sh
nix develop -c cargo test -p animus-runtime scaffold
```

The tests cover positive profile validation, malformed IDs, duplicate revisions, unsupported knobs, missing provenance, unreviewed active revisions, mismatched hashes, deterministic selection, disabled profiles, route mismatch, no-compatible-profile failure, denied/unavailable/stale admissions, receipt leak rejection, and executable-harness rejection.
