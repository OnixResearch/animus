# Optional Lean 4 loop fabric

Animus provides `animus-std-lean-loop` as a **non-default** statically compiled implementation of the existing `animus_runtime::LoopFabric` contract. Steel remains the canonical scriptable fabric. The Lean adapter is an internal fabric boundary, not ACP hosting, a second runtime, a session owner, or an authoritative-completion mechanism.

## Reviewed cohort and package

- Lean toolchain: exact locked-Nix cohort `lean4-4.30.0`.
- Lake package: `lean/animus-lean/`, package identity `animus-lean-0.1.0`.
- Protocol: `animus.lean-loop.protocol.v1` with schema `animus.lean-loop.envelope.v1`.
- Rust feature: `lean-loop`; Rust crate: `animus-std-lean-loop`.
- Configuration source: `config/lean-loop-profile.ncl`; checked projection: `config/generated/lean-loop-profile.json`.
- Identity: stack-owned profile, focused-source, compiled executable, protocol, reviewed proof-source, and receipt identities use domain-separated BLAKE3. The checked Nickel projection and compiled Rust validator both carry the exact proof-source identity recomputed from `Core.lean`, `Protocol.lean`, and `ProofTrust.lean` by the focused rail.

The repository-owned Lake package has no external Lake dependencies. `AnimusLean.Core` defines total DTO validation, finite fabric-local phase/budget state, current-shape tool data, and the pure reference model-then-respond transition. `AnimusLean.Protocol` defines pure closed JSON-line decoding/encoding and correlation. `Main.lean` is the thin bounded stdin/stdout shell. It reads finite frames one byte at a time, flushes one response per request, and performs no environment, filesystem, socket, provider, tool, credential, session, transcript, or dynamic-source discovery.

## Proof surface

The maintained theorem suite covers only the encoded Lean kernel:

- identical explicit input has one deterministic result;
- candidate decisions are in the closed `model`, `tool`, `respond`, or `stop` set;
- step and payload budgets do not increase;
- terminal state has no dispatchable result;
- invalid profile input fails closed; and
- the reference policy follows its initial-model and model-response phases.

Maintained proof source contains no `sorry`, `admit`, or repository-defined `axiom`. `ProofTrust.lean` queries each named theorem with `#print axioms`; the focused rail accepts only Lean's reviewed `propext`, `Classical.choice`, and `Quot.sound` trust dependencies. This does **not** prove the specification correct, Lean/compiler/runtime soundness, JSON codec correctness, theorem-to-Rust equivalence, or whole-agent correctness.

## Host activation

Build the child and enable the adapter explicitly:

```sh
nix develop -c lake -d lean/animus-lean build AnimusLean animus_lean animus_lean_tests
cargo check -p animus --features lean-loop --locked
```

An embedder loads the checked generated profile, supplies an absolute executable path, expected executable BLAKE3 identity, absolute working directory, sandbox observation, and explicit cancellation signal, then calls `LeanLoopFabric::launch`. The adapter clears the child environment, does not search `PATH`, verifies the executable bytes observed before launch, uses bounded pipes and finite deadline/cleanup polls, and never opens a socket. A required sandbox profile accepts only host-supplied enforcement evidence; `fixture-only` evidence remains explicitly non-confining. The identity observation does not provide atomic path pinning across the hash-to-spawn interval, so a production host would need an immutable admitted artifact path or a stronger platform launch primitive; the current evidence makes no such claim.

Every Lean output is an untrusted candidate. Rust checks exact protocol/profile/artifact/sequence identity, rejects duplicate/unknown/trailing JSON data, and reconstructs the candidate through current `ModelRequest`, full structured `ToolRequest`, `ModelResponse`, or `Stop` contracts. Existing runtime capability, catalog, policy, approval, sandbox, executor, max-turn, event, failure, and completion boundaries still run afterward.

## Failure and fallback

Spawn, identity, sandbox, handshake, malformed or oversized frame, unknown decision, stale correlation, invalid tool shape, timeout, cancellation, EOF, broken pipe, backpressure, stderr overflow, non-zero exit, or cleanup failure is terminal for that transport. The adapter does not restart, replay an ambiguous turn, invoke Steel, switch transport, or dispatch a model/tool after failure. Higher-level code may create a separately admitted run, but that is not implicit fallback.

## Verification

Run:

```sh
nix develop -c scripts/cairn/check-lean4-loop-fabric --root .
nix build .#checks.x86_64-linux.lean4-loop-fabric -L
```

The rail checks Nickel freshness and negative profiles, offline Lake build/tests, theorem trust output, the actual compiled Lean child, protocol ambiguity fixtures, current Rust DTO reconstruction, process failure fixtures, metadata-only receipt leak rejection, root feature compilation, and default/no-std/Wasm dependency isolation.

## Evidence and non-claims

Receipts contain only safe cohort/profile/artifact/proof/decision/failure/sandbox/exit/cleanup identities or classes, bounded traffic counts, redaction markers, explicit non-claims, and a domain-separated BLAKE3 identity. They exclude raw summaries, prompts, model/tool bodies, provider payloads, source, executable paths, environment values, credentials, stderr, hidden reasoning, and unbounded diagnostics.

Passing fixture and focused evidence does not establish arbitrary policy correctness, arbitrary compiled-policy safety, child confinement, atomic executable path pinning, external sandbox correctness, provider/tool correctness, transcript/session correctness, Lean soundness, theorem-to-Rust equivalence, authoritative task completion, production readiness, or release eligibility.
