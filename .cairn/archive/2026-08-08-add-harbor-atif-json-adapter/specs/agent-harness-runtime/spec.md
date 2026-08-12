# Agent Harness Runtime

## ADDED Requirements

### Requirement: Harbor ATIF JSON remains an exact optional boundary

r[agent_harness_runtime.harbor_atif.boundary] Animus MUST provide ATIF JSON interchange through a focused non-default adapter family pinned to Harbor `v0.18.0`, revision `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, and `ATIF-v1.7`, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire Harbor, Python, Pydantic, filesystem, process, network, environment, credential, clock, scheduler, or persistence authority.

#### Scenario: Exact ATIF adapter is selected

r[agent_harness_runtime.harbor_atif.boundary.selected]
- GIVEN an embedder selects the reviewed ATIF-v1.7 profile and supplies already-loaded input bytes or an admitted Animus trajectory snapshot
- WHEN the adapter is composed
- THEN Animus MAY decode, validate, import, or export the exact reviewed ATIF cohort
- AND the existing ledger, batch, subagent, dataset, Harbor evaluation, redaction, receipt, and lifecycle owners MUST remain authoritative.

#### Scenario: Ambient Harbor or reference access is requested

r[agent_harness_runtime.harbor_atif.boundary.no_ambient]
- GIVEN a caller supplies a file path, image path, continuation path, subagent path, URL, object-store locator, Python command, Harbor installation, or credential and expects reusable code to resolve it
- WHEN adapter admission runs
- THEN Animus MUST return a typed unsupported-authority or unresolved-reference outcome before the effect
- AND it MUST NOT read, write, fetch, discover, spawn, authenticate, schedule, persist, or substitute an ambient implementation.

### Requirement: ATIF interoperability policy is Nickel-authored and revalidated

r[agent_harness_runtime.harbor_atif.profile] Human-authored ATIF interoperability policy MUST use typed Nickel contracts for exact cohort and codec identities, named byte, graph, depth, collection, string, content, tool, metrics, and extension bounds, import and export inclusion rules, redaction, unresolved-reference treatment, receipt fields, and required non-claims, and generated runtime policy MUST be deterministically exported and revalidated by Rust before use.

#### Scenario: Reviewed bounded policy is admitted

r[agent_harness_runtime.harbor_atif.profile.valid]
- GIVEN a checked profile names the exact v1.7 cohort, finite local and aggregate bounds, explicit content and extension dispositions, external-reference treatment, export defaults, and required non-claims
- WHEN Rust revalidates the generated profile
- THEN it MUST return a deterministic policy decision and domain-separated BLAKE3 policy identity
- AND no decode, projection, external effect, or receipt emission may occur before admission.

#### Scenario: Policy drifts or leaves an unsafe default implicit

r[agent_harness_runtime.harbor_atif.profile.invalid]
- GIVEN generated policy is stale, names another cohort, omits an aggregate bound, admits an unknown extension class, permits ambient reference resolution, or weakens privacy or claim boundaries
- WHEN profile validation runs
- THEN validation MUST fail closed with a bounded typed issue
- AND no ATIF document or Animus snapshot may be decoded, imported, exported, persisted, or promoted.

### Requirement: ATIF JSON coding is bounded and deterministic

r[agent_harness_runtime.harbor_atif.codec] The ATIF codec MUST consume only caller-supplied bounded UTF-8 JSON bytes, reject malformed JSON, duplicate fields, unknown non-extension fields, unsupported schema versions, and configured byte or structural limits, and serialize validated ATIF values into deterministic UTF-8 JSON without filesystem, network, process, environment, credential, clock, or persistence effects.

#### Scenario: Valid ATIF JSON decodes to typed data

r[agent_harness_runtime.harbor_atif.codec.decode]
- GIVEN caller-supplied bytes fit the admitted outer bound and contain one structurally valid ATIF-v1.7 document
- WHEN pure decoding runs
- THEN it MUST return a typed document plus bounded structural counts for semantic validation
- AND repeated decoding of identical bytes and policy MUST return the same value or the same ordered diagnostics.

#### Scenario: Invalid or oversized JSON is supplied

r[agent_harness_runtime.harbor_atif.codec.rejects]
- GIVEN input is malformed, non-UTF-8, oversized, duplicate-keyed, contains an unknown non-`extra` field, or exceeds a nested structural bound
- WHEN decoding runs
- THEN it MUST fail with bounded location and issue classes without echoing sensitive values
- AND it MUST NOT return a partial document, resolve references, or emit a projection or receipt identity.

### Requirement: ATIF-v1.7 semantic validation is pure and complete for the selected cohort

r[agent_harness_runtime.harbor_atif.validation] Animus MUST validate the selected ATIF-v1.7 root, agent, step, content, tool, observation, metrics, continuation, extension, and subagent shapes with pure deterministic logic, including one-based sequential step IDs, source-specific fields, timestamp syntax, multimodal conditional fields, tool-call correlation, finite metric relationships, copied-context and no-LLM semantics, embedded trajectory identity and reference rules, and named local and aggregate bounds.

#### Scenario: Complete embedded trajectory validates

r[agent_harness_runtime.harbor_atif.validation.valid]
- GIVEN a v1.7 document has valid agent metadata, sequential system/user/agent steps, admitted text or image parts, unique correlated tool calls, finite bounded metrics, valid copied-context and `llm_call_count` semantics, unique embedded `trajectory_id` values, and resolvable embedded references
- WHEN pure validation runs
- THEN it MUST return a typed validated graph, safe counts, reference dispositions, content classifications, and projection inputs
- AND repeated validation of identical typed input and policy MUST return the same result.

#### Scenario: Semantic or graph invariant is violated

r[agent_harness_runtime.harbor_atif.validation.invalid]
- GIVEN a document has invalid ordering, timestamp, source field, content variant, media type, metric, no-LLM combination, copied-context treatment, tool correlation, trajectory identity, session-ID resolution attempt, ambiguous or missing reference, recursive cycle, or local or aggregate bound violation
- WHEN validation runs
- THEN it MUST return the matching typed failure without echoing sensitive content
- AND no partial ledger, batch, subagent, dataset, evaluation, persistence, or receipt projection may be emitted.

#### Scenario: ATIF reference names external material

r[agent_harness_runtime.harbor_atif.validation.external_reference]
- GIVEN a valid document contains an image, continuation, or subagent descriptor with a path, URL, or external locator
- WHEN reusable validation runs
- THEN it MUST classify the descriptor as unresolved or policy-denied without dereferencing it
- AND `session_id` MUST remain informational rather than becoming an embedded-subagent resolution key.

#### Scenario: Terminal outcome is evaluated

r[agent_harness_runtime.harbor_atif.validation.no_wire_terminal]
- GIVEN a validated ATIF document ends with any schema-valid step
- WHEN Harbor evaluation composition needs a terminal trial outcome
- THEN the outcome MUST come from an explicit Animus host observation outside the ATIF StepObject
- AND the adapter MUST NOT require or fabricate a non-ATIF terminal field in the JSON document.

### Requirement: ATIF import and export preserve Animus ownership and expose loss

r[agent_harness_runtime.harbor_atif.projection] Validated ATIF documents MUST import deterministically into existing Animus ledger, batch, subagent-lineage, metrics, and dataset drafts, and admitted Animus trajectory snapshots MUST export deterministically to ATIF-v1.7, under explicit policy and field-level omission, redaction, unresolved-reference, and semantic-loss reporting without fabricating unavailable facts.

#### Scenario: Validated ATIF imports under safe policy

r[agent_harness_runtime.harbor_atif.projection.import]
- GIVEN a validated document and policy admit selected message, tool, metrics, multimodal, and lineage facts
- WHEN pure import runs
- THEN it MUST produce ordered Animus drafts, domain-separated BLAKE3 semantic and projection identities, and an explicit loss report
- AND copied-context steps MUST be ineligible for supervised dataset rows while reasoning, token arrays, logprobs, paths, notes, and extensions remain omitted unless independently admitted.

#### Scenario: Animus snapshot exports without invented data

r[agent_harness_runtime.harbor_atif.projection.export]
- GIVEN an admitted Animus trajectory snapshot contains enough facts for the selected ATIF subset
- WHEN pure export runs
- THEN it MUST emit deterministic one-based ATIF steps and deterministic JSON bytes plus explicit omission and loss classes
- AND it MUST NOT invent timestamps, costs, token counts, reasoning, tool outcomes, external resources, copied-context status, LLM-call counts, or terminal trial state.

#### Scenario: Round trip exceeds the supported semantic subset

r[agent_harness_runtime.harbor_atif.projection.lossy]
- GIVEN input contains arbitrary extensions, provider material, external resources, unsupported Animus events, or policy-denied content
- WHEN import followed by export or export followed by import runs
- THEN the result MUST expose the exact semantic loss, redaction, omission, and unresolved-reference classes
- AND it MUST NOT be described as byte-preserving, lossless, complete, private, or semantically equivalent outside the admitted subset.

### Requirement: ATIF adapter evidence is metadata-only

r[agent_harness_runtime.harbor_atif.evidence] ATIF adapter receipts MUST bind only exact cohort, profile, codec, direction, source class, status, semantic input and output identities, bounded trajectory, step, content, tool, subagent, metrics, and extension counts, omission, redaction, unresolved-reference and loss classes, projection target, required non-claims, and a domain-separated BLAKE3 receipt identity.

#### Scenario: Safe adapter receipt is projected

r[agent_harness_runtime.harbor_atif.evidence.safe]
- GIVEN decode, validation, and the selected import or export complete under the admitted cohort and policy
- WHEN receipt projection runs
- THEN the receipt MUST bind safe identities, bounded counts, dispositions, status, projection class, and required non-claims
- AND it MUST describe only processing and projection of supplied values rather than source authenticity, completeness, execution, privacy, training fitness, evaluation validity, or correctness.

#### Scenario: Receipt contains content or overclaiming material

r[agent_harness_runtime.harbor_atif.evidence.rejects]
- GIVEN a receipt draft includes trajectory bodies, messages, reasoning, tool arguments or results, token arrays, logprobs, notes, extension bodies, credentials, provider payloads, raw paths or URLs, environment values, unapproved content hashes, or a lossless or authoritative claim
- WHEN receipt validation runs
- THEN it MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to persistence, dataset, evaluation, lifecycle, completion, or release evidence.

### Requirement: ATIF adapter adoption has positive and negative conformance evidence

r[agent_harness_runtime.harbor_atif.verification] The ATIF adapter MUST include positive and negative tests through production codec, semantic validator, import, export, graph/reference, loss-report, and receipt entrypoints; exact-cohort fixture parity; default and no-std dependency isolation; focused checks; and deterministic semantic round-trip coverage for the explicitly supported subset.

#### Scenario: Exact fixture cohort passes

r[agent_harness_runtime.harbor_atif.verification.positive]
- GIVEN minimized fixtures cover text, multimodal content, tools and observations, metrics, copied context, no-LLM dispatch, embedded subagents, external descriptors, bounded extensions, import, export, and safe semantic round trips
- WHEN the focused conformance rail runs
- THEN production entrypoints and the exact reviewed Harbor validator cohort MUST agree on admitted schema facts for the fixtures
- AND fixture provenance, expected loss classes, and any validator limitation MUST remain explicit.

#### Scenario: Adversarial fixture fails before widening

r[agent_harness_runtime.harbor_atif.verification.negative]
- GIVEN a fixture attempts malformed or oversized JSON, schema drift, invalid semantics, graph/reference abuse, ambient resolution, privacy leakage, automatic downstream promotion, or an unsupported interoperability claim
- WHEN the focused conformance rail runs
- THEN the fixture MUST fail with the expected bounded typed diagnostic before the prohibited effect or claim
- AND default, reusable, no-std, and Wasm surfaces MUST remain free of Harbor/Python and host authority dependencies.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.harbor_atif.verification.closeout]
- GIVEN maintainers intend to sync and archive the ATIF adapter change
- WHEN closeout validation runs
- THEN focused adapter, runtime, std codec, Nickel, exact-cohort parity, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with current positive and negative fixtures
- AND live Harbor execution, private trajectories, external reference resolution, arbitrary-version support, training suitability, production readiness, and release eligibility MUST remain separate non-claims.
