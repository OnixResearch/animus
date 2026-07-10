# Tasks

- [x] [serial] Extend roadmap validation data structures with topological batches, blocked-change explanations, cycle path summaries, and transitive safety-path summaries over already-loaded records. r[agent_harness_runtime.roadmap_dag.explanations]
- [x] [depends:explanations] Keep roadmap explanation logic pure over exported roadmap records, active change IDs, and accepted requirement IDs while the script shell owns Nickel export, filesystem reads, stdout, and optional artifact writes. r[agent_harness_runtime.roadmap_dag.core_shell]
- [x] [depends:core_shell] Add safe Markdown or DOT-style review projection with change IDs, roles, tiers, prerequisite IDs, issue codes, counts, and redaction markers only. r[agent_harness_runtime.roadmap_dag.review_projection]
- [x] [depends:review_projection] Add product-surface safety path explanations that name accepted foundational and runtime safety markers or identify the missing marker class. r[agent_harness_runtime.roadmap_dag.product_safety]
- [x] [depends:product_safety] Add positive and negative fixtures for valid topological batches, blocked dependents, cycle paths, unknown prerequisites, duplicate IDs, unclassified active changes, unsafe product surfaces, malformed records, and leak attempts. r[agent_harness_runtime.roadmap_dag.verification]
- [x] [depends:verification] Update docs and focused evidence guidance so closeout summaries can cite roadmap DAG batches and blockers without copying raw change bodies or command logs. r[agent_harness_runtime.roadmap_dag.docs]
