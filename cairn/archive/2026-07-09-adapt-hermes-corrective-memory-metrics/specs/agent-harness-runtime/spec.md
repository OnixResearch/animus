# agent-harness-runtime Change Spec: Hermes corrective memory and review metrics

## Requirements

### Requirement: Corrective records and metrics are typed optional data

r[agent_harness_runtime.hermes_corrective_memory.records] Animus MUST represent corrective memory and review metrics as optional typed records with kind, source, trust scope, subject, class/scope/route/promoted tags when applicable, freshness metadata, safe body identity, and redaction state.

#### Scenario: Host supplies corrective records from a non-file store

r[agent_harness_runtime.hermes_corrective_memory.records.host_store]
- GIVEN a host loads corrective records from an application database, embedded bundle, repo file, or operator-supplied packet
- WHEN Animus validates those records
- THEN valid records normalize into safe typed descriptors
- AND reusable logic does not require `.agent/napkin.md`, global memory directories, JSONL sidecars, or Hermes runtime state.

### Requirement: Corrective prompt influence requires explicit admission

r[agent_harness_runtime.hermes_corrective_memory.admission] Corrective records MUST NOT influence prompt assembly, tool activation, policy selection, review outcomes, or workflow routing unless host policy admits them by trust, scope, freshness, subject relevance, and prompt budget.

#### Scenario: Untrusted napkin-like note is omitted

r[agent_harness_runtime.hermes_corrective_memory.admission.untrusted]
- GIVEN a project-local corrective note exists but no host trust decision admits its content identity
- WHEN prompt context assembly requests corrective guidance
- THEN the note is omitted or represented by a safe diagnostic
- AND its body cannot silently override user requests, Cairn specs, tool policy, or runtime capability checks.

### Requirement: Promotion candidates are advisory

r[agent_harness_runtime.hermes_corrective_memory.promotions] Repeated-finding analysis SHOULD produce advisory promotion candidates that name safe repeated tags, counts, evidence hashes, and suggested target routes such as deterministic check, policy guidance, spec rule, memory guidance, or human checkpoint, but MUST NOT mutate tasks, specs, policy, memory stores, or code automatically.

#### Scenario: Repeated findings do not edit tasks automatically

r[agent_harness_runtime.hermes_corrective_memory.promotions.no_auto_edit]
- GIVEN multiple review metric records contain the same unpromoted class/scope/route finding
- WHEN promotion grouping runs
- THEN it returns a candidate with count, reason, suggested target, and safe evidence identity
- AND no Cairn task, spec, policy file, corrective store, or code file is written by reusable logic.

### Requirement: Corrective persistence is an explicit host action

r[agent_harness_runtime.hermes_corrective_memory.host_actions] Appending corrective records, marking promotions, superseding records, exporting summaries, or writing promotion stubs MUST be represented as explicit host-owned action requests with validation, authorization inputs, and safe receipts.

#### Scenario: Missing store blocks append

r[agent_harness_runtime.hermes_corrective_memory.host_actions.missing_store]
- GIVEN a host asks to append a corrective record
- AND no corrective-record store adapter is injected
- WHEN append action validation runs
- THEN Animus returns a missing-store failure
- AND it does not create `.agent/napkin.md`, write JSONL, mutate memory, or persist metrics by fallback.

### Requirement: Corrective memory verification

r[agent_harness_runtime.hermes_corrective_memory.verification] The corrective-memory adaptation MUST include focused positive and negative tests for valid records, metrics records, trust admission, prompt-budget omission, repeated-finding grouping, explicit append/promote actions, missing-store failure, no default napkin behavior, stale or unsafe records, and receipt leak rejection.

#### Scenario: Closeout evidence preserves framework non-claims

r[agent_harness_runtime.hermes_corrective_memory.verification.closeout]
- GIVEN maintainers intend to archive the corrective-memory adaptation
- WHEN closeout validation runs
- THEN evidence names record, admission, promotion, host-action, receipt, docs, Cairn validation, and Cairn gate checks
- AND it lists mandatory napkin loading, global corrective memory, automatic writes, task/spec mutation, and scoring authority as non-claims.
