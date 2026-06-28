# agent-harness-runtime Change Spec: Clankers batch trajectory runner adaptation

## Requirements

### Requirement: Local batch run planning

r[agent_harness_runtime.clankers_batch_runner.planning] Animus MUST validate local batch inputs into a deterministic batch plan with run identity, ordered jobs, output format, concurrency policy, execution mode, and safe output labels before execution begins.

#### Scenario: Remote input fails closed

r[agent_harness_runtime.clankers_batch_runner.planning.remote_rejected]
- GIVEN a batch request names an HTTP, HTTPS, S3, or otherwise remote input or output location
- WHEN batch planning validates the request
- THEN planning returns a typed unsupported-location failure
- AND no execution service or renderer is invoked.

### Requirement: Injected batch execution

r[agent_harness_runtime.clankers_batch_runner.execution] Batch jobs MUST execute through an injected headless or session execution service and MUST NOT require reusable runtime code to construct providers, spawn daemons, write session stores, or call shell commands directly.

#### Scenario: Execution service unavailable

r[agent_harness_runtime.clankers_batch_runner.execution.unavailable]
- GIVEN a valid batch plan requires job execution
- AND no compatible execution service is injected
- WHEN the runner starts
- THEN Animus records an unavailable-execution failure
- AND no provider, daemon, or shell fallback is attempted.

### Requirement: Deterministic resume manifests

r[agent_harness_runtime.clankers_batch_runner.resume] Batch resume MUST use sidecar manifest evidence to skip only completed jobs whose run ID, job ID, output format, redaction policy, and output evidence match the current plan.

#### Scenario: Stale manifest does not skip job

r[agent_harness_runtime.clankers_batch_runner.resume.stale]
- GIVEN a resume manifest contains a completed job with a mismatched plan hash or output format
- WHEN resume planning evaluates the manifest
- THEN the job is not skipped
- AND the resume report records the stale evidence reason.

### Requirement: Stable trajectory output

r[agent_harness_runtime.clankers_batch_runner.output] Batch renderers MUST produce stable job ordering and safe trajectory metadata even when jobs complete concurrently or are retried.

#### Scenario: Concurrent completions remain ordered

r[agent_harness_runtime.clankers_batch_runner.output.stable_order]
- GIVEN multiple batch jobs execute with different completion times
- WHEN results are rendered
- THEN output records appear in deterministic plan order
- AND each record carries safe job status and redaction metadata.

### Requirement: Batch metadata redaction

r[agent_harness_runtime.clankers_batch_runner.metadata] Batch run and job receipts MUST record safe metadata such as counts, statuses, prompt sizes, model/session handles, objective summaries, output labels, redaction status, and BLAKE3 hashes, and MUST NOT include raw prompts, provider payloads, credentials, or remote destinations.

#### Scenario: Metadata rejects raw prompt

r[agent_harness_runtime.clankers_batch_runner.metadata.rejects_prompt]
- GIVEN a batch receipt fixture includes a raw prompt body or provider payload
- WHEN metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Batch runner verification

r[agent_harness_runtime.clankers_batch_runner.verification] The adaptation MUST include focused positive and negative tests for local planning, remote-path rejection, injected execution, unavailable services, resume manifests, stable ordering, renderer metadata, objective receipts, and redaction.

#### Scenario: Closeout evidence covers batch fixtures

r[agent_harness_runtime.clankers_batch_runner.verification.closeout]
- GIVEN maintainers intend to archive the batch runner adaptation
- WHEN closeout validation runs
- THEN evidence names focused batch planning, execution, resume, output, metadata, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
