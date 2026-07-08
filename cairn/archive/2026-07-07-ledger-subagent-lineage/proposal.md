# Proposal: Record subagent lineage in the ledger

## Why

Subagent outputs are only trustworthy when parent sessions can explain where a child run came from, which authority it had, what context it saw, what it returned, and how the parent imported the result. Current subagent contracts carry much of that data locally, but there is no unified lineage ledger projection across child-run kinds.

This change adds parent/child lineage records for audit, replay, and safe receipt generation.

## What Changes

- Add a subagent lineage ledger record with parent id, child id, child-run kind, delegation source, effective capabilities, context packet identity, output hash, handoff mode, status, and failure class.
- Store safe hashes and redacted summaries rather than raw child transcripts or parent prompts.
- Add replay validation that can audit child-run influence using lineage records and receipts.
- Connect coordinator lifecycle events, provenance, and session tree projections to the lineage record.

## Impact

- **Files**: future ledger DTOs, provenance projection helpers, receipt validators, replay tests, docs, and fixture updates.
- **Testing**: positive parent/child lineage recording and replay tests; negative raw transcript, missing hash, mismatched parent/child, malformed receipt, and unsafe summary tests.
- **Non-claims**: this does not store raw child logs in the parent ledger or make child results automatically authoritative.
