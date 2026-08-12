# agent-harness-runtime Change Spec: Packaged live OpenAI smoke

## ADDED Requirements

### Requirement: Packaged live OpenAI smoke uses the accepted host

r[agent_harness_runtime.packaged_live_openai_smoke.contract] Animus MUST provide a non-default packaged live OpenAI smoke that uses the accepted authority-scoped host and its exact one-attempt text cohort.

#### Scenario: Pure planning builds the reviewed request

r[agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
- GIVEN an operator supplies an admitted model identity
- WHEN pure smoke planning runs
- THEN it builds the fixed endpoint, prompt, timeout, response limit, and non-streaming dispatch plan
- AND it adds no retry, fallback, discovery, tool, image, or structured-output behavior.

#### Scenario: Successful response produces safe evidence

r[agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
- GIVEN the accepted host returns a supported canonical outcome
- WHEN pure smoke evidence projection runs
- THEN it emits provider, endpoint, status, count, and BLAKE3 identity metadata
- AND it omits the prompt, API key, model output, raw response, headers, and full transport diagnostics.

### Requirement: Packaged live OpenAI input is explicit and bounded

r[agent_harness_runtime.packaged_live_openai_smoke.binary] The packaged smoke MUST require the model as an exact argument and the API key through bounded standard input before provider contact.

#### Scenario: Explicit inputs can reach one provider attempt

r[agent_harness_runtime.packaged_live_openai_smoke.binary.pass]
- GIVEN the operator supplies `--model MODEL`, `--credential-stdin`, and one admitted API key value
- WHEN the packaged smoke runs
- THEN it executes one request through `ReqwestOpenAiHost`
- AND it prints only metadata-only smoke evidence.

#### Scenario: Invalid input fails before provider contact

r[agent_harness_runtime.packaged_live_openai_smoke.binary.failure]
- GIVEN an argument is missing, duplicated, unknown, oversized, malformed, or unsafe
- WHEN the packaged smoke validates its input
- THEN it returns a stable failure without provider contact
- AND it does not print the API key or raw input.

### Requirement: Live provider claims require current-head evidence

r[agent_harness_runtime.packaged_live_openai_smoke.current_head_evidence] A `live-providers` release evidence claim MUST require a passing optional live-provider check whose receipt matches the recorded head.

#### Scenario: Current-head live receipt supports the bounded claim

r[agent_harness_runtime.packaged_live_openai_smoke.current_head_evidence.pass]
- GIVEN a passing optional `live-providers` check matches the recorded clean head
- WHEN release evidence validation runs
- THEN the index can contain the `live-providers` claim
- AND unrelated optional surfaces remain claims or non-claims from their own evidence.

#### Scenario: Stale or absent live receipt cannot support the claim

r[agent_harness_runtime.packaged_live_openai_smoke.current_head_evidence.failure]
- GIVEN the live-provider check is stale, absent, skipped, missing, or failed
- WHEN the index claims `live-providers`
- THEN validation rejects the claim
- AND a missing explicit live run remains a `live-providers` non-claim.

### Requirement: Packaged live OpenAI smoke has focused verification

r[agent_harness_runtime.packaged_live_openai_smoke.verification] The change MUST include positive and negative tests for smoke planning, credential input, argument parsing, output omission, stale receipt rejection, host reuse, dependency isolation, and packaged binary gating.

#### Scenario: Credential-free closeout remains honest

r[agent_harness_runtime.packaged_live_openai_smoke.evidence]
- GIVEN deterministic checks pass without an OpenAI credential
- WHEN maintainers publish closeout evidence
- THEN they can claim the packaged smoke implementation and denial coverage
- AND they MUST retain actual live OpenAI behavior as a non-claim with the explicit credential blocker.

#### Scenario: Closeout validation passes

r[agent_harness_runtime.packaged_live_openai_smoke.closeout]
- GIVEN implementation, tests, docs, and lifecycle artifacts are complete
- WHEN focused Rust and Nix rails and Cairn gates run
- THEN they pass without network access or provider credentials
- AND the evidence records whether the optional live run ran or remained blocked.
