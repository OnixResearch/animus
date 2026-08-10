# Proposal: Add a real-check completion host smoke

## Why

The packaged authoritative-completion smoke currently supplies deterministic passing check observations. It proves completion composition, but it does not run the selected repository commands. A stronger optional rung must execute exact selected commands under finite limits before it can claim observed check execution.

## What changes

- Add a separate non-default AGPL host crate for bounded selected-check execution.
- Reuse the packaged four-role and completion workflow through injected execution and Cairn-summary services.
- Execute exact registry argv without a shell, with explicit root, environment, timeout, output, and process-group limits.
- Validate the captured result rows with the pinned Cairn completion core before Animus grants completion.
- Add positive and negative fixture-child tests, a clean-current-head live mode, a focused rail, a Nix check, documentation, and metadata-only receipts.

## Impact

The reusable runtime and default `animus-std` features gain no process authority. The new crate is `AGPL-3.0-or-later` because it links the reviewed `bounded-exec` cohort. The smoke proves bounded child observations for two reviewed quick checks. It does not prove check-oracle soundness, executable authenticity, sandboxing, hermeticity, child correctness, release readiness, or production readiness.
