# Agent harness runtime delta

## ADDED Requirements

### Requirement: Published holder observation cohort
r[agent_harness_runtime.holder_observation.cohort]
Animus MUST consume the exact reviewed published Basalt revision and retain minimal-profile and generic-contract isolation.

#### Scenario: Exact owner cohort
GIVEN published Basalt containing the qualified UCAN recheck
WHEN the adapter dependency is admitted
THEN manifests, locks, runtime identities, config projections, and verification rails agree on the cohort.

### Requirement: Non-consuming scoped observations
r[agent_harness_runtime.holder_observation.boundary]
A consumer SHALL recheck only through the original borrowed holder admission and explicit verification inputs. Rechecks MUST NOT create dispatch authority or consume another replay admission.

#### Scenario: Repeated observation
GIVEN a committed direct or delegated scoped callback
WHEN the same holder authorization is rechecked twice with valid current facts
THEN both observations succeed without another replay commit or decision commit.

#### Scenario: Denied current facts
GIVEN a previously admitted holder
WHEN current verification receives expired, revoked, or substituted token facts
THEN observation fails without another protected effect or replay admission.

#### Scenario: Callback lifetime
GIVEN the higher-ranked scoped dispatch callback
WHEN a consumer attempts to return the borrowed observation outside that callback
THEN compilation fails.

### Requirement: Scoped evidence
r[agent_harness_runtime.holder_observation.evidence]
Evidence MUST bind the qualified source and distinguish holder observations from current policy, service authenticity, resource custody, and release permission.

#### Scenario: Owner qualification
GIVEN passing positive and negative controls and connected authority checks
WHEN evidence is recorded
THEN it names the exact cohort and retains the observation non-claims.
