# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Sessions declare one runtime authority profile

r[agent_harness_runtime.session_authority_profile.contract] Animus MUST represent each resumable session lineage with a typed metadata-only runtime authority profile containing safe identities for authorization and policy posture, permission mode, requested sandbox descriptor, model runtime profile, effective tool catalog and capability ceilings, admitted extension posture, and context/retrieval policy.

#### Scenario: New resumable session is prepared

- GIVEN a host has already admitted every required policy, permission, sandbox, model, tool, extension, and context input
- WHEN the session authority profile is constructed
- THEN it MUST contain the validated safe identity and class for every required component
- AND it MUST NOT contain credential values, endpoint values, environment values, prompts, tool payloads, extension bodies, or physical host paths.

#### Scenario: Required component is missing

- GIVEN the selected session profile requires a sandbox, tool catalog, model profile, or context policy identity
- WHEN profile validation receives no corresponding admitted component
- THEN validation MUST return a typed missing-component failure
- AND no resumable profile may be inferred from ambient or prior-session state.

### Requirement: Authority profile identity is deterministic

r[agent_harness_runtime.session_authority_profile.identity] A session runtime authority profile and each durable profile binding MUST use domain-separated BLAKE3 over canonical validated metadata and MUST NOT depend on clocks, randomness, locale, filesystem order, process-global state, raw secrets, or physical host paths.

#### Scenario: Identical profile inputs repeat

- GIVEN identical validated component identities and safe classes in different input orders
- WHEN profile identity construction runs in separate processes
- THEN it MUST produce the same canonical component order and BLAKE3 profile identity
- AND repeated encoding MUST produce identical canonical bytes.

#### Scenario: Unsafe material enters identity input

- GIVEN an identity draft contains a credential, environment value, prompt body, raw tool schema, extension body, provider payload, or absolute host path
- WHEN profile identity validation runs
- THEN it MUST reject the draft without echoing the unsafe value
- AND no accepted profile or receipt identity may be emitted.

### Requirement: Authority profiles are committed before dependent effects

r[agent_harness_runtime.session_authority_profile.durable_binding] The durable session host MUST commit the initial authority profile before the first provider or tool effect and MUST represent every accepted profile change as an explicit committed lineage transition without rewriting prior bindings.

#### Scenario: First prompt is admitted

- GIVEN a new durable session has a validated authority profile but no committed profile binding
- WHEN the coding-session facade prepares the first prompt
- THEN its first dependent persistence work MUST commit the profile binding before provider contact
- AND provider and tool effects MUST remain blocked until that commit succeeds.

#### Scenario: Explicit profile rebind is authorized

- GIVEN resume found a changed profile and a separate authority decision admits a rebind
- WHEN the host applies the transition
- THEN it MUST commit a new branch or successor-lineage profile binding linked to the prior head
- AND it MUST preserve the prior committed profile record unchanged.

### Requirement: Resume fails closed on profile drift

r[agent_harness_runtime.session_authority_profile.resume_admission] Resume MUST compare the committed requested profile with current admitted profile inputs before continuation and MUST block automatic continuation on widening, narrowing, substitution, missing components, malformed identities, or legacy unbound state.

#### Scenario: Current profile matches

- GIVEN the committed profile and current admitted requested profile have identical validated component and aggregate identities
- WHEN resume admission runs
- THEN it MAY proceed to current host capability and enforcement checks
- AND the identity match MUST NOT itself authorize an effect.

#### Scenario: One component changed

- GIVEN a permission, sandbox, model, tool, extension, context, or policy component differs from the committed profile
- WHEN resume admission runs
- THEN automatic continuation MUST be blocked with the changed component class
- AND the caller MUST explicitly abandon, select an enforceable prior profile, fork, or request reauthorization and durable rebind.

### Requirement: Requested profile and realized enforcement remain separate

r[agent_harness_runtime.session_authority_profile.realized_enforcement] A matching requested authority profile MUST NOT replace current authorization, capability probes, adapter-cohort checks, tool and provider readiness, credential admission, sandbox preflight, or per-run enforcement observations.

#### Scenario: Profile matches but sandbox capability disappeared

- GIVEN the committed and current requested profiles match
- AND the current host cannot prove one required sandbox mechanism
- WHEN resumed-run preflight executes
- THEN the run MUST fail closed before child process, provider, or tool dispatch
- AND the outcome MUST distinguish profile identity continuity from enforcement unavailability.

#### Scenario: Adapter cohort changed

- GIVEN the requested profile is unchanged but the selected host adapter or enforcement cohort identity changed
- WHEN current host admission runs
- THEN the host MUST require a fresh compatible authorization and capability observation
- AND it MUST NOT reuse a prior run's realized-enforcement success.

### Requirement: Authority-profile evidence is metadata only

r[agent_harness_runtime.session_authority_profile.receipts] Session authority-profile events and receipts MUST contain only safe status and changed-component classes, session/branch/head/profile identities, authorization and enforcement observation identities, bounded counts, redaction markers, non-claims, and a domain-separated BLAKE3 receipt identity.

#### Scenario: Resume is blocked by drift

- GIVEN profile comparison detects one or more changed component classes
- WHEN a blocked receipt is projected
- THEN it MUST identify the safe classes and relevant BLAKE3 identities needed for review
- AND it MUST omit raw policy, prompts, schemas, extension bodies, credentials, environment values, paths, provider payloads, and unbounded diagnostics.

### Requirement: Authority-profile behavior has conformance evidence

r[agent_harness_runtime.session_authority_profile.verification] The session authority-profile change MUST include positive and negative tests for canonical identity, durable ordering, exact-match resume, explicit rebind, drift classification, legacy sessions, current host re-probes, fail-closed effects, and receipt redaction across fake and local durable-store scenarios.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to sync and archive the authority-profile change
- WHEN closeout validation runs
- THEN focused pure-core, store, coding-session, model-profile, tool, extension, context, sandbox, receipt, Cairn, and relevant Nix checks MUST pass
- AND unavailable optional host checks MUST retain their blocker and next-best validation.
