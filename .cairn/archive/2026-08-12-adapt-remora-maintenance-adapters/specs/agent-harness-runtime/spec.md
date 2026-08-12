## ADDED Requirements

### Requirement: Change-request adapters form a focused authority-free family

r[agent_harness_runtime.remora_adapters.change_request] Animus MUST expose forge-neutral change-request and code-review meaning through a focused non-default `#![no_std]` adapter-family crate, and the crate MUST NOT open networks, resolve credentials, query VCS state, read diffs, call models, publish comments, mutate provider state, or persist review data.

#### Scenario: Host selects change-request contracts

r[agent_harness_runtime.remora_adapters.change_request.selected]
- GIVEN a host selects the change-request adapter feature and supplies explicit services
- WHEN request admission runs
- THEN only typed DTOs, pure reducers, and implementation-free ports enter the reusable dependency graph
- AND every fetch, analysis, publication, credential, network, VCS, and persistence effect remains host-owned.

#### Scenario: Inline anchor is not observed

r[agent_harness_runtime.remora_adapters.change_request.anchor_missing]
- GIVEN a review finding requests inline publication without a matching observed commit, path, side, and line anchor
- WHEN review planning runs
- THEN inline publication is denied or converted to an explicit summary fallback according to policy
- AND the adapter does not guess a provider position.

### Requirement: Dependency maintenance forms a separate focused family

r[agent_harness_runtime.remora_adapters.maintenance] Animus MUST expose dependency inventory, candidate, advisory, policy, grouping, update-plan, and validation-profile meaning through a focused non-default `#![no_std]` adapter-family crate that depends on change-request contracts only where a prepared change-request draft is required.

#### Scenario: Routine updates form compatible bounded groups

r[agent_harness_runtime.remora_adapters.maintenance.grouping]
- GIVEN routine update candidates share one ecosystem and manifest identity and stay within the configured batch bound
- WHEN update planning runs
- THEN the planner can place them in one deterministic group
- AND the group requests only typed host effects and trusted validation profiles.

#### Scenario: Security or major update is present

r[agent_harness_runtime.remora_adapters.maintenance.isolation]
- GIVEN a candidate is a security or major update
- WHEN update planning runs
- THEN the planner isolates the candidate from routine updates
- AND it records the required review or security posture without claiming advisory freshness or update safety.

#### Scenario: Candidates are not compatible

r[agent_harness_runtime.remora_adapters.maintenance.incompatible]
- GIVEN routine candidates use different ecosystems or manifest identities
- WHEN grouping runs
- THEN the planner creates separate groups or returns a typed bound error
- AND it does not combine incompatible package-manager effects.

### Requirement: Maintenance effects use separate least-authority ports

r[agent_harness_runtime.remora_adapters.ports] Dependency inventory, candidate discovery, advisory observation, workspace update, validation execution, change-request fetch, review analysis, change-request publication, and review publication MUST use separate ports with explicit capability, availability, implementation, and failure descriptors.

#### Scenario: Host implements only inventory and candidate discovery

r[agent_harness_runtime.remora_adapters.ports.subset]
- GIVEN a host implements dependency inventory and candidate discovery but no workspace, validation, review, or publication services
- WHEN the host requests an observation-only maintenance operation
- THEN the implemented ports can return bounded observations
- AND no mutation, validation, review, or forge authority is implied.

#### Scenario: Required service is absent

r[agent_harness_runtime.remora_adapters.ports.absent]
- GIVEN an admitted plan requires a service that is absent, denied, stale, incompatible, or unsupported
- WHEN service admission runs
- THEN it returns a typed blocked outcome before dispatch
- AND it does not discover or construct an ambient fallback.

### Requirement: Maintenance requests and observations are exactly bound

r[agent_harness_runtime.remora_adapters.binding] Every maintenance and change-request request or observation MUST bind the applicable repository, policy, plan, generation, implementation, correlation, branch, commit, change-request, manifest, candidate, validation-profile, and evidence identities, and reusable logic MUST reject stale, mismatched, cross-repository, cross-policy, cross-plan, or incomplete observations.

#### Scenario: Service returns matching evidence

r[agent_harness_runtime.remora_adapters.binding.match]
- GIVEN a service returns bounded evidence with every identity required by its admitted request
- WHEN observation validation runs
- THEN the evidence is eligible for workflow reduction
- AND the observation retains exact implementation and correlation attribution.

#### Scenario: Observation is replayed across repositories

