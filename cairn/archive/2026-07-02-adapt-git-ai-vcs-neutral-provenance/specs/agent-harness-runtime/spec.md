# agent-harness-runtime Change Spec: VCS-neutral provenance from Git AI patterns

## Requirements

### Requirement: Bounded Git AI provenance reference

r[agent_harness_runtime.provenance_git_ai.scope] Animus MUST treat Git AI as a bounded reference for explicit checkpointing, line-level attribution, session/model/tool identity, accepted-line stats, and optional Git AI v3 authorship-log interoperability, and MUST NOT adopt Git AI's daemon, installer, updater, hosted telemetry, SQLite prompt store, Git operation watcher, or Git-only workflow assumptions as reusable runtime architecture.

#### Scenario: Git AI remains a compatibility boundary

r[agent_harness_runtime.provenance_git_ai.scope.boundary]
- GIVEN maintainers adapt a Git AI-inspired provenance behavior
- WHEN the behavior is added to Animus
- THEN reusable runtime and provenance APIs are expressed in VCS-neutral terms
- AND Git-specific formats, refs, commands, and history-operation names are isolated to optional backend or compatibility modules.

### Requirement: VCS-neutral provenance domain

r[agent_harness_runtime.provenance_git_ai.domain] Animus provenance types MUST model content observations, repository-relative file identities, content hashes, line ranges, session identities, trace IDs, mutating tool kinds, and safe receipt metadata without requiring Git, Git notes, commit SHAs, branch names, hooks, stash, rebase, or any other Git-specific concept.

#### Scenario: Provenance core is directly testable without a VCS

r[agent_harness_runtime.provenance_git_ai.domain.no_vcs]
- GIVEN a test constructs provenance observations, line ranges, sessions, trace IDs, and content hashes in memory
- WHEN pure provenance validation and range transformation functions run
- THEN they return deterministic typed results without reading files, executing commands, invoking Git, opening a repository, or depending on a concrete VCS backend.

#### Scenario: Git terms are rejected from core DTOs

r[agent_harness_runtime.provenance_git_ai.domain.rejects_git_terms]
- GIVEN a proposed core provenance DTO adds a required Git note ref, commit SHA, branch, stash, rebase, or Git command field
- WHEN provenance boundary review or focused validation runs
- THEN the change fails the VCS-neutrality check unless the field is moved behind an optional Git compatibility adapter.

### Requirement: Injected version-control provenance backend

r[agent_harness_runtime.provenance_git_ai.vcs_backend] Runtime provenance persistence MUST use an injected version-control provenance backend or explicitly disabled backend setting, and MUST fail closed with a typed unavailable-backend or unsupported-operation error when provenance recording is required but no suitable backend is injected.

#### Scenario: In-memory non-Git backend records checkpoints

r[agent_harness_runtime.provenance_git_ai.vcs_backend.in_memory]
- GIVEN runtime dispatch records mutating tool checkpoints through an injected in-memory backend that does not implement Git semantics
- WHEN the tool call succeeds
- THEN the backend receives before and after provenance observations with session, trace, file identity, mutation kind, and content-hash metadata
- AND no Git command, Git hook, or Git note API is called.

#### Scenario: Missing required backend fails before mutation evidence is claimed

r[agent_harness_runtime.provenance_git_ai.vcs_backend.missing]
- GIVEN provenance recording is required by policy for a mutating tool action
- WHEN no backend is injected or the selected backend reports unsupported mutation tracking
- THEN runtime returns a typed provenance-backend failure
- AND no receipt claims complete line-level attribution for that action.

### Requirement: Nickel-governed mutating-tool checkpoint policy

r[agent_harness_runtime.provenance_git_ai.tool_policy] Mutating-tool checkpoint classification MUST be governed by Nickel-authored policy that maps raw tool names to canonical mutation kinds, file-identity fields, backend requirements, and receipt-redaction behavior before runtime dispatch records provenance.

#### Scenario: Valid policy classifies mutating tools

r[agent_harness_runtime.provenance_git_ai.tool_policy.valid]
- GIVEN Nickel policy maps raw tools to canonical edit, write, replace, or rename mutation kinds with explicit file-identity fields
- WHEN runtime dispatch prepares a matching tool action
- THEN the provenance observer records a before observation, calls the injected tool executor, records an after observation, and emits safe receipt metadata for the selected mutation kind.

#### Scenario: Invalid policy fails closed

r[agent_harness_runtime.provenance_git_ai.tool_policy.invalid]
- GIVEN Nickel policy references an unknown mutation kind, missing file-identity field, unsupported backend requirement, or unsafe receipt field
- WHEN policy export, drift check, or runtime loading runs
- THEN validation fails before the tool action is treated as provenance-trackable
- AND the failure identifies the policy issue without recording raw tool input or full paths.

