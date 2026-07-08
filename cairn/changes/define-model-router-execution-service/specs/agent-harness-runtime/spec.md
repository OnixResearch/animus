# agent-harness-runtime Change Spec: Model-router execution service

## Requirements

### Requirement: Router execution remains host-owned

r[agent_harness_runtime.model_router_execution.scope] Animus MUST treat model-router execution as a host-owned service seam that starts from validated router plans, and MUST NOT move live provider clients, credential lookup, model discovery, network transport, health checks, latency telemetry, or concurrent scheduling into `animus-router` or reusable runtime defaults.

#### Scenario: Router core remains plan-only

- GIVEN a host asks Animus to route a request through a collaborative recipe
- WHEN the router selection and execution-plan materialization functions run
- THEN they return bounded plans, budgets, fallback chains, and safe labels only
- AND they do not call providers, discover models, read credentials, open networks, spawn tasks, or schedule live concurrency.

### Requirement: Injected router execution service

r[agent_harness_runtime.model_router_execution.service_contract] Runtime-facing router execution MUST cross an injected service contract that accepts a validated `RouterExecutionPlan`, a validated model request or safe request handle, host-approved model catalog bindings, cancellation and timeout metadata, and budget state, and returns a typed bounded outcome without constructing concrete providers internally.

#### Scenario: Missing execution service fails closed

- GIVEN a runtime has a valid `RouterExecutionPlan` but no admitted router execution service
- WHEN it attempts to execute the plan
- THEN execution is denied with a missing-service or unavailable-service outcome
- AND no fallback provider client, ambient model registry, shell command, or network request is created.

### Requirement: Bounded dispatch envelopes

r[agent_harness_runtime.model_router_execution.dispatch_envelopes] Router execution dispatch envelopes MUST identify the recipe, role step, candidate model label, output contract, timeout class, cancellation posture, budget ledger state, and safe correlation metadata using typed DTOs; durable outcomes MUST store only safe labels, counts, hashes, status classes, and redaction markers.

#### Scenario: Provider-native payload stays inside host adapter

- GIVEN a host implementation translates a role-step dispatch into a provider-native request
- WHEN durable runtime evidence or a router execution outcome is rendered
- THEN provider-native request and response bodies are omitted or represented by safe hashes and redaction markers
- AND the envelope remains valid without storing raw prompts, provider JSON, hidden reasoning, credentials, or raw worker output.

### Requirement: Fan-out and quorum obey materialized plans

r[agent_harness_runtime.model_router_execution.fanout_quorum] Same-request fan-out to multiple model candidates MUST occur only when a materialized `RouterExecutionPlan` role step authorizes the candidate labels, maximum parallelism, minimum success quorum, and candidate budget, and implementations MUST NOT add ambient candidates or exceed those bounds.

#### Scenario: Over-parallel fan-out is denied

- GIVEN a role step declares bounded candidate labels, a parallelism cap, and a quorum threshold
- WHEN a host implementation attempts to dispatch more concurrent candidate calls than the plan permits
- THEN execution records an over-budget or policy-denied outcome
- AND extra candidates are not sent to a model provider.

### Requirement: Fallback and repair are policy-bound

r[agent_harness_runtime.model_router_execution.fallback_repair] Router execution fallback and repair MUST consume `BudgetLedger` state and `FallbackChain` entries, MUST use `evaluate_fallback` or an equivalent pure policy result, and MUST deny provider-side fallback, health-check fallback, or model-registry fallback unless the selected fallback policy and chain explicitly authorize it.

#### Scenario: Provider fallback bypass is rejected

- GIVEN a provider client reports that it can automatically fall back to another model outside the materialized fallback chain
- WHEN router execution evaluates the failed step
- THEN the fallback is denied or replaced with a chain-authorized fallback decision
- AND the receipt records the denial or authorized fallback class without raw provider payloads.

### Requirement: Safe router execution outcomes and receipts

r[agent_harness_runtime.model_router_execution.receipts] Router execution outcomes and receipts MUST record safe metadata for recipe ID, route family, role-step statuses, selected model labels, bounded candidate and success counts, quorum class, fallback and repair classes, output-contract status, policy hash, duration bucket, redaction markers, and BLAKE3 identity, and MUST NOT include raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

#### Scenario: Unsafe execution evidence is rejected

- GIVEN a router execution receipt fixture contains raw prompt text, provider JSON, worker output, judge rationale, hidden reasoning, a credential-looking token, an absolute path dump, or an unbounded trace
- WHEN receipt validation runs
- THEN validation rejects the receipt with a typed leak diagnostic
- AND the receipt is not eligible for durable evidence.

### Requirement: Execution service verification and non-claims

r[agent_harness_runtime.model_router_execution.verification] The execution-service change MUST include positive and negative tests for service admission, deterministic fixture execution, single-route execution, collaborative fan-out, quorum success and failure, fallback and repair policy, cancellation or timeout classes, receipt redaction, and missing live-provider non-claims before archive.

#### Scenario: Closeout evidence separates fixtures from live providers

- GIVEN maintainers intend to archive the router execution service change
- WHEN closeout validation runs
- THEN evidence names deterministic fixture-executor tests, pure reducer tests, documentation, facade inventory updates, Cairn validation, and Cairn gates
- AND live provider routing, production latency routing, health checks, credential storage, provider discovery, and networked multi-model execution are listed as non-claims unless an optional host implementation explicitly proves them.
