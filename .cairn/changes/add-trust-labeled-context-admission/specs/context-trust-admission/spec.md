# Context Trust Admission Delta

## ADDED Requirements

### Requirement: Protected context carries explicit trust facts

r[animus.context_fencing.facts] Animus MUST require protected external context to carry distinct origin, owner, trust-tier, screening, quarantine, freshness, redaction, sensitivity, payload, policy, and non-claim facts.

#### Scenario: Valid external context is described

- GIVEN a host supplies an admitted external record with all required bounded trust facts
- WHEN contribution validation runs
- THEN Animus MUST preserve each fact in a nominal role
- AND a safe label, source class, or payload hash MUST NOT substitute for another role.

#### Scenario: Required trust facts are absent

- GIVEN protected context omits origin, trust tier, quarantine state, freshness, policy identity, or non-claims
- WHEN validation runs
- THEN Animus MUST reject the contribution before payload loading or provider dispatch.

### Requirement: Context admission is pure and fail-closed

r[animus.context_fencing.admission] Animus MUST evaluate protected context through pure deterministic admission before final prompt assembly and provider dispatch.

#### Scenario: External context is admitted

- GIVEN contribution, trust, freshness, policy, authority, and context-use facts agree
- WHEN pure context admission runs
- THEN Animus MUST return an admitted fencing decision bound to the exact contribution and payload identity
- AND it MUST perform no catalog, file, database, classifier, clock, policy-loading, credential, network, provider, persistence, logging, or UI effect.

#### Scenario: Quarantined context is selected

- GIVEN a selected contribution has an active quarantine state
- WHEN admission runs
- THEN Animus MUST return a quarantined or denied result
- AND the payload MUST NOT enter model context.

### Requirement: Screening observations remain advisory

r[animus.context_fencing.screening_boundary] Animus MUST treat screening and classifier results as bounded observations rather than truth, safety, maliciousness, authority, or execution decisions.

#### Scenario: Screening reports clear

- GIVEN a classifier reports a clear observation for one payload and profile
- WHEN Animus records the trust facts
- THEN the result MAY inform policy
- AND Animus MUST NOT label the content safe or trusted solely from that observation.

#### Scenario: Screening is unavailable or fails

- GIVEN a protected source requires screening and the classifier is unavailable, stale, malformed, or failed
- WHEN admission evaluates the contribution
- THEN Animus MUST deny or require review according to explicit policy
- AND it MUST NOT silently treat the contribution as clear.

### Requirement: Basalt policy decisions are exact and narrow

r[animus.context_fencing.policy] Animus MUST accept protected context only through a current exact Basalt-compatible decision bound to subject, context action, resource, contribution, payload, policy, and trust facts.

#### Scenario: Exact context-read decision allows admission

- GIVEN a current allow decision matches every required binding
- WHEN Animus validates the decision
- THEN it MAY admit only the declared context-read use
- AND the decision MUST NOT grant model, tool, filesystem, process, network, memory, retention, workflow, or provider authority.

#### Scenario: Decision binding is wrong

- GIVEN a decision names another payload, contribution, resource, subject, action, policy, or trust observation
- WHEN binding validation runs
- THEN Animus MUST reject the decision before payload loading or provider dispatch.

### Requirement: Trust fencing survives projection

r[animus.context_fencing.projection] Animus MUST preserve selected trust facts, policy identity, fencing decision, and non-claims through contribution reduction, payload validation, and model-request snapshots.

#### Scenario: Final payload matches the admitted contribution

- GIVEN a selected contribution has an admitted fencing decision and the host loads the exact payload bytes
- WHEN final payload validation runs
- THEN Animus MUST bind the payload to the contribution and fencing identities
- AND the model-request snapshot MUST retain safe trust metadata without raw external content.

#### Scenario: Host replaces the payload after admission

- GIVEN loaded payload bytes do not match the selected payload identity
- WHEN final validation runs
- THEN provider dispatch MUST stop
- AND no stale fencing decision MAY authorize the changed payload.

### Requirement: Effects remain host-owned

r[animus.context_fencing.host_boundary] Catalog queries, payload loading, classifier execution, quarantine storage, policy loading, UCAN verification, provider calls, and persistence MUST remain injected host behavior outside reusable pure context logic.

#### Scenario: A required host service is absent

- GIVEN protected context needs a catalog, classifier, policy, or payload service that is not injected
- WHEN runtime prepares the request
- THEN Animus MUST return a typed unavailable or denied outcome
- AND it MUST NOT discover an ambient fallback.

### Requirement: Fencing receipts are metadata-only

r[animus.context_fencing.receipts] Animus MUST emit context-fencing receipts with safe identities, labels, counts, decisions, freshness, redaction, and non-claims only.

#### Scenario: Safe receipt is emitted

- GIVEN admission reaches a terminal admitted, omitted, quarantined, review-required, or denied result
- WHEN receipt projection runs
- THEN the receipt MUST bind the contribution, payload, policy, trust, screening, quarantine, and decision identities
- AND it MUST omit raw external content.

#### Scenario: Receipt contains sensitive content

- GIVEN a receipt candidate contains prompt, document, email, message, classifier, credential, header, provider, or host-path material
- WHEN receipt validation runs
- THEN Animus MUST reject the receipt with a typed leak diagnostic.

### Requirement: Verification covers positive and negative paths

r[animus.context_fencing.verification] The change MUST include positive and negative tests for trust facts, admission, screening, policy binding, projection, host services, and receipt safety.

#### Scenario: Closeout evidence is complete

- GIVEN maintainers intend to archive the change
- WHEN closeout review runs
- THEN evidence MUST name focused Rust, Clippy, Octet, request-context, request-lifecycle, evidence-safety, Cairn, positive, and negative checks
- AND each skipped check MUST include an exact blocker.

### Requirement: Evidence preserves context non-claims

r[animus.context_fencing.evidence] Context-fencing evidence MUST state that it does not prove content safety, semantic truth, classifier correctness, provider compliance, model behavior, task correctness, or release readiness.

#### Scenario: Context is admitted

- GIVEN a protected contribution passes current policy
- WHEN the result is displayed or recorded
- THEN the result MUST describe policy admission over supplied facts
- AND it MUST NOT describe the payload as safe, true, or instruction-worthy.
