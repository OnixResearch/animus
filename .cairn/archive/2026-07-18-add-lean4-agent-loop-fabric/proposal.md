## Why

Animus already exposes an implementation-neutral `LoopFabric` contract and uses Steel as its canonical scriptable loop fabric. It does not yet provide a statically compiled loop fabric whose transition kernel and selected safety invariants are checked by Lean 4. Embedders therefore cannot choose a Lean-authored loop without building an unreviewed protocol bridge or moving provider, tool, process, and session authority outside Animus's existing boundaries.

A Lean integration is useful only if it remains an optional fabric implementation. It must not replace `animus-runtime`, create a second transcript or session owner, treat Lean typechecking as whole-system correctness, or introduce Rust FFI that violates the workspace's `unsafe_code = "forbid"` policy.

## What Changes

- Add a non-default `lean/animus-lean/` Lake package with pure typed turn, policy-state, decision, validation, and transition modules plus a compiled reference model-then-respond policy. r[agent_harness_runtime.lean4_loop.boundary] r[agent_harness_runtime.lean4_loop.core]
- Add reviewed Lean theorems for the encoded kernel's closed decision set, deterministic transition, non-increasing budgets, terminal absorption, and fail-closed invalid-input behavior, with no `sorry`, `admit`, or repository-defined axioms. r[agent_harness_runtime.lean4_loop.proofs]
- Define a versioned bounded line-delimited JSON protocol because JSON is natively supported at the Lean/Rust interoperability boundary; keep human-authored profile configuration in typed Nickel. r[agent_harness_runtime.lean4_loop.protocol] r[agent_harness_runtime.lean4_loop.profile]
- Add an optional authority-scoped `animus-std-lean-loop` Rust crate that launches an explicitly selected Lean executable, implements `animus_runtime::LoopFabric`, and revalidates every returned model, current-shape tool, response, or stop decision through existing Rust contracts. r[agent_harness_runtime.lean4_loop.bridge]
- Keep provider dispatch, tool execution, transcript/session state, clocks, cancellation, process launch, sandbox enforcement, persistence, and authoritative completion in existing host/runtime owners. The Lean process receives only the bounded neutral turn envelope and returns a candidate fabric decision. r[agent_harness_runtime.lean4_loop.authority]
- Bind toolchain, protocol, source/profile, executable-artifact, launch, resource-bound, fallback, and redaction posture to exact identities. Use BLAKE3 for stack-owned profile, executable, transcript-envelope, and receipt identities. r[agent_harness_runtime.lean4_loop.profile] r[agent_harness_runtime.lean4_loop.receipts]
- Add positive and negative Lean, Rust, protocol, fixture-child, dependency-isolation, proof-trust, and end-to-end fake-adapter tests plus a focused Cairn/Nix verification rail. r[agent_harness_runtime.lean4_loop.verification]

## Impact

- **Lean source**: a new repository-owned Lake package and executable with a pure functional core and thin stdio shell.
- **Rust workspace**: one optional std host-adapter crate and root feature; reusable/default/no-std/Wasm crates do not gain Lean, Lake, JSON transport, process, or FFI dependencies.
- **Configuration**: a typed Nickel Lean-loop profile and deterministic runtime projection; numeric protocol and resource limits are named policy fields rather than literals scattered through code.
- **Builds**: the pinned Nix input's reviewed Lean 4 cohort is added to the development and focused-check surfaces without manually editing `flake.lock`.
- **Compatibility**: Steel remains the canonical scriptable fabric and default runtime policy surface. Lean is a statically compiled optional fabric behind the same neutral contract.
- **Claims**: passing evidence establishes bounded protocol conformance, Rust boundary revalidation, and the named theorems over the encoded Lean kernel. It does not establish arbitrary policy correctness, Lean compiler/runtime soundness, child-process confinement, provider/tool correctness, transcript correctness, theorem-to-Rust semantic equivalence, or whole-agent correctness.
