# Agent Harness Runtime

## ADDED Requirements

### Requirement: The component catalog records complete composition seams

r[agent_harness_runtime.composition_activation_graph.catalog] The Nickel component catalog MUST record each application-owned port, owner, provider, direct consumer, scope class, multiplicity, lifecycle owner, activation posture, feature gate, authority class, event route, and focused check required by a maintained composition.

#### Scenario: Required capability seam is complete

- GIVEN a required port has one declared owner, admitted provider set, direct consumers, lifecycle owner, and focused checks
- WHEN catalog validation runs
- THEN the seam MUST be admitted as complete
- AND the catalog MUST grant no runtime authority.

#### Scenario: Required provider is missing

- GIVEN a required port has no admitted provider
- WHEN catalog validation runs
- THEN validation MUST reject the catalog
- AND no composition root may treat an undeclared or ambient provider as a fallback.

### Requirement: Composition maps are generated from one source

r[agent_harness_runtime.composition_activation_graph.projections] Animus MUST generate component dependency, capability-seam, and event-ownership maps from the validated Nickel catalog and MUST reject generated projection drift.

#### Scenario: Maintainer inspects event ownership

- GIVEN the catalog declares one event producer, its consumers, delivery class, durability class, and lifecycle owner
- WHEN documentation generation runs
- THEN the generated event map MUST show those exact safe facts
- AND prose MUST NOT become a second source of truth.

### Requirement: Selected compositions pass pure activation preflight

r[agent_harness_runtime.composition_activation_graph.preflight] Each maintained composition root MUST run a pure preflight over selected components, feature and authority facts, the validated graph, and current host-supplied readiness observations before external effects.

#### Scenario: Required composition is ready

- GIVEN every required component and seam is selected and every required readiness observation is current and exact
- WHEN preflight runs
- THEN it MUST return `ready` with one deterministic graph identity
- AND it MUST request no host effect.

#### Scenario: Optional component is unavailable

- GIVEN every required seam is ready and one declared optional provider is unavailable
- WHEN preflight runs
- THEN it MAY return `degraded_optional` with an explicit issue
- AND it MUST NOT substitute another provider.

### Requirement: Required activation failures stop startup

r[agent_harness_runtime.composition_activation_graph.fail_loud] Missing required providers, undeclared consumers, invalid multiplicity, authority or feature mismatch, missing lifecycle owners, stale readiness, and required event-route gaps MUST block startup before store, credential, provider, process, filesystem, network, terminal, daemon, or ready-endpoint effects.

#### Scenario: Required readiness is stale

- GIVEN one required provider reports readiness for an older activation generation
- WHEN the composition root runs preflight
- THEN startup MUST return a typed blocked outcome
- AND the shell MUST perform no protected startup effect.

### Requirement: Activation inspection is bounded and read-only

r[agent_harness_runtime.composition_activation_graph.inspection] Animus MUST provide a bounded metadata-only activation snapshot with component identities, safe classes, generation, statuses, issue codes, counts, graph identity, and non-claims, without service handles or mutation commands.

#### Scenario: Activation snapshot is projected

- GIVEN preflight returns a ready, degraded, or blocked decision
- WHEN safe inspection projection runs
- THEN the snapshot MUST preserve exact status and issue classes
- AND it MUST omit raw configuration, endpoints, credentials, private paths, prompts, payloads, and executable controls.

### Requirement: Activation graph verification covers failure paths

r[agent_harness_runtime.composition_activation_graph.verification] The focused rail MUST cover catalog validity, generated map freshness, required and optional preflight, startup effect ordering, event routes, safe inspection, and ambient-fallback denial.

#### Scenario: Activation graph rail passes

- GIVEN positive and negative catalog and composition fixtures are present
- WHEN the focused rail runs
- THEN valid closure MUST pass deterministically
- AND missing, stale, unsafe, undeclared, or effect-before-preflight cases MUST fail closed.
