# agent-harness-runtime Change Spec: Clankers cost-aware model routing adaptation

## Requirements

### Requirement: Nickel-authored routing policy

r[agent_harness_runtime.clankers_cost_model_routing.policy] Animus model-routing policy MUST be authored as Nickel contracts or checked fixtures defining role names, thresholds, complexity weights, user-hint mappings, budget behavior, downgrade rules, provider constraints, and missing-price behavior.

#### Scenario: Malformed routing policy rejected

r[agent_harness_runtime.clankers_cost_model_routing.policy.invalid]
- GIVEN a routing policy has duplicate role names, negative weights, unordered thresholds, unknown downgrade targets, or unspecified missing-price behavior
- WHEN policy validation runs
- THEN validation rejects the policy before any routing decision is used.

### Requirement: Pure model routing decisions

r[agent_harness_runtime.clankers_cost_model_routing.decision] Model routing MUST be a pure decision over task signals, current usage/cost summaries, model catalog projections, and validated policy, returning selected role/model metadata or typed failure without contacting providers.

#### Scenario: Complex task selects stronger eligible role

r[agent_harness_runtime.clankers_cost_model_routing.decision.complex]
- GIVEN task signals exceed the configured high-complexity threshold
- AND an eligible stronger model role is available within policy
- WHEN routing runs
- THEN the decision selects that role with a complexity-score reason
- AND no provider request is sent by routing logic.

### Requirement: Budget-aware routing

r[agent_harness_runtime.clankers_cost_model_routing.budget] Routing MUST account for current usage and configured budget behavior, including soft-budget pressure, hard-budget downgrade, and hard-budget denial when no compliant model is available.

#### Scenario: Hard budget blocks upgrade

r[agent_harness_runtime.clankers_cost_model_routing.budget.hard_limit]
- GIVEN current cost meets or exceeds the configured hard budget
- AND the requested or scored model would be a cost upgrade
- WHEN routing evaluates budget policy
- THEN the decision downgrades to an allowed cheaper role or denies routing if no cheaper role is available
- AND the receipt records the hard-budget reason.

### Requirement: Injected safe model catalog projections

r[agent_harness_runtime.clankers_cost_model_routing.catalog] Routing MUST consume injected safe model catalog projections and MUST fail closed when required model capability, availability, or pricing metadata is absent under the active policy.

#### Scenario: Missing price denied by policy

r[agent_harness_runtime.clankers_cost_model_routing.catalog.missing_price]
- GIVEN policy requires price metadata for budgeted routing
- AND a candidate model lacks pricing metadata
- WHEN routing evaluates candidates
- THEN the candidate is rejected or routing fails according to policy
- AND no provider fallback is attempted.

### Requirement: Safe routing receipts

r[agent_harness_runtime.clankers_cost_model_routing.receipts] Routing receipts MUST record safe metadata such as decision status, selected role/model label, score bucket, budget status, candidate counts, policy hash, fallback class, and BLAKE3 receipt hash, and MUST NOT include raw prompts, provider payloads, credentials, full pricing-store dumps, or hidden reasoning text.

#### Scenario: Routing receipt leak rejected

r[agent_harness_runtime.clankers_cost_model_routing.receipts.rejects_leaks]
- GIVEN a routing receipt fixture contains raw prompt text, provider JSON, credentials, full pricing file contents, or hidden reasoning text
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Cost-aware routing verification

r[agent_harness_runtime.clankers_cost_model_routing.verification] The adaptation MUST include focused positive and negative tests for policy validation, simple/complex task selection, explicit user hints, soft and hard budget behavior, missing prices, unavailable models, malformed policies, and receipt redaction.

#### Scenario: Closeout evidence covers routing fixtures

r[agent_harness_runtime.clankers_cost_model_routing.verification.closeout]
- GIVEN maintainers intend to archive the cost-aware routing adaptation
- WHEN closeout validation runs
- THEN evidence names focused policy, decision, budget, catalog, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
