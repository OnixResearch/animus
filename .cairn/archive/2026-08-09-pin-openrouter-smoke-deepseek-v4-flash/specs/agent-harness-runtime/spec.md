# agent-harness-runtime Change Spec: Reviewed OpenRouter smoke model

## ADDED Requirements

### Requirement: Packaged live OpenRouter smoke pins the reviewed model

r[agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model] The packaged live OpenRouter smoke MUST use model `deepseek/deepseek-v4-flash-0731` and MUST reject every other model before credential use or network contact.

#### Scenario: Reviewed model reaches planning

r[agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.pass]
- GIVEN the operator supplies `--model deepseek/deepseek-v4-flash-0731`
- WHEN pure OpenRouter smoke planning runs
- THEN it binds that exact model to the reviewed OpenRouter request.

#### Scenario: Model substitution fails closed

r[agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.failure]
- GIVEN the operator supplies any other model identity
- WHEN pure OpenRouter smoke planning runs
- THEN it returns a stable model error before credential use or network contact.

### Requirement: Reviewed OpenRouter model pin has focused verification

r[agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model_verification] The model pin MUST have positive fixed-model tests and negative substitution tests in the packaged focused rail.

#### Scenario: Focused checks bind the reviewed model

r[agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model_closeout]
- GIVEN code, tests, docs, and lifecycle artifacts are complete
- WHEN focused Rust, Nix, Cairn, and clean-head live checks run
- THEN they pass with DeepSeek V4 Flash 0731
- AND model quality, routing, reliability, billing, and production readiness remain non-claims.
