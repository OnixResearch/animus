## Phase 1: Exact cohort admission

- [x] [serial] Add one pure provider-cohort selector for exact OpenAI and OpenRouter tuples. r[agent_harness_runtime.live_steel_agent_provider_cohort]
- [x] [serial] Bind selected-model credential metadata to the admitted tuple without ambient lookup. r[agent_harness_runtime.live_steel_agent_provider_cohort.valid]
- [x] [serial] Add positive tests for both cohorts and negative tests for crossed, arbitrary, and unknown tuples. r[agent_harness_runtime.live_steel_agent_provider_cohort.crossed]

## Phase 2: Closed execution and evidence

- [x] [serial] Document the two-cohort shell and its discovery, OAuth, and fallback non-claims. r[agent_harness_runtime.live_steel_agent_provider_cohort.closed]
- [ ] [serial] Run focused live-agent and provider rails, strict Clippy, Octet, Cairn validation, and the relevant Nix checks. r[agent_harness_runtime.live_steel_agent_provider_cohort.closed.unknown]
- [ ] [serial] Record safe evidence, sync the accepted specification, and archive the change. r[agent_harness_runtime.live_steel_agent_provider_cohort.closed.no_fallback]
