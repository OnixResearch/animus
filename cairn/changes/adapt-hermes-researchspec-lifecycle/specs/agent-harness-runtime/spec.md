# agent-harness-runtime Change Spec: Hermes ResearchSpec lifecycle adaptation

## Requirements

### Requirement: Typed study contract

r[agent_harness_runtime.hermes_researchspec.study_contract] Animus MUST define a typed ResearchSpec-inspired study contract with study ID, objective, primary metric, metric direction, threshold or noise policy, workload command identity, correctness checks, files in scope, off-limits files, guarded measurement files, promotion thresholds, and provenance.

#### Scenario: Valid study contract is ready for baseline

r[agent_harness_runtime.hermes_researchspec.study_contract.valid]
- GIVEN a host supplies a complete typed study contract with objective, metric, workload, checks, scope, off-limits files, guardrails, and promotion policy
- WHEN study validation runs
- THEN the study is accepted as ready for baseline measurement
- AND a fresh agent can discover the required lifecycle state without relying on conversation context.

#### Scenario: Incomplete study contract is rejected

r[agent_harness_runtime.hermes_researchspec.study_contract.incomplete]
- GIVEN a study contract is missing a primary metric, metric direction, workload command identity, correctness checks declaration, scope, or off-limits declaration
- WHEN study validation runs
- THEN validation fails before any candidate run can be kept
- AND the diagnostic names the missing field.

### Requirement: Baseline evidence is immutable and fingerprinted

r[agent_harness_runtime.hermes_researchspec.baseline] Animus MUST require a baseline record before candidate runs can be evaluated, including primary metric value, secondary metrics when configured, check status, config snapshot, workload fingerprints, guarded input fingerprints, and BLAKE3 hashes unless an interoperability constraint requires another algorithm.

#### Scenario: Baseline records guarded fingerprints

r[agent_harness_runtime.hermes_researchspec.baseline.fingerprints]
- GIVEN a valid study and successful workload/check execution reported by the host
- WHEN baseline evidence is recorded
- THEN the baseline includes metric values, check status, config snapshot, guarded file/input fingerprints, and BLAKE3 hashes
- AND later candidate gates can detect guarded measurement drift.

#### Scenario: Missing primary metric blocks baseline

r[agent_harness_runtime.hermes_researchspec.baseline.missing_metric]
- GIVEN the workload completes but host-reported output lacks the configured primary metric
- WHEN baseline validation runs
- THEN the baseline fails closed
- AND no candidate run can be marked kept against that baseline.

### Requirement: Hypotheses are first-class artifacts

r[agent_harness_runtime.hermes_researchspec.hypotheses] Animus MUST track optimization hypotheses with stable IDs, claims, planned interventions, expected signals, falsification criteria, risk notes, linked runs, and final decisions.

#### Scenario: Vague hypothesis is rejected

r[agent_harness_runtime.hermes_researchspec.hypotheses.vague]
- GIVEN a hypothesis only says to try random tweaks or lacks expected signal and falsification criteria
- WHEN hypothesis validation runs
- THEN validation rejects it as not actionable
- AND the diagnostic requests a concrete claim, intervention, expected signal, and discard condition.

### Requirement: Run evidence is complete and resumable

r[agent_harness_runtime.hermes_researchspec.runs] Every run record MUST include study ID, hypothesis ID, run timestamp or sequence metadata, candidate revision or diff fingerprint, primary metric, secondary metrics when configured, status, check status, changed-file summary, side information, rollback reason for discarded or failed runs, and next-action hint.

#### Scenario: Failed run without rollback context is incomplete

r[agent_harness_runtime.hermes_researchspec.runs.failed_requires_context]
- GIVEN a candidate run crashes, fails checks, or is discarded
- WHEN the run record lacks rollback reason or next-action hint
- THEN run validation reports the record as incomplete
- AND the study cannot be archived as complete until the missing learning is added or explicitly waived.

### Requirement: Keep and promotion gates defend measurement integrity

r[agent_harness_runtime.hermes_researchspec.keep_promotion] A kept run MUST satisfy the configured metric comparison rule, required checks, threshold or noise policy, hypothesis traceability, guarded-file drift policy, and measurement-integrity guardrails before promotion eligibility is reported.

#### Scenario: Suspicious metric improvement is blocked

r[agent_harness_runtime.hermes_researchspec.keep_promotion.suspicious]
- GIVEN a candidate improves the primary metric but weakens checks, changes metric extraction, excludes benchmark inputs, hardcodes workload inputs, defers measured work, or edits guarded files without approved instrumentation-change metadata
- WHEN the keep gate evaluates the run
- THEN the run is rejected or blocked from promotion
- AND the report classifies the issue as a measurement-integrity risk with concrete evidence labels.

#### Scenario: Promotion groups detect overlap

r[agent_harness_runtime.hermes_researchspec.keep_promotion.overlap]
- GIVEN multiple kept runs are proposed as independent promotion groups
- WHEN two groups touch the same file or one group depends on an API introduced by another group
- THEN promotion evaluation reports the overlap or dependency
- AND the groups are merged or marked dependent before finalization.

### Requirement: Archive preserves study conclusions

r[agent_harness_runtime.hermes_researchspec.archive] Completed studies MUST archive objective, baseline, best result, kept changes, discarded hypotheses, final metric delta, unresolved ideas, downstream lifecycle links, and run evidence needed to audit keep/discard decisions.

#### Scenario: Unresolved decisions block archive

r[agent_harness_runtime.hermes_researchspec.archive.unresolved]
- GIVEN a study has linked runs without hypothesis decisions or in-scope ideas without keep, discard, supersede, defer, or waive decisions
- WHEN archive validation runs
- THEN archival is rejected
- AND the report lists unresolved hypotheses, runs, or ideas.

### Requirement: ResearchSpec adaptation verification

r[agent_harness_runtime.hermes_researchspec.verification] The ResearchSpec adaptation MUST include focused tests for study contract validation, baseline evidence, BLAKE3 fingerprints, missing metric rejection, hypothesis validation, run evidence completeness, keep/promotion integrity gates, promotion overlap detection, archive rejection, and optional host-adapter isolation.

#### Scenario: Closeout evidence covers lifecycle gates

r[agent_harness_runtime.hermes_researchspec.verification.closeout]
- GIVEN maintainers intend to archive the Hermes ResearchSpec adaptation
- WHEN closeout validation runs
- THEN evidence names focused study tests, baseline tests, hypothesis/run tests, keep/promotion tests, archive tests, workspace checks, Cairn validation, and Cairn gates
- AND any skipped rail includes a concrete blocker and next-best validation.
