# agent-harness-runtime Change Spec: Packaged live OpenRouter smoke

## ADDED Requirements

### Requirement: OpenAI-compatible host admits the exact OpenRouter endpoint

r[agent_harness_runtime.packaged_live_openrouter_smoke.endpoint] Animus MUST admit the exact OpenRouter Responses identity and URL pair without weakening official OpenAI endpoint admission.

#### Scenario: Exact OpenRouter pair is admitted

r[agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.pass]
- GIVEN the OpenRouter endpoint identity and `https://openrouter.ai/api/v1/responses`
- WHEN host configuration validation runs
- THEN it admits the pair with the existing timeout and response bounds.

#### Scenario: Crossed or arbitrary endpoint is denied

r[agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.failure]
- GIVEN an OpenAI identity with the OpenRouter URL, an OpenRouter identity with the OpenAI URL, or an arbitrary URL
- WHEN host configuration validation runs
- THEN it rejects the configuration before credential use or network contact.

### Requirement: Packaged live OpenRouter smoke uses the accepted host

r[agent_harness_runtime.packaged_live_openrouter_smoke.contract] Animus MUST provide a non-default packaged OpenRouter smoke that uses the accepted request projection, bounded normalization, and one-attempt HTTPS shell.

#### Scenario: Pure planning builds the reviewed request

r[agent_harness_runtime.packaged_live_openrouter_smoke.contract.plan]
- GIVEN an admitted OpenRouter model identity
- WHEN pure smoke planning runs
- THEN it binds provider `openrouter`, endpoint `openrouter-responses`, the provider-qualified model, the fixed prompt, and the safe receipt policy
- AND model identifiers admit `/` without widening provider or credential identifiers
- AND it adds no retry, fallback, discovery, tool, image, or structured-output behavior.

#### Scenario: Successful response produces safe evidence

r[agent_harness_runtime.packaged_live_openrouter_smoke.contract.evidence]
- GIVEN the host returns a supported canonical outcome
- WHEN pure evidence projection runs
- THEN it emits provider, endpoint, status, count, and BLAKE3 identity metadata
- AND it omits the prompt, API key, model output, raw response, headers, and full transport diagnostics.

### Requirement: Packaged live OpenRouter input is explicit and bounded

r[agent_harness_runtime.packaged_live_openrouter_smoke.binary] The packaged smoke MUST require an exact model argument and the API key through bounded standard input.

#### Scenario: Explicit inputs reach one provider attempt

r[agent_harness_runtime.packaged_live_openrouter_smoke.binary.pass]
- GIVEN `--model MODEL`, `--credential-stdin`, and one admitted OpenRouter API key
- WHEN the packaged smoke runs
- THEN it executes one request through the accepted host
- AND it prints only metadata-only evidence.

#### Scenario: Invalid input fails before provider contact

r[agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
- GIVEN an argument is missing, duplicated, unknown, oversized, malformed, or unsafe
- WHEN input validation runs
- THEN it returns a stable failure without provider contact
- AND it does not print the API key or raw input.

### Requirement: Packaged live OpenRouter smoke has focused verification

r[agent_harness_runtime.packaged_live_openrouter_smoke.verification] The change MUST include positive and negative tests for endpoint admission, planning, credential input, argument parsing, output omission, host reuse, and package gating.

#### Scenario: Focused documentation and rails cover the cohort

r[agent_harness_runtime.packaged_live_openrouter_smoke.docs]
- GIVEN the OpenRouter cohort is implemented
- WHEN maintainers inspect operator docs and focused rails
- THEN they find the exact endpoint, explicit input command, safe output boundary, and deterministic checks.

#### Scenario: Current-head live closeout passes

r[agent_harness_runtime.packaged_live_openrouter_smoke.closeout]
- GIVEN the final indexed revision is clean and the Pi OpenRouter key is supplied through standard input
- WHEN the packaged smoke and current-head release evidence validation run
- THEN both pass
- AND the evidence retains reliability, quality, billing, routing, and production readiness as non-claims.
