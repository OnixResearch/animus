# Tasks

- [x] [serial] Define typed confirmation request DTOs with request ID, action kind, resource, ability, safe summary, policy hash, session/run identity, confirmation level, expiry, and redaction class. r[agent_harness_runtime.clankers_confirmation_broker.requests]
- [x] [depends:requests] Add protected mutation confirmation-request tests proving raw tool payloads are not embedded in receipts. r[agent_harness_runtime.clankers_confirmation_broker.requests.mutation]
- [x] [serial] Implement default-deny confirmation behavior for absent, unavailable, timed-out, cancelled, stale, malformed, and no-decision brokers. r[agent_harness_runtime.clankers_confirmation_broker.default_deny]
- [x] [depends:default_deny] Add missing-broker, timeout, cancellation, malformed-decision, and no-decision tests proving no action dispatch occurs. r[agent_harness_runtime.clankers_confirmation_broker.default_deny.missing]
- [x] [serial] Add decision binding validation for request ID, action kind, resource, ability, policy hash, and expiry. r[agent_harness_runtime.clankers_confirmation_broker.binding]
- [x] [depends:binding] Add mismatched request/resource/policy and stale-expiry approval tests. r[agent_harness_runtime.clankers_confirmation_broker.binding.mismatch]
- [x] [serial] Enforce host-owned interaction boundaries for UI prompts, terminal input, notifications, external services, and durable approval storage. r[agent_harness_runtime.clankers_confirmation_broker.host_owned]
- [x] [depends:host_owned] Add boundary tests proving reusable runtime only calls injected broker contracts. r[agent_harness_runtime.clankers_confirmation_broker.host_owned.no_ui]
- [x] [serial] Define safe confirmation receipts with request/action/resource/ability/decision/broker/policy/timing/expiry metadata and BLAKE3 receipt hash. r[agent_harness_runtime.clankers_confirmation_broker.receipts]
- [x] [depends:receipts] Add leak fixtures for raw prompts, credentials, full paths, provider JSON, raw tool payloads, and secret summaries. r[agent_harness_runtime.clankers_confirmation_broker.receipts.rejects_leaks]
- [x] [depends:requests.mutation,default_deny.missing,binding.mismatch,host_owned.no_ui,receipts.rejects_leaks] Update docs and run focused confirmation tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_confirmation_broker.verification]
