# Tasks

## 1. Baseline and ownership

- [x] [serial] 1.1 Run the current agent-definition, semantic-role, UCAN-aligned authority, coding-session effect-gate, feature-isolation, and workspace baselines before core changes. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 1.2 Record the owning boundary for agent names, definition identities, DID principals, UCAN verification inputs, Basalt policy, local ceilings, dispatch, and receipts. r[agent_harness_runtime.basalt_authority_adapter.boundary]
- [x] [serial] 1.3 Inventory every live model, tool, filesystem, process, network, Steel, and semantic agent-role guarded-action path. Classify unsupported paths explicitly. r[agent_harness_runtime.basalt_authority_adapter.dispatch]
- [x] [serial] 1.4 Record the reviewed Basalt revision, vendored UCAN revision, schemas, features, license metadata, and compatibility assumptions. r[agent_harness_runtime.basalt_authority_adapter.compatibility]

## 2. Define implementation-neutral contracts

- [x] [serial] 2.1 Add generic host-authority request, decision, issue, binding, receipt-projection, and injected-service contracts without Basalt or UCAN concrete types. r[agent_harness_runtime.basalt_authority_adapter.port]
- [x] [serial] 2.2 Add pure validation for action, contract, resource, ability, agent-definition, subject, task, actor, role-plan, session, binding, policy, freshness, digest, bound, and correlation identities. r[agent_harness_runtime.basalt_authority_adapter.port]
- [x] [serial] 2.3 Add positive and negative tests for generic contract validation, deterministic issue order, unsupported actions, and unsafe input rejection. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 2.4 Prove through dependency and public-API checks that generic Animus crates expose no Basalt or UCAN concrete types. r[agent_harness_runtime.basalt_authority_adapter.isolation]

## 3. Bind agents to UCAN principals

- [x] [serial] 3.1 Add a pure `AgentAuthorityBinding` that binds an agent-definition BLAKE3 identity to one UCAN audience DID, policy identity, and freshness identity. r[agent_harness_runtime.basalt_authority_adapter.agent_binding]
- [x] [serial] 3.2 Keep the agent name as safe discovery and presentation metadata that cannot satisfy principal, grant, role, or dispatch checks. r[agent_harness_runtime.basalt_authority_adapter.agent_binding]
- [x] [serial] 3.3 Reject duplicate, stale, cross-definition, cross-DID, cross-subject, cross-task, cross-role, and cross-session binding reuse. r[agent_harness_runtime.basalt_authority_adapter.agent_binding]
- [x] [serial] 3.4 Add positive binding fixtures and negative name-only, identity-drift, stale-binding, and replayed-binding fixtures. r[agent_harness_runtime.basalt_authority_adapter.verification]

## 4. Build the optional Basalt adapter

- [x] [serial] 4.1 Add the optional `animus-std-basalt-authority` crate with a pure translation core and thin imperative shell. r[agent_harness_runtime.basalt_authority_adapter.boundary]
- [x] [serial] 4.2 Add a non-default root `basalt-authority` feature without changing the compatibility-only `basalt-reference` behavior. r[agent_harness_runtime.basalt_authority_adapter.isolation]
- [x] [serial] 4.3 Add typed Nickel adapter policy for action-to-contract mapping, exact dependency identity, named limits, required non-claims, and deterministic runtime projection. r[agent_harness_runtime.basalt_authority_adapter.policy]
- [x] [serial] 4.4 Add pure exact translation from Animus requests and bindings to Basalt action, contract, resource, ability, operation profile, and request digest. r[agent_harness_runtime.basalt_authority_adapter.translation]
- [x] [serial] 4.5 Add pure projection from Basalt results to generic Animus decisions without widening or hiding denial classes. r[agent_harness_runtime.basalt_authority_adapter.intersection]
- [x] [serial] 4.6 Add property tests for deterministic mapping, exact binding, order rules, non-widening projection, and denial preservation. r[agent_harness_runtime.basalt_authority_adapter.verification]

## 5. Wire raw UCAN enforcement

