# agent-harness-runtime Change Spec: Live OpenAI provider host

## ADDED Requirements

### Requirement: Live OpenAI provider authority is isolated

r[agent_harness_runtime.live_openai_provider_host.boundary] Animus MUST expose live OpenAI Responses execution only through a non-default authority-scoped host crate that requires explicit network and credential authority.

#### Scenario: Default graphs omit live provider authority

r[agent_harness_runtime.live_openai_provider_host.boundary.default_graph]
- GIVEN a consumer builds the default, no-default, or reusable runtime graph
- WHEN Cargo resolves its dependencies
- THEN the graph excludes the live OpenAI host and its HTTPS client
- AND no reusable crate gains network or credential authority.

#### Scenario: Explicit root feature enables the host

r[agent_harness_runtime.live_openai_provider_host.boundary.explicit_feature]
- GIVEN a consumer selects the root live OpenAI host feature
- WHEN the root facade compiles
- THEN it exposes the authority-scoped host crate
- AND the selected feature does not enable provider discovery or default credential lookup.

### Requirement: Live request projection fails closed

r[agent_harness_runtime.live_openai_provider_host.request_projection] The live OpenAI host MUST project only admitted non-streaming text requests and MUST reject unsupported or lossy request fields before network contact.

#### Scenario: Supported text request becomes exact JSON

r[agent_harness_runtime.live_openai_provider_host.request_projection.text]
- GIVEN an admitted OpenAI Responses plan with text messages and supported controls
- WHEN pure live request projection runs
- THEN it produces deterministic provider JSON for the exact selected model
- AND it preserves the output limit, reasoning effort, sampling control, and safe metadata.

#### Scenario: Unsupported request cannot reach transport

r[agent_harness_runtime.live_openai_provider_host.request_projection.unsupported]
- GIVEN a plan requests streaming, images, tools, structured output, or an unsupported role
- WHEN live request admission runs
- THEN it returns a typed rejection before transport
- AND it does not silently remove or downgrade the unsupported field.

### Requirement: Live response normalization is bounded and safe

r[agent_harness_runtime.live_openai_provider_host.response_normalization] The live OpenAI host MUST parse only bounded successful response bytes and MUST normalize supported output through the existing canonical provider outcome path.

#### Scenario: Completed text response becomes a canonical outcome

r[agent_harness_runtime.live_openai_provider_host.response_normalization.text]
- GIVEN a bounded successful OpenAI Responses body
- WHEN pure response normalization runs
- THEN it returns canonical assistant text, usage, finish status, and a safe provider receipt
- AND raw transport bodies, headers, credentials, and hidden reasoning are absent from durable evidence.

#### Scenario: Unsafe response fails closed

r[agent_harness_runtime.live_openai_provider_host.response_normalization.failure]
- GIVEN a response is oversized, malformed, incomplete, non-successful, or unsupported
- WHEN response classification or normalization runs
- THEN it returns a bounded typed error
- AND its diagnostic contains no response body or credential value.

### Requirement: HTTPS transport uses explicit bounded authority

r[agent_harness_runtime.live_openai_provider_host.transport] The live OpenAI shell MUST require the exact admitted endpoint, explicit API key value, timeout, response-byte limit, and cancellation observation for each HTTPS call.

#### Scenario: Admitted call uses one bounded attempt

r[agent_harness_runtime.live_openai_provider_host.transport.single_attempt]
- GIVEN the request, endpoint, credential, timeout, limit, and cancellation state are admitted
- WHEN the host executes the call
- THEN it performs one HTTPS POST with bounded response reading
- AND it performs no hidden retry, fallback, endpoint discovery, or credential discovery.

#### Scenario: Cancellation blocks provider contact

r[agent_harness_runtime.live_openai_provider_host.transport.cancelled]
- GIVEN the host observes cancellation before dispatch
- WHEN execution starts
- THEN it returns a cancellation error before provider contact
- AND it emits no network request.

### Requirement: Live provider evidence remains explicit

r[agent_harness_runtime.live_openai_provider_host.evidence] Animus MUST distinguish deterministic host-adapter evidence from optional live-provider evidence and MUST retain absent live evidence as a non-claim.

#### Scenario: Default closeout does not claim live behavior

r[agent_harness_runtime.live_openai_provider_host.evidence.non_claim]
- GIVEN deterministic host tests and focused rails pass without an API credential
- WHEN maintainers publish the result
- THEN they may claim the bounded OpenAI host implementation and fake-transport coverage
- AND they MUST NOT claim live OpenAI availability, quality, reliability, billing behavior, or production readiness.

### Requirement: Live OpenAI host verification covers failure paths

r[agent_harness_runtime.live_openai_provider_host.verification] The change MUST include positive and negative tests for projection, endpoint admission, credentials, cancellation, transport status, body bounds, response parsing, receipt safety, facade wiring, and dependency isolation.

#### Scenario: Closeout evidence is complete

r[agent_harness_runtime.live_openai_provider_host.verification.closeout]
- GIVEN maintainers prepare to archive the change
- WHEN closeout validation runs
- THEN focused tests, Clippy, dependency checks, Cairn validation, and all Cairn gates pass
- AND skipped live smoke remains an explicit non-claim.
