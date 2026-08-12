# Tasks

- [x] [serial] Define std-shell adapter families for model providers, filesystem tools, process execution, HTTP/network helpers, and deterministic fixture backends. r[agent_harness_runtime.std_runtime_adapters.adapter_families]
- [x] [depends:adapter_families] Add capability descriptors and prerequisite checks for provider registry, tool contracts, policy, sandbox, receipts, and event lifecycle contracts. r[agent_harness_runtime.std_runtime_adapters.effect_gates]
- [x] [depends:effect_gates] Implement deterministic fixture adapters for model, filesystem, process, network, clock, and credential behavior. r[agent_harness_runtime.std_runtime_adapters.fixtures]
- [x] [depends:fixtures] Add positive tests for injected model, tool, filesystem, process, and HTTP helper flows using fixtures. r[agent_harness_runtime.std_runtime_adapters.tests_positive]
- [x] [depends:effect_gates] Add negative tests for missing credentials, denied capabilities, sandbox mismatch, malformed outputs, timeout/cancellation, and leak attempts. r[agent_harness_runtime.std_runtime_adapters.tests_negative]
- [x] [depends:effect_gates] Add dependency-isolation checks proving reusable runtime/core crates do not depend on concrete provider, filesystem, process, network, or credential crates. r[agent_harness_runtime.std_runtime_adapters.dependency_isolation]
- [x] [depends:tests_positive,tests_negative,dependency_isolation] Document opt-in live smoke-test configuration and run deterministic adapter tests, workspace checks, and Cairn gates with evidence. r[agent_harness_runtime.std_runtime_adapters.closeout]
