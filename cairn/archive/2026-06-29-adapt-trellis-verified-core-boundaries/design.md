# Design: Trellis verified-core boundaries

## Context

Animus follows a functional-core / imperative-shell split. That makes some safety decisions small, deterministic, and suitable for proof-oriented treatment. The goal is not broad formal verification; it is to reserve clean seams for the logic where bugs would undermine safety claims.

## Decisions

### 1. Verify only pure decision cores

Candidates include graph validation, dependency ordering, redaction classification, schema compatibility, capability/effect authorization, and cut-point selection. Filesystem, process, provider, network, terminal, and clock behavior remain outside verified cores.

### 2. Write obligations before proofs

Each verified candidate declares inputs, outputs, invariants, preconditions, postconditions, rejected states, and equivalence expectations. Proof code must not be a substitute for stating the safety property in Cairn specs and tests.

### 3. Keep proof dependencies isolated

Trellis or proof-helper crates should be optional, test-only, or located in dedicated verified modules. Default embedders should not inherit proof toolchains unless they opt in.

### 4. Cross-check with production Rust

When a verified or model implementation coexists with production Rust, deterministic fixtures and property tests compare outcomes for valid and invalid inputs. Divergence is a release-blocking diagnostic unless explicitly waived.

### 5. Preserve reviewable counterexamples

Negative fixtures should capture malformed graphs, duplicate IDs, cycles, unsupported compatibility payloads, unsafe redaction inputs, and authorization bypass attempts as durable regression evidence.

## Risks

- Proof scope can expand beyond useful seams. Keep candidates small and pure.
- Optional proof tooling can rot. Gate it with focused checks or clearly mark unsupported status.
