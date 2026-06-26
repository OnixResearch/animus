# agent-harness-runtime Change Spec: Qualifier-inspired quality records

## Requirements

### Requirement: Bounded Qualifier quality-record reference

r[agent_harness_runtime.qualifier_quality.scope] Animus MUST treat Qualifier as a bounded reference for ambient review annotations, Metabox-style envelopes, append-only JSONL sidecars, span freshness, unknown-record preservation, annotation lifecycle semantics, and AGENTS-CLI guidance, and MUST NOT adopt Qualifier as a core runtime dependency, session ledger, transcript store, Git-only diff engine, or numeric scoring authority.

#### Scenario: Qualifier remains a compatibility boundary

r[agent_harness_runtime.qualifier_quality.scope.boundary]
- GIVEN maintainers adapt Qualifier-inspired quality-record behavior
- WHEN reusable Animus runtime, core, or adapter APIs are reviewed
- THEN the APIs use Animus-owned DTOs and traits
- AND they do not require the `qualifier` crate, `.qual` files, Git commands, filesystem discovery, CLI execution, or scoring semantics in reusable core contracts.

#### Scenario: Raw payloads are outside quality records

r[agent_harness_runtime.qualifier_quality.scope.redaction]
- GIVEN a quality observation is derived from a prompt, provider payload, tool input, tool output, model output, credential-bearing configuration, or absolute host path
- WHEN an Animus quality/provenance record body is constructed
- THEN raw sensitive material is omitted or represented only by approved BLAKE3 hashes, bounded counts, labels, and explicit redaction markers
- AND validation rejects any record body that stores the raw material directly.

### Requirement: Animus-owned Metabox-style record envelope

r[agent_harness_runtime.qualifier_quality.envelope] Animus MUST define a safe record envelope with explicit version, record type, subject, issuer, optional issuer classification, shell-supplied creation metadata, content-addressed BLAKE3 id, and typed body, with deterministic canonicalization and validation implemented as pure functional-core logic.

#### Scenario: Canonical IDs are deterministic

r[agent_harness_runtime.qualifier_quality.envelope.deterministic]
- GIVEN equivalent safe record envelopes with body fields and nested fields in different source orders
- WHEN canonicalization and id generation run
- THEN field order is normalized, optional absent fields are omitted, and the same lowercase BLAKE3 id is produced
- AND the canonicalization function does not read files, clocks, environment, process state, or network resources.

#### Scenario: Malformed envelopes fail closed

r[agent_harness_runtime.qualifier_quality.envelope.invalid]
- GIVEN a record envelope has an unsupported version, empty subject, empty issuer, non-URI issuer, invalid id, mismatched id, missing required body field, or malformed issuer classification
- WHEN envelope validation runs
- THEN validation returns typed errors naming the failed invariant
- AND the invalid record is not accepted as runtime, review, or release evidence.

### Requirement: Optional Qualifier sidecar adapter

r[agent_harness_runtime.qualifier_quality.sidecar_adapter] Animus MAY provide an optional std-only Qualifier sidecar adapter that reads and writes `.qual`-compatible JSONL annotations for repository-local quality observations while keeping sidecar persistence, filesystem traversal, ignore handling, and compaction outside no-std core and adapter-contract crates.

#### Scenario: Sidecar export preserves safe annotation semantics

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.export]
- GIVEN an Animus safe quality observation contains subject, kind, span, summary, suggested fix, references, supersedes, tags, issuer classification, and redaction-safe metadata
- WHEN Qualifier sidecar export is explicitly requested
- THEN the adapter writes a `.qual`-compatible annotation record with a deterministic id and safe body fields
- AND raw prompts, provider payloads, tool/model outputs, credentials, full paths, and unbounded content are not written.

#### Scenario: Unknown sidecar records round-trip without becoming core data

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.unknown]
- GIVEN a `.qual` file contains unknown record types or future body shapes
- WHEN the optional sidecar adapter loads, filters, compacts, or rewrites the file
- THEN unknown records are preserved byte-for-byte or as an explicitly equivalent opaque sidecar record where the sidecar operation requires rewriting
- AND reusable Animus core receives only sanitized safe metadata or a typed compatibility error, not the raw unknown body.

#### Scenario: Default builds remain isolated

r[agent_harness_runtime.qualifier_quality.sidecar_adapter.isolated]
- GIVEN default Animus features are built without Qualifier compatibility
- WHEN dependency and feature-isolation checks run
- THEN `animus-core`, `animus-adapters`, `animus-preserves`, `animus-steel`, and default `animus-std` contracts do not require Qualifier crates, `.qual` discovery, Git diff libraries, or sidecar compaction code.

### Requirement: Annotation lifecycle state transitions

