## Why

Animus already parses usage records from provider streams and routes on cost through `clankers_cost_model_routing`, but it has no per-agent accounting projection. Hosts cannot report an approximate per-agent subtree spend, priced against the model that served each record, without reconciliation against a provider billing system.

A pure accounting projection contract should accept typed usage records, price them with registry price metadata and a documented fallback, project an approximate per-agent subtree figure, and mark the result as a non-authoritative estimate. Provider clients, billing APIs, and ledger persistence remain host-owned.

## What Changes

- Accept typed usage records and bind each record to the agent and session that produced it. r[agent_harness_runtime.usage_accounting.records]
- Price each accepted record against the model that served it using registry price metadata, with a documented fallback price and explicit missing-price handling. r[agent_harness_runtime.usage_accounting.pricing]
- Project an approximate per-agent subtree figure that follows authenticated same-session creator relationships and resets with the session. r[agent_harness_runtime.usage_accounting.subtree]
- Mark every projection as a non-authoritative estimate, never provider billing reconciliation. r[agent_harness_runtime.usage_accounting.non_authoritative]
- Reject malformed records, unknown models, missing required prices, negative totals, overflow, and duplicate summaries with bounded safe error classes. r[agent_harness_runtime.usage_accounting.fail_closed]
- Emit metadata-only receipts. r[agent_harness_runtime.usage_accounting.receipts]
- Add positive and negative fixtures. r[agent_harness_runtime.usage_accounting.verification]

## Impact

- **Rust**: a pure no-std accounting decision core plus a non-default adapter surface; the existing cost-routing and provider-streaming surfaces stay authoritative.
- **Consumers**: hosts and UI surfaces can show an approximate per-agent subtree estimate with a documented pricing rule.
- **Claims**: the estimate is not provider billing reconciliation.
