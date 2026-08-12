## Why

Remora already owns useful deterministic dependency-update and change-request behavior. Animus has generic review, schedule, orchestration, workspace, and receipt contracts, but it lacks the narrow maintenance contracts that connect these parts.

Copying the Remora application into `animus-adapters` would create another broad facade. Animus needs focused optional families that preserve Remora's domain meaning without importing registry, filesystem, VCS, forge, scheduler, process, credential, or persistence authority.

## What Changes

- Add a focused `animus-adapter-change-request` `#![no_std]` crate for forge-neutral repository, change-request, diff, review, CI-failure, publication, and metadata-only receipt contracts. r[agent_harness_runtime.remora_adapters.change_request]
- Add a focused `animus-adapter-maintenance` `#![no_std]` crate for dependency inventory, update candidates, advisory observations, deterministic grouping, update effect plans, validation-profile requests, and metadata-only receipts. r[agent_harness_runtime.remora_adapters.maintenance]
- Split inventory, candidate discovery, advisory lookup, workspace mutation, validation, change-request source, review analysis, and publication into separate injected ports. r[agent_harness_runtime.remora_adapters.ports]
- Bind requests and observations to exact repository, policy, plan, generation, implementation, and correlation identities before reduction. r[agent_harness_runtime.remora_adapters.binding]
- Compose the new families with existing Animus schedule, queue, orchestration, review, model, quality, authority, workspace, and receipt families without duplicating their owners. r[agent_harness_runtime.remora_adapters.composition]
- Record the reviewed Remora source repository and immutable revision, plus the owner-authorized adaptation basis and bounded imported scope. r[agent_harness_runtime.remora_adapters.provenance]
- Add positive and negative tests, no-std and Wasm checks, compatibility re-exports, component metadata, focused documentation, and a focused verification rail. r[agent_harness_runtime.remora_adapters.verification]

## Impact

- **Crates**: new optional `animus-adapter-change-request` and `animus-adapter-maintenance` crates.
- **Facades**: non-default compatibility re-exports through `animus-adapters` and root `animus`.
- **Dependencies**: authority-free local adapter crates and BLAKE3 only.
- **Host boundary**: concrete registry, advisory database, manifest parsing, lockfile mutation, VCS, forge, scheduler, network, credential, process, and persistence behavior remains host-owned.
- **Testing**: focused crate tests, facade checks, no-std and Wasm builds, dependency isolation, static source validation, Cairn gates, and Nix checks.

## Out of Scope

- A concrete package registry, vulnerability database, SBOM service, manifest scanner, lockfile writer, Git or Jujutsu client, forge client, cron service, queue backend, dashboard store, or notification service.
- Dependabot parity, Remora CLI compatibility, automatic merge, signed commits, private registry access, release-note fetching, compatibility scoring, or production operation.
- Moving Remora's extraction radar, Tiger Style runner, or external-analysis storage into these crates. Those observations compose through existing Animus codegraph, quality, model-evidence, document, orchestration, and receipt families.

## Affected Specs

- `agent-harness-runtime`: Remora-derived change-request and maintenance adapter families, ports, binding, composition, provenance, and verification.
