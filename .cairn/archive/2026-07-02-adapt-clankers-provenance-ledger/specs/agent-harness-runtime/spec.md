# agent-harness-runtime Change Spec: Animus-owned provenance ledger

## Requirements

### Requirement: Bounded Clankers and Toolpath provenance references

r[agent_harness_runtime.provenance_ledger.scope] Animus MUST define its own internal provenance/session-ledger model, MUST treat Clankers as a bounded reference for neutral replay, safe typed facts, BLAKE3 artifact identity, tool-host/effect receipts, and redaction patterns, and MUST treat Toolpath as an optional interchange reference rather than the internal source of truth.

#### Scenario: Reference scope rejects wholesale imports

r[agent_harness_runtime.provenance_ledger.scope.rejects_wholesale]
- GIVEN maintainers adapt provenance, session-ledger, or receipt behavior from Clankers or Toolpath
- WHEN the reusable Animus core or adapter crates are reviewed
- THEN the implementation uses Animus-owned DTOs and traits
- AND it does not require `toolpath`, `toolpath-convo`, `clankers-session`, `clanker-message`, Automerge, daemon, TUI, Pathbase, GitHub, Git, Matrix, plugin runtime, or global session-directory dependencies in reusable core contracts.

### Requirement: Animus-owned neutral session ledger

r[agent_harness_runtime.provenance_ledger.session_ledger] Animus MUST provide neutral session-ledger DTOs for model-visible messages, summaries, usage observations, safe receipts, and unsupported entries without depending on a concrete storage backend, daemon, UI, provider, transcript compatibility type, or wall-clock construction.

#### Scenario: Ledger replay rebuilds model-visible context

r[agent_harness_runtime.provenance_ledger.session_ledger.replay]
- GIVEN an in-memory ledger containing user, assistant, tool, summary, usage, and receipt entries
- WHEN pure replay runs
- THEN it returns model-visible messages in deterministic order with safe replay metadata counts
- AND it does not read files, inspect environment, allocate timestamps, spawn processes, call providers, or depend on Clankers desktop transcript IDs.

#### Scenario: Unsupported ledger entries fail closed

r[agent_harness_runtime.provenance_ledger.session_ledger.unsupported]
- GIVEN replay encounters an unsupported or adapter-owned ledger entry
- WHEN replay is required for a model request
- THEN it returns a typed unsupported-entry error
- AND no raw unsupported payload is included in the error or receipt.

### Requirement: Append-only safe provenance facts

r[agent_harness_runtime.provenance_ledger.typed_facts] Animus MUST define append-only typed provenance facts for model activity, tool activity, runtime blocks, errors, artifact references, authorization decisions, provenance observations, and opaque unknown records with safe query fields only.

#### Scenario: Typed facts are queryable without raw payloads

r[agent_harness_runtime.provenance_ledger.typed_facts.query]
- GIVEN a ledger contains model, tool, artifact, authorization, and error facts
- WHEN safe query filtering runs by artifact hash, tool kind, error class, requirement id, request shape, authorization status, effect ability, or effect resource
- THEN matching facts are returned deterministically
- AND raw prompts, provider payloads, tool input/output, credentials, and full paths are not required for the query.

#### Scenario: Unknown facts become safe opaque records

r[agent_harness_runtime.provenance_ledger.typed_facts.opaque]
- GIVEN a future or unknown fact shape is loaded from a compatibility boundary
- WHEN the fact cannot be decoded as a current typed record
- THEN Animus preserves only sanitized safe metadata or returns a typed compatibility error
- AND it does not retain or emit the unknown raw payload by default.

### Requirement: BLAKE3 artifact identity and redaction classes

r[agent_harness_runtime.provenance_ledger.artifacts] Animus artifact identities MUST use versioned canonical envelopes with artifact kind, envelope version, redaction class, normalized payload, sorted dependencies, and BLAKE3 hashes for Animus-owned content identity.

#### Scenario: Canonical artifact hashes are deterministic

r[agent_harness_runtime.provenance_ledger.artifacts.deterministic]
- GIVEN equivalent artifact envelopes with differently ordered dependency inputs
- WHEN artifact identity is computed
- THEN dependencies are sorted and deduplicated before hashing
- AND the resulting BLAKE3 hash is deterministic.

#### Scenario: Redaction classes gate payload display

r[agent_harness_runtime.provenance_ledger.artifacts.redaction]
- GIVEN an artifact envelope is marked metadata-only, redacted-payload, or secret
- WHEN receipt, CLI, or review output is rendered
- THEN payload display is omitted or replaced by explicit redaction markers
- AND only public artifacts may expose payload bodies by default.

### Requirement: Pure parent/branch provenance queries

r[agent_harness_runtime.provenance_ledger.branch_queries] Animus MUST provide pure parent/branch query helpers for ancestry, active branch reconstruction, branch divergence, unique branch entries, and abandoned/dead-end attempts over Animus-owned ledger/provenance DTOs.

#### Scenario: Branch queries detect abandoned attempts

r[agent_harness_runtime.provenance_ledger.branch_queries.dead_ends]
- GIVEN a provenance graph has one active head and a sibling branch that is not an ancestor of that head
- WHEN dead-end query runs
- THEN the sibling branch entries are returned as abandoned attempts
- AND the active branch entries are not misclassified as dead ends.

#### Scenario: Malformed parent references fail without panic

r[agent_harness_runtime.provenance_ledger.branch_queries.invalid]
- GIVEN a graph contains a missing parent reference, cycle, duplicate entry id, or reversed ordering assumption
- WHEN validation or query preparation runs
- THEN it returns a typed graph validation error
- AND no panic, infinite traversal, or silently accepted invalid ancestry is produced.

### Requirement: Safe runtime and provenance receipts

