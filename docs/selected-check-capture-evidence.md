# Selected-check capture evidence — 2026-09-06

## Scope

The implementation retains actual bounded output in `animus-std-check-exec`.
It does not complete Neural Stream materialization or admit a trainer.
The owner change remains active until the broader completion gate passes.
No accepted spec, archive, production authority, or default runtime changes.

Baseline owner source: `d6668e0e8b148f168b33de4d0f206fdd7c891fee`.
The selected-check implementation matches the earlier `4f08f89` source at baseline.
The implementation preserves the Bounded Exec revision `29dac88ecded94457572db3fdfaaaab95fa91525`.
No dependency version or lock changes occur.

## Observed checks

The baseline passed two library tests and four real-child completion tests.
The final focused run passed those six tests, nine new capture tests,
and one compile-fail constructor test.
Strict all-target/all-feature Clippy passed for the host package.
The source-bound real-check fixture rail passed with its existing non-claims.
The lifecycle fixture passed its two tests and each missing-policy-field control.

The capture tests cover exact binary streams, legacy observation equality,
metadata-only Debug, nonzero exit, signal, timeout, each overflow direction,
exact bounds, drift, missing executables, spawn failure, cancellation,
null stdin, and the four-key explicit environment.
A private marker proves no spawn for pre-cancellation and only one normal invocation.
Running-child cancellation waits for a startup marker before raising its flag.
No cancellation test depends only on a guessed startup delay.

The first synthetic registry omitted required artifact and changed-file declarations.
The owner rejected both errors before execution. The fixture now uses valid declarations.
The declarations do not claim that their artifacts exist.

One repeated abort control exceeded its original 100 ms deadline.
The owner correctly retained `TimedOut`, rather than `ExitFailed`.
The SIGABRT fixture now has a separate named 1,000 ms deadline because abort can include kernel core-dump work.
The timeout control remains 100 ms. Production budgets do not change.

## Maintained gate findings

The full Octet gate exposed inherited lifecycle-test findings: an underscore filename
and a 1,772-line Nix file attributed to a Rust `include_str!` span.
The fixture now uses an explicit Cargo target and a nested Rust path.
It reads the same repository file through a test-only I/O helper.
The test harness propagates I/O errors through `Result`, without suppressions.
Commit `1adf8dd` records the initial prerequisite repair; later checks refine error propagation.
The final full Nix Octet gate passed with zero findings, warnings, and errors.
`owner-octet-verified.log` records this result after the error-propagation repair.

## Exact remaining blocker

`scripts/cairn/check-authoritative-agent-completion --root .` does not pass.
The current pinned lifecycle CLI rejects the unchanged positive contract:

- Contract/registry/runtime policy: `2c0b44345b9d2e2b3eff5fa7db7a14b9681c6bfcf7a4bb651f13bcadc1f91757`.
- Current lifecycle policy: `302bc1f8bcb27920c37a6bdc21865e2ca1fbb5a38c268570b3fc326734e59942`.
- Diagnostic: `verification_contract.policy_drift`.
- Positive completion receipt: `99b1a7b0195381b44c5c6db35365ce34b9a40331ac75eb9602c099d1c06c4611`.

The positive and stale-subject CLI calls both exit zero, but both report `valid: false` and `verdict: FAIL`.
The stale-subject response also reports `verification_contract.subject_stale`.
Exit status and the `authoritative` classification label do not establish a passing completion.
The capture fixture uses the separately pinned `cairn-core` at `695124d`, not the newer lifecycle CLI at `5e1fe08`.

The next repair must align the intended current policy and completion cohorts.
It must preserve stale-policy and stale-subject rejection across the registry,
runtime contract, core adapter, CLI, generated fixtures, and maintained gate.
Changing only an expected hash, substituting saved success, or ignoring the gate is not a repair.

## Retained evidence and non-claims

The implementing assistant performed serial, correlated reviews. No independent agent review occurred.
Both-stream command logs and raw positive/negative CLI receipts remain in the persistent
Neural Stream `.pi/drains/2026-09-06-execution-host/` directory.
`owner-capture-final.log` records the final package controls and focused rail.
`owner-verified-final.log` records the broader gate failure.
`completion-positive.json` and `completion-negative.json` retain the actual rejected responses.

The initial full development shell acquisition was stopped before baseline tests.
The baseline and focused runs use Rust tools from the same owner-locked Nixpkgs through `nix shell --inputs-from .`.
No remote trust changes or dependency overrides occurred.

Captured bytes and metadata hashes do not authenticate callers, executable paths,
output semantics, current authority, native builds, or promotion.
Identical runs can have identical metadata hashes. These hashes are not fresh execution identifiers.
No candidate training, held-out output, tensor admission, or production effect occurred.
