# agent-harness-runtime Change Spec: Router round-robin selection

## Requirements

### Requirement: Router rotation state is safe and host-owned

r[agent_harness_runtime.model_router.round_robin.state] Animus MUST represent router round-robin state as safe typed policy, cursor, snapshot, decision, and receipt DTOs supplied by the host, and reusable router crates MUST NOT persist cursor state, read clocks, inspect provider health, read credentials, or discover providers to implement rotation.

#### Scenario: Host supplies explicit cursor state

r[agent_harness_runtime.model_router.round_robin.state.host_cursor]
- GIVEN a host wants round-robin selection for a route family and role
- WHEN router selection receives a validated cursor snapshot and rotation policy
- THEN reusable router logic treats that cursor as explicit input only
- AND it returns selected safe labels plus next-cursor metadata without writing storage, reading host state, probing providers, or contacting networks.

### Requirement: Round-robin selection rotates only eligible candidates

r[agent_harness_runtime.model_router.round_robin.selection] Router round-robin selection MUST be a pure deterministic function over eligible catalog projections, route role, rotation policy, cursor state, and budget bounds, and MUST rotate only candidates that have already passed role, backend allowlist, privacy/locality, duplicate-label, candidate-cap, and recipe-family checks.

#### Scenario: Cursor cannot authorize a denied backend

r[agent_harness_runtime.model_router.round_robin.selection.eligible_only]
- GIVEN a cursor points at a catalog entry whose backend is not allowed by the active budget
- WHEN round-robin selection runs
- THEN that entry is excluded before rotation
- AND selection either chooses another eligible candidate or returns a typed no-eligible-model denial.

#### Scenario: Cursor wraparound is deterministic

r[agent_harness_runtime.model_router.round_robin.selection.wraparound]
- GIVEN three eligible candidates for the same role and a cursor at the last candidate
- WHEN round-robin selection runs for the next request
- THEN the first eligible candidate is selected
- AND the next-cursor metadata identifies the following deterministic position without using clocks or random state.

### Requirement: Fallback remains policy-bound under rotation

r[agent_harness_runtime.model_router.round_robin.fallback] Router fallback rotation MAY reorder eligible fallback entries while materializing `FallbackChain`, but fallback execution MUST still consume the materialized chain in order, MUST use `BudgetLedger` and `evaluate_fallback` or an equivalent pure decision, MUST NOT revisit tried models, and MUST deny provider-side, health-check, or ambient model-registry fallback outside the chain.

#### Scenario: Rotated fallback still denies provider bypass

r[agent_harness_runtime.model_router.round_robin.fallback.provider_bypass]
- GIVEN a materialized fallback chain was produced with round-robin rotation
- WHEN a provider reports an automatic fallback target outside that chain
- THEN router execution denies the provider bypass
- AND only a chain-authorized fallback decision may proceed.

### Requirement: Rotation receipts are safe evidence

r[agent_harness_runtime.model_router.round_robin.receipts] Router round-robin receipts MUST record only safe metadata such as recipe ID, route family, role labels, rotation policy label, cursor class, selected model labels, bounded candidate counts, catalog hash, next-cursor hash, policy hash, and BLAKE3 identity, and MUST reject raw prompts, provider payloads, health probes, credentials, endpoints, full paths, and unbounded traces.

#### Scenario: Unsafe rotation receipt is rejected

r[agent_harness_runtime.model_router.round_robin.receipts.reject_unsafe]
- GIVEN rotation evidence includes a raw prompt, provider-native payload, credential, endpoint URL, full path, or health-check body
- WHEN receipt validation runs
- THEN validation rejects the receipt with a typed unsafe-material diagnostic
- AND no durable router receipt stores the unsafe field.

### Requirement: Router round-robin verification

r[agent_harness_runtime.model_router.round_robin.verification] The round-robin change MUST include focused positive and negative tests for host-owned cursor validation, deterministic per-role rotation, cursor wraparound, eligible-only filtering, selected-plan reproducibility, fallback-chain rotation, provider-bypass denial, stale or malformed cursor handling, budget exhaustion, receipt redaction, docs, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers rotation and fallback

r[agent_harness_runtime.model_router.round_robin.verification.closeout]
- GIVEN maintainers intend to archive the router round-robin change
- WHEN closeout validation runs
- THEN evidence names focused rotation DTO, pure selection, fallback-chain, provider-bypass, receipt, docs, router policy, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
