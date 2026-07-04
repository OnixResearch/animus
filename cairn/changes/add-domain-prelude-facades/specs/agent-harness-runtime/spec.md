# agent-harness-runtime Change Spec: Domain prelude facades

## Requirements

### Requirement: Domain prelude modules

r[agent_harness_runtime.composability.domain_preludes] Animus SHOULD provide curated domain prelude modules that expose small, documented composition surfaces for runtime loop execution, tool contracts, headless protocol, provenance, worker-team delegation, policy/middleware, and status selection.

#### Scenario: Prelude imports stay focused

r[agent_harness_runtime.composability.domain_preludes.compile]
- GIVEN an embedder imports one domain prelude
- WHEN compile or import-surface tests construct representative DTOs and call pure helpers
- THEN the prelude exposes the expected domain contracts
- AND unrelated shell, terminal, provider, process, filesystem, or persistence types are not required.

### Requirement: Compatibility facade remains stable

r[agent_harness_runtime.composability.compatibility_facade] Existing flat root and crate-level compatibility exports MUST remain available or receive explicit migration documentation when domain preludes are introduced.

#### Scenario: Existing imports are not silently removed

r[agent_harness_runtime.composability.compatibility_facade.stable]
- GIVEN existing consumers import DTOs or helpers from the broad compatibility facade
- WHEN domain preludes are added
- THEN compatibility imports continue to compile or emit explicit deprecation/migration guidance
- AND no public behavior changes solely because a prelude exists.

### Requirement: Prelude inventory classification

r[agent_harness_runtime.composability.prelude_inventory] Each domain prelude MUST be recorded in the runtime facade inventory with owning crate, authority class, side-effect class, and any feature requirements.

#### Scenario: Reusable prelude rejects authority leaks

r[agent_harness_runtime.composability.prelude_inventory.no_leak]
- GIVEN a reusable-contract prelude is inventoried
- WHEN facade inventory validation runs
- THEN shell, provider, filesystem, process, persistence, network, credential, and terminal imports are rejected
- AND host-injection or shell-adapter preludes are explicitly labeled rather than hidden inside reusable domains.

### Requirement: Domain prelude verification

r[agent_harness_runtime.composability.domain_preludes.verification] The prelude change MUST include focused positive and negative tests for import surfaces, compatibility exports, facade inventory classification, authority-leak fixtures, workspace checks, and Cairn gates before archive.

#### Scenario: Closeout evidence covers prelude composition

r[agent_harness_runtime.composability.domain_preludes.verification.closeout]
- GIVEN maintainers intend to archive the domain prelude change
- WHEN closeout validation runs
- THEN evidence names prelude compile, compatibility facade, inventory, leak-fixture, workspace, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
