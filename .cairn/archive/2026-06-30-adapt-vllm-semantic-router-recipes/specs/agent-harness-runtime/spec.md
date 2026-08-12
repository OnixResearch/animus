# agent-harness-runtime Change Spec: vLLM Semantic Router recipe adaptation

## Requirements

### Requirement: Typed model-router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts] Animus model-router recipes MUST be typed contracts for single-route, confidence escalation, ratings ensemble, ReMoM-style quorum synthesis, fusion judge/finalizer, and bounded role workflow route families, including allowed roles, output contract, budget, concurrency, quorum, synthesis, repair, and fallback fields.

#### Scenario: Malformed recipe rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.contracts.invalid]
- GIVEN a router recipe has duplicate roles, unknown model roles, a missing output contract, or an unsupported route family
- WHEN recipe validation runs
- THEN validation rejects the recipe before provider dispatch.

### Requirement: Pure request-signal projection

r[agent_harness_runtime.vllm_semantic_router_recipes.signals] Request-signal projection MUST be a pure deterministic function over bounded metadata for task shape, risk, contract pressure, context pressure, latency band, cost band, privacy/locality, and fallback eligibility.

#### Scenario: Signal projection omits unsafe payloads

r[agent_harness_runtime.vllm_semantic_router_recipes.signals.safe]
- GIVEN request metadata is available with sensitive prompt bodies, provider payloads, credentials, or unbounded text omitted
- WHEN signal projection runs
- THEN projection returns bounded labels and score buckets
- AND it does not require or emit raw prompt bodies, provider payloads, credentials, or unbounded text.

### Requirement: Deterministic recipe selection

r[agent_harness_runtime.vllm_semantic_router_recipes.selection] Recipe selection MUST be deterministic over projected signals, validated recipe policy, budget state, allowed backends, and safe model catalog projections, returning a selected recipe plan or typed denial before any provider call.

#### Scenario: Hard task selects collaborative recipe

r[agent_harness_runtime.vllm_semantic_router_recipes.selection.valid]
- GIVEN projected signals indicate high reasoning variance, strict output contract pressure, and eligible model roles within budget
- WHEN recipe selection runs
- THEN the selected plan uses the configured quorum synthesis or fusion recipe
- AND no provider request is sent during selection.

### Requirement: Bounded execution plans

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds] Collaborative router execution plans MUST declare hard limits for max candidates, max parallelism, min-success quorum, timeout class, cancellation behavior, synthesis attempts, repair attempts, and fallback policy before host execution begins.

#### Scenario: Unbounded fan-out denied

r[agent_harness_runtime.vllm_semantic_router_recipes.bounds.invalid]
- GIVEN a recipe plan omits max parallelism, has zero quorum, requires more successes than candidates, allows unlimited repair, or falls back to a disallowed model
- WHEN execution-plan validation runs
- THEN validation returns a typed denial
- AND no model/provider service is called.

### Requirement: Output contracts gate collaborative success

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract] Collaborative routes MUST validate the final response contract after synthesis, finalization, repair, or fallback before reporting success to callers or automation.

#### Scenario: Malformed final output fails closed

r[agent_harness_runtime.vllm_semantic_router_recipes.output_contract.invalid]
- GIVEN worker responses produce evidence but synthesis or repair returns malformed final output
- WHEN response validation runs
- THEN the route returns a typed malformed-output failure or a policy-declared best-valid-evidence fallback
- AND success is not claimed for invalid output.

### Requirement: Safe router-recipe receipts

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts] Router-recipe receipts MUST record safe metadata such as recipe ID, signal buckets, selected model labels, bounded counts, quorum class, fallback class, policy hash, duration buckets, redaction markers, and BLAKE3 identity, and MUST NOT include raw prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

#### Scenario: Router receipt leak rejected

r[agent_harness_runtime.vllm_semantic_router_recipes.receipts.rejects_leaks]
- GIVEN a router-recipe receipt fixture contains raw prompt text, provider JSON, worker output, judge rationale, hidden reasoning text, credentials, full paths, or an unbounded trace
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Router-recipe verification

r[agent_harness_runtime.vllm_semantic_router_recipes.verification] The adaptation MUST include focused positive and negative tests for recipe validation, signal projection, deterministic selection, execution bounds, output-contract validation, fallback behavior, and receipt redaction before closeout.

#### Scenario: Closeout evidence covers router recipes

r[agent_harness_runtime.vllm_semantic_router_recipes.verification.closeout]
- GIVEN maintainers intend to archive the router-recipe adaptation
- WHEN closeout validation runs
- THEN evidence names focused recipe, signal, selection, bounds, output, receipt, roadmap, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