- [x] [serial] 5.1 Use Basalt raw holder-signed invocation enforcement instead of treating caller-created grant DTOs as live authority. r[agent_harness_runtime.basalt_authority_adapter.raw_ucan]
- [x] [serial] 5.2 Inject key, proof, revocation, caveat, replay, time, and verification-limit services through explicit host-owned interfaces. r[agent_harness_runtime.basalt_authority_adapter.raw_ucan]
- [x] [serial] 5.3 Preserve Basalt ordering so replay admission is the final consuming authorization step and no later adapter step can widen authority. r[agent_harness_runtime.basalt_authority_adapter.raw_ucan]
- [x] [serial] 5.4 Add positive direct and delegated invocation fixtures plus negative token, holder, audience, time, proof, revocation, caveat, replay, policy, resource, ability, and digest fixtures. r[agent_harness_runtime.basalt_authority_adapter.verification]

## 6. Integrate guarded dispatch

- [x] [serial] 6.1 Invoke the generic authority port before every supported live guarded-action host effect identified by the inventory. r[agent_harness_runtime.basalt_authority_adapter.dispatch]
- [x] [serial] 6.2 Bind semantic agent-role assignment, activation, handoff, and output influence to their existing action-specific resources rather than a shared role grant. r[agent_harness_runtime.basalt_authority_adapter.dispatch]
- [x] [serial] 6.3 Compose Basalt as an additive gate with agent-definition, role, task, model, tool, workspace, approval, sandbox, cancellation, timeout, budget, and runtime-authority-seal decisions. r[agent_harness_runtime.basalt_authority_adapter.intersection]
- [x] [serial] 6.4 Fail closed when the configured provider, mapping, action, current verification input, decision commit, or required host gate is missing or unsupported. r[agent_harness_runtime.basalt_authority_adapter.dispatch]
- [x] [serial] 6.5 Add one fake-host end-to-end named-agent dispatch and negative tests proving that Basalt allow cannot reverse another Animus denial. r[agent_harness_runtime.basalt_authority_adapter.verification]

## 7. Add safe evidence and conformance rails

- [x] [parallel] 7.1 Add BLAKE3-bound metadata-only authority decisions and receipts with deterministic leak rejection. r[agent_harness_runtime.basalt_authority_adapter.receipts]
- [x] [parallel] 7.2 Add negative receipt fixtures for tokens, keys, signatures, proofs, caveat bodies, replay nonces, credentials, prompts, tool arguments, provider payloads, full paths, environment values, and outputs. r[agent_harness_runtime.basalt_authority_adapter.receipts]
- [x] [serial] 7.3 Add a focused structural and conformance rail that detects supported guarded dispatch paths without the required authority-port call. r[agent_harness_runtime.basalt_authority_adapter.dispatch]
- [x] [serial] 7.4 Add exact Basalt and vendored UCAN cohort checks, positive and negative compatibility fixtures, and explicit AGPL consumption evidence. r[agent_harness_runtime.basalt_authority_adapter.compatibility]
- [x] [serial] 7.5 Update authority, agent-definition, semantic-role, component, contract, facade, safety, and release-evidence documentation with bounded non-claims. r[agent_harness_runtime.basalt_authority_adapter.receipts]

## 8. Validate and close out

- [x] [serial] 8.1 Run focused generic-port, agent-binding, translation, raw-UCAN, dispatch, receipt, compatibility, and license tests. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 8.2 Run default, `no_std`, Wasm, runtime, Steel, TUI, unrelated `std`, and feature-enabled dependency-isolation checks. r[agent_harness_runtime.basalt_authority_adapter.isolation]
- [x] [serial] 8.3 Run `cargo fmt --check`, focused Clippy, `cargo check --workspace --all-targets`, and the relevant workspace tests. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 8.4 Run Nickel policy freshness, adapter conformance, component catalog, runtime facade, safe receipt, and relevant Nix checks. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 8.5 Run Cairn validation, requirement coverage, proposal gate, design gate, and tasks gate. r[agent_harness_runtime.basalt_authority_adapter.verification]
- [x] [serial] 8.6 Record exact evidence, blockers, non-claims, immutable dependency identities, and an archived completion receipt before sync and archive. r[agent_harness_runtime.basalt_authority_adapter.verification]