r[agent_harness_runtime.remora_adapters.binding.replay]
- GIVEN an observation matches a safe label but uses a different repository, policy, plan, generation, branch, commit, or implementation identity
- WHEN reusable logic validates it
- THEN validation returns a typed stale or mismatch outcome
- AND no update, validation, review, publication, completion, or receipt state accepts the observation.

### Requirement: Remora-derived adapters compose with existing Animus owners

r[agent_harness_runtime.remora_adapters.composition] The maintenance families MUST compose with existing Animus schedule, queue, orchestration, semantic-review, model, quality, model-evidence, authority, validation-planning, workspace, VCS-snapshot, and safe-receipt owners without duplicating their generic semantics or granting their effects.

#### Scenario: Scheduled maintenance workflow is assembled

r[agent_harness_runtime.remora_adapters.composition.schedule]
- GIVEN a host combines a validated scheduled workflow, orchestration service, maintenance plan, authority decision, and explicit service implementations
- WHEN the composition root admits one run
- THEN each family retains its own policy, effect, and evidence ownership
- AND the maintenance planner does not start timers, enqueue work, execute commands, or publish forge changes.

#### Scenario: Radar evidence is supplied

r[agent_harness_runtime.remora_adapters.composition.radar]
- GIVEN a host supplies bounded clone, quality, or external-analysis observations through existing Animus codegraph, quality, model-evidence, or document contracts
- WHEN maintenance policy consumes the safe summaries
- THEN the observations remain advisory inputs with their original owner and evidence scope
- AND the maintenance adapter does not import scanners, raw reports, dashboards, or gate authority.

### Requirement: Remora adaptation records exact owner-authorized provenance

r[agent_harness_runtime.remora_adapters.provenance] The adapter families MUST record `git@github.com:OnixResearch/remora.git`, revision `e3c857c857f4aa170d8a5b6286aad5855db541be`, the declared `MIT OR Apache-2.0` license, the bounded imported scope, and the user's owner-grant request without claiming a public-license change or third-party permission.

#### Scenario: Source cohort is reviewed

r[agent_harness_runtime.remora_adapters.provenance.cohort]
- GIVEN maintainers inspect the adapted contracts and focused source-cohort evidence
- WHEN provenance validation runs
- THEN repository, revision, scope, declared license, and owner-grant basis match the reviewed values
- AND concrete Remora shell code is absent from reusable Animus crates.

### Requirement: Maintenance evidence is bounded and secret-free

r[agent_harness_runtime.remora_adapters.evidence] Maintenance and change-request receipts MUST bind only safe operation, service, capability, availability, policy, status, count, redaction, and BLAKE3 identity metadata and MUST reject dependency names, versions, raw manifests, diffs, comments, commands, logs, credentials, environment values, provider payloads, physical paths, and unsupported effect or readiness claims.

#### Scenario: Adapter call has safe evidence

r[agent_harness_runtime.remora_adapters.evidence.safe]
- GIVEN a service call has validated request and observation identities plus bounded status metadata
- WHEN receipt projection runs
- THEN it emits a metadata-only receipt
- AND the receipt states that host behavior and external effects remain outside the adapter claim.

#### Scenario: Adapter receipt leaks or overclaims

r[agent_harness_runtime.remora_adapters.evidence.reject]
- GIVEN a receipt contains prohibited content or claims registry freshness, advisory completeness, update correctness, VCS isolation, check sufficiency, forge delivery, review quality, merge safety, production readiness, or release eligibility
- WHEN receipt validation runs
- THEN validation rejects the receipt
- AND no success identity is emitted.

### Requirement: Remora-derived families have positive and negative verification

r[agent_harness_runtime.remora_adapters.verification] Remora adapter adoption MUST include positive and negative tests for independent port subsets, deterministic grouping, isolation, exact binding, review anchor handling, CI-failure triage, failure reduction, safe receipts, no-std, Wasm, dependency isolation, facade isolation, source provenance, and composition boundaries.

#### Scenario: Adapter families are ready to archive

r[agent_harness_runtime.remora_adapters.verification.closeout]
- GIVEN maintainers intend to sync and archive the Remora adapter change
- WHEN closeout checks run
- THEN focused Rust, formatting, Clippy, no-std, Wasm, facade, component, provenance, receipt, Cairn, and Nix checks pass or record an exact blocker
- AND no registry, advisory database, filesystem, VCS, forge, scheduler, process, credential, persistence, merge, production, or release behavior is claimed.
