# Autonomous gates

Animus provides an optional `autonomous-gates` contract family. The family adds bounded continuation decisions, host-owned quality gates, admitted-file fingerprints, bounded failure output, and metadata-only receipts.

The reviewed reference is PrimeIntellect `prime-agent` at revision `c5991bc853d27754aed345c13ff4d2e05c40f1f4`. The reference is MIT licensed. Animus adopts only bounded gate, continuation, output, and change-detection patterns. It does not adopt the reference command runner, provider, persistence, or sandbox behavior.

## Authority boundary

The runtime receives typed gate descriptors and typed host results. It does not run commands, read files, read clocks, inspect environment values, or persist state. Hosts own those effects.

The host supplies a `WorkspaceFingerprint` over an explicit admitted file set. The fingerprint does not cover files outside that set. The runtime denies a failed-gate rerun when the same gate and fingerprint already have a failed or timed-out attempt.

## Outcome boundary

`gates_passed` means that the admitted gates passed within their declared scopes. `limit_reached` means that a named continuation, turn, token, or wall-clock limit stopped the run. Neither outcome means task success or objective completion.

## Output and receipts

Failed gate output is bounded before it enters the next prompt. The projection keeps head and tail text and records the original byte count. Autonomous receipts contain policy, gate, scope, outcome, fingerprint, limit, and non-claim metadata. They omit raw output, prompts, transcripts, credentials, environment values, and unadmitted paths.

## Validation

Run the focused rail:

```sh
scripts/cairn/check-autonomous-gates --root .
```

The rail runs positive and negative autonomous-gate tests, the Nickel export drift check, formatting, no-default Wasm compilation, and Cairn validation. It does not claim gate sandboxing, verification completeness, production readiness, release eligibility, or task completion.
