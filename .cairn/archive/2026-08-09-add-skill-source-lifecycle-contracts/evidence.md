# Evidence

## Baseline

Pueue task `14089` established the pre-change baseline:

- 6 host asset reconciliation tests passed.
- 15 skill manifest and disclosure tests passed.
- 2 safe receipt tests passed.
- `animus-adapters` passed no-default and `wasm32-unknown-unknown` checks.
- Proposal gate receipt: `815ba4fdc7fdf626b5bcb6a6956de53e05fac2e1e1dea5c5fabfd718192b0dde`.
- Design gate receipt: `f1d3927338c8fe5d6327af8689bec5c2c6bd8dc5259f56f398df6174a417d779`.
- Tasks gate receipt: `84a0caa589a51377d7f6f5ba5ed312d1d167e88fd2811d12b08e8b12633e44b1`.

## QM reference review

The requirements reference is `yc-software/qm` commit `7f2c916360f1797a8ff2a77ce2ce40c5fabab087`, dated 2026-07-31. QM is MIT licensed.

Reviewed mechanisms:

- `src/skills/skill-pack-store.ts` distinguishes `pinned` and `tracked` Git packs.
- `src/skills/pack-fetcher.ts` resolves refs, clones, checks out a detached commit, bounds files and total bytes, skips links, isolates selected Git environment variables, and supports credentials.
- `src/skills/skill-sync-engine.ts` checks moving refs. A tracked ref calls reconciliation when its head changes. A pinned ref records update availability.
- `test/skill-sync-engine.test.ts` covers unchanged and changed tracked heads plus pinned update availability.

Animus adopts only the pinned-versus-tracked lifecycle scenarios. It rejects QM's automatic tracked reconciliation or activation behavior. It does not copy the Node fetcher, Git execution, URL and credential model, timers, storage, installation, or deployment layout. QM tests were not run and are not Animus conformance evidence.

## Implemented boundary

- `animus_adapters::skill_source` is `no_std` plus `alloc`.
- Frozen descriptors require an exact opaque revision, BLAKE3 tree and manifest identities, policy, canonicalization, bounds, provenance, trust, and review references.
- Tracked observations produce only `Unchanged` or `ChangedInactive` results.
- Stale, partial, link-bearing, submodule-bearing, duplicated, colliding, conflicting, unsafe, and policy-mismatched inputs fail closed.
- Promotion binds the exact candidate and target frozen descriptor. It permits only install or update actions and reuses the existing host asset dry-run and injected mutation service.
- Legacy host asset manifests remain compatible and make no source-lifecycle claim.
- Metadata-only receipts hash public canonical fields only. Unsupported trust, authenticity, safety, authority, confinement, and release claims fail validation.
- Acquisition, raw locations, credentials, VCS, transport, archive handling, traversal, links, extraction, storage, clocks, review, rollback, mutation, and activation remain host owned.

## Policy and fixtures

- Generated policy BLAKE3: `d7e006a75c2c8399b5d931d375c793b8272bb7d1f51b81f6f85fceb0e114a372`.
- Positive fixture BLAKE3: `04e06693a9c87e2dcfa7e4d3578cb94b6f211112598d7c16103b12ba120875e7`.
- Negative fixture BLAKE3: `aa0361d246d69efae669113a8d56d78f19fca381e5e529c8868f297ea512a348`.

The typed Nickel policy closes source kinds and modes, names positive file, byte, depth, process, and stream bounds, rejects symlinks and submodules, disables ambient discovery and automatic activation, fixes transitions and diagnostics, and requires non-claims.

## Validation

The focused rail covers:

- exact frozen, unchanged tracked, changed inactive candidate, explicit promotion, legacy compatibility, and safe receipt positives;
- moving named reference, malformed identity, stale observation, duplicate asset, source collision, partial result, denied path and entry facts, symlink, submodule, automatic activation, promotion drift, missing service, receipt leak, and overclaim negatives;
- Nickel freshness and positive and negative policy fixtures;
- focused Clippy;
- default, no-default, core, runtime, Steel, TUI, Wasm, and dependency isolation;
- component catalog, runtime facade inventory, and evidence safety.

The focused receipt is `target/cairn/focused-rails/skill-source-lifecycle.md`. It is metadata only.

Final checks also passed:

- Workspace check and workspace Clippy in Pueue task `14102`.
- Focused rail after all source changes in Pueue task `14104`.
- `nix build .#checks.x86_64-linux.skill-source-lifecycle --no-link -L` in Pueue task `14105`.
- Final proposal gate receipt `7c8b7f14aded7b5f12eaaca32f08c06a15f4e9375fb04f908bb122f3ce252d55`.
- Final design gate receipt `1a73aebf4d5e4dc839fc3150f9a642ba46251f77433477d01efcc9a0e1490c7e`.
- Final tasks gate receipt `1c7ca1f01b056bf2cecf75c527c1add227b39e939b9afd7e43da8baf51deece1`.

## Accepted-spec sync

The unblocked dry-run plan was `d31b2b1e1f2b94868de9d3d489c46c6f7bd4b0e52bca1823b6c0eceb27843e7d`. Execution added all 8 `agent_harness_runtime.skill_source_lifecycle.*` requirements. The execution plan was `96ba385636a298026d3b77dc45ac99b69a4f5ee904aa0e4262cedcfc8f606339`, and its receipt was `c39e1cf1e1a6154185ae89c85d62c89be936392faf60b7c850046e8b6b54e739`.

## Non-claims

This evidence does not prove source authenticity, authorship, VCS correctness, network safety, extraction safety, review quality, script safety, host confinement, runtime authority, mutation success, durability, release eligibility, or production readiness.
