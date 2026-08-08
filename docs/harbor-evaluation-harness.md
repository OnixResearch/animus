# Harbor evaluation harness adaptation

Animus adapts a local fixture-backed evaluation slice from Harbor `v0.18.0` at commit `527d50deb63a5d279e8c20593c18a2cbc7f61f9e`, task schema `1.3`, and ATIF `v1.7`. Harbor remains an external Apache-2.0 reference; its Python application, Pydantic models, Hub, registry, Docker/cloud backends, scheduler, Reward Kit, and training stack are not runtime dependencies.

## Ownership

Typed Nickel owns the reviewed task, dataset, route, environment, verifier, budget, reward, network, and secret posture. Rust revalidates the generated projection. `animus_runtime::harbor_eval` keeps Harbor-specific ATIF projection and receipts. Its generic matrix and reward reducers delegate to `animus-eval-core`. `animus_adapters::harbor` owns implementation-free environment and verifier ports. `animus_std::harbor` calls explicitly injected services and requires a separate verifier identity and allowlisted artifact manifest.

Existing batch, evaluation-feedback, ResearchSpec, dataset-export, trust-boundary, sandbox, orchestration, model-profile, role, and completion owners remain authoritative; the Harbor composition does not replace them.

ATIF projection accepts only `ATIF-v1.7`, sequential one-based steps, correlated tool calls and observations, resolvable embedded-subagent identities, bounded collections, and an explicit terminal step. Raw messages, reasoning, tool arguments/results, provider/environment payloads, image paths, extensions, credentials, and hidden tests are omitted; only counts, identities, and redaction markers cross the safe boundary.

Rewards require finite attributed values, unique trial/metric pairs, deterministic input order, immutable verifier identity, and explicit advisory classification. Judge values cannot authorize effects or completion.

Run `scripts/cairn/check-harbor-evaluation --root .`.

## Non-claims

The local fixture slice does not establish arbitrary Harbor task compatibility, Hub/cloud/container parity, verifier soundness or independence beyond the declared fixture boundary, benchmark superiority, agent/model correctness, automatic completion, training correctness, production readiness, or release eligibility.
