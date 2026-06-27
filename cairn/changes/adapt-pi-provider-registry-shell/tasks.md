# Tasks

- [ ] [serial] Define shell-owned provider/model registry boundary types and document that reusable runtime crates consume only injected services. r[agent_harness_runtime.pi_provider_registry.shell_registry]
- [ ] [depends:shell_registry] Add boundary tests or static checks proving runtime model dispatch does not read provider config, inspect credentials, or construct clients. r[agent_harness_runtime.pi_provider_registry.shell_registry.injected]
- [ ] [serial] Define typed provider/model metadata for IDs, API kind, endpoint identity, modalities, context/output budgets, reasoning support, thinking levels, and cost metadata. r[agent_harness_runtime.pi_provider_registry.metadata]
- [ ] [depends:metadata] Add selection tests for supported models and thinking-level clamp/reject behavior. r[agent_harness_runtime.pi_provider_registry.metadata.thinking]
- [ ] [serial] Add Nickel contracts for Animus-owned provider registry config and generated/check workflow if config is checked in. r[agent_harness_runtime.pi_provider_registry.nickel_config]
- [ ] [depends:nickel_config] Add invalid-config fixtures for bad IDs, unsupported API kinds, non-positive budgets, and missing endpoint identity. r[agent_harness_runtime.pi_provider_registry.nickel_config.invalid]
- [ ] [serial] Define credential resolver shell service and redacted diagnostic classes for missing/unavailable credentials. r[agent_harness_runtime.pi_provider_registry.credentials]
- [ ] [depends:credentials] Add negative credential tests proving secret values, env values, OAuth tokens, and credential file contents are not exposed. r[agent_harness_runtime.pi_provider_registry.credentials.missing]
- [ ] [serial] Add validation path for host-provided or dynamically registered provider entries. r[agent_harness_runtime.pi_provider_registry.dynamic_validation]
- [ ] [depends:dynamic_validation] Add dynamic-provider tests proving malformed entries are rejected and active model services remain unchanged. r[agent_harness_runtime.pi_provider_registry.dynamic_validation.reject]
- [ ] [depends:shell_registry.injected,metadata.thinking,nickel_config.invalid,credentials.missing,dynamic_validation.reject] Update docs and run focused registry tests, config checks, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.pi_provider_registry.shell_registry]
