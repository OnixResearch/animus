## Phase 1: Ownership and profile contract

- [x] [serial] Inventory the durable store, coding-session resume, model-profile, tool-catalog, permission, authorization, hook/plugin, context-policy, and Sandlock owners and document the no-duplication mapping. r[agent_harness_runtime.session_authority_profile.contract] r[agent_harness_runtime.session_authority_profile.verification]
- [x] [serial] Define metadata-only profile, component identity, comparison input, transition plan, realized-enforcement observation, event, and receipt DTOs with explicit redaction boundaries. r[agent_harness_runtime.session_authority_profile.contract] r[agent_harness_runtime.session_authority_profile.receipts]
- [x] [depends:agent_harness_runtime.session_authority_profile.contract] Define canonical ordering and domain-separated BLAKE3 identity inputs for the profile and every durable binding. r[agent_harness_runtime.session_authority_profile.identity]

## Phase 2: Functional core

- [x] [depends:agent_harness_runtime.session_authority_profile.identity] Implement pure component validation, canonicalization, profile identity construction, and legacy/unbound classification without filesystem, environment, process, network, clock, terminal, credential, or persistence effects. r[agent_harness_runtime.session_authority_profile.identity]
- [x] [depends:agent_harness_runtime.session_authority_profile.identity] Implement pure resume comparison for exact match, changed component, missing input, malformed identity, and unavailable required profile outcomes. r[agent_harness_runtime.session_authority_profile.resume_admission]
- [x] [depends:agent_harness_runtime.session_authority_profile.resume_admission] Implement pure fork/rebind transition planning that cannot authorize itself and never rewrites an existing committed profile. r[agent_harness_runtime.session_authority_profile.durable_binding] r[agent_harness_runtime.session_authority_profile.resume_admission]
- [x] [depends:agent_harness_runtime.session_authority_profile.contract] Implement metadata-only event and receipt projection plus leak validation. r[agent_harness_runtime.session_authority_profile.receipts]

## Phase 3: Store, facade, and host integration

- [x] [depends:agent_harness_runtime.session_authority_profile.durable_binding] Add committed profile-binding records to the durable store and enforce initial-profile commit before the first provider or tool effect. r[agent_harness_runtime.session_authority_profile.durable_binding]
- [x] [depends:agent_harness_runtime.session_authority_profile.resume_admission] Integrate profile comparison into coding-session resume before interrupted-tool repair, prompt admission, continuation, provider dispatch, or tool dispatch. r[agent_harness_runtime.session_authority_profile.resume_admission]
- [x] [depends:agent_harness_runtime.session_authority_profile.realized_enforcement] Wire the host shell to re-run authorization, capability, adapter-cohort, tool-readiness, provider, and Sandlock checks after profile comparison and before effects. r[agent_harness_runtime.session_authority_profile.realized_enforcement]
- [x] [depends:agent_harness_runtime.session_authority_profile.resume_admission] Expose explicit abandon, fork, and reauthorization/rebind outcomes through the session facade and UI-neutral runtime event stream. r[agent_harness_runtime.session_authority_profile.durable_binding] r[agent_harness_runtime.session_authority_profile.receipts]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive tests for deterministic identity, exact-profile restart, explicit authorized rebind, branch-local profile lineage, matching requested profile with newly successful host probes, and safe receipts. r[agent_harness_runtime.session_authority_profile.verification]
- [x] [parallel] Add negative tests for changed permission, sandbox, model, tool, hook/plugin, agent-definition, context/retrieval, policy, authorization, or adapter-cohort identity; missing and malformed components; legacy unbound sessions; failed reauthorization; unavailable enforcement; stale head; in-place rewrite attempts; and receipt leaks. r[agent_harness_runtime.session_authority_profile.resume_admission] r[agent_harness_runtime.session_authority_profile.realized_enforcement] r[agent_harness_runtime.session_authority_profile.verification]
- [x] [depends:agent_harness_runtime.session_authority_profile.verification] Add restart fault-injection scenarios proving no provider or tool effect crosses an uncommitted or mismatched authority-profile boundary. r[agent_harness_runtime.session_authority_profile.durable_binding] r[agent_harness_runtime.session_authority_profile.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update session restore, durable store, coding-session facade, model profiles, tool executor, sandbox, request lifecycle, safety, dogfood, component catalog, runtime facade, and README reference documentation. r[agent_harness_runtime.session_authority_profile.contract] r[agent_harness_runtime.session_authority_profile.receipts]
- [x] [serial] Add a focused authority-profile rail covering Nickel/Rust identity parity where configured, pure comparison, store ordering, facade resume, host re-probes, positive/negative fixtures, and receipt safety. r[agent_harness_runtime.session_authority_profile.verification]
- [x] [depends:agent_harness_runtime.session_authority_profile.verification] Run focused store/facade/profile/tool/sandbox tests, formatting, Clippy, composition and safe-receipt rails, dogfood claim hygiene, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.session_authority_profile.verification]
