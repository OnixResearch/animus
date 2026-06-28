# Tasks

- [ ] [serial] Define optional dataspace adapter DTOs for publishing runtime assertions, subscribing to safe query patterns, and receiving host orchestration events. r[agent_harness_runtime.syndicate_dataspace.adapter_contract]
- [ ] [depends:adapter_contract] Define redacted assertion shapes for runtime events, tool states, background work, session/provenance summaries, and agent/subagent lifecycle states. r[agent_harness_runtime.syndicate_dataspace.assertion_projection]
- [ ] [depends:assertion_projection] Add scope, facet, session, tenant, workspace, branch, cancellation, expiry, and ownership-transfer semantics. r[agent_harness_runtime.syndicate_dataspace.scoping]
- [ ] [depends:adapter_contract] Implement deterministic fake-dataspace fixtures for publish/subscribe ordering, stale assertion cleanup, cancellation, malformed subscription, and denied query tests. r[agent_harness_runtime.syndicate_dataspace.fake_backend]
- [ ] [depends:fake_backend,scoping] Add positive and negative tests proving dataspace projections are bounded, scoped, cancellable, and safe by default. r[agent_harness_runtime.syndicate_dataspace.tests]
- [ ] [depends:adapter_contract] Add dependency-isolation checks proving Syndicate crates and live transport dependencies stay out of default reusable runtime/core crates. r[agent_harness_runtime.syndicate_dataspace.dependency_isolation]
- [ ] [depends:tests,dependency_isolation] Document optional live integration configuration and run fake-dataspace tests, dependency checks, workspace checks, and Cairn gates with evidence. r[agent_harness_runtime.syndicate_dataspace.closeout]
