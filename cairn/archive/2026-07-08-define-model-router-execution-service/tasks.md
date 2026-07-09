# Tasks

- [x] [serial] Document the model-router execution boundary, non-claims, crate ownership, and host-owned responsibilities. r[agent_harness_runtime.model_router_execution.scope]
- [x] [depends:scope] Add the injected execution service contract and fail-closed missing/denied-service behavior. r[agent_harness_runtime.model_router_execution.service_contract]
- [x] [depends:service_contract] Add bounded dispatch request and step-outcome envelope DTOs for validated plans, role steps, candidates, cancellation, timeout, and output contracts. r[agent_harness_runtime.model_router_execution.dispatch_envelopes]
- [x] [depends:dispatch_envelopes] Enforce plan-authorized fan-out, parallelism caps, candidate budgets, quorum, and no ambient model discovery in pure reducers or fixture execution. r[agent_harness_runtime.model_router_execution.fanout_quorum]
- [x] [depends:fanout_quorum] Bind repair and fallback execution to `BudgetLedger`, `FallbackChain`, and explicit fallback policy, with denial for provider-side bypasses. r[agent_harness_runtime.model_router_execution.fallback_repair]
- [x] [depends:fallback_repair] Add safe execution outcome and receipt validation that redacts prompts, provider payloads, worker outputs, judge rationales, hidden reasoning, credentials, full paths, and unbounded traces. r[agent_harness_runtime.model_router_execution.receipts]
- [x] [depends:scope,service_contract,dispatch_envelopes,fanout_quorum,fallback_repair,receipts] Add deterministic fixture-executor coverage, docs, facade inventory updates, Cairn validation, and explicit live-provider non-claim evidence. r[agent_harness_runtime.model_router_execution.verification]
