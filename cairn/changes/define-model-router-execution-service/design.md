# Design: Define model-router execution service contracts

Animus should make router execution composable without becoming a concrete provider runtime. The design adds a service seam around existing router plans so downstream agents can execute multi-model workflows while reusable crates remain deterministic, credential-free, and no-ambient-authority.

## Decisions

### 1. Keep `animus-router` plan-first and data-only

**Choice:** `animus-router` remains the no-std decision core for recipes, signals, selected plans, execution plans, budget/fallback decisions, and safe receipt metadata.

**Rationale:** The router crate must stay usable in Wasm and embedded contexts. It can define pure DTOs and validation rules, but it must not know how to call providers, run tasks concurrently, read credentials, or collect live telemetry.

### 2. Define a host-injected execution service seam

**Choice:** A model-router execution service is injected by host shells or agents. The service accepts a validated `RouterExecutionPlan`, a validated model request or safe request handle, host-approved model catalog bindings, and bounded control metadata. It returns typed execution outcomes without leaking raw provider data into durable runtime evidence.

**Rationale:** This lets Animus-owned runtimes request plan execution while preserving the existing adapter-first rule: missing or denied services fail closed instead of creating provider clients or discovering ambient models.

### 3. Treat fan-out as plan-authorized execution, not a default runtime behavior

**Choice:** Same-request fan-out to multiple models is allowed only when a materialized role step authorizes candidate labels, parallelism, quorum, and budget. Implementations may schedule those calls however they choose, but durable outcomes must show which bounded plan path was followed.

**Rationale:** Multi-model execution is useful, but unplanned fan-out can explode cost, leak prompts, or bypass safety policy. Binding it to the plan keeps the behavior auditable and testable.

### 4. Keep fallback and repair policy-bound

**Choice:** Repair and fallback execution consumes `BudgetLedger` state and `FallbackChain` entries. Implementations must not use provider fallback, health-check fallback, or model-registry fallback unless the router plan and fallback policy authorize it.

**Rationale:** Provider-side fallback is convenient but can violate privacy, cost, capability, and provenance constraints. Animus should make fallback explicit and deny-by-default.

### 5. Add deterministic fixture execution before live adapters

**Choice:** Closeout should include a deterministic fixture executor or service stub that proves orchestration, quorum, fallback, repair, and receipt behavior without live providers.

**Rationale:** The contract needs executable evidence, but live provider behavior belongs to downstream agents or optional host crates. Fixture execution proves the seam without overclaiming production routing.

## Risks / Trade-offs

- A service seam adds DTO surface before live provider implementations exist. The benefit is a stable boundary that downstream agents can implement without moving provider logic into core crates.
- Plan-authorized fan-out may feel less flexible than provider-registry fallback. The benefit is bounded cost, privacy, and auditability.
- A deterministic fixture executor does not prove provider reliability. Evidence must label live-provider checks as non-claims unless a host implementation runs them.
