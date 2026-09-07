# Verification and publication blocker

## Implemented scope

The parser now admits data-only records while preserving explicit-name agreement and the existing typed envelope checks.
One unnamed transport marker can follow a validated completed response.
The marker changes no canonical frame or semantic terminal state.
Premature, repeated, named, failed-response, and post-marker cases remain rejected.

The owner implementation is `8e300b3bcc95c340202c14c97bda5c72182eba80`, based on `eff1773`.
The narrow backport is `a63d478`, based on `958c488eb1d5ac3720e891e49cc9a8cb79271c6b`.
The parser, tests, and affected documentation match between these revisions.
Neither dependency lockfile changed on the backport.
Nineteen unrelated upstream commits remain outside that backport.

## Passing checks

- Baseline package tests: 19 passed and two pre-existing live tests ignored.
- Updated package tests: 24 passed and the same two live tests ignored.
- Package all-target/all-feature Clippy with `-D warnings`: passed on both owner and backport revisions.
- Rust formatting: passed.
- Focused Nix `live-openai-provider-host` check: passed on both revisions.
- Proposal, design, and corrected tasks gates: PASS.
- Offline raw-capture replay: the unmodified retained GLM response produces `Completed` and exact `READY` through the patched parser.
- Replay negative controls reject premature markers, repeated markers, and duplicate sequences.

The replay used capture identity `0dc2f2cda43fe5493392d59fa92addbf345d149c080f743884e3e9169d942602`.
No provider request occurred during implementation or replay.
The broader campaign remains paused, with no candidate, score, held-out request, or activation.

## Strict acceptance remains blocked

The full pinned Octet consumer check was run on the unchanged original revision and on the backport.
It used the published `mkConsumerCheck` helper with package, all-target, and all-feature scope.

| Source | Errors | Warnings | Verdict |
| --- | --- | --- | --- |
| Original `958c488` | 260 | 1 | integration-failure |
| Backport `a63d478` | 258 | 1 | integration-failure |

The warnings include the build-failure summary, not an acceptance allowance.
Existing findings cover file length, module layout, assertion density, naming, owner-path imports, and other strict rules throughout the crate.
A count comparison by lint and source path shows no increases. Two assertion-density findings in `stream.rs` disappeared.
That comparison does not waive any remaining finding and is not a finding baseline for acceptance.
The product's earlier clean Octet gate covered the product crate, not this dependency's own source and tests.

An initial direct hook invocation failed with unknown lint names before target analysis.
The later Nix helper correctly loaded the pinned catalog and supplied the valid baseline and backport results above.

Publication, lifecycle completion, and product pin adoption remain blocked.
Removing the existing crate-wide debt requires a broader cleanup than the approved narrow framing correction.
No lint catalog, warning budget, baseline allowance, or broad source suppression was introduced.

## Retained evidence

All logs remain in the product repository's `.pi/glm-campaign/upstream/` directory:

- `baseline.log`, `tests-final.log`, and `backport-tests-final.log`
- `clippy-final.log` and `backport-clippy-final.log`
- `nix-host.log` and `backport-nix-host-final.log`
- `octet-baseline-nix.log` and `octet-backport-final.log`
- `lint-comparison.json`
- `raw-replay-final.log` and `replay.rs`

The change remains active. No sync, archive, push, product pin update, deployment, or worktree removal occurred.
