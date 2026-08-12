## Phase 1: Baseline and inventory

- [x] [serial] I1 Capture current focused positive and negative Animus process-job, Sandlock descriptor/receipt, Bubblewrap guardrail, and bounded-output behavior plus exact source identities before adding evaluation artifacts. r[agent_harness_runtime.bounded_execution_evaluation.inventory] r[agent_harness_runtime.bounded_execution_evaluation.conformance]
- [x] [serial] I2 Inventory Animus, Octet, Nickel Export, and Mantle execution plans, bounds, capability stages, mechanisms, observations, outcome classes, cleanup semantics, receipts, licenses, dependencies, non-claims, and retained product authority at exact immutable revisions and BLAKE3 source identities. r[agent_harness_runtime.bounded_execution_evaluation.inventory]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.inventory] I3 Classify every inventoried field as shared candidate semantics, consumer extension, mechanism-specific fact, retained authority, interoperability metadata, or blocker; explicitly compare network, filesystem/root, environment, timeout, output, cancellation, signal, child, and cleanup guarantees. r[agent_harness_runtime.bounded_execution_evaluation.capability_states] r[agent_harness_runtime.bounded_execution_evaluation.outcomes]

## Phase 2: Typed evaluation and conformance

- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.inventory] I4 Add a typed Nickel evaluation profile for consumer/source identities, capability states, outcome vocabulary, mechanism facts, mapping dispositions, extraction criteria, collection bounds, and required non-claims, plus a deterministic checked runtime projection with freshness evidence. r[agent_harness_runtime.bounded_execution_evaluation.capability_states] r[agent_harness_runtime.bounded_execution_evaluation.decision] r[agent_harness_runtime.bounded_execution_evaluation.non_claims]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.capability_states] I5 Freeze safe positive and negative cross-product fixtures for success, nonzero exit, launch denial, missing executable, environment rejection, path/root escape, timeout, cancellation, signal termination, output overflow, network-policy differences, cleanup failure, unsupported mechanisms, stale source, unsafe diagnostics, and overclaim wording. r[agent_harness_runtime.bounded_execution_evaluation.conformance]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.decision] I6 Implement a repository-owned Rust evaluation checker with pure in-memory validation/mapping/decision functions and a thin file/projection/receipt shell; emit BLAKE3-bound `extract`, `incubate`, or `blocked` decisions without executing processes or sandboxes. r[agent_harness_runtime.bounded_execution_evaluation.decision] r[agent_harness_runtime.bounded_execution_evaluation.authority]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.conformance] I7 Run every consumer mapping through the fixture corpus, preserve extensions and blockers, and record the terminal decision plus next smallest discriminating action; create no repository or consumer cutover in this change. r[agent_harness_runtime.bounded_execution_evaluation.decision] r[agent_harness_runtime.bounded_execution_evaluation.authority]

## Phase 3: Verification and closeout

- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.decision] V1 Add checker positive tests for lossless mappings and all three honest terminal statuses, plus negative tests for stale sources, missing consumers, duplicate IDs, unknown states/outcomes, semantic loss, dependency cycles, single-consumer extraction, authority leakage, unsafe fixture content, and weakened non-claims. r[agent_harness_runtime.bounded_execution_evaluation.conformance] r[agent_harness_runtime.bounded_execution_evaluation.decision] r[agent_harness_runtime.bounded_execution_evaluation.non_claims]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.authority] V2 Prove the evaluation checker and artifacts perform no process, filesystem-effect, network, clock, signal, child-reaping, timeout, output-capture, cleanup, sandbox, proof, build, or release behavior beyond the thin checker shell's bounded reads and receipt write. r[agent_harness_runtime.bounded_execution_evaluation.authority]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.conformance] V3 Run Nickel contract/projection freshness, the new checker self-tests, existing process-job and Sandlock focused rails, component catalog and contracts-index checks, adapter conformance, Wasm/purity checks where applicable, Cairn validation, and proposal/design/tasks gates; preserve exact outputs and blockers. r[agent_harness_runtime.bounded_execution_evaluation.inventory] r[agent_harness_runtime.bounded_execution_evaluation.conformance] r[agent_harness_runtime.bounded_execution_evaluation.decision]
- [x] [depends:agent_harness_runtime.bounded_execution_evaluation.decision] V4 Document the inventory, matrix, source identities, mapping losses, terminal decision, authority boundary, replay procedure, and bounded non-claims; if status is `extract`, scope but do not implement a successor repository-extraction Cairn, then verify accepted-spec sync before archive. r[agent_harness_runtime.bounded_execution_evaluation.decision] r[agent_harness_runtime.bounded_execution_evaluation.non_claims]

## Verification coverage

- `Scenario: Complete inventory classifies ownership` -> I1, I2, I3
- `Scenario: Missing execution semantics block a decision` -> I3, V1
- `Scenario: Mechanism observations remain visible` -> I3, I4, I7
- `Scenario: Stronger claim laundering is rejected` -> I4, V1
- `Scenario: Stable common outcome maps losslessly` -> I4, I5, I7
- `Scenario: Ambiguous outcome is not coerced` -> I3, I5, V1
- `Scenario: Positive fixtures preserve reviewed meaning` -> I5, I7, V1
- `Scenario: Negative fixtures fail with stable classes` -> I5, V1
- `Scenario: Extraction criteria are satisfied` -> I6, V1
- `Scenario: Evidence supports incubation or blocking` -> I6, I7, V1
- `Scenario: Evaluation consumes observations only` -> I6, V2
- `Scenario: No repository is created by evaluation` -> I7, V2, V4
- `Scenario: Required non-claims are present` -> I4, V1, V4
- `Scenario: Universal-equivalence wording is rejected` -> I5, V1
