# Tasks

- [x] [serial] Inventory pure safety-critical candidates for verified-core treatment across policy/effect gates, ledger graphs, branch replay, receipt redaction, schema compatibility, compaction cut points, and roadmap dependency checks. r[agent_harness_runtime.trellis_verified_core.candidate_inventory]
- [x] [depends:candidate_inventory] Define proof obligations, invariants, preconditions, postconditions, and rejected-state classes for selected candidates. r[agent_harness_runtime.trellis_verified_core.obligations]
- [x] [depends:obligations] Add optional or test-only Trellis/verified-helper dependency boundaries with default dependency isolation. r[agent_harness_runtime.trellis_verified_core.dependency_isolation]
- [x] [depends:obligations] Add model or verified-core implementations for at least one selected pure decision boundary. r[agent_harness_runtime.trellis_verified_core.model_logic]
- [x] [depends:model_logic] Add positive equivalence tests comparing model/verified logic with production Rust behavior. r[agent_harness_runtime.trellis_verified_core.equivalence]
- [x] [depends:model_logic] Add negative invariant fixtures for malformed graphs, bypass attempts, unsafe redaction, unsupported compatibility, and rejected-state counterexamples. r[agent_harness_runtime.trellis_verified_core.negative_fixtures]
- [x] [depends:equivalence,negative_fixtures,dependency_isolation] Document proof scope and run verification/model checks, workspace tests, dependency checks, and Cairn gates with evidence. r[agent_harness_runtime.trellis_verified_core.closeout]
