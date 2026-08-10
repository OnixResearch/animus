# Skill-source lifecycle

Animus defines source lifecycle as pure metadata. It does not fetch, inspect, install, or activate skills.

## Modes

A **frozen** source binds one skill asset to an exact observed revision, BLAKE3 tree identity, manifest identity, canonicalization profile, bounds profile, policy, provenance, trust, and review reference. An active lifecycle-aware manifest must match this exact binding. A branch or tag name is not a frozen revision.

A **tracked candidate** binds one fresh host observation to a tracking label and exact revision, tree, and manifest identities. An unchanged observation is a no-op. A changed observation is inactive. It cannot install, enable, select, trust, or replace a skill.

Legacy host asset manifests remain valid, but they make no source-lifecycle claim.

## Host flow

1. The host selects the source and credentials.
2. The host runs an authorized client with explicit limits and cancellation. Bounded Exec can supply process mechanics.
3. The host admits paths and entry types. Cap Root can supply lexical path admission.
4. The host computes the canonical BLAKE3 tree and manifest identities.
5. Animus validates the loaded observation and returns a no-op, inactive candidate, or denial.
6. A reviewer selects one current candidate and records exact review facts.
7. Animus checks candidate, policy, bounds, observation, trust, and target frozen identities.
8. The existing host asset dry-run and injected mutation service make the separate mutation decision.

Mantle can materialize a selected frozen source. This does not transfer acquisition, review, mutation, or runtime authority to Animus.

## Request-local selection

Source promotion does not select a skill for a model request. The [request-context contribution reducer](request-context-contributions.md) binds an already-admitted selection to the exact manifest, body, target, profile, model plan, provider request, and next terminal response. The host still owns body loading and all skill effects.

## Rotation and rollback

To update a skill, create and review a new inactive candidate. Promote it to a new frozen identity through a fresh host asset dry-run. Do not move the old frozen record in place.

To roll back, select a previously reviewed frozen identity, create a fresh dry-run against the current host observation, and use the existing injected mutation service. A stored revision or matching hash does not bypass current policy or capability checks.

## Safe evidence

Receipts can contain safe labels, opaque revisions, BLAKE3 tree and manifest identities, bounds and policy references, fixed outcomes, and fixed diagnostics. They exclude URLs, full paths, credentials, tokens, environment values, fetched bytes, file bodies, scripts, VCS arguments, stdout, stderr, and unrestricted errors.

BLAKE3 proves byte identity under the named canonicalization profile only. It does not prove source authenticity, authorship, review quality, script safety, authority, host confinement, or release eligibility.

## Consumer pinning

A downstream consumer must pin an immutable Animus revision. Workspace-relative paths are test inputs only. The consumer retains source acquisition, credential, review, storage, mutation, rollback, and runtime authority.

## QM reference boundary

QM commit `7f2c916360f1797a8ff2a77ce2ce40c5fabab087` distinguishes pinned and tracked Git skill packs. Its tracked sync engine reconciles when a branch advances. Animus uses only the pinned-versus-tracked lifecycle scenarios. Animus rejects automatic reconciliation or activation, and it does not copy QM's Node Git fetcher, credentials, storage, timers, or deployment layout. QM is MIT licensed. Its tests were not run as Animus evidence.
