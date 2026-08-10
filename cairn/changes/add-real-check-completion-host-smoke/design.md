# Design: Real-check completion host smoke

## Goal

Produce one optional packaged receipt only after the reviewed quick checks run as bounded child processes and the pinned Cairn core validates those exact observations.

## Portfolio decision

Three independent routes were considered:

1. Add `bounded-exec` directly to `animus-std`. This is rejected because it would mix an AGPL process mechanism into the MIT compatibility crate.
2. Run commands in the focused shell script and pass result JSON to the existing binary. This is rejected because the packaged host would not own command admission, limits, or result construction.
3. Add a separate AGPL authority crate and inject its executor plus Cairn summary into the existing workflow. This route is selected because it preserves license and authority boundaries while reusing the accepted completion reducer.

## Boundaries

`animus-std` exposes a generic packaged workflow entrypoint over two existing host seams:

- `AgentCheckExecutionService` for exact selected execution; and
- a new factory that creates a `CairnCompletionValidationService` from the selected checks and admitted observations.

The default authoritative-completion smoke still injects deterministic fixtures. The new crate injects:

- an exact allowlist for the `nickel-config` and `cairn-validate` quick checks;
- `bounded-exec` with no shell, null stdin, process-group teardown, accepted exit code zero, and named timeout/output budgets;
- an explicit repository root, executable search path, home, and temporary directory;
- a cleared child environment containing only reviewed entries; and
- the pinned `cairn-core` evaluator over generated contract and result values.

The executor hashes bounded stdout and stderr bytes. It records byte counts, exit code, signal, timeout state, cleanup state, command identity, subject identity, and receipt identity. It does not print child output.

## Completion order

1. Load exact generated role, model, and check-registry projections.
2. Compose the four-role advisory result and require judge selection.
3. Select the reviewed quick checks for the fixed smoke scope.
4. Execute each exact command through the bounded host.
5. Admit every observation through the existing pass-only result validator.
6. Build Cairn contract and result values from the same selected rows and observations.
7. Run the pinned Cairn evaluator and inject its normalized summary.
8. Evaluate authoritative completion and render metadata only.

Any non-passing child result, spawn failure, timeout, output overflow, command drift, unsupported check, stale subject, dirty state, missing result, veto, or failing Cairn summary blocks output.

## Verification

The crate keeps positive and negative fixture-child tests together. The focused rail runs these tests in all environments. Its `--live` mode additionally requires a clean Git head and executes the actual current registry commands. The Nix check runs sandbox-safe fixture coverage. A final clean-current-head live receipt is required before publication.

## Non-claims

The receipt does not prove that a selected check is sufficient, correct, or honest. It does not authenticate executable bytes, establish sandboxing or hermeticity, prevent all child side effects, prove process-runner correctness, or establish release or production readiness.