r[agent_harness_runtime.provenance_ledger.receipts] Animus runtime and provenance receipts MUST expose safe metadata only, including status, issue code, actor/session/trace labels, bounded counts, artifact hashes, mutation or effect kind, backend/interchange labels, and redaction markers, and MUST NOT include raw prompt text, provider payloads, credentials, full paths, raw tool bodies, raw model outputs, or unbounded outputs.

#### Scenario: Safe receipts link evidence without leaking content

r[agent_harness_runtime.provenance_ledger.receipts.safe]
- GIVEN a model call, tool call, authorization decision, or provenance observation has raw sensitive inputs and outputs
- WHEN a receipt is constructed
- THEN the receipt contains only safe labels, bounded counts, redaction markers, and BLAKE3 hashes
- AND the sensitive raw material is omitted.

#### Scenario: Receipt leak fixture is rejected

r[agent_harness_runtime.provenance_ledger.receipts.rejects_leaks]
- GIVEN a receipt fixture contains raw prompt text, provider JSON, credential-looking values, an absolute path dump, raw tool input, raw model output, or unbounded output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND the invalid receipt is not accepted as review or release evidence.

### Requirement: Tool-host and effect receipts remain adapter-safe

r[agent_harness_runtime.provenance_ledger.tool_effects] Animus MUST model tool-host outcomes, service availability, capability decisions, hook phases, runtime policy decisions, cancellation, truncation metadata, effect requests, and effect results as adapter-safe DTOs before provenance facts or receipts claim tool execution evidence.

#### Scenario: Missing tool service fails before evidence is claimed

r[agent_harness_runtime.provenance_ledger.tool_effects.missing_service]
- GIVEN a tool action requires storage, search, hooks, process, progress, capability, cancellation, or runtime-policy service support
- WHEN the service is missing or unavailable
- THEN dispatch returns a typed unavailable-service or capability-denied outcome
- AND no provenance receipt claims the tool mutation or effect completed.

#### Scenario: Tool output truncation is recorded safely

r[agent_harness_runtime.provenance_ledger.tool_effects.truncation]
- GIVEN a tool emits output larger than configured byte or line limits
- WHEN the tool-host accumulator finishes
- THEN the model-visible output is bounded
- AND the receipt records original and truncated counts without storing the omitted output.

### Requirement: Optional Toolpath interchange adapter

r[agent_harness_runtime.provenance_ledger.toolpath_adapter] Animus MAY provide an optional std-only Toolpath adapter that exports redacted Animus provenance as Toolpath Graph/Path/Step documents and imports redacted Toolpath documents into review fixtures, MUST treat Toolpath as an interchange and review vocabulary rather than the internal ledger, and MUST keep Toolpath crates, `toolpath-convo`, Pathbase, provider importers, Git/GitHub derivation, and CLI cache/share/resume behavior out of default reusable contracts.

#### Scenario: Redacted Toolpath export maps Animus DAGs

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.export]
- GIVEN an Animus provenance ledger contains actors, parent relationships, active heads, artifact hashes, safe structural changes, typed refs, correlation links, and redacted receipts
- WHEN Toolpath export is explicitly requested
- THEN the adapter emits a Toolpath graph with equivalent path head, step parents, actor labels, typed refs, artifact hash markers, and safe structural perspectives
- AND raw prompts, provider payloads, tool input/output, full paths, credentials, raw unified diffs, and unbounded before/after content are omitted or represented by hashes/redaction markers.

#### Scenario: Default Toolpath export remains redacted

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.redacted_default]
- GIVEN a Toolpath export candidate includes raw prompt text, provider JSON, credential-looking values, absolute path dumps, raw tool input, raw tool output, raw model output, unbounded before/after content, or raw unified diffs
- WHEN default redacted export validation runs
- THEN the adapter rejects the unsafe fields or replaces them with explicit redaction markers and BLAKE3 artifact hashes
- AND full-fidelity raw export is unavailable unless a future explicit policy enables it.

#### Scenario: Toolpath adoption matrix is documented

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.docs]
- GIVEN maintainers evaluate Toolpath features for Animus
- WHEN the adoption matrix is reviewed
- THEN Graph/Path/Step, actor taxonomy, parent/head semantics, dead-end queries, typed refs/correlation, JSONL streaming shape, and Markdown/DOT review surfaces are documented as optional projection or review concerns
- AND Toolpath is not documented as Animus' internal ledger, runtime conversation IR, hashing authority, provider importer, or default storage format.

#### Scenario: Toolpath dependency isolation is verified

r[agent_harness_runtime.provenance_ledger.toolpath_adapter.isolated]
- GIVEN default Animus features are built without Toolpath compatibility
- WHEN workspace dependency checks run
- THEN reusable core, adapter, Steel, and standard-shell contracts do not require Toolpath crates, `toolpath-convo`, Pathbase clients, provider session importers, GitHub/Git derivation crates, or Toolpath cache/share/resume CLI behavior.

### Requirement: Focused provenance-ledger verification

r[agent_harness_runtime.provenance_ledger.verification] The provenance-ledger adaptation MUST include focused positive and negative verification for bounded reference scope, neutral ledger replay, unsupported-entry failure, typed fact queries, opaque record handling, BLAKE3 artifact identity, redaction classes, branch/dead-end queries, malformed graph rejection, tool/effect outcomes, safe receipts, optional Toolpath export, redacted-by-default Toolpath behavior, Toolpath adoption documentation, and dependency isolation.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.provenance_ledger.verification.closeout]
- GIVEN maintainers intend to archive the provenance-ledger adaptation
- WHEN closeout validation runs
- THEN evidence names every successful focused rail, workspace test, clippy check, Nickel/config drift check if applicable, Cairn validation, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.
