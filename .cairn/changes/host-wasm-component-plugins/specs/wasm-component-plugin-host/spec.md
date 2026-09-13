# Wasm Component Plugin Host Specification

## ADDED Requirements

### Requirement: Component-plugin contracts are authority-free

r[animus.wasm_plugin.contract] Animus MUST define a focused `no_std + alloc` component-plugin contract for descriptor, component, WIT world, bundle, runtime, resource, input, output, effect-intent, outcome, limit, and metadata-only receipt facts without host effects or product authority.

#### Scenario: Complete facts are admitted structurally

- GIVEN a caller supplies complete bounded in-memory component-plugin facts
- WHEN contract validation runs
- THEN the core MUST return deterministic admission facts or typed blockers
- AND it MUST perform no file, process, network, clock, credential, provider, runtime, terminal, persistence, or product effect.

#### Scenario: Descriptor claims execution authority

- GIVEN a descriptor treats its runtime label, component identity, or WIT compatibility as loading authority
- WHEN contract validation runs
- THEN the contract MUST reject the claim.

### Requirement: Component host is separate and non-default

r[animus.wasm_plugin.host] Animus MUST place Wasmtime, Mantle bundle file verification, deadlines, cancellation, teardown, and runtime observations in a separate non-default standard-library host that is absent from default and reusable-core feature graphs.

#### Scenario: Host is explicitly enabled

- GIVEN an application selects the component-host feature and injects exact dependencies and services
- WHEN composition preflight runs
- THEN the host MAY become available under its declared authority posture.

#### Scenario: Default graph reaches Wasmtime

- GIVEN a default, no-std, compile-only Wasm, descriptor-only, or reusable-core feature graph reaches the component host or Wasmtime
- WHEN dependency-isolation validation runs
- THEN validation MUST fail.

### Requirement: First plugin world has no guest imports

r[animus.wasm_plugin.world] The first component-plugin WIT world MUST accept one bounded request envelope, return one bounded result envelope, expose no WASI or host imports, and bind the exact package, world, method, codec, and schema identities.

#### Scenario: Import-free component matches the world

- GIVEN a Mantle-materialized component implements the exact world with no imports
- WHEN world admission runs
- THEN Animus MAY continue to backend and authority admission.

#### Scenario: Component imports a host interface

- GIVEN a component imports WASI or any host function
- WHEN world or linker admission runs
- THEN the host MUST reject before instantiation or execution.

### Requirement: Invocation binds backend plan and runtime seal

r[animus.wasm_plugin.admission] Every component invocation MUST bind an embedded-component backend plan and one current runtime-authority seal with matching adapter, application-host binding, bundle, WIT, runtime, resource, enforcement, session, and dispatch identities.

#### Scenario: Plan and seal match current observations

- GIVEN the selected plan, seal, bundle, and enforcement observations are current and exact
- WHEN host preflight runs
- THEN Animus MAY instantiate only the declared component method.

#### Scenario: Seal or plan drifts

- GIVEN an identity is missing, stale, widened, narrowed, unavailable, cross-session, or cross-dispatch
- WHEN host preflight runs
- THEN invocation MUST fail before component execution.

### Requirement: Guest effect intents re-enter existing tool stages

r[animus.wasm_plugin.effects] Component output MAY contain bounded typed effect intents, but every intent MUST be revalidated as an untrusted proposal through existing Animus tool-execution stages before any effect can start.

#### Scenario: Declared intent is returned

- GIVEN a component returns a schema-valid intent within bounds
- WHEN result admission runs
- THEN Animus MAY submit the intent to the existing tool-stage planner
- AND component success MUST NOT mean that the effect ran.

#### Scenario: Intent bypasses tool admission

- GIVEN a result requests direct model, tool, file, process, network, credential, clock, terminal, or persistence authority
- WHEN result admission runs
- THEN the result MUST be rejected before any requested effect.

### Requirement: Component failures do not trigger fallback

r[animus.wasm_plugin.fallback] A denial, trap, fuel exhaustion, deadline, cancellation, invalid output, resource failure, or cleanup failure MUST remain on the component attempt and MUST NOT retry through stdio, host process, Steel, native code, or another backend without a new explicit plan and seal.

#### Scenario: Component traps

- GIVEN an admitted component traps during invocation
- WHEN outcome handling runs
- THEN Animus MUST record the component failure and MUST NOT select another backend.

### Requirement: Plugin receipts are safe and bounded

r[animus.wasm_plugin.receipts] Component-plugin receipts MUST bind safe component, bundle, world, runtime, plan, seal, resource, outcome, effect-intent-count, and cleanup identities while excluding payloads, prompts, responses, credentials, host paths, environment values, raw engine errors, and tool bodies.

#### Scenario: Receipt contains a private value

- GIVEN a receipt candidate contains a payload, credential, private path, prompt, response, or raw diagnostic
- WHEN receipt validation runs
- THEN it MUST be rejected or redacted before persistence.

### Requirement: Plugin host has positive and negative fixtures

r[animus.wasm_plugin.fixtures] The focused host rail MUST include positive exact-bundle execution, deterministic replay inputs, valid output, valid effect-intent proposal, and cleanup fixtures plus negative missing dependency, descriptor drift, wrong world, import, stale plan, stale seal, resource, trap, deadline, cancellation, invalid output, effect bypass, receipt leak, fallback, and overclaim fixtures.

#### Scenario: Host is proposed for product use

- GIVEN the host, bundle, runtime, contracts, docs, and fixtures are complete
- WHEN focused Cargo, no-std, Wasm isolation, Octet, Cairn, and Nix validation runs
- THEN every positive fixture MUST pass
- AND every negative fixture MUST fail at its declared boundary.

### Requirement: Component evidence preserves non-claims

r[animus.wasm_plugin.nonclaims] Passing component-plugin evidence MUST NOT claim plugin honesty, component correctness, sandbox completeness, effect success, provider behavior, session correctness, production readiness, or release eligibility.

#### Scenario: Passing fixture is promoted to sandbox proof

- GIVEN a passing fixture is labeled as sandbox containment or plugin correctness
- WHEN non-claim validation runs
- THEN the evidence MUST fail.
