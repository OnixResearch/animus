## Context

Harbor's Agent Trajectory Interchange Format is an active JSON specification. The reviewed Animus cohort is Harbor `v0.18.0` at revision `527d50deb63a5d279e8c20593c18a2cbc7f61f9e` with ATIF-v1.7. The official documentation and RFC define root and agent metadata, one-based steps, system/user/agent source roles, text or image content parts, tool calls and observations, per-step and final metrics, optional copied-context and LLM-call-count semantics, bounded custom `extra` objects, continuation descriptors, and recursively embedded or externally referenced subagent trajectories.

Animus currently owns only a metadata-level `AtifTrajectory` projection inside `animus-runtime::harbor_eval`. That projection is useful for evaluation receipts, but it neither parses nor emits ATIF JSON and does not represent the full v1.7 contract. It also carries an internal terminal marker that is not an ATIF StepObject field; terminal trial outcome remains a separate Animus evaluation fact and must not be invented as wire-format validation.

A bounded architecture search used two authoritative upstream sources, repository ownership and dependency inspection, three mechanism families, and one adversarial pass. Completion requires a Cairn package that distinguishes the existing metadata projector from a real JSON adapter, names exact schema and authority boundaries, covers import and export, includes positive and negative evidence, and passes Cairn validation plus proposal/design/tasks gates. A duplicate metadata facade, runtime Pydantic call, unbounded recursive parser, parser-only lane, or receipt containing trajectory bodies is false completion.

## Decisions

### Decision: add a focused ATIF adapter-family crate

**Choice:** Add `animus-adapter-atif` as a focused `#![no_std]` plus `alloc` family crate. It owns ATIF-v1.7 DTOs, policy inputs, pure structural and semantic validation, graph/reference decisions, deterministic semantic identity inputs, loss reports, and safe receipt drafts. `animus-adapters` re-exports the compatibility surface under Harbor/ATIF names. The existing `animus-runtime::harbor_eval` module consumes validated projections and stops acting as the schema owner.

**Rationale:** ATIF interchange is independently useful outside Harbor evaluation jobs. A focused family prevents task planning, reward reduction, JSON codec concerns, and trajectory schema ownership from accumulating in one module while preserving the smallest-dependency rule.

### Decision: pin one exact Harbor and ATIF cohort

