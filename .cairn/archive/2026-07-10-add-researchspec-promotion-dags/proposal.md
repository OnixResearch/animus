# Proposal: Add ResearchSpec promotion DAG ordering

## Why

ResearchSpec promotion groups already record touched files and dependency group IDs, but current evaluation only distinguishes independent, dependent, and blocked outcomes. Once experiments produce several kept runs, maintainers need deterministic promotion batches, cycle diagnostics, stale-evidence rejection, and receipts that explain why one group must land before another.

Animus should make ResearchSpec promotion dependencies an explicit DAG so promotion planning remains auditable, safe, and host-effect free.

## What Changes

- Define ResearchSpec promotion DAG DTOs for group nodes, dependency edges, kept-run evidence, touched-file summaries, guardrail status, and safe promotion receipt identities.
- Validate unknown groups, cycles, duplicate groups, overlapping write sets, stale or failed run evidence, guarded-file conflicts, and missing keep decisions before promotion eligibility is reported.
- Produce deterministic topological promotion batches and blocked-group explanations without applying patches, invoking VCS commands, editing files, or persisting archive state.
- Project promotion DAG receipts with BLAKE3 hashes, bounded counts, safe group IDs, dependency edges, issue codes, and redaction markers.

## Impact

- **Files**: future ResearchSpec DTOs/core reducers, docs, tests, runtime facade inventory updates, and focused rail coverage.
- **Testing**: positive independent, dependent, and batched promotion plans plus negative cycle, unknown dependency, overlap, stale evidence, failed run, guarded-file conflict, unsafe receipt, and raw diff leak fixtures.
- **Non-claims**: this does not apply promotions, run benchmarks, execute VCS commands, create branches, merge work, archive studies, or store raw diffs/output in receipts.
