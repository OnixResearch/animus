## Context

`animus-runtime` already owns the reusable agent loop and exposes `LoopFabric::decide(RuntimeLoopTurn) -> RuntimeLoopDecision`. Steel is the canonical dynamic fabric, while the neutral trait deliberately permits non-Steel implementations. Runtime state, max-turn enforcement, adapter admission, event production, and final status remain Rust-owned.

The requested Lean 4 surface should exploit that seam rather than reproduce the runtime. The useful unit is a statically compiled Lean fabric: a pure Lean policy kernel selects a candidate decision, a thin Lean executable transports that decision, and an optional Rust std adapter revalidates it before the existing runtime may dispatch a model or tool.

A bounded architecture search considered three mechanisms:

1. **In-process Lean FFI** would reduce process overhead, but it requires an ABI/unsafe boundary in Rust, couples the workspace to Lean runtime details, and conflicts with `unsafe_code = "forbid"`.
2. **Proof-only Lean models or generated Lean from Rust** could check selected invariants, but would not provide the requested Lean-authored runtime fabric and would overlap the separate Aeneas proof-campaign scope.
3. **A compiled Lean fabric over bounded stdio** preserves the existing trait and authority model, uses Lean's native JSON support, and keeps unsafe/ABI code out of Rust. This is the selected mechanism.

The ACP stdio host change is related but directionally different: ACP exposes Animus as an external agent and maps sessions. The Lean bridge is an internal optional `LoopFabric` implementation and must not implement ACP, own sessions, or reuse ACP correlation as runtime authority.

## Decisions

### 1. Implement an optional fabric, not a second runtime

**Choice:** Add `lean/animus-lean/` as a repository-owned Lake package and `crates/animus-std-lean-loop` as an optional authority-scoped Rust adapter. The Rust adapter implements the existing neutral `LoopFabric` trait. `animus-runtime` remains the owner of turn sequencing, max-turn enforcement, adapter dispatch, events, failures, and completion.

The Lean package may retain bounded fabric-local policy state such as a phase and consumed-step count, but it must not own transcript entries, session identity, active leaf, queue state, durable commits, provider state, tool execution state, or authoritative completion. Fabric-local state is disposable when the child exits.

**Rationale:** This satisfies the Lean loop request through the extension point Animus already maintains and prevents divergent lifecycle ownership.

### 2. Keep a pure Lean core and a thin Lean shell

**Choice:** Lean modules separate pure DTO validation and `step` reduction from `IO` framing. The core consumes explicit policy/profile values, fabric-local state, and a normalized turn; it returns a typed next state and candidate decision or a typed error. It does not read stdin, write stdout/stderr, inspect environment variables, open files or sockets, read clocks, invoke providers/tools, or launch processes.

The shell performs only bounded stdin frame reads, JSON decode/encode, one-request/one-response correlation, stdout flush, safe bounded diagnostics, EOF classification, and clean exit. It does not load source dynamically or discover profiles, executables, providers, tools, credentials, or sessions.

**Rationale:** The transition logic remains testable with Lean evaluation and theorem checking, while unavoidable transport effects stay explicit and narrow.

### 3. Ship a compiled policy interface and one small reference policy

**Choice:** Define a Lean `LoopPolicy` interface for statically compiled policies and ship one reference model-then-respond policy: an initial valid turn requests the model with the normalized summary; a later valid model result is returned as the response; explicit invalid or exhausted states fail closed or stop according to the bound profile. Tool-decision construction remains part of the generic kernel and fixture policies so the complete current contract is covered, but the reference policy does not infer tool calls from unstructured model text.

Third-party policies are separate compiled artifacts with distinct BLAKE3 identities and review evidence. The adapter does not evaluate arbitrary Lean source at runtime.

**Rationale:** A concrete reference loop proves the path is usable without pretending a plain model string is a trustworthy tool-call language. Static compilation distinguishes this surface from Steel's dynamic scripting role.

### 4. Prove only explicit encoded invariants

**Choice:** Maintain named Lean theorems showing, for the encoded kernel and reference policy, that:

- identical explicit inputs produce the same result;
- successful steps return only the closed `model`, `tool`, `respond`, or `stop` decision variants;
- admitted remaining-step and payload budgets never increase;
- terminal state cannot emit a model or tool request;
- invalid profile, turn, correlation, or decision input yields an error without a dispatchable decision; and
- the reference policy's phase transitions follow its declared model-then-respond sequence.

Maintained proof modules contain no `sorry`, `admit`, or repository-defined `axiom`. A focused trust report checks named theorem `#print axioms` output against a reviewed allowlist rather than claiming the whole Lean implementation is axiom-free.

**Rationale:** Small explicit proof obligations are reviewable. Lean typechecking is not promoted into proof of the Rust runtime, protocol codec, compiler, OS process, provider, tool, or product.

### 5. Use versioned bounded line-delimited JSON only for interop

**Choice:** Define a closed protocol with handshake, turn request, decision response, error response, and shutdown envelopes. Every envelope binds protocol version, schema identity, profile identity, executable identity, run-local sequence/correlation identity, payload lengths, and decision kind. Numeric frame, text, sequence, diagnostic, startup, response, and shutdown limits come from named typed profile fields.

Line-delimited JSON is selected because Lean and Rust both provide reviewed native JSON representations. It is not a human-authored configuration format. Unknown fields, duplicate keys where detectable, missing fields, invalid Unicode/control data, unsupported versions, oversized frames, stale/duplicate sequence identities, multiple responses, trailing data, and non-canonical decision shapes fail closed.

