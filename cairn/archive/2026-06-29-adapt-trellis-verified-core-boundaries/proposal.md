# Proposal: Adapt Trellis verified-core boundaries

## Why

Trellis is listed as a reference for verified-logic building blocks, but Animus does not yet have a Cairn plan for which safety-critical core decisions should receive proof-oriented treatment. Runtime policy checks, ledger graph validation, receipt redaction, schema compatibility, and adapter effect gates are pure enough to benefit from verified invariants if their boundaries are designed intentionally.

This change defines how Animus can adopt Trellis-style verification without making every crate depend on proof tooling.

## What Changes

- Identify candidate pure cores for proof-oriented hardening: capability/effect gate decisions, ledger graph validation, branch replay, receipt redaction classification, schema compatibility, and roadmap dependency checks.
- Define proof obligations and invariants for each candidate before implementation.
- Keep Trellis dependencies optional, feature-gated, or test-only until a stable verified core boundary is accepted.
- Add equivalence tests between verified/model logic and production Rust logic where both exist.
- Require negative tests for malformed graphs, bypass attempts, redaction leaks, unsupported compatibility cases, and invariant violations.

## Impact

- **Files**: verification design docs, optional verified-core modules, proof fixtures, equivalence tests, dependency-isolation checks, and Cairn artifacts.
- **Testing**: invariant/property/equivalence tests plus negative fixtures for rejected unsafe states.
- **Non-claims**: this does not require all Animus logic to be formally verified, does not make Trellis a default runtime dependency, and does not replace ordinary Rust tests or review.
