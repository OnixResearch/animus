# Tasks

## B. Contract and storage model

- [x] [serial] B1: Record current in-memory replay behavior and the exact Steel, RLM, bridge, and durable-store cohorts. r[agent_harness_runtime.steel_kernel_restore.verification]
- [x] [serial] B2: Define versioned private kernel record, safe metadata projection, chain identity, bounds, and failure classes. r[agent_harness_runtime.steel_kernel_restore.journal]
- [x] [depends:B2] B3: Define restore admission over session, policy, cohort, schema, generation, and committed head facts. r[agent_harness_runtime.steel_kernel_restore.admission]

## R. Replay and recovery

- [x] [depends:B3] R1: Add commit-boundary journal projection from successful bridged cells and explicit kernel clears. r[agent_harness_runtime.steel_kernel_restore.safe_points]
- [x] [depends:R1] R2: Implement effect-disabled deterministic replay with exact captured-result matching. r[agent_harness_runtime.steel_kernel_restore.replay]
- [x] [depends:R2] R3: Integrate private records with the explicit durable store without leaking bodies to safe ledger records. r[agent_harness_runtime.steel_kernel_restore.journal]
- [x] [depends:R3] R4: Integrate interrupted-cell classification with active-turn recovery and tool repair without rerunning effects. r[agent_harness_runtime.steel_kernel_restore.safe_points]
- [x] [depends:R4] R5: Add metadata-only restore receipts and leak rejection. r[agent_harness_runtime.steel_kernel_restore.evidence]

## T. Tests and closeout

- [x] [depends:R5] T1: Add positive restart tests for helpers, variables, context, clear markers, and captured host results. r[agent_harness_runtime.steel_kernel_restore.verification]
- [x] [depends:R5] T2: Add negative tests for missing results, extra requests, order drift, wrong session, wrong cohort, wrong policy, schema drift, chain corruption, interrupted cells, and effect replay attempts. r[agent_harness_runtime.steel_kernel_restore.verification]
- [x] [depends:T2] T3: Add private-channel leak tests and default dependency-isolation checks. r[agent_harness_runtime.steel_kernel_restore.evidence]
- [x] [depends:T3] T4: Update recovery docs and component metadata, add a focused rail, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.steel_kernel_restore.verification]
