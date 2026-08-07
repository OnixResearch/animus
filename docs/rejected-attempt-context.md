# Rejected attempt context

Animus can project one reviewed, rejected attempt into a later fresh context. This projection is advisory evidence only.

## Reference boundary

The design reference is [Ralph the Riveter](https://github.com/rivet-dev/riveter/tree/9615b634df5498682cf3f9f3295df6eb6585d5cb) at revision `9615b634df5498682cf3f9f3295df6eb6585d5cb`.

The reviewed workspace manifest declares the MIT license. Animus adapts three concepts:

- retain the rejected revision instead of deleting it;
- bind a rejection reason to one attempt;
- show bounded failure context to a later iteration.

Animus does not adopt Riveter's `prd.toml` format, commit subjects, agent-owned pass flag, raw-diff prompt insertion, or mandatory Jujutsu workflow.

## Pure core

`animus_runtime::rejected_attempt` accepts already-loaded facts. It validates these items:

- nominal task, run, attempt, revision, workspace, content, handoff, summary, and receipt identities;
- one validated Cairn revision handoff or policy-declared equivalent;
- one exact rejected attempt;
- current VCS provider, revision, workspace, and content facts;
- redaction, classification, and byte bounds;
- metadata-only receipt claims and required non-claims.

The core does not inspect a repository, read a diff, mutate a task, create a run, or assemble a provider request.

## Host shell

`animus_std` defines injected review, VCS, and rejection-context services. A product host retains all authority to load receipts, inspect VCS state, read diffs, classify content, redact text, assemble prompts, and mutate lifecycle state.

The optional `JujutsuAttemptObservationFixture` maps already-observed Jujutsu facts. It does not run `jj` or grant VCS authority.

If a required service is absent, the host fails before it infers evidence from commit text or ambient repository state.

## Safe projection

The default Nickel policy denies raw diffs. A raw diff requires all of these conditions:

1. Policy permits raw diffs.
2. The caller supplies an explicit capability.
3. Host classification marks the diff safe.
4. The diff fits the byte budget.

The metadata-only receipt excludes raw summaries, raw diffs, prompts, transcripts, credentials, host paths, provider payloads, and tool output.

## Non-claims

A rejected-attempt plan or receipt does not authorize a retry or any other lifecycle action. It does not:

- prove that the rejection was correct;
- open or complete a task;
- authorize a retry;
- create a run;
- mutate VCS state;
- prove that a later attempt fixes the findings.
