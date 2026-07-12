# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Portfolio contracts are explicit and bounded

r[agent_harness_runtime.portfolio_search.contract] Animus MUST define a deterministic portfolio contract containing objective identity, completion-evidence labels, false-completion labels, required adversarial risk classes, source-policy identity, validator-policy identity, and finite positive budgets.

#### Scenario: Missing success or failure criteria fail closed

r[agent_harness_runtime.portfolio_search.contract.rejects_incomplete]
- GIVEN a portfolio contract omits completion evidence, false-completion cases, audit risks, source policy, validator policy, or a positive budget
- WHEN contract validation runs
- THEN validation MUST reject the contract before exploration.

### Requirement: Portfolio phases are typed

r[agent_harness_runtime.portfolio_search.phases] Animus MUST represent framing, independent exploration, synthesis, targeted redirection, adversarial audit, validation, validated result, exhaustion, and blocking as explicit portfolio phases with deterministic transition rules.

#### Scenario: Illegal phase transition is rejected

r[agent_harness_runtime.portfolio_search.phases.illegal]
- GIVEN an event is not admitted from the current phase
- WHEN the pure transition function evaluates it
- THEN it MUST return a typed phase error without mutating the supplied state.

### Requirement: Independent exploration delays cross-pollination

r[agent_harness_runtime.portfolio_search.independence] Dispatch metadata in independent exploration MUST NOT disclose prior approach IDs, while synthesis and later phases MAY admit reviewed cross-approach references.

#### Scenario: Early disclosure is rejected

r[agent_harness_runtime.portfolio_search.independence.rejects_disclosure]
- GIVEN an independent-exploration dispatch cites a prior approach
- WHEN dispatch validation runs
- THEN it MUST be rejected as a cross-pollination violation.

### Requirement: Approach registry records mechanisms and evidence

r[agent_harness_runtime.portfolio_search.registry] Each approach MUST record a unique approach ID, family label, BLAKE3 mechanism fingerprint, claim fingerprint, safe artifact and evidence references, gap strength, state, optional exact blocker, and next discriminating check. Duplicate mechanism fingerprints MUST be rejected even when IDs or wording differ.

#### Scenario: Distinct mechanisms remain independently visible

r[agent_harness_runtime.portfolio_search.registry.distinct]
- GIVEN two approaches have different mechanism fingerprints and valid metadata
- WHEN they are registered during exploration
- THEN both MUST remain visible in deterministic registry order.

#### Scenario: Paraphrased mechanism is deduplicated

r[agent_harness_runtime.portfolio_search.registry.duplicate]
- GIVEN a new approach has a mechanism fingerprint already in the registry
- WHEN registration runs
- THEN it MUST be rejected with the existing approach ID.

### Requirement: Gap strength prevents reduction theater

r[agent_harness_runtime.portfolio_search.gap_strength] Animus MUST classify each route gap as `Simpler`, `Equivalent`, `Stronger`, or `Unknown`; equivalent, stronger, and unknown gaps MUST remain blocked from audit and validation until valid novelty reclassifies the route.

#### Scenario: Equivalent-strength helper remains blocked

r[agent_harness_runtime.portfolio_search.gap_strength.equivalent]
- GIVEN an approach leaves a helper lemma equivalent to the objective
- WHEN approach admission or audit selection runs
- THEN the route MUST require an equivalent-strength blocker
- AND it MUST NOT enter audit or validation as solved progress.

### Requirement: Blocked routes reopen only for novelty

r[agent_harness_runtime.portfolio_search.redirection] A blocked route MUST reopen only for a new safe evidence reference or a new mechanism fingerprint, and underexplored live families MUST be selected deterministically without requiring a fixed number of approaches.

#### Scenario: Rephrasing does not reopen a route

r[agent_harness_runtime.portfolio_search.redirection.no_novelty]
- GIVEN a blocked approach receives no new evidence and retains its mechanism fingerprint
- WHEN reopen is requested
- THEN the transition MUST reject the request and preserve the blocker.

#### Scenario: Least explored families are selected

r[agent_harness_runtime.portfolio_search.redirection.underexplored]
- GIVEN live approach families have different route counts
- WHEN redirection selection runs
- THEN every family with the minimum live count MUST be returned in deterministic lexical order.

### Requirement: Adversarial audit precedes validation

r[agent_harness_runtime.portfolio_search.audit] A candidate MUST receive one audit finding for every contract-required risk class before validation, and every required finding MUST pass with safe evidence and explicit residual uncertainty.

#### Scenario: Missing or failed audit blocks validation

r[agent_harness_runtime.portfolio_search.audit.fail_closed]
- GIVEN a candidate omits a required risk class or records a failed or unresolved finding
- WHEN validation entry is requested
- THEN the transition MUST reject validation and identify the missing or failed risk.

### Requirement: Validation and terminal reporting are honest

r[agent_harness_runtime.portfolio_search.validation] A portfolio result MUST become `Validated` only from the validation phase with a pass receipt matching the contract, contract-admitted validator policy, selected approach, mechanism, and evidence identity. Budget exhaustion MUST produce `Exhausted`, and user, policy, authority, or tool barriers MUST produce `Blocked`; neither outcome may be promoted to `Validated`.

#### Scenario: Matching receipt validates one bounded result

r[agent_harness_runtime.portfolio_search.validation.match]
- GIVEN an audited simpler-gap candidate and a pass receipt matching its contract, admitted validator-policy, and mechanism fingerprints
- WHEN receipt validation runs
- THEN the portfolio phase MUST become `Validated` for that selected route only.

#### Scenario: Exhaustion is not completion

r[agent_harness_runtime.portfolio_search.validation.exhausted]
- GIVEN an explicit budget is exhausted without a matching pass receipt
- WHEN terminal transition runs
- THEN the phase MUST become `Exhausted`
- AND no repository, release, or agent completion claim may be inferred.

### Requirement: Portfolio receipts are redacted and non-authoritative

r[agent_harness_runtime.portfolio_search.receipts] Animus MUST emit BLAKE3-identified portfolio receipts containing only bounded counters, safe labels/references, phase, selected approach identity, and evidence identity. Raw prompts, full home paths, raw tool output, secrets, and provider payloads MUST be rejected. Portfolio receipts MUST NOT claim proof semantics, validator soundness, repository readiness, release readiness, or authoritative agent completion.

#### Scenario: Unsafe receipt metadata is rejected

r[agent_harness_runtime.portfolio_search.receipts.rejects_raw]
- GIVEN a receipt label or evidence reference contains a raw prompt, full home path, raw tool output, secret, or provider payload marker
- WHEN receipt validation runs
- THEN it MUST fail closed without emitting an accepted receipt.

### Requirement: Portfolio core has positive and negative evidence

r[agent_harness_runtime.portfolio_search.verification] The portfolio-search core MUST include positive end-to-end, deterministic selection, and safe receipt tests plus negative tests for incomplete contracts, illegal phases, early disclosure, duplicate mechanisms, equivalent-strength gaps, no-novelty reopening, incomplete audits, stale receipts, unadmitted validator policies, unsafe metadata, and exhaustion masquerading as completion.

#### Scenario: Pure tests need no running host

r[agent_harness_runtime.portfolio_search.verification.pure]
- GIVEN the focused portfolio-search test suite
- WHEN tests execute
- THEN all phase, registry, audit, selection, and receipt behavior MUST be exercised from in-memory values without model, tool, network, clock, filesystem, or process dependencies.
