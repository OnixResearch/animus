# Proposal: Adapt Hermes review report contracts for Animus

## Why

Hermes review and gate workflows use a compact structured report contract: a `VERDICT: PASS | WARN | FAIL` line plus tagged findings and optional sections. That pattern gives agents, gates, and metrics tools a deterministic boundary between human-readable review text and machine-readable evidence. Animus should adapt the contract for review, gate, and quality-evidence surfaces.

## What Changes

- Define an Animus-owned review report DTO with verdict, sections, findings, diagnostics, source, stage, subject, and safe metadata.
- Parse findings only from an explicit findings section and require `class`, `scope`, `route`, and `promoted` tags for metric-bearing bullets.
- Separate read-only report parsing/summary from explicit append-only metrics recording.
- Add repeated-finding promotion summaries without treating metrics as an automatic mutation trigger.
- Add positive and negative fixtures based on Hermes review-gate reports.

## Impact

- **Files**: future review report parser/core, metrics record DTOs, fixture reports, prompt/gate docs, and tests.
- **Testing**: valid reports, malformed verdicts, malformed tags, checklist bullets outside Findings, metrics-record rejection, append-only write boundaries, and promotion candidate grouping.
- **Non-claims**: this change does not adopt Hermes `.agent/review-metrics.jsonl` as the only storage, does not auto-dispatch reviewers, and does not make review metrics a core scoring mechanism.
