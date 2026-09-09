# Scoped holder observation qualification

## Qualified source

Source: `dee4e85a17ad2faa3ad88983c7ad0317d5d8761a`.
Base: `c62a1ae8817f9b43395e490fb7f9b50c5d23d964`.
Published Basalt: `b10a0465bc3ee966a4864d6e779184f4eae0cd61`.
Its vendored UCAN: `3a2aa043068a82663cf72ec05ed5d56a7d65ecd5`.
Basalt NAR: `sha256-wb+8lqq6GedHWqS3/5EPkQV5u/vjrut334m0pX284D0=`.

The existing `ScopedAuthority::holder()` accessor supplies the original borrowed admission to the published UCAN recheck API.
No additional runtime wrapper, verifier, replay adapter, or dispatch permission was added.
Generic no-std authority contracts remain unchanged.

## Observed verification

Checks ran on 2026-09-08. Complete logs remain in the research workspace's private
`neural-stream/.pi/drains/materialization-complete/` evidence directory.

| Evidence | Result |
| --- | --- |
| `animus-recheck-baseline-v1.log` | Before changes: 18 tests and five compile-fail controls passed |
| `animus-recheck-adoption-v1.log` | Generated cohort update: existing 18 tests and five compile-fail controls passed |
| `animus-recheck-focused-v1.log` | Expanded 18 tests, six compile-fail controls, and strict all-target adapter Clippy passed |
| `animus-recheck-owner-v1.log` | Connected authority rail and workspace compilation passed |
| `animus-recheck-owner-v2.log` | Same complete rail and workspace compilation passed on the committed source above |
| `animus-recheck-owner-cairn-v1.json` | Repository-owned Cairn invocation: strict valid true, no findings or issues |

The owner rail ran without skip flags. It covered the generic gate tests, adapter tests,
Clippy, default/no-default isolation, maintained Wasm compile boundary, connected Clash authority,
coding-session host, runtime authority seal, catalog, facade, and evidence checks.
`cargo check --workspace --all-targets --locked` is compilation evidence, not full workspace test execution.

The existing direct/delegated callback test now also exercises repeated observations and its narrower time-bound fixture.
Observations preserve holder pointer identity, receipt identity, replay count, and decision commit count.
Expired, revoked, and substituted token inputs fail. The subsequent duplicate invocation remains denied.
The added compile-fail example prevents an observation from escaping through the callback result.
These are deterministic service fixtures, not live revocation or resource-release experiments.

## Generated dependency graph review

Nix and Cargo generated the lock updates. Five Basalt-source packages changed source identity.
Registry package versions and checksums did not change in the inspected diff.
The separately owned Basalt and Trellis `verified-logic` packages remain distinct; their serialized order changed.

Cargo also reselected existing Windows-only edges for colored, errno, nu-ansi-term,
quinn-udp, rustix, tempfile, terminal_size, and winapi-util. This was not a pin-only diff.
The inspected manifests permit multiple Windows binding versions. Tempfile 3.27.0 permits
getrandom `>=0.3.0,<0.5`; its edge moved from the existing 0.3.4 to existing 0.4.3.
No manifest constraint was relaxed and no lockfile was manually patched.

Locked all-platform reverse-graph queries passed; their logs are
`animus-recheck-windows-edges-v1.log` and `animus-recheck-windows59-edges-v1.log`.
These default-feature graph projections are not Windows compilation tests or complete optional-feature coverage.
The nu-ansi-term manifest was unavailable at the inspected local path; no separate manual source review is claimed for it.
The generated graph is accepted for the executed Linux and maintained Wasm qualification scope only.

## Preserved failure

The first lifecycle validation failed because tasks lacked requirement references.
The references were added without changing policy or requirement meaning.
The corrected validation and repository-owned validation passed.
Failed receipts remain retained and are not counted as qualification.

## Non-claims

A holder observation does not refresh Animus or Basalt policy, authenticate service freshness,
prove continuing key custody, or enforce monotonic successive observation times.
It does not establish persistent resource custody or authorize atomic release.
Consumers retain those obligations and must stop later effects when rechecks fail.
No issuer was accessed, and no real grant, training, activation, release, or promotion occurred.
Neural Stream's consumer pin remains unchanged at this source checkpoint.

## Observed lifecycle

The repository-owned Cairn task gate passed with four completed tasks and no issues.
Synchronization applied all three requirements to the accepted runtime specification.
Archive moved the change to `.cairn/archive/1970-01-01-adopt-scoped-holder-observation/`.
The epoch prefix is the deterministic CLI label, not the execution date.
Post-archive strict validation reported zero active changes, no findings, and `valid: true`.

Receipts: `animus-recheck-tasks-v1.json`, `animus-recheck-sync-v1.json`,
`animus-recheck-archive-v1.json`, and `animus-recheck-archived-validation-v1.json`.
The evidence/archive commit changes lifecycle and research evidence only; qualified product source remains unchanged.
