# agent-harness-runtime Change Spec: Add context adapter family

## Requirements

### Requirement: Context adapter family

r[agent_harness_runtime.context_adapters.family] Animus MUST maintain `family:context` as a public adapter contract family that names owner modules, stability or version posture, authority boundary, safe receipt behavior, serialization bridge status, focused surfaces, positive evidence, negative evidence, and focused validation checks for context-reference resolvers, context-layer descriptor sources, external context handles, and prompt context assembly boundaries.

#### Scenario: Context family has discoverable ownership

r[agent_harness_runtime.context_adapters.family.indexed]
- GIVEN an embedder needs to provide model-visible context through Animus
- WHEN they inspect the contracts index or adapter conformance index
- THEN the context family identifies the prompt/context modules and std-shell resolver surfaces they can implement
- AND the index states that reusable crates own parsing, planning, budgeting, admission, and safe provenance rather than host I/O.

### Requirement: Role-specific context contracts

r[agent_harness_runtime.context_adapters.role_contracts] Context adapter contracts MUST remain split by explicit role, including file, directory, diff, image, URL, context-layer source, and external-handle resolution roles, and MUST NOT require a monolithic adapter that combines filesystem, VCS, image, network, crawler, credential, and global discovery authority.

#### Scenario: Host registers only the needed context role

r[agent_harness_runtime.context_adapters.role_contracts.positive]
- GIVEN a host supports local file context but not URL fetching or image decoding
- WHEN it composes context services
- THEN it can provide a file resolver without providing URL, image, crawler, credential, or global discovery authority
- AND planner logic treats absent roles as unavailable instead of falling back to ambient access.

#### Scenario: Missing or denied context role fails closed

r[agent_harness_runtime.context_adapters.role_contracts.negative]
- GIVEN a prompt requests a directory, diff, image, URL, layered context source, or external context handle
- AND the corresponding resolver role is absent, unsupported, unavailable, or denied by policy
- WHEN context planning runs
- THEN the request fails closed before resolver dispatch
- AND no filesystem read, VCS command, image decode, network fetch, credential lookup, or global discovery is attempted by reusable code.

### Requirement: Context adapter conformance matrix

r[agent_harness_runtime.context_adapters.conformance] The context adapter family MUST declare conformance expectations for happy path, missing service, denied capability, unavailable service, timeout, cancellation, malformed output, oversized output, unsafe reference, schema mismatch, duplicate field, unknown field, and leak rejection cases, or provide explicit safe non-applicability reasons for cases where reusable context contracts own no clock, cancellation handle, or open-map parser.

#### Scenario: Context matrix includes positive and negative evidence

r[agent_harness_runtime.context_adapters.conformance.coverage]
- GIVEN maintainers add or change context adapter behavior
- WHEN adapter-conformance checks run
- THEN the context family has a covered positive evidence label and a covered negative evidence label for every applicable case
- AND unsupported or not-applicable cases include safe reasons without raw prompts, file bodies, private paths, credentials, provider payloads, or unbounded logs.

### Requirement: Safe context evidence

r[agent_harness_runtime.context_adapters.safe_evidence] Context adapter outcomes, receipts, and provenance records MUST store only safe target labels, resolver role, source kind, status, policy decision, bounded counts, truncation state, redaction markers, omission reasons, and BLAKE3 content or body hashes when durable evidence is needed.

#### Scenario: Context evidence rejects leaks

r[agent_harness_runtime.context_adapters.safe_evidence.leak_rejection]
- GIVEN a context resolver output, context-layer descriptor, external context handle, or receipt draft includes raw expanded body text, raw prompt text, credentials, provider payloads, headers, environment values, full private paths, hidden transcripts, or unbounded logs in durable metadata
- WHEN context evidence validation runs
- THEN validation rejects or redacts the unsafe material before persistence, logging, metrics, or review summaries
- AND the failure evidence itself remains bounded and safe.

### Requirement: Standard-library context resolvers

r[agent_harness_runtime.context_adapters.std_resolvers] `animus-std` SHOULD provide explicit no-op/null context services and policy-gated local file, directory, and diff resolver implementations for deterministic embedding, while URL fetching, image decoding, repository-wide crawling, credentialed remote handles, and external context stores MUST remain absent by default or downstream/feature-gated host-owned adapters.

#### Scenario: Std local context resolves within policy

r[agent_harness_runtime.context_adapters.std_resolvers.positive]
- GIVEN a host selects the std local file, directory, or diff resolver with a policy that admits the requested target
- WHEN context-reference expansion runs
- THEN the resolver returns an already-loaded candidate that reusable code budgets, hashes, and classifies deterministically
- AND no default network, image, crawler, credential, or global discovery service is enabled.

#### Scenario: Std local context denies unsafe targets

r[agent_harness_runtime.context_adapters.std_resolvers.negative]
- GIVEN a context request attempts path escape, absolute private path access, binary or oversized body inclusion, unavailable VCS state, denied network/image authority, or malformed resolver output
- WHEN the std resolver or reusable validator handles the request
- THEN it returns a typed denied, unavailable, malformed, oversized, or unsafe outcome
- AND no unsafe body or private host path is emitted as model-visible prompt content or durable evidence.

### Requirement: Context adapter documentation

r[agent_harness_runtime.context_adapters.docs] Animus MUST document how embedders choose between host context, context-reference resolvers, context-layer descriptors, fresh-context task descriptors, and std-shell context services, and MUST state that absence of a resolver means absence of authority rather than implicit fallback.

#### Scenario: Embedder selects the smallest context surface

r[agent_harness_runtime.context_adapters.docs.smallest_surface]
- GIVEN an embedder only needs direct host-supplied project notes or only needs local file references
- WHEN they read Animus context adapter documentation
- THEN they can identify the minimal DTOs and resolver roles to implement
- AND docs do not imply default network fetching, repository crawling, image decoding, credentialed remote access, or ambient global context discovery.

### Requirement: Context adapter verification

r[agent_harness_runtime.context_adapters.verification] Context adapter family work MUST include focused positive and negative tests for family indexing, role-specific contracts, missing service, denied capability, unavailable service, malformed output, oversized output, unsafe references, safe evidence, std local resolvers, documentation, workspace checks, adapter/facade safety rails, Cairn validation, and Cairn gates before archive.

#### Scenario: Closeout evidence covers context adapters

r[agent_harness_runtime.context_adapters.verification.closeout]
- GIVEN maintainers intend to archive the context adapter family change
- WHEN closeout validation runs
- THEN evidence names focused context-reference, context-layering, std resolver, adapter-conformance, facade inventory, workspace, Cairn validation, and Cairn gate checks
- AND any skipped URL fetching, image decoding, crawler, credentialed remote handle, or external context-store implementation is recorded as a non-claim with a blocker and next-best validation.