## ADDED Requirements

### Requirement: Exact local-dspark Responses endpoint is admitted

r[agent_harness_runtime.local_dspark_responses.endpoint] Animus MUST admit only the exact `local-dspark-responses` identity and `http://192.168.1.244:8888/v1/responses` URL pair for the local cohort.

#### Scenario: Exact local pair is admitted

r[agent_harness_runtime.local_dspark_responses.endpoint.pass]
- GIVEN the exact local endpoint identity and URL
- WHEN live host configuration validation runs
- THEN the pair is admitted with explicit timeout and response-byte bounds

#### Scenario: Crossed or arbitrary pair is denied

r[agent_harness_runtime.local_dspark_responses.endpoint.failure]
- GIVEN a local identity with another URL, another identity with the local URL, or an arbitrary URL
- WHEN live host configuration validation runs
- THEN it fails before credential use or network contact

### Requirement: Cleartext transport remains an exact private exception

r[agent_harness_runtime.local_dspark_responses.transport] The live host MUST require HTTPS for the OpenAI and OpenRouter cohorts and MUST permit HTTP only for the exact admitted local-dspark pair.

#### Scenario: Local transport is constructed without broad HTTP authority

r[agent_harness_runtime.local_dspark_responses.transport.pass]
- GIVEN the exact local-dspark configuration
- WHEN the concrete client is constructed
- THEN it permits that HTTP request while redirects, ambient proxies, retries, discovery, and fallback remain disabled

#### Scenario: Another cleartext endpoint is rejected

r[agent_harness_runtime.local_dspark_responses.transport.failure]
- GIVEN any other HTTP endpoint identity or URL
- WHEN configuration admission runs
- THEN the client is not constructed

### Requirement: Local reasoning disable is explicit

r[agent_harness_runtime.local_dspark_responses.reasoning] The local-dspark request projection MUST send explicit `reasoning.effort = "none"` when canonical reasoning is disabled, without changing projection for other cohorts.

#### Scenario: Disabled local reasoning is explicit

r[agent_harness_runtime.local_dspark_responses.reasoning.pass]
- GIVEN an admitted local-dspark request with reasoning disabled
- WHEN text or tool-capable request projection runs
- THEN the wire body contains explicit disabled reasoning

#### Scenario: Other cohorts keep their existing projection

r[agent_harness_runtime.local_dspark_responses.reasoning.isolation]
- GIVEN an OpenAI or OpenRouter request with reasoning disabled
- WHEN request projection runs
- THEN the wire body omits reasoning as before

### Requirement: Local tool streams retain the common safety boundary

r[agent_harness_runtime.local_dspark_responses.streaming] The local cohort MUST use the existing bounded production SSE parser and MUST continue to reject hidden-reasoning, malformed, overbound, stale, or uncorrelated events.

#### Scenario: Compatible local tool stream is accepted

r[agent_harness_runtime.local_dspark_responses.streaming.pass]
- GIVEN the reviewed local model emits the admitted Responses tool-event sequence with reasoning disabled
- WHEN the production parser processes the stream
- THEN it returns correlated bounded tool-call or content outcomes

#### Scenario: Hidden reasoning still fails closed

r[agent_harness_runtime.local_dspark_responses.streaming.failure]
- GIVEN a local stream contains `response.reasoning_text.*`
- WHEN the production parser processes the stream
- THEN it returns the existing unsafe-reasoning failure

### Requirement: Local-dspark verification binds the reviewed model

r[agent_harness_runtime.local_dspark_responses.verification] Verification MUST include positive and negative endpoint, transport, request-projection, parser, credential, and model-pin checks plus a sanitized live smoke for `deepseek-v4-flash-0731-ablit-100`.

#### Scenario: Current-head live smoke passes safely

r[agent_harness_runtime.local_dspark_responses.verification.live]
- GIVEN the exact private service and explicit bounded credential value are available
- WHEN the packaged current-head smoke runs
- THEN it passes without printing prompts, credentials, reasoning, model output, tool arguments, tool output, or raw responses
