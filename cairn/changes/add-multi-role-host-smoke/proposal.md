# Proposal: Add capability-bound multi-role host smoke

## Why

Animus has accepted semantic role, model-profile, capability, separation, output, receipt, and completion contracts. No packaged host path activates implementer, reviewer, verifier, and judge roles as one bounded workflow.

A deterministic smoke can prove role separation, exact authority intersections, judge veto, and completion non-equivalence without provider, tool, credential, network, or storage effects.

## What Changes

- Add a non-default `animus-multi-role-smoke` binary in `animus-std`.
- Load explicit generated role and model-profile projections.
- Compose and activate implementer, reviewer, verifier, and judge assignments over one immutable subject.
- Bind exact model profiles, capability ceilings, separation observations, output grants, and safe receipts.
- Admit candidate, finding, verification-evidence, and veto outputs.
- Prove that reviewer mutation, collapsed separation, denied veto authority, and completion claims fail closed.
- Prove that role output cannot satisfy the separate authoritative-completion candidate gate.
- Add positive and negative Rust fixtures, a focused script, a Nix check, and a current-head receipt.
- Update the README and dogfood roadmap with the new bounded evidence rung and non-claims.

## Impact

- **Crate**: `animus-std` gains one feature-gated smoke module and binary.
- **Checks**: a focused multi-role smoke rail and Nix check run without credentials or network access.
- **Docs**: the dogfood roadmap records packaged role activation and completion non-equivalence.
- **Non-claims**: no provider quality, verifier soundness, human independence, mutation execution, authoritative completion, TUI, daemon, or production claim.