**Choice:** The initial adapter accepts only Harbor `v0.18.0`, revision `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, and `schema_version = "ATIF-v1.7"`. The profile binds the docs/RFC cohort, schema version, fixture provenance, codec version, and validation policy. Other versions return a typed unsupported-version result.

**Rationale:** ATIF is evolving and v1.7 changes subagent identity and run/session semantics from v1.6. Exact pinning makes drift and upgrade work explicit.

### Decision: keep JSON coding pure and byte-oriented

**Choice:** A non-default codec accepts a bounded byte slice and returns a typed value or bounded diagnostics; export accepts a validated typed value and returns deterministic UTF-8 JSON bytes. Serialization dependencies are feature-gated and authority-free. The codec performs no path reads, writes, URL fetches, object-store access, environment lookup, process launch, Python invocation, Harbor discovery, or clock access.

The shell enforces the outer byte ceiling before decode. Pure code enforces named nested limits for trajectories, recursion depth, steps, strings, content parts, tool definitions, calls, observations, token arrays, log probabilities, metrics, and extension depth/node/byte counts. Numeric values must be finite and satisfy the selected non-negative and cross-field constraints.

**Rationale:** JSON parsing and serialization are deterministic transformations over supplied bytes. File and network acquisition are separate authorities and must not be smuggled into a format adapter.

### Decision: validate the actual v1.7 contract

**Choice:** Validation covers the complete selected shape and rejects unknown non-`extra` fields. It validates one-based sequential step IDs; ISO 8601 timestamps when present; source-specific agent-only fields; conditional text/image content fields and admitted media types; non-empty unique tool-call IDs and same-step observation correlation; finite bounded metrics and token/logprob length relationships; `llm_call_count = 0` restrictions; and `is_copied_context` training exclusion semantics.

Embedded subagents require unique per-parent `trajectory_id` values and recursively valid v1.7 documents. Embedded references resolve by `trajectory_id`; `session_id` is informational and never a resolution key. A reference requires `trajectory_id`, `trajectory_path`, or both. External subagent, continuation, and image references are represented as untrusted descriptors but are never dereferenced by reusable code. Recursion, aggregate graph size, duplicate identities, ambiguous references, and cycles in caller-supplied resolved graphs fail closed.

ATIF has no terminal StepObject field. Terminal trial status remains an explicitly supplied evaluation observation outside the JSON document and cannot be inferred from the last ATIF step.

**Rationale:** Field-level interoperability depends on v1.7's exact identity, copied-context, multimodal, and no-LLM rules rather than the smaller existing metadata projection.

### Decision: make extension handling explicit and conservative

**Choice:** `extra` objects are accepted only within named structural limits and are classified by location. They are omitted from safe projection by default. A reviewed policy may allow selected scalar keys or retain a domain-separated BLAKE3 identity for an admitted extension object, but raw extension bodies never enter safe receipts. Unknown authority-bearing or content-bearing keys fail or remain explicitly unresolved according to policy; they are never silently trusted.

**Rationale:** ATIF extensibility is useful for interchange but is also the main route for provider payloads, credentials, paths, hidden reasoning, and unbounded data to cross boundaries.

### Decision: support bidirectional projection with visible loss

**Choice:** Import maps validated ATIF documents into existing Animus ledger, batch, subagent-lineage, metrics, and dataset draft types under explicit content policy. Reasoning, token IDs, logprobs, tool bodies, image references, notes, paths, arbitrary extensions, copied context, and provider/model details are independently admitted, redacted, omitted, hashed, or marked unresolved. `is_copied_context = true` records are ineligible for supervised dataset projection.

Export consumes an already-admitted Animus trajectory snapshot. It emits deterministic step order and IDs, does not fabricate timestamps, costs, token counts, reasoning, tool results, or completion state, and reports every field that cannot be represented or was omitted. Exported bytes are deterministic for the same typed input and policy, while semantic identity is computed over domain-separated typed fields rather than incidental input whitespace or object-key order.

A semantic round-trip claim is limited to the explicitly supported safe subset. Byte-for-byte input preservation and arbitrary `extra` preservation are non-claims.

**Rationale:** A parser alone does not provide an adapter. Bidirectional projection is useful only when loss and privacy decisions remain observable.

### Decision: preserve existing owners and evidence boundaries

**Choice:** The neutral session ledger owns replay and persistence semantics; batch owns ordering; subagent contracts own delegation lineage; dataset export owns SFT/RL admission; the Harbor evaluation lane owns trial composition; and safe receipts own evidence hygiene. The ATIF adapter supplies validated drafts and metadata only.

Receipts bind exact cohort, profile, codec, direction, source class, semantic input/output identities, trajectory/step/content/tool/subagent/extension counts, omission/redaction/unresolved/loss classes, status, and required non-claims with BLAKE3. They exclude trajectory bodies, messages, reasoning, tool arguments/results, token arrays, logprobs, notes, extension bodies, credentials, provider payloads, raw paths/URLs, environment values, and unapproved content-derived fingerprints.

**Rationale:** Format conformance and deterministic conversion are bounded facts, not evidence that the underlying interactions happened or are safe for downstream use.

## Alternatives Considered

### Extend the current metadata projector in place

Rejected as the sole design because the module also owns evaluation planning, rewards, and receipts and cannot represent complete JSON import/export without becoming another broad compatibility facade. It remains a downstream consumer during migration.

### Call Harbor's Pydantic validator and converter at runtime

Rejected because it introduces Python/Harbor process, package, environment, and potentially filesystem authority. The exact upstream validator remains fixture-parity evidence only.

### Accept all ATIF versions through permissive serde models

Rejected because v1.7 changed subagent resolution semantics and permissive unknown-field handling would hide schema drift. New versions require a reviewed cohort update and fixtures.

### Preserve arbitrary JSON losslessly

Rejected for the safe projection lane because raw `extra`, reasoning, tool, image, path, and provider data can contain sensitive or authority-bearing material. Separately authorized private artifact retention remains host-owned.

## Risks / Trade-offs

- A full v1.7 model is larger than the current metadata facts and increases maintenance when the RFC changes; exact cohort tests and explicit upgrades contain the risk.
- Recursive embedded trajectories and extension objects require aggregate limits, not only per-vector limits; the profile and validator must enforce both.
- Deterministic JSON output does not make semantically equivalent third-party JSON byte-identical; identities therefore bind typed semantics and selected policy rather than source formatting.
- Conservative external-reference and extension handling reduces out-of-the-box fidelity but prevents ambient file/network authority and silent privacy widening.
- Official golden fixtures may contain unsafe or unstable data; repository fixtures must be minimized or synthetic, retain Apache-2.0 provenance, and remain pinned to the reviewed revision.

## Non-Claims

This design does not establish arbitrary future ATIF compatibility, full Harbor runtime parity, byte-preserving round trips, source transcript authenticity or completeness, timestamp or metric truth, image availability, external-reference safety, tool execution, hidden-reasoning safety, privacy, semantic equivalence outside the admitted subset, dataset or training fitness, model correctness, evaluation validity, production readiness, or release eligibility.
