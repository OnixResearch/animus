# Proposal: Bind durable model-request snapshots

## Why

Animus already composes provider-neutral model requests from durable session history, model profiles, request-context contributions, tool declarations, and current authority facts. Provider dispatch also requires durable user-message acceptance and a current runtime authority seal.

The current contracts do not require one private durable record that reconstructs the complete model-visible request before provider contact. A later audit can therefore prove that separate inputs existed without proving which exact combination reached the selected provider adapter.

DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a` provides a useful model-visible-state invariant. Animus should adapt that invariant through its existing ledger, model-invocation, request-context, authority-seal, and provider-dispatch owners. It should not adopt Cordis, TypeScript plugins, provider-native payload storage, or ambient service lookup.

## What Changes

- Add a provider-neutral private model-request snapshot contract that binds the complete ordered model-visible request and every composition identity. r[agent_harness_runtime.model_request_snapshot.projection]
- Commit the exact snapshot through the existing durable session boundary before provider dispatch. r[agent_harness_runtime.model_request_snapshot.durability]
- Require model dispatch and same-request retry to cite the committed snapshot identity and exact adapter cohort. r[agent_harness_runtime.model_request_snapshot.dispatch]
- Treat a committed request without a terminal provider observation as an explicit uncertain recovery state. r[agent_harness_runtime.model_request_snapshot.recovery]
- Keep public events and receipts metadata-only while preserving the exact request in the private durable journal. r[agent_harness_runtime.model_request_snapshot.evidence]
- Add positive and negative production-path tests and a focused verification rail. r[agent_harness_runtime.model_request_snapshot.verification]

## Prerequisites

- `agent_harness_runtime.coding_session_host.commit_order`
- `agent_harness_runtime.active_turn_recovery.durable_acceptance`
- `agent_harness_runtime.model_runtime_profiles.dispatch_binding`
- `agent_harness_runtime.request_context_contributions.composition`
- `agent_harness_runtime.runtime_authority_seal.monotonic`
- `agent_harness_runtime.provider_retry.same_request`

## Impact

- **Pure core**: provider-neutral snapshot validation, canonical identity, dispatch binding, and recovery decisions in `animus-session-core` with compatibility re-exports from `animus-runtime`.
- **Shell**: private durable commit and provider gating in `animus-std::coding_session_host`.
- **Evidence**: safe identities, counts, adapter cohort, status, and non-claims only.
- **Existing active work**: `add-live-steel-agent-shell` must consume this contract before live provider dispatch.

## Out of Scope

- Provider-native wire-byte capture, credentials, transport headers, hidden provider defaults, provider correctness, token-by-token persistence, request replay by default, or task correctness.
- A new session store, provider router, profile format, plugin system, or event bus.

## Affected Specs

- `agent-harness-runtime`: private request projection, durability, dispatch binding, recovery, evidence, and verification.
