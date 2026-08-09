# Proposal: Add profile-bound provider-retry host smoke

## Why

Animus has accepted per-model profile loading and bounded provider-retry contracts. No packaged host path binds both contracts before deterministic provider dispatch.

This gap leaves profile-specific dispatch and retry as separate contract claims. A focused fake-backed smoke can prove their bounded composition without credentials or network access.

## What Changes

- Add a non-default `animus-profile-retry-smoke` binary in `animus-std`.
- Require explicit profile policy, retry policy, provider, model, prompt, and failure-class inputs.
- Resolve one exact loaded profile and bind its identity into one immutable retry invocation.
- Exercise one policy-admitted retry through injected deterministic timer and model services.
- Emit metadata-only profile, decision, dispatch, and receipt identities.
- Write a focused receipt with the source BLAKE3 identity and current-head metadata when Git is available.
- Reject missing profiles, terminal failure classes, cancelled timers, changed requests, malformed policy, and raw-data evidence.
- Add a focused script and Nix check with positive and negative fixtures.
- Update the dogfood roadmap and README with the new bounded evidence rung and non-claims.

## Impact

- **Crate**: `animus-std` gains one feature-gated smoke module and binary.
- **Checks**: a focused profile-retry smoke rail and Nix check run without credentials or network access.
- **Docs**: the dogfood roadmap records packaged profile loading and retry execution.
- **Non-claims**: no live provider, retry reliability, provider quality, fallback, streaming, persistence, TUI, daemon, or production claim.
