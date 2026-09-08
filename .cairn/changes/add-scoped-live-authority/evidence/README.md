# Scoped authority evidence

Base: published Animus `39c3411c548eb84f49f3df17569d893337c7f7e6`.
Owner implementation: `animus-std-basalt-authority` only. No dependency or lock
changed. Generic no-std contracts contain no new Basalt or UCAN types.

## Observed checks

The baseline passed 15 owner tests. The implementation passes 18 owner tests and
five compile-fail doctests. Positive cases cover direct and delegated holders,
a narrower token interval, and process execution. The callback observes the
successful commit and one replay admission before it reads live authority.
The actual decision, receipt, operation, audience, nonce, and time bounds agree.
Metadata output remains separate from live authority.

Denial controls cover malformed and expired credentials, replay, local gates,
failed commit, invalid commit identity, and effect failure. A second dispatch
with the same nonce never reaches the callback. Effect failure retains the
committed receipt. Compile-fail cases reject view cloning, private construction,
Debug formatting, view escape, and borrowed-holder escape.

Strict all-target Clippy and formatting pass. The complete focused
`check-basalt-authority-adapter` rail passes without skip flags. It includes
Nickel, default/no-default/Wasm isolation, the connected Clash authority,
coding-session host, runtime authority seal, component catalog, facade inventory,
and evidence safety checks.

## Qualification corrections

The old generic-authority filter `host_authority` selected zero tests.
The corrected `effect::gate::tests` filter selects three passing controls.
The first broad run is not qualification evidence: its script changed while
that run was active. It ended with `.: filename argument required`.
The corrected script then ran unchanged and passed the complete rail.
No policy relaxation, skipped connected check, or lint suppression was added.

## Retained private logs

The initiating Neural Stream checkout retains the logs under
`.pi/drains/materialization-complete/`: `owner-live-baseline.log`,
`owner-live-checks.log`, `owner-live-full-rail.log`, and
`owner-live-corrected-rail.log`. Logs remain outside this removable worktree.
The tests use fixture keys, not the approved research issuer.

## Non-claims and remaining checks

This API exposes verification-time authority. It does not provide a continuing
revocation recheck, resource custody, input authenticity, process containment,
training, consumer activation, or production/release eligibility.
`cargo check --workspace --all-targets --locked` passed. Cairn validation and
`git diff --check` passed. `owner-live-workspace-check.log` and
`owner-live-validation.log` retain those results. This evidence prepares the
accepted-spec and publication handoff. No publication or consumer pin change
is claimed by these tests.
