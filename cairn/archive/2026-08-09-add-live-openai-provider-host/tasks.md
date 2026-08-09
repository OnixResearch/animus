# Tasks

## Phase 1: Contract and core

- [x] [serial] Add the non-default authority-scoped crate, explicit host configuration, endpoint admission, secret wrapper, and root feature. r[agent_harness_runtime.live_openai_provider_host.boundary]
- [x] [depends:boundary] Implement pure text-only OpenAI request projection with fail-closed rejection for unsupported plan fields. r[agent_harness_runtime.live_openai_provider_host.request_projection]
- [x] [depends:request_projection] Implement bounded pure OpenAI response parsing and canonical outcome projection without raw transport evidence. r[agent_harness_runtime.live_openai_provider_host.response_normalization]

## Phase 2: Shell and evidence

- [x] [depends:response_normalization] Add the thin optional HTTPS shell with explicit credentials, timeout, cancellation, bounded reads, and no retry or fallback. r[agent_harness_runtime.live_openai_provider_host.transport]
- [x] [depends:transport] Add positive and negative fake-transport tests, an ignored opt-in live smoke, root facade coverage, and dependency-isolation checks. r[agent_harness_runtime.live_openai_provider_host.verification]
- [x] [depends:verification] Update host metadata, readiness docs, README guidance, focused Rust/Nix rails, and explicit live-evidence non-claims. r[agent_harness_runtime.live_openai_provider_host.evidence]
- [x] [depends:evidence] Run focused tests, Clippy, Cairn validation and gates, sync the accepted spec, archive the change, and record closeout evidence. r[agent_harness_runtime.live_openai_provider_host.verification]
