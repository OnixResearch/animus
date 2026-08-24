# Agent Harness Runtime Delta

## ADDED Requirements

### Requirement: Exact live Steel provider cohort

r[agent_harness_runtime.live_steel_agent_provider_cohort] The live Steel agent shell MUST admit only the reviewed OpenAI and OpenRouter Responses endpoint tuples. It MUST bind each provider to its matching injected credential reference before provider construction.

#### Scenario: Reviewed provider is selected

r[agent_harness_runtime.live_steel_agent_provider_cohort.valid]
- GIVEN a complete OpenAI or OpenRouter provider tuple
- WHEN the live Steel configuration is admitted
- THEN the shell MUST select the matching provider, endpoint, and credential reference

#### Scenario: Provider tuple is crossed

r[agent_harness_runtime.live_steel_agent_provider_cohort.crossed]
- GIVEN a provider identity with the other provider's endpoint or credential reference
- WHEN the live Steel configuration is admitted
- THEN admission MUST fail before credential use or network contact

### Requirement: Provider selection remains closed

r[agent_harness_runtime.live_steel_agent_provider_cohort.closed] The live Steel agent shell MUST NOT discover providers, accept arbitrary endpoint URLs, use OAuth, or fall back to another provider.

#### Scenario: Unknown provider is requested

r[agent_harness_runtime.live_steel_agent_provider_cohort.closed.unknown]
- GIVEN an unknown provider or arbitrary endpoint URL
- WHEN the live Steel configuration is admitted
- THEN admission MUST return a typed provider mismatch
- AND it MUST NOT construct a provider host

#### Scenario: Selected provider fails

r[agent_harness_runtime.live_steel_agent_provider_cohort.closed.no_fallback]
- GIVEN one admitted provider and no declared fallback
- WHEN provider execution fails
- THEN the shell MUST return the provider failure
- AND it MUST NOT contact another provider
