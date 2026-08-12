# Proposal: Adapt OpenAI Codex dedicated diff review mode for Animus

## Why

OpenAI Codex exposes a dedicated review workflow that inspects a selected diff or commit and returns prioritized, actionable findings without modifying the working tree. This is a valuable pattern for Animus because review agents need stronger side-effect boundaries than general coding agents.

Animus should adapt dedicated review mode as a read-only, diff-scoped runtime profile with typed findings, evidence references, and rerun support.

## What Changes

- Define review run requests over VCS-neutral change sets such as uncommitted changes, commits, branch comparisons, or externally supplied patches.
- Add a read-only review sandbox profile that forbids file mutation, tool mutation, and approval escalation by default.
- Define typed review findings with severity, category, affected span, evidence, rationale, confidence, and suggested remediation.
- Add review summary and resolution tracking so repeated review runs can compare open, fixed, dismissed, and newly introduced findings.
- Record review mode events without creating PRs, commits, or direct code edits.

## Impact

- **Files**: review DTOs, VCS-neutral diff selectors, read-only policy profile, finding schema, report aggregation, docs, and positive/negative tests.
- **Testing**: positive tests for review findings from supplied diffs and branch comparisons; negative tests for mutation attempts, missing diff input, invalid spans, duplicate findings, and unbounded evidence bodies.
- **Non-claims**: this change does not open or manage pull requests, apply fixes automatically, or depend on Git-only branch semantics.
