# agent-harness-runtime Change Spec: Clankers confirmation broker kit adaptation

## Requirements

### Requirement: Typed confirmation requests

r[agent_harness_runtime.clankers_confirmation_broker.requests] Animus confirmation requests MUST be typed protected-action envelopes with request ID, action kind, resource, ability, safe summary, policy hash, session or run identity, required confirmation level, optional expiry, and redaction class.

#### Scenario: Protected mutation requests confirmation

r[agent_harness_runtime.clankers_confirmation_broker.requests.mutation]
- GIVEN a protected mutating tool action requires confirmation by policy
- WHEN runtime prepares the action
- THEN it emits a typed confirmation request with action/resource/ability identity
- AND the raw tool payload is not embedded in the request receipt.

### Requirement: Default deny confirmation behavior

r[agent_harness_runtime.clankers_confirmation_broker.default_deny] Confirmation evaluation MUST fail closed when the broker is absent, unavailable, timed out, cancelled, stale, malformed, or returns no decision.

#### Scenario: Missing broker denies protected action

r[agent_harness_runtime.clankers_confirmation_broker.default_deny.missing]
- GIVEN a protected action requires confirmation
- AND no confirmation broker is injected
- WHEN confirmation evaluation runs
- THEN the protected action is denied
- AND no fallback approval or action dispatch occurs.

### Requirement: Decision binding and freshness

r[agent_harness_runtime.clankers_confirmation_broker.binding] Confirmation decisions MUST bind to the pending request ID, action kind, resource, ability, policy hash, and expiry, and MUST be rejected when stale or mismatched.

#### Scenario: Mismatched approval rejected

r[agent_harness_runtime.clankers_confirmation_broker.binding.mismatch]
- GIVEN a broker returns an approval for a different request ID or resource
- WHEN runtime validates the decision
- THEN the decision is rejected with a mismatched-confirmation failure
- AND the protected action remains unexecuted.

### Requirement: Host-owned confirmation interaction

r[agent_harness_runtime.clankers_confirmation_broker.host_owned] UI prompts, human approval collection, notification channels, external approval services, and durable approval storage MUST remain host-owned adapter behavior outside reusable runtime crates.

#### Scenario: Runtime does not prompt directly

r[agent_harness_runtime.clankers_confirmation_broker.host_owned.no_ui]
- GIVEN a confirmation request is required
- WHEN reusable runtime evaluates the protected action
- THEN it calls the injected broker contract
- AND it does not display UI, read terminal input, send notifications, or write approval storage directly.

### Requirement: Safe confirmation receipts

r[agent_harness_runtime.clankers_confirmation_broker.receipts] Confirmation receipts MUST record safe metadata such as request ID hash, action kind, resource and ability labels or hashes, decision status, broker class, policy hash, timing class, expiry class, and BLAKE3 receipt hash, and MUST NOT include raw prompts, credentials, full paths, provider payloads, raw tool payloads, or secret summaries.

#### Scenario: Confirmation receipt leak rejected

r[agent_harness_runtime.clankers_confirmation_broker.receipts.rejects_leaks]
- GIVEN a confirmation receipt fixture includes raw prompt text, credentials, a full path dump, provider JSON, raw tool payload, or secret-like summary
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Confirmation broker verification

r[agent_harness_runtime.clankers_confirmation_broker.verification] The adaptation MUST include focused positive and negative tests for typed requests, approved and denied decisions, missing brokers, timeouts, cancellations, stale and mismatched decisions, host-owned interaction boundaries, and receipt redaction.

#### Scenario: Closeout evidence covers confirmation fixtures

r[agent_harness_runtime.clankers_confirmation_broker.verification.closeout]
- GIVEN maintainers intend to archive the confirmation broker kit adaptation
- WHEN closeout validation runs
- THEN evidence names focused request, default-deny, binding, host-boundary, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
