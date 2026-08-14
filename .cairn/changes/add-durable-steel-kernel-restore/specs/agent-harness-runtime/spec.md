# Agent Harness Runtime

## ADDED Requirements

### Requirement: Committed kernel state has a private durable journal

r[agent_harness_runtime.steel_kernel_restore.journal] Animus MUST represent committed Steel kernel cells with versioned private records that bind source, ordered captured host results, clear markers, compatibility identities, bounds, and a BLAKE3-chained committed head.

#### Scenario: A bridged cell commits

r[agent_harness_runtime.steel_kernel_restore.journal.commit]
- GIVEN a bridged Steel cell reaches a successful final value
- WHEN the durable host commits its kernel record
- THEN the private record MUST bind the source and ordered result bodies to safe identities
- AND the safe ledger MUST contain only bounded metadata, identities, counts, and status classes.

### Requirement: Kernel replay performs no host effect

r[agent_harness_runtime.steel_kernel_restore.replay] Restore MUST rebuild a fresh kernel by replaying committed cells with host execution disabled and by supplying the exact captured result for each recorded bridge call.

#### Scenario: Restore replays a prior file-read result

r[agent_harness_runtime.steel_kernel_restore.replay.no_effect]
- GIVEN a committed cell used one captured file-read result
- WHEN a fresh process restores the kernel
- THEN replay MUST supply the recorded bounded result to the cell
- AND it MUST NOT call a file, process, tool, network, context, or child executor.

#### Scenario: Replay requests an unrecorded effect

r[agent_harness_runtime.steel_kernel_restore.replay.mismatch]
- GIVEN replayed source issues a request that has no exact ordered captured result
- WHEN the replay driver handles that request
- THEN restore MUST fail closed with a typed mismatch
- AND no host effect or partially restored kernel may become active.

### Requirement: Restore admission binds exact compatibility facts

r[agent_harness_runtime.steel_kernel_restore.admission] Restore MUST require matching session, kernel generation, Steel cohort, RLM policy, bridge schema, journal schema, and committed chain-head facts before replay.

#### Scenario: The Steel cohort changed

r[agent_harness_runtime.steel_kernel_restore.admission.cohort_drift]
- GIVEN a journal was committed under another Steel cohort
- WHEN restore admission evaluates the journal
- THEN admission MUST return a typed compatibility denial
- AND it MUST NOT attempt best-effort replay or silently clear the kernel.

### Requirement: Restore uses committed cell safe points

r[agent_harness_runtime.steel_kernel_restore.safe_points] Durable kernel heads MUST advance only at complete committed cell boundaries, and interrupted cells MUST remain explicit recovery outcomes whose completed external effects are not rerun.

#### Scenario: The process exits during a cell

r[agent_harness_runtime.steel_kernel_restore.safe_points.interrupted]
- GIVEN the prior kernel head is committed and the next cell is interrupted after one observed effect
- WHEN restart recovery runs
- THEN restore MUST activate only the prior committed kernel head
- AND active-turn recovery MUST retain the interrupted effect observation without rerunning it.

### Requirement: Kernel restore evidence is metadata-only

r[agent_harness_runtime.steel_kernel_restore.evidence] Kernel restore receipts MUST exclude source bodies, result bodies, prompts, credentials, private paths, and raw diagnostics while binding safe compatibility identities, chain heads, counts, outcomes, and required non-claims.

#### Scenario: A receipt draft includes source code

r[agent_harness_runtime.steel_kernel_restore.evidence.leak]
- GIVEN a restore receipt draft contains model-written source or a captured result body
- WHEN receipt validation runs
- THEN validation MUST reject the draft without echoing the private material
- AND the receipt MUST NOT contribute to readiness or release claims.

### Requirement: Durable kernel restore has adversarial evidence

r[agent_harness_runtime.steel_kernel_restore.verification] Durable restore MUST include production-entrypoint tests for successful replay, clear markers, captured results, effect suppression, wrong identity, cohort drift, policy drift, missing and extra results, order mismatch, corruption, interrupted cells, bounds, and leaks.

#### Scenario: The restore change is ready to archive

r[agent_harness_runtime.steel_kernel_restore.verification.closeout]
- GIVEN maintainers intend to sync and archive durable kernel restore
- WHEN focused and lifecycle validation runs
- THEN Steel, bridge, store, recovery, repair, receipt, Cairn, and relevant Nix checks MUST pass
- AND mid-cell restore, VM serialization, effect rollback, encrypted storage, power-loss immunity, and production durability MUST remain non-claims.
