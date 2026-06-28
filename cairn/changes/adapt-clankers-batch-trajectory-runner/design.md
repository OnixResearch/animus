# Design: Adapt Clankers batch trajectory runner for Animus

## Context

Clankers runs bounded local prompt batches and writes deterministic output plus a sidecar manifest for resume. Animus should adapt the underlying contract, not the product CLI, so later headless, evaluation, and dataset export surfaces can share one safe run model.

## Decisions

### 1. Batch planning is pure and local-only

**Choice:** Parse input job descriptors into a `BatchPlan` with deterministic run ID, ordered jobs, output target labels, format, concurrency limit, and resume-manifest expectation. Validation rejects remote input/output locations and malformed job IDs before execution.

**Rationale:** Batch runs are high-volume prompt workflows. Their safety boundary must be established before any model calls or file writes.

### 2. Execution is injected

**Choice:** Batch execution consumes an injected headless/session execution service that accepts typed job input and returns typed job outcomes. Reusable batch logic does not construct providers, launch daemons, or persist sessions directly.

**Rationale:** Animus should support both embedded and std-shell hosts without importing one product execution substrate.

### 3. Resume is evidence-based

**Choice:** `--resume`-style behavior must read a manifest, verify run/job identity, output format, redaction policy, and completed job evidence, then skip only matching completed jobs.

**Rationale:** Resume should prevent duplicate work without hiding missing or stale outputs.

### 4. Outputs are renderer adapters

**Choice:** JSONL, eval JSONL, ShareGPT-style, or future dataset formats are renderer adapters over normalized job outcomes. Renderers receive safe metadata and model-visible messages according to export policy.

**Rationale:** Dataset export should be layered on normalized trajectories, not on raw provider transcripts.

### 5. Metadata is redacted

**Choice:** Run and job receipts record counts, IDs, statuses, output labels, objective result summaries, model/session handles, and BLAKE3 hashes where needed. Raw prompts, provider requests/responses, credentials, and remote destinations are excluded.

**Rationale:** Batch artifacts are likely to be shared for eval or training. Metadata must not leak hidden prompt or credential material.

## Risks / Trade-offs

- Batch output formats can pull export behavior forward too early. Keep this change focused on normalized trajectories and renderer boundaries.
- Resume manifests can become trusted too much. Verify identity and status before skipping jobs.
- Concurrency can hide ordering bugs. Tests must prove stable output ordering independent of completion order.
