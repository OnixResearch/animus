# Runtime authority seal evidence

## Outcome

The runtime authority seal is implemented with a pure core and an injected standard-host shell.

The core binds one dispatch to exact supplied authority identities. The host commits the metadata-only seal receipt before the dependent effect.

## Baseline and ownership

The branch already contained the implementation before this drain. The initial focused command did not start because its wrapper lacked executable permission.

The exact failure was `scripts/cairn/check-runtime-authority-seal: Permission denied`. The drain restored the executable bit and ran the complete rail.

`animus-runtime` owns validation, canonical BLAKE3 identities, runtime-view comparison, state transitions, and safe receipt preparation.

`animus-std` owns service preparation, current observations, receipt commit, effect dispatch, cancellation, and cleanup through injected services.

## Passing evidence

- The proposal, design, tasks, and repository validation gates passed.
- The focused rail passed 10 runtime tests and 4 host tests.
- The focused rail source identity is `c4b2e2f6e015856ce5afe37a99155f98d527292543b22d5dccb98372d6b29b8d`.
- The focused rail evidence identity is `7a34d1ff40fd92eccbd5191f6f554e41cadc57fb036eec1b34872068b025d057`.
- Rust formatting and focused Clippy checks passed with warnings denied.
- `scripts/config/check-nickel-configs` passed.
- `cargo check --workspace --all-targets --locked` passed.
- `nix build .#checks.x86_64-linux.runtime-authority-seal --no-link -L` passed on the local builder.

The configured remote builder was unavailable. Nix continued on the local builder and completed the requested check.

## Positive and negative coverage

Positive tests cover canonical sealing, exact runtime views, deterministic identity, commit-before-effect ordering, dispatch, cleanup, and a later new unit.

Negative tests cover missing, extra, duplicate, stale, unknown, and unsupported observations. They also cover widening, rebind, fallback, early effects, commit failure, leaks, and overclaims.

Property tests cover canonical input order, deterministic issue order, and monotonic authority after sealing.

## Non-claims

This evidence does not prove authorization, operating-system isolation, hidden-authority absence, sandbox correctness, host integrity, effect correctness, or release eligibility.
