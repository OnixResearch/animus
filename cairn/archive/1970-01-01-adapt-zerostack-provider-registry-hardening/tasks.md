# Tasks

- [x] [serial] Define OpenAI-family API style metadata and resolution policy for first-party and compatible gateway providers. r[agent_harness_runtime.zerostack_provider_hardening.api_style]
- [x] [depends:api_style] Add positive first-party and custom-gateway tests plus ambiguous-style negative tests. r[agent_harness_runtime.zerostack_provider_hardening.api_style.gateway]
- [x] [serial] Define custom header descriptors with safe environment-reference resolution and redacted diagnostics. r[agent_harness_runtime.zerostack_provider_hardening.headers]
- [x] [depends:headers] Add negative tests for invalid header names, missing environment references, and secret-leaking diagnostics. r[agent_harness_runtime.zerostack_provider_hardening.headers.missing]
- [x] [serial] Define provider/model-scoped extra body descriptor admission, merge, and collision policy. r[agent_harness_runtime.zerostack_provider_hardening.extra_body]
- [x] [depends:extra_body] Add negative tests proving extra body descriptors cannot silently cross provider/model scope. r[agent_harness_runtime.zerostack_provider_hardening.extra_body.scope]
- [x] [serial] Add dynamic model-listing normalization and non-agent model class filtering. r[agent_harness_runtime.zerostack_provider_hardening.model_filter]
- [x] [depends:model_filter] Add negative model selection tests for embedding, image, audio, moderation, and rerank classes. r[agent_harness_runtime.zerostack_provider_hardening.model_filter.reject]
- [x] [depends:api_style,headers,extra_body,model_filter] Add provider selection receipts with redacted credential, endpoint, header, and payload metadata. r[agent_harness_runtime.zerostack_provider_hardening.receipts]
- [x] [depends:receipts] Update docs and run focused provider registry tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.zerostack_provider_hardening.receipts.endpoint]
