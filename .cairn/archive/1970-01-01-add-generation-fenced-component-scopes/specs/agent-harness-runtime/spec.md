# Agent Harness Runtime

## ADDED Requirements

### Requirement: Component registrations are explicit and generation-fenced

r[agent_harness_runtime.component_scope.registration] Animus MUST represent a component scope, registration, lease, parent, dependency set, visibility, and lifecycle state with explicit typed values and one exact scope generation.

#### Scenario: Component registers in an open scope

- GIVEN an open scope, a catalog-declared component, an admitted dependency set, and the current generation
- WHEN pure registration validation runs
- THEN it MUST return one exact generation-bound lease
- AND the lease MUST grant no effect authority.

#### Scenario: Registration targets a stale generation

- GIVEN a registration cites an older scope generation
- WHEN validation runs
- THEN Animus MUST reject it
- AND it MUST NOT make the component visible.

### Requirement: Component resolution uses an explicit scope

r[agent_harness_runtime.component_scope.resolution] Component resolution MUST receive an explicit scope or lease and MUST reject ambient, stale, closing, unauthorized, undeclared, and out-of-scope lookup.

#### Scenario: Exact registered component resolves

- GIVEN a caller presents a current lease for one visible declared component
- WHEN resolution runs
- THEN it MAY return the matching safe binding identity
- AND every later effect MUST still pass its existing authority gates.

#### Scenario: Lookup occurs after close starts

- GIVEN the scope has entered `closing`
- WHEN a caller requests a new component resolution
- THEN resolution MUST fail closed
- AND no parent, global, or name-only fallback may run.

### Requirement: Scope teardown is ordered and explicit

r[agent_harness_runtime.component_scope.teardown] Scope teardown MUST revoke new resolution, close new delivery, request cancellation, require generation-bound quiescence observations, release owned children and resources in reverse dependency order, and commit an explicit terminal outcome.

#### Scenario: Scope closes cleanly

- GIVEN all owned work acknowledges cancellation and every current member reaches quiescence
- WHEN the host supplies exact observations to the pure reducer
- THEN the reducer MUST request reverse-order releases
- AND the scope MAY become `closed` only after every required release observation matches.

#### Scenario: Cleanup remains unresolved

- GIVEN a member fails to reach quiescence or return an exact release observation within host policy
- WHEN teardown reaches its terminal decision
- THEN the scope MUST enter `failed_cleanup` or remain explicitly incomplete
- AND it MUST NOT report clean closure.

### Requirement: Concurrent close requests converge

r[agent_harness_runtime.component_scope.concurrency] The first admitted close command MUST fix one close generation and plan identity. Repeated or concurrent close requests MUST return the same in-progress or terminal identity and MUST NOT restart teardown or restore visibility.

#### Scenario: Two clients request close

- GIVEN one current scope receives two valid close requests
- WHEN the reducer admits them in either order
- THEN both callers MUST observe one close plan identity
- AND only one logical teardown sequence may advance.

### Requirement: Scope evidence is metadata-only

r[agent_harness_runtime.component_scope.evidence] Scope lifecycle receipts MUST contain only safe scope, generation, component, transition, count, cleanup-class, issue, and non-claim fields.

#### Scenario: Failed cleanup receipt is projected

- GIVEN teardown ends with unresolved members
- WHEN receipt projection runs
- THEN the receipt MUST identify bounded safe issue classes and counts
- AND it MUST omit handles, commands, paths, payloads, prompts, credentials, and raw host errors.

### Requirement: Component scopes have positive and negative verification

r[agent_harness_runtime.component_scope.verification] The focused rail MUST cover registration, resolution, parent and child visibility, cancellation, quiescence, reverse release, concurrent close, daemon mapping, stale generations, late delivery, unresolved cleanup, evidence safety, and authority separation.

#### Scenario: Component-scope rail passes

- GIVEN all required pure-core, fake-shell, daemon-mapping, positive, and negative fixtures are present
- WHEN the focused rail runs
- THEN valid lifecycle sequences MUST be deterministic
- AND invalid or incomplete sequences MUST fail before false visibility, authority, or cleanup claims.
