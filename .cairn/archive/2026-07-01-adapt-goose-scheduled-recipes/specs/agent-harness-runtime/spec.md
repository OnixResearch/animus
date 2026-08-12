# agent-harness-runtime Change Spec: Goose scheduled recipe adaptation

## Requirements

### Requirement: Scheduled workflow descriptors are explicit

r[agent_harness_runtime.goose_scheduled_recipes.schedule_descriptor] Animus MUST define scheduled workflow descriptors for schedule identity, recipe identity and version, parameter values, cadence, timezone or calendar policy, execution mode, concurrency policy, workspace scope, retry bounds, retention policy, and provenance.

#### Scenario: Schedule descriptor validation rejects invalid plans

r[agent_harness_runtime.goose_scheduled_recipes.schedule_descriptor.validation]
- GIVEN a scheduled workflow descriptor has an invalid cadence, missing recipe identity, unsupported execution mode, or malformed retention policy
- WHEN schedule validation runs
- THEN validation returns a typed schedule descriptor error
- AND no timer, queue task, process, or workflow run is started.

### Requirement: Schedule planning is data-only in reusable core

r[agent_harness_runtime.goose_scheduled_recipes.data_only] Reusable schedule validation and next-run planning MUST operate over explicit in-memory clock and calendar inputs and MUST NOT start timers, daemons, cron jobs, processes, filesystem mutations, or environment reads.

#### Scenario: Next-run planning has no ambient effects

r[agent_harness_runtime.goose_scheduled_recipes.data_only.neutral]
- GIVEN a schedule descriptor and explicit current time input
- WHEN next-run planning runs in reusable core
- THEN it returns a deterministic plan or typed validation error
- AND ambient cron edits, daemon startup, process launch, filesystem mutation, and environment reads are absent.

### Requirement: Every scheduled run requires admission

r[agent_harness_runtime.goose_scheduled_recipes.run_admission] Scheduled recipe execution MUST require recipe trust, capability policy, sandbox policy, queue admission, and cancellation handles before dispatch.

#### Scenario: Missing queue service denies scheduled run

r[agent_harness_runtime.goose_scheduled_recipes.run_admission.denied]
- GIVEN a schedule is due but no queue service is injected or the recipe is not trusted
- WHEN runtime evaluates run admission
- THEN the scheduled run is denied with a typed unavailable-service or untrusted-recipe outcome
- AND no model, tool, or host-effect dispatch begins.

### Requirement: Scheduled runs handle concurrency explicitly

r[agent_harness_runtime.goose_scheduled_recipes.concurrency] Scheduled workflow concurrency policy MUST prevent overlapping writes from merging silently by requiring serialization, isolated workspace execution, explicit handoff, or denial.

#### Scenario: Overlapping writes are not silent

r[agent_harness_runtime.goose_scheduled_recipes.concurrency.conflict]
- GIVEN a scheduled recipe run is still modifying a workspace when the next run becomes due
- WHEN concurrency policy evaluates the new run
- THEN the new run is serialized, isolated, or denied according to policy
- AND no shared-resource write conflict is merged silently.

### Requirement: Scheduled run records are safe

r[agent_harness_runtime.goose_scheduled_recipes.run_records] Scheduled run records MUST link schedule ID, recipe identity, parameter hashes, queue or job IDs, ledger references, retry counts, outcomes, and redacted evidence without raw prompts, secrets, full paths, or raw tool output.

#### Scenario: Secret parameters are redacted from run record

r[agent_harness_runtime.goose_scheduled_recipes.run_records.redacted]
- GIVEN a scheduled recipe uses parameter values that include sensitive strings or file-derived content
- WHEN a scheduled run record is rendered
- THEN the record contains hashes, safe labels, counts, and redaction markers
- AND raw prompts, secret parameters, full paths, and raw tool output are omitted.

### Requirement: Schedule controls are host-mediated

r[agent_harness_runtime.goose_scheduled_recipes.control] Pause, resume, edit, run-now, and cancellation controls for scheduled recipes MUST be host-mediated control requests with admission decisions and safe receipts.

#### Scenario: Unauthorized schedule mutation is denied

r[agent_harness_runtime.goose_scheduled_recipes.control.denied]
- GIVEN a session lacks authority to edit or run a schedule
- WHEN it requests pause, resume, edit, run-now, or cancellation
- THEN host-control admission denies the request
- AND the schedule state and queued runs remain unchanged.

### Requirement: Scheduled recipe closeout is evidenced

r[agent_harness_runtime.goose_scheduled_recipes.verification] Scheduled recipe adoption MUST include positive schedule validation tests, negative data-only runtime-neutrality tests, run-admission tests, concurrency conflict tests, run-record redaction tests, control-denial tests, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Scheduled recipe evidence is complete

- GIVEN the Goose scheduled recipe change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused schedule tests, queue-admission tests, concurrency tests, receipt redaction tests, control-denial tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
