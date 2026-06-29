# agent-harness-runtime Change Spec: Claude Code layered memory adaptation

## Requirements

### Requirement: Bounded Claude Code memory reference

r[agent_harness_runtime.claude_code_memory.scope] Animus MUST treat Claude Code memory as a bounded reference for project instructions, session preferences, and learned facts, and MUST NOT adopt Claude Code prompt text, hidden memory writes, provider-specific semantics, or `CLAUDE.md` as the canonical Animus storage contract.

#### Scenario: Compatibility import remains non-authoritative

r[agent_harness_runtime.claude_code_memory.scope.compat_import]
- GIVEN a host imports project guidance from a Claude Code-style file
- WHEN Animus prepares prompt context
- THEN the imported content is represented as typed Animus descriptors
- AND the original file format does not bypass trust, policy, validation, or redaction checks.

### Requirement: Typed instruction and memory descriptors

r[agent_harness_runtime.claude_code_memory.descriptors] Instruction and learned-memory inputs MUST be represented as typed descriptors with stable ID, source identity, trust scope, precedence class, freshness state, prompt-inclusion policy, and redaction metadata.

#### Scenario: Malformed descriptor is rejected

r[agent_harness_runtime.claude_code_memory.descriptors.invalid]
- GIVEN a memory descriptor is missing source identity, trust scope, or prompt-inclusion policy
- WHEN descriptor validation runs
- THEN validation returns a typed malformed-descriptor error
- AND the descriptor cannot influence prompt assembly.

### Requirement: Deterministic prompt assembly with attribution

r[agent_harness_runtime.claude_code_memory.prompt_assembly] Prompt assembly MUST deterministically order admitted instruction and memory descriptors by policy-defined precedence and MUST preserve source attribution for every included prompt section.

#### Scenario: Conflicting layers produce diagnostics

r[agent_harness_runtime.claude_code_memory.prompt_assembly.conflict]
- GIVEN trusted user, project, and session descriptors contain conflicting guidance for the same prompt section
- WHEN prompt assembly evaluates the descriptors
- THEN the selected precedence order is deterministic
- AND the assembly result includes a conflict diagnostic that names descriptor IDs without exposing redacted bodies.

### Requirement: Learned memory admission gate

r[agent_harness_runtime.claude_code_memory.admission] Learned memory candidates MUST require an injected host admission decision before they persist or influence future model prompts.

#### Scenario: Unadmitted candidate is not reused

r[agent_harness_runtime.claude_code_memory.admission.denied]
- GIVEN runtime proposes a memory candidate from a prior turn
- WHEN the host denies or omits admission
- THEN the candidate is not stored for future prompt influence
- AND the receipt records the denied admission status without embedding the raw candidate body.

### Requirement: Swappable memory adapter boundary

r[agent_harness_runtime.claude_code_memory.adapter_boundary] Memory discovery, storage, deletion, compatibility import, provider-assisted extraction, and admission MUST be represented as swappable injected adapter contracts that exchange typed descriptor, candidate, and admission DTOs; reusable core/runtime logic MUST NOT depend on a concrete memory database, file format, provider client, or host path layout.

#### Scenario: Memory backend swap preserves reusable behavior

r[agent_harness_runtime.claude_code_memory.adapter_boundary.swappable]
- GIVEN two hosts provide different memory source, store, importer, or extraction adapters
- WHEN they supply the same validated descriptors and admission outcomes to prompt assembly
- THEN reusable Animus logic produces equivalent prompt sections, diagnostics, and receipts
- AND missing adapters produce explicit unavailable outcomes rather than fallback ambient storage or discovery.

### Requirement: Safe memory influence receipts

r[agent_harness_runtime.claude_code_memory.receipts] Memory receipts MUST record safe metadata for prompt influence, including descriptor IDs, source labels, trust status, freshness state, inclusion status, redaction classes, and BLAKE3 hashes for redacted bodies, and MUST NOT include raw credentials, raw provider payloads, absolute path dumps, or unbounded prompt text.

#### Scenario: Secret-looking memory body is redacted

r[agent_harness_runtime.claude_code_memory.receipts.redacted]
- GIVEN an admitted memory descriptor body matches host redaction policy
- WHEN prompt-influence receipts are emitted
- THEN the receipt includes redaction class and BLAKE3 body hash metadata
- AND the raw body is absent from model-visible receipts and audit metadata.
