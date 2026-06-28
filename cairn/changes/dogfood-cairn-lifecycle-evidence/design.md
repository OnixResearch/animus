# Design: Cairn lifecycle dogfooding

## Context

Animus has native Cairn changes but no accepted spec baseline or repo-local lifecycle workflow. Cairn can provide planning discipline, validation, gates, sync/archive behavior, and release evidence, but only if the repository treats those artifacts as first-class.

## Decisions

### 1. Accepted specs are the baseline

`cairn/specs/` records accepted requirements. Active changes under `cairn/changes/` describe deltas. Closeout syncs accepted changes into specs and archives completed packages.

### 2. Gates are evidence, not ceremony

Proposal, design, tasks, validation, sync dry-run, archive dry-run, and release-readiness outputs are captured or summarized as review evidence. Failed gates block archive unless explicitly waived with a replacement path.

### 3. Policy identity is part of lifecycle evidence

Validation evidence records the policy artifact used, including source identity and hash when available. This prevents a change from passing only under an unstated local policy.

### 4. Direct edits remain allowed for narrow safe work

Small fixture, test, typo, or doc hardening that does not change public behavior can be direct. Public CLI/API behavior, compatibility semantics, architecture, mutation behavior, schema contracts, release/evidence contracts, or roadmap scope should use Cairn changes.

### 5. Evidence stays bounded

Durable evidence should include command names, status, issue codes, hashes, and paths relative to the repo where safe. It should avoid raw prompts, credentials, provider payloads, full private paths, and unbounded command output.

## Risks

- Lifecycle overhead can slow small repairs. Keep the change threshold explicit.
- Evidence can become noisy. Store summaries and hashes by default, with raw logs only in local or intentionally redacted artifacts.
