# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Lean 4 loop fabric is optional and isolated

r[agent_harness_runtime.lean4_loop.boundary] Animus MUST provide the Lean 4 agent loop only as a non-default statically compiled implementation of the existing neutral `LoopFabric` contract, while `animus-runtime` remains the owner of turn sequencing, adapter dispatch, events, max-turn enforcement, failures, and completion and default, no-std, Wasm, core, adapter, runtime, and Steel surfaces remain free of Lean toolchain, Lake, protocol transport, process, dynamic-library, and FFI dependencies.

#### Scenario: Lean fabric is explicitly selected

- GIVEN an embedder enables the Lean-loop feature and supplies an admitted exact executable and checked profile
- WHEN the fabric is constructed
- THEN it MAY participate through the existing neutral `LoopFabric` boundary
- AND it MUST NOT replace the runtime or coding-session owners.

#### Scenario: Default or reusable build is selected

- GIVEN the Lean-loop feature is disabled
- WHEN default, no-std, Wasm, core, adapter, runtime, and Steel dependency checks run
- THEN those surfaces MUST remain free of Lean, Lake, stdio protocol, child-process, dynamic-library, and FFI dependencies.

### Requirement: Lean loop logic has a pure functional core

r[agent_harness_runtime.lean4_loop.core] The Lean package MUST define total typed validation and transition functions over explicit profile, fabric-local state, neutral turn, budget, and closed model, current-shape tool, response, and stop decision values, and its pure modules MUST NOT perform `IO`, filesystem, environment, process, network, clock, provider, tool, terminal, persistence, session, or transcript effects.

#### Scenario: Identical valid inputs repeat

- GIVEN identical checked profile, fabric-local state, budget, and normalized turn values
- WHEN the pure Lean step function runs repeatedly
- THEN it MUST return the same typed next state and candidate decision or the same typed error.

#### Scenario: Invalid input reaches the core

- GIVEN a turn, profile, phase, budget, correlation, or decision payload is malformed, stale, unsupported, or outside its named bounds
- WHEN pure validation and transition run
- THEN they MUST return a typed error without a dispatchable model or tool decision.

### Requirement: Lean proof claims are explicit and bounded

r[agent_harness_runtime.lean4_loop.proofs] Maintained Lean proof modules MUST establish the declared closed-decision, deterministic-transition, non-increasing-budget, terminal-absorption, fail-closed-invalid-input, and reference-policy phase invariants for the encoded kernel, MUST contain no `sorry`, `admit`, or repository-defined `axiom`, and MUST check named theorem axiom output against a reviewed allowlist.

#### Scenario: Reviewed theorem suite passes

- GIVEN the maintained kernel, reference policy, theorem list, and reviewed axiom allowlist are unchanged
- WHEN Lean typechecking and the proof-trust rail run
- THEN every named theorem MUST typecheck and report only allowed trust dependencies.

#### Scenario: Proof body or trust closure weakens

- GIVEN a maintained proof introduces `sorry`, `admit`, a repository-defined axiom, an unexpected theorem axiom, or removes a required invariant
- WHEN the proof-trust rail runs
- THEN the rail MUST fail and no checked-kernel claim may be emitted.

### Requirement: Lean protocol is versioned and bounded

r[agent_harness_runtime.lean4_loop.protocol] The Lean executable and Rust host MUST exchange only a closed versioned line-delimited JSON handshake, turn, decision, error, and shutdown protocol whose envelopes bind exact schema, profile, executable, and run-local sequence identities and obey named finite frame, text, diagnostic, startup, response, and shutdown bounds.

#### Scenario: One valid turn exchange completes

- GIVEN handshake identities and bounds match and the host sends one canonical valid turn envelope
- WHEN the Lean shell decodes, validates, reduces, and responds
- THEN it MUST emit exactly one correlated decision or typed error envelope and flush within the admitted bounds.

#### Scenario: Protocol input is ambiguous or stale

- GIVEN a frame is malformed, oversized, unsupported, missing required data, contains unknown or duplicate data, has invalid control text, uses a stale or duplicate sequence, produces multiple responses, or carries trailing data
- WHEN either protocol endpoint validates it
- THEN the exchange MUST fail closed without model or tool dispatch.

### Requirement: Rust revalidates every Lean candidate decision

r[agent_harness_runtime.lean4_loop.bridge] The optional Rust host bridge MUST implement `LoopFabric` by reconstructing every Lean candidate through current Rust `ModelRequest`, full structured `ToolRequest`, `ModelResponse`, or `Stop` validation, and later capability, catalog, policy, sandbox, approval, and executor admission MUST remain mandatory before effects.

#### Scenario: Current-shape tool decision is valid

- GIVEN the Lean response contains an admitted tool name, exact schema identity, prepared named arguments, current correlation, finite visible-output budget, allowed mutation targets, required capabilities, and supported approval posture
- WHEN Rust bridge validation runs
- THEN it MAY return the corresponding neutral runtime tool decision for normal downstream admission.

