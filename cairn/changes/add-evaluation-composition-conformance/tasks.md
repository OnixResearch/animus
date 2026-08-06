## Phase 1: Prerequisites and inventory

- [ ] [serial] B1 Block implementation until `add-native-evaluation-core`, `add-evaluation-adapter-family`, and `add-std-evaluation-host` are accepted or completed in the same reviewed batch. r[agent_harness_runtime.native_evaluation_conformance.production_paths]
- [ ] [serial] B2 Inventory current public entrypoints and focused rails for native evaluation, Harbor, ATIF, batch, ledger, roles, active-turn, provider streams, artifacts, evaluation feedback, dataset export, safe receipts, components, facades, and composition. r[agent_harness_runtime.native_evaluation_conformance.matrix]
- [ ] [serial] B3 Record the exact Verifiers scenario sources and immutable cohort used for fixture inspiration without copying Python runtime behavior or raw upstream traces. r[agent_harness_runtime.native_evaluation_conformance.evidence]

## Phase 2: Conformance profile and harness

- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.matrix] C1 Add a typed Nickel conformance profile for schema, crate/feature cohort, fixture families, public entrypoints, fake service catalog, bounds, expected outcomes, redaction, evidence, and non-claims. r[agent_harness_runtime.native_evaluation_conformance.matrix]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.matrix] C2 Export checked fixture metadata and add strict Rust revalidation plus domain-separated BLAKE3 profile, family, fixture, and expected-outcome identities. r[agent_harness_runtime.native_evaluation_conformance.matrix]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.production_paths] C3 Add a public-entrypoint fixture harness that constructs inputs and scripted fake services but delegates planning, admission, execution, reduction, resume, projection, and receipt behavior to production APIs. r[agent_harness_runtime.native_evaluation_conformance.production_paths]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.production_paths] C4 Add guards that reject private-helper-only fixtures, alternate planners or reducers, mock-call-count-only evidence, unknown entrypoints, and unregistered fixture families. r[agent_harness_runtime.native_evaluation_conformance.production_paths]

## Phase 3: Positive scenario families

- [ ] [parallel] P1 Add positive exact finite and bounded lazy taskset scenarios with stable BLAKE3 identity. r[agent_harness_runtime.native_evaluation_conformance.positive]
- [ ] [parallel] P2 Add positive single-agent, best-of-N/pass-at-N, user-simulation, and isolated-agentic-judge scenarios with exact episode and role bindings. r[agent_harness_runtime.native_evaluation_conformance.positive]
- [ ] [parallel] P3 Add positive ledger-reference, service-binding, cancellation, gateway exact-retry, terminal-after-commit, and sealed-teardown scenarios. r[agent_harness_runtime.native_evaluation_conformance.positive]
- [ ] [parallel] P4 Add positive isolated-verifier, allowlisted-artifact, finite-reward, atomic-output, reopen, exact-resume, and cleanup scenarios. r[agent_harness_runtime.native_evaluation_conformance.positive]
- [ ] [parallel] P5 Add positive native-to-Harbor, ATIF safe-subset, evaluation-feedback advisory, dataset-draft admission, and metadata-only receipt scenarios. r[agent_harness_runtime.native_evaluation_conformance.positive]

## Phase 4: Negative scenario families

- [ ] [parallel] N1 Add negative unstable-generator, missing-bound, unsafe-shuffle, duplicate-task, role-widening, fan-out-overflow, and shared-judge-promotion scenarios. r[agent_harness_runtime.native_evaluation_conformance.negative]
- [ ] [parallel] N2 Add negative copied-transcript, stale-ledger, wrong-subject, missing-service, wrong-plan, wrong-task, wrong-attempt, denied-capability, and malformed-observation scenarios. r[agent_harness_runtime.native_evaluation_conformance.negative]
- [ ] [parallel] N3 Add negative early-terminal, changed-retry, dispatch-before-acceptance, late-write, public-bind, wildcard-network, response-rewrite, and over-budget gateway scenarios. r[agent_harness_runtime.native_evaluation_conformance.negative]
- [ ] [parallel] N4 Add negative shared-verifier-mutation, symlink, artifact collision, hidden-test exposure, NaN, infinity, duplicate reward, identity mismatch, and silent-clamp scenarios. r[agent_harness_runtime.native_evaluation_conformance.negative]
- [ ] [parallel] N5 Add negative torn-output, stale-resume, failed-cleanup, lossy-Harbor, unsupported-ATIF, automatic-feedback promotion, unsafe-dataset export, receipt leak, and overclaim scenarios. r[agent_harness_runtime.native_evaluation_conformance.negative]
- [ ] [parallel] N6 Assert for each negative fixture the exact diagnostic family, no prohibited service call, unchanged reusable state, no success receipt, and bounded evidence. r[agent_harness_runtime.native_evaluation_conformance.negative] r[agent_harness_runtime.native_evaluation_conformance.evidence]

## Phase 5: Evidence and dependency isolation

- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.evidence] E1 Add a metadata-only conformance summary with profile, fixture, family, entrypoint, dependency, pass/fail/block, expected diagnostic, redaction, count, and non-claim identities. r[agent_harness_runtime.native_evaluation_conformance.evidence]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.evidence] E2 Reject task bodies, transcripts, hidden tests, reasoning, provider/tool bodies, credentials, environment values, physical paths, unrestricted logs, content-derived hashes outside policy, and unsupported live or release claims. r[agent_harness_runtime.native_evaluation_conformance.evidence]
- [ ] [parallel] E3 Add dependency and feature guards proving the suite activates no live provider, tool, process, socket, container, cloud, credential, network, scheduler, persistence, benchmark, Verifiers, or Python runtime. r[agent_harness_runtime.native_evaluation_conformance.verification]

## Phase 6: Rails, documentation, and closeout

- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.verification] D1 Add `scripts/cairn/check-evaluation-composition-conformance` with family-level progress and bounded failure summaries. r[agent_harness_runtime.native_evaluation_conformance.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.verification] D2 Add a dedicated Nix check that runs profile freshness, public-entrypoint guard, every fixture family, dependency isolation, component/facade coverage, and safe evidence validation. r[agent_harness_runtime.native_evaluation_conformance.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.evidence] D3 Document matrix coverage, fixture authoring, expected diagnostics, public-entrypoint rules, live-check separation, evidence shape, and non-claims. r[agent_harness_runtime.native_evaluation_conformance.matrix] r[agent_harness_runtime.native_evaluation_conformance.evidence]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.verification] D4 Update contracts, focused verification, component, facade, request-lifecycle, safety, roadmap, README check, and bounded Verifiers reference indexes. r[agent_harness_runtime.native_evaluation_conformance.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.verification] D5 Run the new focused rail, relevant crate and integration tests, formatting, Clippy, Nickel checks, all connected focused rails, Cairn validation and gates, and the dedicated Nix check. r[agent_harness_runtime.native_evaluation_conformance.verification]
- [ ] [depends:agent_harness_runtime.native_evaluation_conformance.verification] D6 Review dry-run sync and archive plans, verify requirement promotion, and archive only after the complete positive and negative matrix passes without live-behavior or release overclaims. r[agent_harness_runtime.native_evaluation_conformance.verification]
