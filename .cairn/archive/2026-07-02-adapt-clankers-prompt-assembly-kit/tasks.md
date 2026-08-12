# Tasks

- [x] [serial] Define typed prompt-source DTOs with source kind, scope, trust/admission status, priority, budget class, redaction class, and safe identity. r[agent_harness_runtime.clankers_prompt_assembly.sources]
- [x] [depends:sources] Add host-context-only tests proving assembly can use admitted host context without filesystem, memory, skill, URL, or project discovery. r[agent_harness_runtime.clankers_prompt_assembly.sources.host_context]
- [x] [serial] Add deterministic precedence policy handling for section ordering, source precedence, conflicts, and required/optional source rules. r[agent_harness_runtime.clankers_prompt_assembly.precedence]
- [x] [depends:precedence] Add conflict fixtures proving deterministic selected order and safe skipped-source diagnostics. r[agent_harness_runtime.clankers_prompt_assembly.precedence.conflict]
- [x] [serial] Define prompt assembly ledger states for included, skipped, truncated, redacted, denied, conflicting, and unsupported sources. r[agent_harness_runtime.clankers_prompt_assembly.ledger]
- [x] [depends:ledger] Add over-budget source tests with budget use, truncation status, and BLAKE3 source identity. r[agent_harness_runtime.clankers_prompt_assembly.ledger.truncated]
- [x] [serial] Enforce host-owned swappable discovery boundaries for filesystem, project files, memory sources, skills, URLs, and tools. r[agent_harness_runtime.clankers_prompt_assembly.discovery]
- [x] [depends:discovery] Add missing-discovery-adapter tests proving no ambient fallback discovery, memory-store lookup, or provider extraction occurs. r[agent_harness_runtime.clankers_prompt_assembly.discovery.missing]
- [x] [serial] Define safe prompt assembly receipts with source labels/hashes, trust status, sections, budget, truncation, conflicts, policy hash, and BLAKE3 body hashes. r[agent_harness_runtime.clankers_prompt_assembly.receipts]
- [x] [depends:receipts] Add leak fixtures for raw prompt bodies, credentials, full paths, provider JSON, and hidden memory bodies. r[agent_harness_runtime.clankers_prompt_assembly.receipts.rejects_leaks]
- [x] [depends:sources.host_context,precedence.conflict,ledger.truncated,discovery.missing,receipts.rejects_leaks] Update docs and run focused prompt assembly tests, workspace checks, Cairn validation, and Cairn gates with reviewable evidence. r[agent_harness_runtime.clankers_prompt_assembly.verification]
