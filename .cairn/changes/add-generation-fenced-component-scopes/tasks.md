# Tasks

## B. Baseline and boundary

- [ ] [serial] B1: Record DeepSeek Harness revision `47f943859bef60e4160492346772ded9b24f765a`, MIT license, developer-preview status, selected scoped-registration semantics, and rejected ambient-context behavior. r[agent_harness_runtime.component_scope.registration]
- [ ] [serial] B2: Run and record lifecycle-hook, agent-control, daemon-worker, transactional-handoff, component-catalog, runtime-facade, and evidence-safety baselines. r[agent_harness_runtime.component_scope.verification]

## C. Pure scope core

- [ ] [depends:generate-composition-activation-graph] C1: Define no-std scope, registration, lease, generation, parent, dependency, state, observation, effect-plan, issue, and receipt DTOs. r[agent_harness_runtime.component_scope.registration]
- [ ] [depends:C1] C2: Implement explicit-scope resolution with exact component, generation, visibility, parent, and scope checks and no ambient fallback. r[agent_harness_runtime.component_scope.resolution]
- [ ] [depends:C1] C3: Implement the pure open, closing, quiescing, releasing, closed, and failed-cleanup reducer. r[agent_harness_runtime.component_scope.teardown]
- [ ] [depends:C3] C4: Derive reverse release order from the admitted activation graph and reject unsupported ownership cycles. r[agent_harness_runtime.component_scope.teardown]
- [ ] [depends:C3] C5: Make repeated and concurrent close commands return one generation-bound plan and terminal identity. r[agent_harness_runtime.component_scope.concurrency]

## H. Host integration

- [ ] [depends:C4] H1: Define narrow injected ports for delivery close, cancellation request, quiescence observation, resource release, and terminal receipt commit. r[agent_harness_runtime.component_scope.teardown]
- [ ] [depends:H1] H2: Add a deterministic fake shell that executes only reducer-requested effects and preserves unresolved cleanup. r[agent_harness_runtime.component_scope.teardown] r[agent_harness_runtime.component_scope.evidence]
- [ ] [depends:H2] H3: Integrate worker-root scopes and separate client delivery leases into `add-local-daemon-supervisor-host` planning without changing client detach into worker stop. r[agent_harness_runtime.component_scope.registration]

## V. Verification and closeout

- [ ] [depends:H2] V1: Add positive fixtures for child registration, exact resolution, parent visibility, cancellation, quiescence, reverse release, repeated close, and clean terminal evidence. r[agent_harness_runtime.component_scope.verification]
- [ ] [depends:H2] V2: Add negative fixtures for stale generations, out-of-scope lookup, lookup after close, late delivery, wrong-generation observations, dependency cycles, duplicate release, deadline expiry, unresolved cleanup, receipt leaks, and authority confusion. r[agent_harness_runtime.component_scope.verification]
- [ ] [depends:V2] V3: Add a focused rail and update lifecycle, daemon, activation-graph, runtime-facade, component-catalog, safety, and README reference documentation. r[agent_harness_runtime.component_scope.evidence] r[agent_harness_runtime.component_scope.verification]
- [ ] [depends:V3] V4: Run focused tests before and after core changes, no-std and Wasm checks, formatting, Clippy, relevant lifecycle and composition rails, Cairn validation and gates, and relevant Nix checks. r[agent_harness_runtime.component_scope.verification]