#### Scenario: Lean-side validation accepted a bad decision

- GIVEN a candidate uses an unknown kind, legacy raw tool payload, empty or oversized field, unsafe label, stale correlation, unsupported approval posture, unknown capability, or mismatched schema/profile identity
- WHEN Rust reconstructs the decision
- THEN reconstruction MUST fail before provider or tool dispatch
- AND Lean-side acceptance MUST NOT override the Rust contract.

### Requirement: Lean process authority is explicit and fail closed

r[agent_harness_runtime.lean4_loop.authority] The optional std adapter MUST own explicit executable selection, BLAKE3 artifact verification, child launch, minimal environment, working-directory policy, bounded pipes and diagnostics, deadline and cancellation observation, exit classification, sandbox-evidence binding, and cleanup, and it MUST NOT use unsafe Rust, ambient `PATH` discovery, sockets, implicit restart, turn replay, transport switching, Steel fallback, or adapter dispatch after an ambiguous failure.

#### Scenario: Admitted child lifecycle succeeds

- GIVEN the executable identity, launch profile, environment, working-directory posture, resource bounds, and required sandbox evidence match
- WHEN the host runs bounded fabric turns and shuts the child down
- THEN it MAY report successful protocol and cleanup outcomes separately from model or tool outcomes.

#### Scenario: Child lifecycle becomes ambiguous

- GIVEN spawn, identity, handshake, timeout, cancellation, EOF, broken pipe, backpressure, stderr, exit, sandbox, or cleanup validation fails
- WHEN the bridge classifies the outcome
- THEN it MUST return a typed fabric failure without restart, replay, fallback, or model/tool dispatch.

### Requirement: Lean loop profiles are Nickel-authored and identity-bound

r[agent_harness_runtime.lean4_loop.profile] Lean-loop profiles MUST be authored with typed Nickel contracts for exact protocol and toolchain cohort, source/module identity, allowed decision classes, named resource bounds, launch/environment/working-directory and sandbox posture, fallback policy, and receipt redaction, and checked runtime projections and domain-separated BLAKE3 source/profile/executable identities MUST be validated before child activation without evaluating Nickel during a turn.

#### Scenario: Checked profile and artifact agree

- GIVEN the generated runtime projection matches the Nickel source and the selected Lean package, toolchain, protocol, and executable BLAKE3 identities match the admitted descriptor
- WHEN activation validation runs
- THEN the child MAY launch under the exact profile.

#### Scenario: Profile or artifact drifts

- GIVEN a profile has unknown cohorts, invalid bounds, ambient discovery, unrestricted environment inheritance, implicit fallback, missing sandbox posture, stale generated data, unsafe labels, weakened redaction, or a mismatched source or executable identity
- WHEN activation validation runs
- THEN activation MUST fail before process launch or fabric influence.

### Requirement: Lean loop receipts are metadata only

r[agent_harness_runtime.lean4_loop.receipts] Lean-loop receipts MUST contain only safe protocol, toolchain, profile, artifact, proof-source, decision, failure, launch, sandbox, exit, and cleanup labels or BLAKE3 identities, bounded traffic counts, redaction markers, and explicit non-claims, and MUST remain non-authoritative for external effects and task completion.

#### Scenario: Safe successful receipt is projected

- GIVEN a bounded child exchange and cleanup completed
- WHEN receipt projection runs
- THEN it MUST bind exact safe identities, counts, statuses, proof-source identity, and non-claims without embedding turn content.

#### Scenario: Receipt leaks or promotes evidence

- GIVEN a receipt contains raw summaries, prompts, model or tool bodies, provider payloads, source, executable paths, environment values, credentials, stderr, hidden reasoning, unbounded diagnostics, or claims authoritative completion or external-effect correctness
- WHEN receipt validation runs
- THEN validation MUST fail.

### Requirement: Lean loop conformance covers success and failure

r[agent_harness_runtime.lean4_loop.verification] Before archive, the change MUST provide paired positive and negative Lean, proof-trust, protocol, Rust bridge, fixture-child, dependency-isolation, source-shape, safe-receipt, and fake model/tool end-to-end tests plus a focused Cairn/Nix rail.

#### Scenario: Lean loop change is ready to archive

- GIVEN maintainers intend to sync and archive the change
- WHEN focused and relevant existing checks run
- THEN Lake build/tests, named theorem and trust checks, Nickel freshness, protocol fixtures, Rust revalidation and child-failure cases, default/no-std/Wasm isolation, safe receipts, fake-adapter end-to-end behavior, Cairn validation and gates, and relevant Nix checks MUST pass
- AND unavailable sandbox or external-policy evidence MUST be recorded as a bounded blocker rather than simulated success.
