# Agent Harness Runtime

## ADDED Requirements

### Requirement: Letta Trajectory remains a pinned optional boundary

r[agent_harness_runtime.letta_trajectory.boundary] Animus MUST adapt `@letta-ai/trajectory` transcript normalization through a non-default boundary pinned to an exact reviewed package, source revision, trajectory-v1 schema, license, supported-source set, and helper artifact identity, and reusable, default, no-std, and Wasm surfaces MUST NOT acquire Node, Bun, npm, TypeScript, Trajectory source, process, filesystem, SQLite, environment, network, credential, clock, or persistence dependencies.

#### Scenario: Exact transcript-only cohort is selected

r[agent_harness_runtime.letta_trajectory.boundary.exact_cohort]
- GIVEN a host selects the reviewed `@letta-ai/trajectory` `0.2.0` cohort at revision `a665556089d6a3c59f5806ee109b4d36f6c2ac83` with its exact trajectory-v1 schema and helper identity
- WHEN the optional adapter is composed
- THEN Animus MAY admit transcript-string normalization for `claude-code`, `codex`, `hermes`, `letta-code`, `openclaw`, `openhands`, and `pi`
- AND default or reusable builds MUST NOT require the helper or its JavaScript runtime.

#### Scenario: Excluded effectful API is requested

r[agent_harness_runtime.letta_trajectory.boundary.excluded_api]
- GIVEN a caller requests local trajectory listing, home-directory scanning, Deep Agents checkpoint or SQLite loading, Python-wrapper execution, canonical-v1 Cloud ingestion, or chunked-upload identity behavior
- WHEN adapter admission runs
- THEN Animus MUST return a typed unsupported-surface outcome
- AND it MUST NOT silently widen the transcript-only adapter's authority or compatibility claim.

### Requirement: Trajectory adapter policy is Nickel-authored and revalidated

r[agent_harness_runtime.letta_trajectory.profile] Human-authored Trajectory adapter policy MUST use typed Nickel contracts for exact cohort identity, supported sources, named transcript/output/record/tool/diagnostic/resource bounds, diagnostic dispositions, reasoning and tool-content inclusion, source metadata handling, projection targets, receipt fields, and required non-claims, and generated runtime data MUST be deterministically projected and revalidated by Rust before use.

#### Scenario: Reviewed policy admits a bounded request

r[agent_harness_runtime.letta_trajectory.profile.valid]
- GIVEN the checked profile names the exact helper cohort, an allowed transcript source, finite named bounds, known diagnostic dispositions, and explicit content/redaction policy
- WHEN Rust revalidates the generated profile and a request
- THEN it MUST produce a deterministic admission decision and BLAKE3 policy identity
- AND no host normalization effect may occur before that decision.

#### Scenario: Policy contains drift or unknown semantics

r[agent_harness_runtime.letta_trajectory.profile.drift]
- GIVEN generated policy is stale, names a different helper or schema, admits an unknown source or diagnostic, omits a required bound, or weakens a required non-claim
- WHEN profile validation runs
- THEN validation MUST fail closed with a typed issue
- AND neither helper invocation nor record projection may begin.

### Requirement: Transcript normalization crosses an injected host service

r[agent_harness_runtime.letta_trajectory.service] Raw transcript normalization MUST cross an implementation-free typed service using an already-loaded transcript string, explicit source and policy identity, bounded request and response envelopes, and an explicitly supplied normalizer or bounded executor; reusable logic MUST NOT discover executables, read transcript paths, inspect environment variables, scan agent stores, spawn processes, or perform fallback normalization.

#### Scenario: Injected service returns a bounded result

r[agent_harness_runtime.letta_trajectory.service.injected]
- GIVEN a request passed pure policy admission and the host supplied an exact helper identity plus a compatible injected service
- WHEN normalization is requested
- THEN the host adapter MAY send the bounded transcript through the service and return a bounded result envelope
- AND raw stdin, stdout, stderr, transcript content, and helper diagnostics MUST remain transient host data until pure validation selects an allowed projection.

#### Scenario: Service is absent or mismatched

r[agent_harness_runtime.letta_trajectory.service.unavailable]
- GIVEN no compatible service is injected or the supplied helper identity differs from the admitted cohort
- WHEN normalization is requested
- THEN Animus MUST return a typed unavailable or cohort-mismatch outcome before process or filesystem effects
- AND it MUST NOT search `PATH`, install a package, inspect a home directory, or substitute another normalizer.

### Requirement: Trajectory-v1 output is revalidated by a pure core

r[agent_harness_runtime.letta_trajectory.validation] Animus MUST revalidate normalization results with pure deterministic Rust logic against the exact result envelope and trajectory-v1 contract, including one leading meta record, supported source identity, allowed record roles, timestamp syntax, assistant content/tool-call exclusivity, non-empty and unique tool-call identities, JSON-object tool arguments, correlated tool results, configured bounds, and known diagnostic codes and dispositions.

#### Scenario: Valid normalized trajectory is accepted

