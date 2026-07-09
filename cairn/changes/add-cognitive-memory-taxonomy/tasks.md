# Tasks

- [ ] [serial] Define category metadata for episodic, semantic, procedural, reflection/pitfall, instruction, fact/preference, summary, candidate, and unsupported records. r[agent_harness_runtime.memory_taxonomy.record_categories]
- [ ] [depends:record_categories] Add compatibility mapping from existing generic memory kinds into category-aware descriptors. r[agent_harness_runtime.memory_taxonomy.compatibility]
- [ ] [depends:record_categories] Ensure retrieval, prompt influence, and receipts treat categories as safe metadata after admission, scope, freshness, redaction, and budget checks. r[agent_harness_runtime.memory_taxonomy.non_authority]
- [ ] [depends:compatibility,non_authority] Add positive category-mapping tests and negative invalid-category, unauthorized-category, stale, redacted, and raw-body fixtures. r[agent_harness_runtime.memory_taxonomy.verification]
- [ ] [depends:verification] Update docs and run focused memory taxonomy tests, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.memory_taxonomy.verification]
