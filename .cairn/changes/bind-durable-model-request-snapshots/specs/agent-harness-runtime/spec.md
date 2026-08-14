# Agent Harness Runtime

## ADDED Requirements

### Requirement: Model-request snapshots preserve provider-neutral request meaning

r[agent_harness_runtime.model_request_snapshot.projection] Animus MUST validate one versioned provider-neutral private snapshot that reconstructs the complete ordered model-visible request and binds its selected model route, invocation plan, request-context plan, tool-schema set, output controls, budgets, runtime authority seal, and provider-adapter cohort.

#### Scenario: Complete request snapshot is admitted

- GIVEN all ordered model-visible values and composition identities are final
- WHEN pure snapshot validation runs
- THEN it MUST return one canonical BLAKE3 snapshot identity
- AND credentials, transport headers, provider SDK objects, and hidden provider defaults MUST remain outside the snapshot.

#### Scenario: Tool schema changes after composition

- GIVEN a snapshot draft cites a tool-schema set that does not match the final invocation plan
- WHEN validation runs
- THEN Animus MUST reject the draft before durable commit or provider contact.

### Requirement: Request snapshots commit before provider dispatch

r[agent_harness_runtime.model_request_snapshot.durability] The coding-session host MUST commit the complete private snapshot through the existing durable session boundary before it requests an initial provider effect.

#### Scenario: Durable snapshot commit succeeds

- GIVEN a validated snapshot matches the active session leaf, turn generation, and request sequence
- WHEN the durable store returns an exact successful commit observation
- THEN the coding-session state MAY advance to provider-dispatch admission
- AND public transcript projections MUST omit the private request body.

#### Scenario: Snapshot commit is partial

- GIVEN the store reports an incomplete, stale, mismatched, or unsynchronized snapshot write
- WHEN the host evaluates provider readiness
- THEN it MUST block provider contact
- AND it MUST preserve a typed durable-state issue.

### Requirement: Provider dispatch binds the committed snapshot

r[agent_harness_runtime.model_request_snapshot.dispatch] Every initial provider dispatch and admitted same-request retry MUST cite the exact committed snapshot identity, adapter cohort, active leaf, request sequence, and current authority facts.

#### Scenario: Same-request retry keeps the snapshot

- GIVEN the existing provider-retry owner admits a retry with no request, context, profile, tool, adapter, or authority drift
- WHEN the retry dispatch is prepared
- THEN it MUST cite the original committed snapshot identity
- AND it MUST still pass every current retry and enforcement gate.

#### Scenario: Request composition drifts

- GIVEN a context lifetime expires or a profile, tool schema, output control, adapter cohort, or authority fact changes
- WHEN another provider attempt is requested
- THEN Animus MUST require a newly composed request and newly committed snapshot
- AND it MUST reject reuse of the prior snapshot.

### Requirement: Unobserved provider attempts remain uncertain

r[agent_harness_runtime.model_request_snapshot.recovery] A committed snapshot without a correlated terminal provider observation MUST recover as an explicit uncertain attempt and MUST NOT imply delivery, failure, cancellation, billing, or safe replay.

#### Scenario: Process exits after request commit

- GIVEN a request snapshot committed before process exit and no terminal provider observation is durable
- WHEN session recovery runs
- THEN the attempt MUST become `committed_unobserved`
- AND no provider replay may occur unless the existing provider-retry contract independently admits the exact request.

### Requirement: Public snapshot evidence is metadata-only

r[agent_harness_runtime.model_request_snapshot.evidence] Public events, inspection values, reports, and receipts MUST contain only safe identities, classes, counts, statuses, omission markers, and non-claims. They MUST NOT contain raw prompts, tool schemas, contribution bodies, credentials, provider payloads, or private paths.

#### Scenario: Public receipt is projected

- GIVEN a private snapshot committed successfully
- WHEN public receipt projection runs
- THEN the receipt MUST bind the snapshot identity and safe composition facts
- AND the exact model-visible request body MUST remain private.

### Requirement: Model-request snapshots have positive and negative verification

r[agent_harness_runtime.model_request_snapshot.verification] The focused rail MUST cover exact reconstruction, commit ordering, dispatch binding, same-request retry, recovery uncertainty, canonical identity, redaction, and fail-closed drift cases through production owners.

#### Scenario: Snapshot rail passes

- GIVEN all required core, durable-store, coding-session, provider-retry, authority, and evidence fixtures are present
- WHEN the focused rail runs
- THEN admitted cases MUST produce exact deterministic identities and ordering
- AND denied cases MUST stop before provider effects or unsafe evidence.
