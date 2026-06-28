# Tasks

- [ ] [serial] Define artifact delivery request DTOs with artifact handle, artifact kind, target kind, active context label, capability/toolset metadata, policy decision, attempt ID, retryability, and adapter outcome. r[agent_harness_runtime.clankers_tool_gateway.delivery_dto]
- [ ] [depends:delivery_dto] Add local/session delivery normalization tests proving raw artifact bytes are not embedded. r[agent_harness_runtime.clankers_tool_gateway.delivery_dto.local]
- [ ] [serial] Implement gateway policy validation for unsupported targets, missing contexts, raw destinations, raw credentials, headers, cloud URLs, webhook URLs, and platform payloads. r[agent_harness_runtime.clankers_tool_gateway.policy]
- [ ] [depends:policy] Add raw webhook/header/cloud/Matrix destination rejection tests proving adapters are not invoked. r[agent_harness_runtime.clankers_tool_gateway.policy.raw_destination]
- [ ] [serial] Define injected delivery adapter contracts and missing/unavailable/denied/malformed-output failure classes. r[agent_harness_runtime.clankers_tool_gateway.adapters]
- [ ] [depends:adapters] Add missing-adapter tests proving no success receipt is emitted without adapter execution. r[agent_harness_runtime.clankers_tool_gateway.adapters.missing]
- [ ] [serial] Define safe delivery outbox records and retry-by-attempt-ID decision logic with current-policy revalidation. r[agent_harness_runtime.clankers_tool_gateway.outbox]
- [ ] [depends:outbox] Add non-retryable policy-denial retry tests preserving original safe failure class. r[agent_harness_runtime.clankers_tool_gateway.outbox.non_retryable]
- [ ] [serial] Define safe delivery receipt DTOs with attempt/status/target/artifact/retry/adapter/handle/error metadata and BLAKE3 receipt hash. r[agent_harness_runtime.clankers_tool_gateway.receipts]
- [ ] [depends:receipts] Add leak fixtures for destinations, tokens, headers, full paths, artifact bytes, and platform payloads. r[agent_harness_runtime.clankers_tool_gateway.receipts.rejects_leaks]
- [ ] [depends:delivery_dto.local,policy.raw_destination,adapters.missing,outbox.non_retryable,receipts.rejects_leaks] Update docs and run focused Tool Gateway tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_tool_gateway.verification]
