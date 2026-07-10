# ResearchSpec promotion DAGs

ResearchSpec promotion DAGs describe ordering and evidence for kept promotion groups. The reusable runtime records group IDs, kept run IDs, dependency edges, touched-file summaries, metric evidence hashes, guardrail status, patch/diff hashes, blocked explanations, topological batches, and safe receipt identities.

Validation rejects duplicate groups, duplicate edges, unknown dependencies, self-dependencies, cycles, unsafe labels, overlapping files without a block decision, stale run evidence, failed/unkept runs, unresolved hypotheses, guarded-file conflicts, and raw receipt leaks.

Non-claims: benchmark execution, patch application, VCS operations, conflict resolution, archive mutation, scheduling, persistence, and study archival remain host-owned.