### Requirement: Pure line-range and hunk-transform core

r[agent_harness_runtime.provenance_git_ai.range_core] Animus MUST provide pure deterministic line-range, range-compression, overlap, removal, hunk-shift, and accepted-line counting logic for provenance attribution, with explicit validation errors for malformed ranges, unsorted ranges, zero line numbers, underflow, overflow, and impossible hunk shapes.

#### Scenario: Positive range transformations are deterministic

r[agent_harness_runtime.provenance_git_ai.range_core.positive]
- GIVEN valid line ranges and diff hunk descriptions for insertions, deletions, replacements, and multiple hunks
- WHEN range transformation functions run
- THEN preserved lines retain attribution, deleted or replaced lines lose attribution, later lines shift by the accumulated offset, and accepted-line counts are deterministic.

#### Scenario: Malformed ranges fail without panic

r[agent_harness_runtime.provenance_git_ai.range_core.negative]
- GIVEN malformed line ranges, zero line numbers, reversed ranges, unsorted entries, invalid hunk headers, or arithmetic overflow inputs
- WHEN range parsing or transformation functions run
- THEN they return typed validation errors
- AND no panic, wrapping line number, or silently accepted invalid attribution is produced.

### Requirement: Optional Git AI v3 compatibility adapter

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter] Animus MAY provide an optional Git AI compatibility adapter that reads and writes `authorship/3.0.0` authorship logs for Git repositories, stores Git AI notes under `refs/notes/ai`, and confines Git AI-required SHA-256 key derivation to that adapter boundary while preserving BLAKE3 defaults for Animus-owned hashes and receipts.

#### Scenario: Git AI v3 authorship log round-trips at the adapter boundary

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter.round_trip]
- GIVEN a valid Git AI v3 authorship log containing session, trace, known-human, legacy prompt, file attestation, and metadata sections
- WHEN the optional compatibility adapter decodes and re-encodes it
- THEN the adapter preserves the externally required schema version, key routing, line ranges, metadata maps, and `refs/notes/ai` storage target
- AND the shared provenance domain receives only VCS-neutral observation data and external compatibility identifiers.

#### Scenario: Git AI-specific data does not become core requirement

r[agent_harness_runtime.provenance_git_ai.git_ai_adapter.not_core]
- GIVEN an embedder uses a non-Git backend or disables Git AI compatibility
- WHEN provenance recording runs
- THEN runtime does not require `refs/notes/ai`, Git commit SHAs, Git CLI availability, or Git AI SHA-256 IDs
- AND receipts continue to use Animus-owned BLAKE3 content hashes where a hash algorithm is not externally mandated.

### Requirement: Safe provenance receipts and session links

r[agent_harness_runtime.provenance_git_ai.receipts] Provenance receipts MUST expose safe metadata only, including status, issue code, backend kind, mutation kind, labels, bounded counts, BLAKE3 content hashes, external compatibility IDs, session IDs, trace IDs, and explicit redaction markers, and MUST NOT include raw prompts, provider payloads, credentials, full paths, raw tool bodies, or unbounded outputs.

#### Scenario: Provenance receipt links to session without leaking prompt text

r[agent_harness_runtime.provenance_git_ai.receipts.session_link]
- GIVEN a mutating tool action is associated with an agent session, model, and trace ID
- WHEN provenance receipt construction runs
- THEN the receipt records safe session, model, tool, mutation, count, content-hash, and backend metadata
- AND raw prompt text, raw model output, raw tool input, raw tool output, credentials, and full paths are omitted or explicitly redacted.

#### Scenario: Receipt leak fixture is rejected

r[agent_harness_runtime.provenance_git_ai.receipts.rejects_leaks]
- GIVEN a provenance receipt fixture contains raw prompt text, credential-looking values, an absolute path dump, raw provider payload, raw tool input, or unbounded tool output
- WHEN receipt validation runs
- THEN validation fails with a typed redaction error
- AND no receipt with leaked content is accepted as release evidence.

### Requirement: Focused VCS-neutral provenance verification

r[agent_harness_runtime.provenance_git_ai.verification] The provenance adaptation MUST include focused positive and negative verification for bounded Git AI scope, VCS-neutral DTOs, Nickel tool policy, non-Git backend recording, missing backend failures, pure range/hunk transforms, optional Git AI v3 round trips, and provenance receipt redaction.

#### Scenario: Verification proves Git is optional

r[agent_harness_runtime.provenance_git_ai.verification.git_optional]
- GIVEN the provenance adaptation is ready for review
- WHEN focused verification runs
- THEN evidence includes a non-Git in-memory backend test that records mutating-tool provenance without Git
- AND Git AI compatibility tests are isolated behind the optional Git adapter boundary.
