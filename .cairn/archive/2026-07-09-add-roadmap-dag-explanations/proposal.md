# Proposal: Add roadmap DAG explanations

## Why

Animus already treats `cairn/roadmap.ncl` as machine-checked sequencing data. The current rail validates classification, prerequisites, cycle absence, and product-surface safety, but maintainers still need clearer DAG evidence: topological order, blocked dependents, transitive prerequisite paths, product-safety paths, and safe review renderings.

Animus should add roadmap DAG explanations so active Cairn sequencing can be audited and debugged without reading every change package by hand.

## What Changes

- Extend roadmap validation output with deterministic topological batches, blocked-change explanations, cycle path summaries, and transitive safety-path evidence.
- Add safe Markdown or DOT-style review projection that uses change IDs, roles, tiers, prerequisite IDs, and issue codes only.
- Preserve Nickel as the source of truth and keep filesystem reads, CLI output, and artifact writing in scripts or shell code rather than reusable runtime core.
- Add negative fixtures for cycles, unknown references, unsafe product surfaces, unclassified changes, duplicate IDs, and receipt/projection leak attempts.

## Impact

- **Files**: future roadmap checker core/shell split, roadmap fixtures, docs, focused rail evidence, and optional generated review artifact under ignored output paths.
- **Testing**: positive topological batches, blocked-change reports, safety-path reports, and safe projection cases plus negative cycle, unknown prerequisite, unsafe product surface, duplicate ID, unclassified change, and leak fixtures.
- **Non-claims**: this does not mutate Cairn changes, create tasks, archive changes, sync specs, open PRs, inspect private file bodies, or execute active changes.
