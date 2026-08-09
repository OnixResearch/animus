# Harbor evaluation harness adaptation

Animus adapts a local fixture-backed evaluation slice from Harbor `v0.18.0` at commit `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, task schema `1.3`, and ATIF `v1.7`. Harbor remains an external Apache-2.0 reference; its Python application, Pydantic models, Hub, registry, Docker/cloud backends, scheduler, Reward Kit, and training stack are not runtime dependencies.

## Ownership

Typed Nickel owns the reviewed task, dataset, route, environment, verifier, budget, reward, network, and secret posture. Rust revalidates the generated projection. `animus-adapter-atif` owns complete ATIF-v1.7 DTOs, caller-byte JSON, validation, import/export drafts, and safe receipts. `animus_runtime::harbor_eval` consumes validated ATIF metadata for evaluation receipts. Its generic matrix and reward reducers delegate to `animus-eval-core`. `animus_adapters::harbor` owns implementation-free environment and verifier ports. Under `evaluation`, it converts exact Harbor calls into `animus-adapter-eval` agent or verifier requests and rejects identity loss. `animus_std::harbor` calls explicitly injected services and requires a separate verifier identity and allowlisted artifact manifest. See [Harbor ATIF JSON adapter](harbor-atif-json-adapter.md).

Existing batch, evaluation-feedback, ResearchSpec, dataset-export, trust-boundary, sandbox, orchestration, model-profile, role, and completion owners remain authoritative; the Harbor composition does not replace them.

The focused ATIF adapter accepts only `ATIF-v1.7`, sequential one-based steps, correlated tool calls and observations, embedded references by `trajectory_id`, bounded collections, and explicit external descriptor classes. It does not require or invent a terminal wire field. Terminal trial outcome remains a separate explicit evaluation observation. Raw messages, reasoning, tool arguments/results, provider/environment payloads, image paths, extensions, credentials, and hidden tests are excluded from safe receipts.

Rewards require finite attributed values, unique trial/metric pairs, deterministic input order, immutable verifier identity, and explicit advisory classification. Judge values cannot authorize effects or completion.

Run `scripts/cairn/check-harbor-evaluation --root .`.

## Non-claims

The local fixture slice does not establish arbitrary Harbor task compatibility, Hub/cloud/container parity, verifier soundness or independence beyond the declared fixture boundary, benchmark superiority, agent/model correctness, automatic completion, training correctness, production readiness, or release eligibility.
