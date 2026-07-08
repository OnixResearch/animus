# Tasks

- [ ] [serial] Define node descriptor scope and non-claims for paired peripherals, remote machine hosts, and platform devices without making nodes gateway owners. r[agent_harness_runtime.openclaw_nodes.scope]
- [ ] [depends:scope] Add node identity, pairing metadata, advertised caps, command allowlists, permission claims, and metadata-pinning validation. r[agent_harness_runtime.openclaw_nodes.advertisement]
- [ ] [depends:advertisement] Add pure node invocation plans and result envelopes over injected host services with idempotency, timeout, cancellation, safe payload, and typed failure classes. r[agent_harness_runtime.openclaw_nodes.invoke]
- [ ] [depends:invoke] Require canonical node-host exec plans, per-node approval evidence, and safe receipts before high-impact node commands can dispatch. r[agent_harness_runtime.openclaw_nodes.approvals]
- [ ] [depends:advertisement,approvals] Add positive and negative pairing tests for new devices, metadata drift, unpaired nodes, stale tokens, and host-owned pairing storage boundaries. r[agent_harness_runtime.openclaw_nodes.pairing]
- [ ] [depends:scope,advertisement,invoke,approvals,pairing] Update docs, facade inventory, and focused verification evidence for node capability hosts. r[agent_harness_runtime.openclaw_nodes.verification]
