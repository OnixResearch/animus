## Why

Animus already pins Harbor `v0.18.0` and ATIF `v1.7`, but its current Harbor evaluation lane accepts pre-normalized metadata facts rather than ATIF JSON. It does not expose a complete parser or exporter for the trajectory format documented at <https://www.harborframework.com/docs/agents/trajectory-format>, and its metadata projection cannot independently establish field-level interoperability for multimodal content, copied context, `llm_call_count`, metrics, extensions, embedded subagents, or continuation and external-reference descriptors.

Treating the existing metadata projector as a complete format adapter would hide malformed or lossy conversions. Calling Harbor's Python/Pydantic validator at runtime would instead add an effectful external stack to a reusable Rust boundary. Animus needs a dedicated, exact-version JSON adapter with pure typed validation, explicit loss reporting, policy-controlled projection, and no ambient file, URL, Python, Harbor, or process authority.

## What Changes

- Add a focused `animus-adapter-atif` family crate for implementation-free ATIF-v1.7 DTOs, named bounds, validation results, projection policy, loss classes, and safe receipt inputs, with compatibility re-exports through the existing Harbor adapter surface. r[agent_harness_runtime.harbor_atif.boundary]
- Add a typed Nickel interoperability profile binding Harbor `v0.18.0`, revision `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, ATIF-v1.7, finite decode/graph/content/metrics/extension bounds, inclusion defaults, and required non-claims. r[agent_harness_runtime.harbor_atif.profile]
- Add a bounded optional JSON codec that consumes caller-supplied bytes and produces typed ATIF values or deterministic diagnostics without reading paths, resolving URLs, invoking Python, or discovering Harbor. r[agent_harness_runtime.harbor_atif.codec]
- Add pure ATIF-v1.7 validation for exact fields, source-specific constraints, ordered step IDs, multimodal content, tool correlation, metrics, copied-context and no-LLM semantics, embedded-subagent identity and reference rules, and bounded extensions. r[agent_harness_runtime.harbor_atif.validation]
- Add deterministic import into existing Animus ledger/batch/dataset drafts and deterministic export from admitted Animus trajectory snapshots, with explicit omission, redaction, unresolved-reference, and semantic-loss reports. r[agent_harness_runtime.harbor_atif.projection]
- Add metadata-only BLAKE3 receipts and positive/negative official-cohort fixture parity without treating Harbor validation, a successful parse, or round-trip similarity as authenticity, completeness, privacy, training fitness, or execution evidence. r[agent_harness_runtime.harbor_atif.evidence] r[agent_harness_runtime.harbor_atif.verification]

## Impact

- **Reusable boundary**: one focused no-std adapter-family crate plus compatibility re-exports; no Harbor/Python/Pydantic/filesystem/process/network authority enters reusable or default surfaces.
- **Runtime composition**: the existing Harbor evaluation projector becomes a consumer of validated ATIF facts rather than the format parser or schema owner; ledger, batch, dataset, redaction, and receipt owners remain authoritative.
- **Host shell**: an opt-in codec handles only caller-supplied and caller-received bytes; filesystem reads/writes and external-reference resolution remain caller-owned services outside the adapter.
- **Configuration**: Nickel remains the human-authored source of truth and generated runtime policy is revalidated by Rust.
- **Testing**: official-cohort positive fixtures, adversarial negative fixtures, import/export semantic round trips, dependency isolation, no-std/Wasm checks, focused Rust/Nickel/Cairn rails, and a dedicated Nix check.

## Out of Scope

- Running Harbor, its Pydantic validator, agents, containers, Hub, registry, scheduler, Reward Kit, or training workflows at runtime.
- Ambient loading of trajectory files, continuation files, subagent files, image paths, URLs, object-store references, credentials, or provider metadata.
- Supporting ATIF versions other than the exact reviewed v1.7 cohort, preserving arbitrary `extra` values by default, or promising byte-for-byte preservation of input JSON.
- Proving source authenticity, complete event capture, timestamp truth, tool execution, hidden-reasoning safety, semantic equivalence, dataset quality, model correctness, evaluation validity, production readiness, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: exact ATIF boundary, policy, codec, validation, bidirectional projection, evidence, and verification.