r[agent_harness_runtime.letta_trajectory.validation.valid]
- GIVEN a bounded result contains one leading meta record, valid ordered user and assistant records, well-formed timestamps, unique correlated tool calls and results, JSON-object arguments, and only policy-admitted diagnostics
- WHEN pure validation runs
- THEN it MUST return typed validated records, bounded role and diagnostic counts, disposition, and redaction/projection inputs
- AND repeated validation of identical inputs and policy MUST return the same result.

#### Scenario: Malformed or ambiguous result fails closed

r[agent_harness_runtime.letta_trajectory.validation.invalid]
- GIVEN a result has an unknown field, role, source, or diagnostic; missing, misplaced, or duplicate meta; malformed timestamp; empty required content; assistant content/tool conflict; malformed or non-object arguments; duplicate call identity; orphan or duplicate result; impossible ordering; or a configured bound violation
- WHEN pure validation runs
- THEN validation MUST return the matching typed failure without echoing sensitive content
- AND no ledger, batch, persistence, dataset, ATIF, receipt, or model-visible projection may be emitted.

### Requirement: Validated records project through existing Animus owners

r[agent_harness_runtime.letta_trajectory.projection] Validated trajectory-v1 records MUST project deterministically into existing Animus session-ledger and batch-trajectory drafts under explicit content policy, caller-supplied source-generation identity, ordered record position, and domain-separated BLAKE3 projection identity, while existing replay, persistence, dataset-export, ATIF, model-streaming reasoning, tool, and receipt owners remain authoritative.

#### Scenario: Admitted records become an Animus projection

r[agent_harness_runtime.letta_trajectory.projection.valid]
- GIVEN validated records, an admitted source-generation identity, and policy that admits selected user, assistant, tool-call, and tool-result content
- WHEN pure projection runs
- THEN records MUST map in deterministic order to the matching Animus ledger and batch drafts with explicit omission and redaction state
- AND source timestamps MUST remain descriptive rather than becoming identity, freshness, causality, or authority evidence.

#### Scenario: Sensitive content is not admitted by default

r[agent_harness_runtime.letta_trajectory.projection.default_redaction]
- GIVEN validated records include reasoning, raw tool arguments or results, cwd, branch, provider/model details, diagnostic prose, or other sensitive metadata
- WHEN projection runs under the default profile
- THEN reasoning and unapproved content MUST be omitted or represented only by explicit safe redaction metadata
- AND the adapter MUST NOT automatically persist the records, emit dataset rows, run ATIF evaluation, or expose omitted reasoning as model-visible text.

### Requirement: Trajectory adapter evidence is metadata-only

r[agent_harness_runtime.letta_trajectory.evidence] Trajectory adapter receipts MUST contain only safe source class, cohort/profile/helper/service identities, status, bounded role and diagnostic-code counts, diagnostic disposition, omission/redaction/truncation classes, projection target, required non-claims, and a domain-separated BLAKE3 receipt identity, and MUST exclude transcript bodies, reasoning, tool bodies, diagnostic messages, credentials, provider payloads, raw cwd or branch, private paths, environment values, helper output, and unapproved content-derived fingerprints.

#### Scenario: Safe receipt records bounded adapter facts

r[agent_harness_runtime.letta_trajectory.evidence.safe]
- GIVEN normalization and projection completed under the exact reviewed cohort and policy
- WHEN receipt projection runs
- THEN the receipt MUST bind the cohort, policy, service, status, bounded counts, repair/loss/redaction state, projection class, and required non-claims
- AND it MUST describe only processing of supplied bytes rather than authenticity, completeness, semantic, execution, privacy, model, evaluation, or export truth.

#### Scenario: Receipt draft contains sensitive or overclaiming material

r[agent_harness_runtime.letta_trajectory.evidence.rejects_leaks]
- GIVEN a receipt draft includes raw transcript, reasoning, tool content, diagnostic prose, credential-looking text, provider payload, raw path metadata, unbounded helper output, an unapproved content hash, or wording that claims lossless semantic equivalence or execution truth
- WHEN receipt validation runs
- THEN validation MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, dataset, evaluation, or release evidence.

### Requirement: Trajectory adapter adoption has positive and negative evidence

r[agent_harness_runtime.letta_trajectory.verification] The adapter MUST include positive and negative tests for exact cohort/profile binding, every supported transcript source, service injection, helper protocol, trajectory-v1 schema and order, timestamps, tool argument and result linkage, diagnostic disposition, bounds, deterministic projection, reasoning/tool privacy, receipt safety, excluded APIs, and default dependency/source-shape isolation.

#### Scenario: Deterministic fixture evidence is ready for archive

r[agent_harness_runtime.letta_trajectory.verification.closeout]
- GIVEN maintainers intend to sync and archive the Trajectory adapter change
- WHEN closeout validation runs
- THEN focused adapter, runtime, host, Nickel, helper-parity, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with both positive and negative fixtures
- AND local private transcripts, store discovery, Deep Agents, canonical-v1, cloud ingestion, automatic dataset export, semantic equivalence, production readiness, and release eligibility MUST remain explicit non-claims rather than inferred evidence.
