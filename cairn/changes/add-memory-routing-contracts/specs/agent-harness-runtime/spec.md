# agent-harness-runtime Change Spec: Memory routing contracts

## Requirements

### Requirement: Memory route plan DTOs

r[agent_harness_runtime.memory_routing.route_plan] Animus memory routing MUST be represented as typed DTOs for route requests, route candidates, route plans, and route receipts covering read/search, source/import, extraction, durable store, update/delete, and prompt-influence intents.

#### Scenario: Query route names required adapter roles

r[agent_harness_runtime.memory_routing.route_plan.read]
- GIVEN a user request needs prior project facts and recent episodes
- WHEN host-supplied route candidates are validated
- THEN the route plan names the selected retrieval/search adapter roles and safe route reasons
- AND no concrete vector, graph, file, or remote backend is constructed by reusable logic.

### Requirement: Pure route planner

r[agent_harness_runtime.memory_routing.pure_planner] Route planning MUST be pure deterministic logic over safe operation intent, category hints, allowed scopes, declared adapter roles, availability, confidence, freshness, budgets, and policy identity, and MUST NOT call providers, classifiers, storage, embedding services, clocks, filesystems, networks, or MCP servers.

#### Scenario: Same safe signals produce same route plan

r[agent_harness_runtime.memory_routing.pure_planner.deterministic]
- GIVEN two hosts provide the same validated route candidates and policy inputs
- WHEN reusable route planning runs
- THEN selected routes, skipped routes, diagnostics, and route receipts are equivalent
- AND backend labels do not affect model-visible ordering except as safe evidence metadata.

### Requirement: Routing fails closed

r[agent_harness_runtime.memory_routing.fail_closed] Memory routing MUST fail closed when policy denies a route, classifier or route-candidate output is malformed, required adapter roles are missing, selected adapters are unavailable, selected roles are unsupported, or route metadata contains unsafe material.

#### Scenario: Malformed classifier output is rejected

r[agent_harness_runtime.memory_routing.fail_closed.malformed]
- GIVEN a host classifier proposes a route with an unsafe label, raw memory body, provider payload, credential, or unknown required role
- WHEN route validation runs
- THEN validation rejects the route candidate with a typed diagnostic
- AND no adapter dispatch, retrieval, extraction, storage, or prompt influence is planned.

### Requirement: Memory routing verification

r[agent_harness_runtime.memory_routing.verification] The routing adaptation MUST include focused positive and negative tests for read and write route plans, deterministic planning, denied policy, malformed classifier outputs, unavailable or unsupported adapters, no ambient fallback, and route receipt leak rejection.

#### Scenario: Closeout evidence covers routing fixtures

r[agent_harness_runtime.memory_routing.verification.closeout]
- GIVEN maintainers intend to archive the memory routing adaptation
- WHEN closeout validation runs
- THEN evidence names focused route DTO, deterministic planner, fail-closed, receipt, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