r[agent_harness_runtime.qualifier_quality.lifecycle] Animus MUST model quality annotation lifecycle behavior as pure state transitions over in-memory records, including built-in and custom kinds, typo warnings, references, supersession chains, resolve records, waiver rationale, suggested fixes, active-record filtering, same-subject supersession checks, and supersession cycle detection.

#### Scenario: References thread context without closing records

r[agent_harness_runtime.qualifier_quality.lifecycle.references]
- GIVEN one quality record references another record by id
- WHEN active quality records are computed
- THEN both records remain active and the referencing record is displayed or queried as thread context
- AND self-references are rejected with a typed validation error.

#### Scenario: Supersession closes only same-subject records

r[agent_harness_runtime.qualifier_quality.lifecycle.supersession]
- GIVEN a resolve or replacement record supersedes a prior record
- WHEN active quality records are computed
- THEN the superseded record is excluded from the active set and the superseding record remains as the visible tip
- AND validation rejects cross-subject supersession, duplicate-id ambiguity, and cycles without panicking.

#### Scenario: Agents do not auto-resolve user concerns

r[agent_harness_runtime.qualifier_quality.lifecycle.no_auto_resolve]
- GIVEN an agent edits code near a human-authored concern or drifted span
- WHEN runtime or sidecar tooling reports quality status
- THEN it surfaces the concern, drift, or missing span for review without resolving it automatically
- AND a resolve record is emitted only when the caller explicitly requests closure.

### Requirement: VCS-neutral span freshness

r[agent_harness_runtime.qualifier_quality.span_freshness] Animus MUST provide VCS-neutral span freshness logic that hashes the full lines covered by a valid span with BLAKE3 and classifies observations as fresh, drifted, missing, or no-hash without requiring Git, branch names, merge-base logic, or Qualifier CLI execution.

#### Scenario: Freshness states are deterministic

r[agent_harness_runtime.qualifier_quality.span_freshness.states]
- GIVEN file content lines, a valid one-indexed span, and an optional recorded content hash
- WHEN pure freshness classification runs over the supplied lines
- THEN matching hashes return fresh, mismatched hashes return drifted with expected and actual hashes, absent hashes return no-hash, and missing span lines return missing
- AND equivalent line content produces the same BLAKE3 hash regardless of caller or storage backend.

#### Scenario: Invalid spans fail closed

r[agent_harness_runtime.qualifier_quality.span_freshness.invalid]
- GIVEN a span has zero line numbers, reversed ranges, columns outside allowed bounds, or arithmetic overflow inputs
- WHEN span parsing or freshness preparation runs
- THEN it returns typed validation errors
- AND no panic, wrapped line number, silently clipped range, or raw file dump is produced.

### Requirement: AGENTS-CLI-inspired Animus guidance

r[agent_harness_runtime.qualifier_quality.agents_cli] When Animus exposes a public CLI, it SHOULD provide an `animus agents` entry point that prints version-locked markdown guidance for AI coding agents, lists available topics, supports topic pages for concepts, workflows, pitfalls, and non-trivial commands, advertises the entry point in `animus --help`, and returns a structured unknown-topic error.

#### Scenario: Bare agents invocation orients coding agents

r[agent_harness_runtime.qualifier_quality.agents_cli.orientation]
- GIVEN the Animus CLI includes the agents guidance feature
- WHEN an agent runs `animus agents`
- THEN stdout contains a self-contained orientation explaining what Animus is, when to use it, when not to use it, and which topics are available
- AND the command exits successfully without reading network resources.

#### Scenario: Topic lookup is deterministic

r[agent_harness_runtime.qualifier_quality.agents_cli.topics]
- GIVEN a known guidance topic and an unknown guidance topic
- WHEN an agent runs `animus agents <topic>` for each topic
- THEN the known topic prints its markdown page and exits successfully
- AND the unknown topic exits with a structured error that names the unknown topic and every available topic.

### Requirement: Focused Qualifier adaptation verification

r[agent_harness_runtime.qualifier_quality.verification] The Qualifier adaptation MUST include focused positive and negative verification for bounded reference scope, deterministic envelope IDs, malformed envelope rejection, safe body redaction, unknown sidecar preservation, sidecar dependency isolation, annotation lifecycle transitions, span freshness states, invalid span handling, no-auto-resolution behavior, and AGENTS-CLI guidance behavior when implemented.

#### Scenario: Closeout evidence is reviewable

r[agent_harness_runtime.qualifier_quality.verification.closeout]
- GIVEN maintainers intend to archive the Qualifier quality-record adaptation
- WHEN closeout validation runs
- THEN evidence names every successful focused rail, workspace test, clippy check, Nickel/config drift check if applicable, Cairn validation, and Cairn gate
- AND any skipped rail includes a concrete blocker and next-best validation.
