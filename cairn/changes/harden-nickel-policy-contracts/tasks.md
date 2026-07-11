## Phase 1: Shared contract vocabulary

- [ ] [serial] Inventory the exact Rust DTO, generated artifact, checker, and runtime owner for model-routing, process-job, Steel-runtime, and tool-catalog policy fields. r[agent_harness_runtime.nickel_policy_contracts.authority_boundary]
- [ ] [serial] Add the shared Nickel policy-contract prelude with exact schemas, bounded integers, closed enums, lowercase BLAKE3 identities, safe repository-relative paths, uniqueness helpers, and deterministic validator diagnostics. r[agent_harness_runtime.nickel_policy_contracts.shared_primitives]
- [ ] [parallel] Add valid-minimal and primitive negative fixtures for the shared prelude. r[agent_harness_runtime.nickel_policy_contracts.fixtures]

## Phase 2: Family hardening

- [ ] [serial] Harden model-routing contracts for role identity, downgrade references and cycles, threshold ordering, integer weights, behavior enums, and policy identity. r[agent_harness_runtime.nickel_policy_contracts.model_routing]
- [ ] [serial] Harden process-job contracts for execution-form choice, backend admission, path and environment policy, unique metadata, and bounded resource limits. r[agent_harness_runtime.nickel_policy_contracts.process_jobs]
- [ ] [serial] Harden Steel-runtime contracts for unique profiles, one enabled default, script path/hash identity, positive budgets, host-action vocabulary, fallback, and redaction. r[agent_harness_runtime.nickel_policy_contracts.steel_runtime]
- [ ] [serial] Harden tool-catalog contracts so descriptors are validated against catalog vocabularies and dangerous or secret-adjacent combinations fail closed. r[agent_harness_runtime.nickel_policy_contracts.tool_catalog]

## Phase 3: Projection and consumer parity

- [ ] [serial] Define explicit policy generation receipts binding source, shared prelude, family contract, evaluator/profile, and generated JSON with BLAKE3. r[agent_harness_runtime.nickel_policy_contracts.projection_freshness]
- [ ] [serial] Make Rust policy tests deserialize generated artifacts and compare normalized DTO meaning without adding runtime Nickel evaluation. r[agent_harness_runtime.nickel_policy_contracts.projection_freshness] r[agent_harness_runtime.nickel_policy_contracts.authority_boundary]
- [ ] [parallel] Add negative family fixtures for unknown enums, invalid bounds, malformed hashes, unsafe paths, duplicate identities, dangling references, cycles, conflicting execution forms, invalid defaults, and unsafe tool combinations. r[agent_harness_runtime.nickel_policy_contracts.fixtures]

## Phase 4: Verification

- [ ] [serial] Run the focused policy checkers, generated-artifact freshness checks, relevant Rust policy tests, `check-nickel-configs`, Cairn validation, and proposal/design/tasks gates. r[agent_harness_runtime.nickel_policy_contracts.fixtures] r[agent_harness_runtime.nickel_policy_contracts.projection_freshness]
- [ ] [serial] Document that contract success establishes source shape and declared invariants only, while runtime admission, authority, effects, and receipts remain Rust-owned. r[agent_harness_runtime.nickel_policy_contracts.authority_boundary]