**Rationale:** This avoids FFI and extra serialization dependencies while preserving a narrow deterministic fixture surface.

### 6. Treat Lean output as an untrusted candidate decision

**Choice:** The Rust bridge converts decoded Lean output into existing neutral/current-shape drafts and reconstructs `ModelRequest`, full structured `ToolRequest`, `ModelResponse`, or `Stop` through current Rust validators. Tool requests include exact schema identity, prepared named arguments, correlation, visible-output budget, mutation targets, required capability labels, and approval posture. Existing runtime capability, catalog, policy, sandbox, and executor checks still run after fabric validation.

Lean-side validation improves local feedback but is never authoritative for Rust DTO validity or effect admission. Unknown decision kinds, legacy raw tool payloads, empty/oversized fields, unsafe labels, unsupported approval posture, hash/profile mismatch, or changed bounds fail before model/tool dispatch.

**Rationale:** Cross-language DTO similarity is not semantic equivalence. Revalidation preserves the established adapter boundary.

### 7. Put process and resource authority in the optional std adapter

**Choice:** `animus-std-lean-loop` owns explicit executable selection, BLAKE3 artifact verification, child process launch, a minimal environment allowlist, working-directory policy, stdin/stdout/stderr pipes, response deadline observation, cancellation, backpressure, exit classification, and bounded cleanup. It does not search `PATH`, fall back to another binary/fabric, restart after an ambiguous turn, open sockets, or infer retry.

Launch policy records whether external sandbox enforcement was required and observed. A source-shape check can establish that the repository-owned Lean program contains no direct ambient effect code outside the shell, but only host enforcement can establish child confinement. Fixture-only execution remains separately labeled when no sandbox evidence exists.

**Rationale:** A child executable is an effectful host integration. Explicit launch and honest enforcement posture avoid treating a reviewed binary hash as a sandbox.

### 8. Keep configuration in Nickel and identities in BLAKE3

**Choice:** Author the Lean-loop profile in typed Nickel. It includes protocol/toolchain cohort, source/module identity, allowed decisions, named payload/resource bounds, launch posture, environment and working-directory policy, sandbox requirement, fallback policy, and receipt redaction. Checked runtime projection is produced before execution; neither Rust nor Lean evaluates Nickel during a turn.

The host computes and verifies domain-separated BLAKE3 identities for the checked profile, Lean source/package manifest, executable artifact, protocol envelope where retained as metadata, and receipts. The executable path, raw source, raw messages, and environment are not embedded in receipts.

The Nix surface uses the Lean 4 package available from the existing locked `nixpkgs` input. Any lockfile change is generated by Nix commands only; `flake.lock` is never edited manually.

**Rationale:** Typed policy catches drift before launch, and exact artifact identity is more meaningful than trusting a binary name or self-reported version.

### 9. Fail closed without implicit fallback

**Choice:** Spawn failure, identity mismatch, handshake failure, malformed/oversized input or output, timeout, cancellation, EOF, broken pipe, stderr overflow, duplicate/stale correlation, extra response, invalid decision, non-zero exit, cleanup failure, or sandbox/profile drift returns a typed fabric failure. The first version does not silently restart, replay a turn, invoke Steel, switch transport, or dispatch an adapter after ambiguity.

**Rationale:** Retrying a fabric turn can duplicate model/tool intent. Explicit higher-level policy may choose a new run after reviewing evidence, but the adapter does not infer safety.

### 10. Keep receipts metadata-only and claims bounded

**Choice:** Receipts contain safe protocol/toolchain/profile/artifact labels and BLAKE3 identities, decision and failure classes, bounded frame/turn/byte counts, launch/sandbox/exit/cleanup status, proof-suite identity, redaction markers, and non-claims. They omit raw summaries, model messages, tool arguments/results, prompts, source, executable paths, environment values, credentials, stderr, provider payloads, hidden reasoning, and unbounded logs.

**Rationale:** Fabric conformance evidence is useful without becoming a transcript or a claim of policy correctness, process confinement, or authoritative task completion.

## Risks / Trade-offs

- A subprocess adds latency and failure modes relative to embedded Steel. The first version favors a persistent bounded child and deterministic failure semantics over FFI.
- The Lean and Rust DTOs can drift. Shared fixture vectors, exact protocol/schema identities, and Rust reconstruction catch known shape drift but do not prove semantic equivalence.
- Lean proofs cover only the modeled kernel and named reference policy. Incorrect specifications, codecs, compiler/runtime bugs, malicious compiled policies, and host integration remain outside the proof.
- The child sees bounded turn content. Binary identity and source review do not prove non-exfiltration; sandbox enforcement and launch policy remain separate evidence.
- JSON duplicate-key and canonicalization behavior needs explicit cross-language tests. Ambiguous encodings are rejected rather than normalized silently.
- A compiled policy is less flexible than Steel. Steel remains canonical for dynamic scripting; Lean is for reviewed static policies and checked local invariants.

## Non-Goals

- Replacing Steel, `animus-runtime`, the coding-session facade, or authoritative completion.
- Loading arbitrary Lean source or tactics at runtime.
- Rust/Lean C FFI, unsafe Rust, dynamic libraries, plugins, or Wasm support in the first profile.
- Implementing ACP, MCP, provider clients, tool executors, transcript persistence, UI, remote transport, or network services in Lean.
- Claiming theorem-to-Rust equivalence, arbitrary policy correctness, child confinement, Lean soundness, provider/tool correctness, production readiness, or whole-agent correctness.
