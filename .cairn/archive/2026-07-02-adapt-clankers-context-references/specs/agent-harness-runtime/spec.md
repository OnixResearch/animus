# agent-harness-runtime Change Spec: Clankers context reference adaptation

## Requirements

### Requirement: Typed context-reference inputs

r[agent_harness_runtime.clankers_context_references.typed_inputs] Animus MUST represent prompt context references as typed DTOs with raw label, normalized target, reference kind, optional range, requested render mode, and source trust metadata before policy or resolver decisions are trusted.

#### Scenario: Local file reference normalizes

r[agent_harness_runtime.clankers_context_references.typed_inputs.file]
- GIVEN a prompt includes a local file reference with an optional line range
- WHEN reference parsing and normalization run
- THEN Animus produces a typed file-reference DTO with the requested range and a safe target label
- AND no filesystem read is performed by the parser.

### Requirement: Expansion uses host-injected resolvers

r[agent_harness_runtime.clankers_context_references.resolvers] Context-reference expansion MUST call host-injected resolver services for filesystem, directory, diff, image, and URL references and MUST fail closed when a required resolver or capability is absent, denied, or unavailable.

#### Scenario: URL reference denied by policy

r[agent_harness_runtime.clankers_context_references.resolvers.url_denied]
- GIVEN a prompt includes an HTTPS reference
- AND URL retrieval is disabled by the active context-reference policy
- WHEN expansion is requested
- THEN Animus returns a typed denied-reference outcome
- AND no network resolver is invoked.

### Requirement: Context expansion bounds

r[agent_harness_runtime.clankers_context_references.bounds] Expanded context fragments MUST be included, truncated, or rejected according to deterministic budgets before they become model-visible prompt content.

#### Scenario: Oversized file reference truncates safely

r[agent_harness_runtime.clankers_context_references.bounds.oversized]
- GIVEN a resolver returns text larger than the configured context-reference visible budget
- WHEN budget evaluation runs
- THEN the model-visible fragment is truncated or rejected according to policy
- AND metadata records byte counts, line counts when available, truncation status, and a BLAKE3 hash without embedding the full raw body.

### Requirement: Safe provenance metadata

r[agent_harness_runtime.clankers_context_references.provenance] Context-reference receipts and ledger metadata MUST record safe provenance only, including kind, status, safe target label or hash, range, counts, truncation state, resolver class, policy decision, and optional BLAKE3 content hash.

#### Scenario: Metadata rejects leaks

r[agent_harness_runtime.clankers_context_references.provenance.rejects_leaks]
- GIVEN a context-reference metadata fixture contains raw expanded content, URL credentials, a full absolute path dump, a credential-looking value, or unbounded output
- WHEN metadata validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Context-reference verification

r[agent_harness_runtime.clankers_context_references.verification] The adaptation MUST include focused positive and negative tests for parsing, resolver injection, policy denial, malformed targets, path escapes, oversized references, unsupported kinds, missing resolver services, and safe metadata validation.

#### Scenario: Closeout evidence covers reference fixtures

r[agent_harness_runtime.clankers_context_references.verification.closeout]
- GIVEN maintainers intend to archive the context-reference adaptation
- WHEN closeout validation runs
- THEN evidence names focused parser, resolver, policy, budget, metadata, workspace, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
