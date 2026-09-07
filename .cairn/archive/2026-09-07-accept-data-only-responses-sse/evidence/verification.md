# Verification

## Change

The parser admits data-only Responses SSE records while keeping the mandatory JSON envelope type and explicit-name agreement.
One unnamed `[DONE]` transport marker is admitted only after a validated completed response.
The marker changes no canonical frame and no semantic terminal state.
Premature, repeated, named, failed-response, and post-marker cases stay rejected.

Owner implementation branch: `fix/openrouter-responses-framing-20260907`, based on Animus `origin/main` (`eff1773`).
Narrow backport branch: `fix/openrouter-responses-backport-20260907`, based on the product pin `958c488eb1d5ac3720e891e49cc9a8cb79271c6b`.
The parser, tests, and affected documentation match across both branches.
Neither dependency lockfile changed on the backport.

## Corrected gate basis

An earlier blocker used a self-built Octet consumer check scoped to `animus-std-openai` alone.
That check reported 260 errors on the unchanged pin and 258 after the fix, with no new finding by lint and source path.
The owning project's own Octet rail (`checks.octet` in the Animus flake) covers ten packages and deliberately excludes `animus-std-openai`, with the comment that the broad rail exposes debt in crates outside a focused change.
The 258 remaining findings are pre-existing debt outside the owning gate, dominated by public-API naming that a narrow transport fix must not churn.
Publication therefore follows the owning project's own gate. The stricter single-crate count comparison is retained as evidence that the fix adds no debt.

## Passing evidence

Focused upstream gates on the owner tree, all green in `owner-focused.log`:

- `cargo-fmt`, `cargo-check`, `cargo-clippy`, `cargo-test` (workspace)
- `octet`, `adapters-octet`, `pueue-octet` (the owning Octet rails)
- `live-openai-provider-host` (includes the new framing tests)

The full upstream `nix flake check . -L` on the final tree (`owner-flake-check.log`) passes every attribute except `basalt-authority-adapter`, which fails at its stale checked-in Nickel export manifest.
That failure reproduces identically on untouched `origin/main` (`basalt-baseline.log`) and is unrelated to this change, which touches no basalt file.

Package-level results on both branches (`tests-final.log`, `backport-tests-final.log`):
24 tests passed with the same two pre-existing live tests ignored as on the untouched baseline (`baseline.log`).
Strict package Clippy passed on both branches (`clippy-final.log`, `backport-clippy-final.log`).
The focused Nix host check passed on both branches (`nix-host.log`, `backport-nix-host-final.log`).

Offline acceptance (`raw-replay-final.log`, script `replay.rs`):
the unmodified retained GLM capture, identity `0dc2f2cda43fe5493392d59fa92addbf345d149c080f743884e3e9169d942602`,
parses to terminal `Completed` with exact text `READY` through the patched parser, with zero network calls.
Negative controls reject a premature marker, a repeated marker, and a duplicate sequence.

Cairn gates for proposal, design, and tasks: PASS (`gate-plan.json`, `gate-tasks.json`, `gate-final.json`).

## Lifecycle execution under a pre-existing tool limit

CLI `sync` and `archive` both fail on this repository root with `file exceeds the bounded read limit`.
The cause is pre-existing upstream debt: the accepted spec `.cairn/specs/agent-harness-runtime/spec.md` is 1,654,192 bytes, above cairn's `SPEC_READ_MAX_BYTES` of 1,048,576, so every root-scoped lifecycle read fails before change-specific logic runs.
This limitation exists on the untouched `origin/main` and is independent of this change.

Following the lifecycle skill's manual-repair provision:

- The three `responses_sse_framing` requirement blocks were merged verbatim from the change delta into the accepted spec's `## Requirements` tail; `grep -c responses_sse_framing` confirms exactly 3 promoted IDs.
- The change package was moved to `.cairn/archive/2026-09-07-accept-data-only-responses-sse/` following the tool's observed archive convention.
- Gates had already recorded PASS before the move: proposal, design, and tasks (`gate-proposal.json`, `gate-design.json`, `gate-tasks-final.json`).
- CLI sync/archive failures are retained in `sync-dryrun.txt` and `archive-dryrun.txt`.

Raising cairn's spec read bound, or splitting the oversized upstream spec, is separate scoped work outside this narrow transport fix.

## Publication and adoption

Publication is the Animus branch and `main` push plus the product pin update to the published revision.
The product pin update regenerates `flake.lock` through `nix flake lock` and `Cargo.lock` through cargo; no lockfile is hand-edited.
After adoption the product reruns its full verification, and one bounded native preflight confirms framing acceptance end to end.
Deployment and lesson activation stay out of scope.

## Retained evidence

All logs live in the product repository under `.pi/glm-campaign/upstream/`:
`baseline.log`, `tests-final.log`, `backport-tests-final.log`, `clippy-final.log`, `backport-clippy-final.log`,
`nix-host.log`, `backport-nix-host-final.log`, `owner-focused.log`,
`octet-baseline-nix.log`, `octet-backport-final.log`, `lint-comparison.json`,
`gate-proposal.json`, `gate-design.json`, `gate-tasks-final.json`, `validate-change.json`,
`sync-dryrun.txt`, `archive-dryrun.txt`,
`raw-replay-final.log`, `replay.rs`, and the gate JSON verdicts.
