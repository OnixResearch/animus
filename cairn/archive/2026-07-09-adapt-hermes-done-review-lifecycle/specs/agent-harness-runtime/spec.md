# agent-harness-runtime Change Spec: Hermes done-review lifecycle contracts

## Requirements

### Requirement: Optional review lifecycle DTOs

r[agent_harness_runtime.hermes_done_review.lifecycle] Animus MUST represent post-completion review lifecycle state as optional typed DTOs for review requests, review subjects, bounded evidence bundles, completion rail descriptors and results, reviewer attempt plans, lifecycle outcomes, and safe receipts.

#### Scenario: Host requests review with bounded evidence

r[agent_harness_runtime.hermes_done_review.lifecycle.request]
- GIVEN a host wants independent review before accepting a completion claim
- WHEN it constructs a review lifecycle request
- THEN the request names safe subject metadata, evidence references, rail policy, reviewer-service requirement, and output contract
- AND reusable Animus logic does not collect diffs, inspect VCS state, call a reviewer model, or persist metrics.

### Requirement: Review evidence is bounded and validated

r[agent_harness_runtime.hermes_done_review.evidence] Review lifecycle validation MUST accept only bounded evidence summaries, safe relative labels, counts, hashes, parsed report metadata, completion rail result summaries, and host-supplied sequence metadata, and MUST reject raw prompts, provider payloads, credentials, full private paths, raw diffs, raw file bodies, raw tool output, and unbounded transcripts.

#### Scenario: Unsafe evidence is rejected before reviewer dispatch

r[agent_harness_runtime.hermes_done_review.evidence.rejects_leaks]
- GIVEN a review evidence bundle contains a raw provider payload, credential-looking value, full private path, unbounded transcript, or raw diff body
- WHEN review lifecycle validation runs
- THEN validation returns a typed unsafe-evidence diagnostic
- AND no reviewer attempt, completion claim, receipt, or continuation intent is accepted from that bundle.

### Requirement: Reviewer and rail execution are host-owned

r[agent_harness_runtime.hermes_done_review.host_owned] Reviewer selection, reviewer model calls, completion rail command execution, VCS/file inspection, clocks, UI prompts, durable metrics writes, and retry/fresh-session orchestration MUST remain host-owned services outside reusable runtime and adapter-contract crates.

#### Scenario: Missing reviewer service fails closed

r[agent_harness_runtime.hermes_done_review.host_owned.missing_reviewer]
- GIVEN a review lifecycle request requires an independent reviewer
- AND no reviewer service is injected by the host
- WHEN lifecycle dispatch is planned
- THEN Animus returns a reviewer-service-unavailable outcome
- AND it does not choose a default model, read provider credentials, call a provider, or mark the review passed.

### Requirement: Review outcomes do not mutate automatically

r[agent_harness_runtime.hermes_done_review.outcomes] PASS, WARN, FAIL, malformed-output, missing-service, cancelled, and timed-out review outcomes MUST produce typed safe outcomes and optional continuation intents, but MUST NOT automatically edit files, fix failures, write review metrics, alter Cairn tasks, open lifecycle changes, commit VCS changes, or approve completion.

#### Scenario: Failed review requests explicit follow-up

r[agent_harness_runtime.hermes_done_review.outcomes.fail_followup]
- GIVEN a reviewer returns a valid `VERDICT: FAIL` report with bounded findings
- WHEN lifecycle outcome reduction runs
- THEN the outcome records failure, safe report identity, and an optional host-follow-up intent
- AND no automatic retry, mutation, task update, metrics append, or completion acknowledgement occurs without a separate host action.

### Requirement: Done-review lifecycle verification

r[agent_harness_runtime.hermes_done_review.verification] The done-review lifecycle adaptation MUST include focused positive and negative tests for valid lifecycle requests, bounded evidence validation, completion rail result handling, reviewer attempt plans, missing reviewer service, malformed reviewer output, host-owned side effects, no baked-in default review command, and safe receipt leak rejection.

#### Scenario: Closeout evidence names non-claims

r[agent_harness_runtime.hermes_done_review.verification.closeout]
- GIVEN maintainers intend to archive the done-review lifecycle adaptation
- WHEN closeout validation runs
- THEN evidence names lifecycle DTO, evidence, host-owned service, outcome, receipt, docs, Cairn validation, and Cairn gate checks
- AND it explicitly lists omitted product behavior such as built-in Hermes commands, default reviewer models, provider calls, VCS inspection, metrics persistence, and automatic fixes as non-claims.
