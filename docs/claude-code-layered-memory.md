# Claude Code layered memory reference

Animus treats Claude Code memory as a bounded UX reference: project instructions, session preferences, and learned facts can influence future turns only after hosts convert them into typed Animus descriptors. `CLAUDE.md`, hidden memory writes, provider-specific memory semantics, and Claude Code prompt text are not canonical Animus contracts.

## Descriptor boundary

`MemoryDescriptor` records stable descriptor ID, kind, safe source label, trust scope, precedence, freshness, inclusion policy, host admission status, redaction class, BLAKE3 body hash, and prompt-budget units. Validation rejects missing source labels, missing trust scope, missing inclusion policy, expired freshness, invalid hashes, and zero budget units before data can reach prompt assembly.

## Prompt assembly

`assemble_memory_prompt` is pure over in-memory descriptors and `MemoryPromptPolicy`. It maps validated memory descriptors to the reusable prompt assembly core without reading files, querying memory stores, using providers, or discovering host paths. Ordering is deterministic by selected section, memory precedence, source kind, and descriptor ID. Conflicts and stale or denied memory are ledgered with descriptor IDs and safe hashes rather than raw bodies.

## Admission and adapters

Learned-memory candidates use `MemoryCandidate`, `MemoryAdmissionDecision`, and `resolve_memory_admission`. Denied or unavailable admission returns `DoNotPersist` and no descriptor, so it cannot persist or influence future prompts. Source, store, importer, extraction, and admission adapters are swappable host boundaries; reusable logic consumes only validated DTOs.

## Receipts

`MemoryInfluenceReceipt` records safe influence metadata: descriptor ID, source label, trust scope, freshness, inclusion status, redaction class, and BLAKE3 body hash. Receipt and admission validation reject raw memory/candidate bodies, credentials, provider payloads, full paths, and unbounded prompt text.

## Non-claims

This contract does not implement a memory database, auto-write hidden facts, read `CLAUDE.md`, discover project files, call providers for extraction, persist denied candidates, or expose raw private memory to model-visible prompts or audit receipts.
