# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Typed usage-record admission and attribution

r[agent_harness_runtime.usage_accounting.records] Animus MUST admit typed usage records and bind each record to the agent and session that produced it before any accounting projection, using host-supplied record projections and safe model catalog metadata.

#### Scenario: Record without attribution is rejected

- GIVEN a usage record cannot be bound to a session or agent
- WHEN accounting admission runs
- THEN admission rejects the record with a typed attribution diagnostic
- AND no unattributed record is priced into a session or agent summary.

### Requirement: Pricing against the serving model

r[agent_harness_runtime.usage_accounting.pricing] Animus MUST price each accepted usage record against the model that served it using registry price metadata, MUST use a documented fallback price when registry pricing is absent, and MUST fail a summary entry when a required price is missing.

#### Scenario: Missing required price fails the summary

- GIVEN a summary entry requires pricing metadata that is absent
- WHEN the accounting projection runs
- THEN the entry fails with a missing-price diagnostic
- AND no entry is priced from an undeclared assumption.

### Requirement: Approximate per-agent subtree projection

r[agent_harness_runtime.usage_accounting.subtree] Animus MUST project an approximate per-agent subtree figure that follows only authenticated same-session creator relationships, retains completed descendants, and resets with the active session.

#### Scenario: Subtree resets with the session

- GIVEN a new active session begins on the same agent
- WHEN the accounting projection runs for that agent
- THEN the subtree figure starts from the new session
- AND usage from completed prior sessions is not carried into the new subtree.

### Requirement: Explicit non-authoritative label

r[agent_harness_runtime.usage_accounting.non_authoritative] Animus MUST mark every accounting projection as a non-authoritative estimate rather than provider billing reconciliation.

#### Scenario: Projection reports its estimate status

- GIVEN an accounting projection is produced
- WHEN a consumer reads the projection header
- THEN the projection carries an explicit estimate marker
- AND no projection claims provider billing reconciliation authority.

### Requirement: Accounting validation fails closed

r[agent_harness_runtime.usage_accounting.fail_closed] Accounting validation MUST reject malformed usage records, unknown models, missing required prices, negative totals, overflow, and duplicate agent summaries with bounded safe error classes.

#### Scenario: Overflow is rejected

- GIVEN an accounting projection totals overflow the bounded counter
- WHEN validation runs
- THEN validation rejects the projection with an overflow diagnostic
- AND no truncated total is promoted as valid.

### Requirement: Metadata-only accounting receipts

r[agent_harness_runtime.usage_accounting.receipts] Accounting receipts MUST include schema version, record counts, model identities, price units, subtree identities, and BLAKE3 projection identity, and MUST NOT include raw usage payloads, provider bodies, credentials, or private paths.

#### Scenario: Receipt rejects raw usage payload

- GIVEN an accounting receipt fixture includes a raw usage payload or provider body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Accounting verification evidence

r[agent_harness_runtime.usage_accounting.verification] The adaptation MUST include focused positive and negative tests for attribution, pricing, fallback pricing, subtree projection, session reset, fail-closed validation, receipt leak rejection, and feature isolation.

#### Scenario: Closeout evidence covers accounting fixtures

- GIVEN maintainers intend to archive the accounting adaptation
- WHEN closeout validation runs
- THEN evidence names focused no-std, Wasm, workspace, receipt, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
