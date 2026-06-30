# agent-harness-runtime Change Spec: Pi provider registry shell adaptation

## Requirements

### Requirement: Shell-owned provider registry

r[agent_harness_runtime.pi_provider_registry.shell_registry] Animus MUST keep provider and model registry discovery, config loading, credential lookup, and concrete client construction in `animus-std` or host shells rather than reusable runtime core crates.

#### Scenario: Runtime receives injected model service

r[agent_harness_runtime.pi_provider_registry.shell_registry.injected]
- GIVEN the shell selects a provider/model from registry config
- WHEN runtime dispatches a model call
- THEN runtime calls only the injected model service with a validated request
- AND it does not read provider config, inspect credentials, or construct a network client.

### Requirement: Typed provider and model metadata

r[agent_harness_runtime.pi_provider_registry.metadata] Provider/model registry entries MUST expose typed provider IDs, model IDs, API kind, endpoint identity, input modalities, context budget, output budget, reasoning support, allowed thinking levels, and optional cost metadata.

#### Scenario: Capability metadata clamps thinking level

r[agent_harness_runtime.pi_provider_registry.metadata.thinking]
- GIVEN a selected model does not support a requested thinking level
- WHEN the shell resolves runtime model settings
- THEN the setting is clamped or rejected according to policy
- AND the runtime receives a valid selected model service and effective capability metadata.

### Requirement: Nickel-authored owned registry config

r[agent_harness_runtime.pi_provider_registry.nickel_config] Animus-owned provider registry configuration MUST be authored in Nickel with typed contracts and exported or checked before shell runtime consumption.

#### Scenario: Invalid provider config fails before use

r[agent_harness_runtime.pi_provider_registry.nickel_config.invalid]
- GIVEN a provider config has an invalid provider ID, unsupported API kind, non-positive budget, or missing required endpoint identity
- WHEN the config export or check runs
- THEN validation fails before the shell can select that provider
- AND no runtime model service is constructed from the invalid entry.

### Requirement: Redacted credential resolution

r[agent_harness_runtime.pi_provider_registry.credentials] Credential resolution MUST be an imperative-shell service that returns usable provider credentials or typed redacted failures without exposing raw tokens, environment values, OAuth secrets, or credential file contents.

#### Scenario: Missing credential reports safe diagnostic

r[agent_harness_runtime.pi_provider_registry.credentials.missing]
- GIVEN a selected provider requires a credential that is absent
- WHEN the shell resolves the provider service
- THEN resolution fails with a typed missing-credential diagnostic
- AND the diagnostic names only the credential class or source label, not any secret value.

### Requirement: Dynamic provider validation

r[agent_harness_runtime.pi_provider_registry.dynamic_validation] Host-provided or dynamically registered provider entries MUST pass the same provider/model metadata validation as file-backed registry entries before they can be selected.

#### Scenario: Dynamic invalid model is rejected

r[agent_harness_runtime.pi_provider_registry.dynamic_validation.reject]
- GIVEN a host registers a provider entry at runtime with malformed model metadata
- WHEN the shell validates the dynamic entry
- THEN the entry is rejected with typed diagnostics
- AND active runtime model services remain unchanged.
