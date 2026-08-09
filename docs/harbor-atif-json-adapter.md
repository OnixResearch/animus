# Harbor ATIF-v1.7 JSON adapter

## Supported cohort

`animus-adapter-atif` supports one exact optional cohort:

- Harbor release: `v0.18.0`
- Harbor revision: `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`
- ATIF schema: `ATIF-v1.7`
- License: Apache-2.0
- Source cohort identity: `b3:78d4a45481206483c89269eeb515854552bb4185b4db41a966047d8f124cfe4f`
- Codec identity: `animus.harbor-atif-json-codec.v1`

The reviewed cohort includes the RFC, trajectory models, validator, and golden trajectories. A new Harbor or ATIF version needs a new review, identity, profile, fixtures, and focused check.

## Crate and features

The focused crate is `#![no_std]` and uses `alloc`. It owns the complete selected DTO shape, pure validation, reference classes, import and export drafts, semantic identities, loss classes, and receipt drafts.

JSON support is optional:

```toml
animus-adapter-atif = { version = "0.1.0", features = ["json-codec"] }
```

The root facade uses `harbor-atif` for typed contracts and `harbor-atif-json` for JSON. These features are not default features. `animus-adapters::atif` is a compatibility re-export.

## Policy

`config/harbor-atif-profile.ncl` is the typed source of truth. Its generated file is `config/generated/harbor-atif-profile.json`. `decode_atif_policy_json` rejects unknown fields, stale cohort values, invalid bounds, unsafe dispositions, authority requests, invented export defaults, incomplete receipt fields, and missing non-claims before document decode.

The profile names outer bytes, graph size, recursion depth, local and aggregate steps, strings, content parts, tool definitions, calls, observations, subagent references, token arrays, metrics, extension nodes, extension depth, extension bytes, and loss-class limits.

## Decode and validation

`decode_atif_json` accepts only caller-supplied bytes. It does not read a path or URL. It rejects malformed UTF-8 or JSON, duplicate fields, unknown non-`extra` fields, oversized input, and invalid typed values. `encode_atif_json` emits deterministic compact UTF-8 JSON from a validated value.

`validate_trajectory` checks:

- exact `ATIF-v1.7`;
- one-based sequential step IDs;
- ISO 8601 timestamps when supplied;
- system, user, and agent field rules;
- text and image content-part conditions and media types;
- unique tool-call IDs and same-step observation correlation;
- finite and non-negative metrics plus token and log-probability lengths;
- `llm_call_count = 0` restrictions;
- copied-context classification;
- embedded trajectory IDs and references;
- external image, continuation, and subagent descriptor classes;
- bounded extension objects; and
- local, aggregate, depth, and graph limits.

Embedded references use `trajectory_id`. `session_id` is an informational run identity and can repeat. `validate_resolved_graph` checks a finite caller-supplied graph for missing nodes, cycles, and depth overflow. The adapter never resolves external descriptors.

ATIF has no terminal StepObject field. Trial outcome remains an explicit Harbor evaluation host observation. Validation and export do not infer or create one.

## Import and export

`import_validated_trajectory` creates drafts for the existing neutral ledger, batch order, subagent lineage, metrics, and dataset owners. It does not persist or promote those drafts. Each sensitive field has an include, omit, redact, hash, reject, or unresolved disposition.

Copied-context steps and deterministic no-LLM dispatch steps are not eligible dataset drafts. Reasoning, token arrays, log probabilities, paths, notes, model labels, and extensions follow independent policy decisions.

`export_trajectory_snapshot` accepts admitted Animus facts. It creates deterministic one-based steps and then runs the production validator. It does not invent timestamps, metrics, reasoning, tool outcomes, external resources, copied-context status, LLM-call counts, or terminal state.

Semantic identities use domain-separated BLAKE3 over the complete typed representation. Object keys have deterministic order. Projection identities also bind the checked policy and ordered loss classes. Projections report omission, redaction, hash-retention, and unresolved disposition classes without retaining private field content. The supported round trip preserves only the admitted typed semantic subset. It does not preserve source whitespace or claim arbitrary byte equality.

## Evidence

`build_atif_receipt` emits metadata only. Receipts can contain exact cohort, policy, codec, direction, safe identities, bounded counts, projection status, loss classes, disposition classes, bounded claims, and required non-claims.

Receipts do not contain trajectory bodies, messages, reasoning, tool arguments or results, token arrays, log probabilities, notes, extension bodies, credentials, provider payloads, paths, URLs, environment values, or unrestricted diagnostics.

## Authority boundary

The reusable crate has no filesystem, network, process, Python, Pydantic, Harbor runtime, environment, credential, clock, scheduler, provider, tool, or persistence interface. Hosts can acquire bytes or resolve descriptors only through separately authorized owner code.

The session ledger owns replay and persistence. Batch owns ordering. Subagent contracts own delegation lineage. Dataset export owns training admission. Harbor evaluation owns trial composition and outcome. Safe receipt and Cairn lifecycle checks own their evidence decisions.

## Verification

Run:

```sh
scripts/cairn/check-harbor-atif-adapter --root .
nix build .#checks.x86_64-linux.harbor-atif-adapter --no-link -L
```

The focused rail checks Nickel drift, source-cohort identity, positive and negative fixtures, production codec and projection tests, runtime migration, Clippy, no-default and Wasm builds, dependency isolation, catalogs, and evidence safety.

## Non-claims

This adapter does not establish future ATIF compatibility, arbitrary Harbor parity, byte-preserving input round trips, source authenticity, transcript completeness, timestamp truth, metric truth, external-resource safety, tool execution, hidden-reasoning safety, privacy, verifier soundness, artifact authenticity, benchmark validity, dataset or training fitness, authoritative completion, production readiness, or release eligibility.
