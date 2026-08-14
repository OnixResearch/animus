# Tasks

## B. Contract and factory

- [ ] [serial] B1: Record current fake-only service implementations and map each child lifecycle operation to accepted owners. r[agent_harness_runtime.child_session_service.verification]
- [ ] [serial] B2: Define the explicit child-session factory, service construction values, handle registry, failure classes, and named bounds. r[agent_harness_runtime.child_session_service.factory]
- [ ] [depends:B2] B3: Add pure parent, profile, capability, context, workspace, model, tool, and budget admission. r[agent_harness_runtime.child_session_service.authority]

## H. Host lifecycle

- [ ] [depends:B3] H1: Implement enqueue by constructing one normal child coding session through the injected factory. r[agent_harness_runtime.child_session_service.lifecycle]
- [ ] [depends:H1] H2: Implement generation-fenced status, cancel, and collect operations over the explicit registry. r[agent_harness_runtime.child_session_service.lifecycle]
- [ ] [depends:H2] H3: Persist safe child lifecycle, session-head, and lineage facts through explicit store services. r[agent_harness_runtime.child_session_service.persistence]
- [ ] [depends:H3] H4: Return bounded typed results and explicit partial, failed, timed-out, and cancelled outcomes to the parent. r[agent_harness_runtime.child_session_service.results]
- [ ] [depends:H4] H5: Add restart and recovery behavior through existing coding-session and active-turn recovery gates. r[agent_harness_runtime.child_session_service.persistence]

## T. Tests and closeout

- [ ] [depends:H5] T1: Add positive tests for enqueue, status, collect, cancellation, restart, lineage, and RLM result projection. r[agent_harness_runtime.child_session_service.verification]
- [ ] [depends:H5] T2: Add negative tests for missing services, stale handles, foreign parents, authority widening, over-budget requests, malformed context, duplicate children, cancellation races, restore drift, transcript leaks, and ambient fallback. r[agent_harness_runtime.child_session_service.verification]
- [ ] [depends:T2] T3: Prove reusable and default dependency isolation and absence of a global registry. r[agent_harness_runtime.child_session_service.factory]
- [ ] [depends:T3] T4: Update docs and component metadata, add a focused rail, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.child_session_service.verification]
