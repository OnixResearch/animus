# agent-harness-runtime Change Spec: Clankers prompt assembly kit adaptation

## Requirements

### Requirement: Typed prompt sources

r[agent_harness_runtime.clankers_prompt_assembly.sources] Animus prompt assembly MUST consume typed prompt-source DTOs with source kind, scope, trust/admission status, priority, budget class, redaction class, and safe identity before any source can influence model-visible prompt content.

#### Scenario: Host context source assembles without discovery

r[agent_harness_runtime.clankers_prompt_assembly.sources.host_context]
- GIVEN a host supplies an admitted host-context prompt source
- WHEN prompt assembly runs in host-context-only mode
- THEN the source can contribute an attributed bounded section
- AND no filesystem, memory, skill, URL, or project discovery is attempted.

### Requirement: Deterministic precedence policy

r[agent_harness_runtime.clankers_prompt_assembly.precedence] Prompt assembly MUST apply deterministic policy-defined section ordering, source precedence, conflict behavior, and required/optional source rules.

#### Scenario: Conflicting sources resolve deterministically

r[agent_harness_runtime.clankers_prompt_assembly.precedence.conflict]
- GIVEN two admitted sources conflict for the same prompt section
- WHEN assembly applies precedence policy
- THEN the selected source order and conflict diagnostic are deterministic
- AND the ledger identifies the skipped or overridden source without leaking raw body text.

### Requirement: Assembly budget ledger

r[agent_harness_runtime.clankers_prompt_assembly.ledger] Prompt assembly MUST produce a ledger recording included, skipped, truncated, redacted, denied, conflicting, and unsupported sources with safe metadata and budget consumption.

#### Scenario: Over-budget source is truncated with evidence

r[agent_harness_runtime.clankers_prompt_assembly.ledger.truncated]
- GIVEN an admitted prompt source exceeds its configured section budget
- WHEN assembly evaluates the source
- THEN model-visible content is truncated or skipped according to policy
- AND the ledger records budget use, truncation status, and BLAKE3 body identity without embedding the full raw source.

### Requirement: Ambient discovery remains host-owned

r[agent_harness_runtime.clankers_prompt_assembly.discovery] Reusable prompt assembly MUST NOT perform ambient filesystem, project-file, memory-store, skill-store, URL, or tool-discovery side effects; hosts MUST inject admitted sources or discovery adapters.

#### Scenario: Missing discovery adapter is explicit

r[agent_harness_runtime.clankers_prompt_assembly.discovery.missing]
- GIVEN assembly policy references a source kind that requires host discovery
- AND no discovery adapter or admitted source exists for that kind
- WHEN assembly runs
- THEN the ledger records an unsupported or unavailable source outcome
- AND no fallback ambient discovery occurs.

### Requirement: Safe prompt assembly receipts

r[agent_harness_runtime.clankers_prompt_assembly.receipts] Prompt assembly receipts MUST record source kind, safe label or hash, trust/admission status, section, budget use, truncation status, conflict reason, policy hash, and BLAKE3 source hashes, and MUST NOT include raw prompt bodies, credentials, full paths, provider payloads, or hidden memory bodies.

#### Scenario: Prompt assembly receipt leak rejected

r[agent_harness_runtime.clankers_prompt_assembly.receipts.rejects_leaks]
- GIVEN a prompt assembly receipt fixture contains raw prompt body text, a credential-looking value, a full path dump, provider JSON, or hidden memory body
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Prompt assembly verification

r[agent_harness_runtime.clankers_prompt_assembly.verification] The adaptation MUST include focused positive and negative tests for typed sources, deterministic precedence, ledger states, budget truncation, missing discovery adapters, untrusted sources, unsupported source kinds, and receipt redaction.

#### Scenario: Closeout evidence covers assembly fixtures

r[agent_harness_runtime.clankers_prompt_assembly.verification.closeout]
- GIVEN maintainers intend to archive the prompt assembly kit adaptation
- WHEN closeout validation runs
- THEN evidence names focused source, precedence, ledger, discovery, receipt, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
