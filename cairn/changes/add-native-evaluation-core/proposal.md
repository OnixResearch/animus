## Why

Animus already has Harbor-shaped trial planning, batch execution contracts, evaluation feedback, safe rewards, role separation, and trajectory exports. Those mechanisms remain distributed across `animus-runtime::harbor_eval`, batch, ledger, dataset, and review modules. The Harbor adaptation also gives a product-specific name to task, episode, recipe, reward, and resume behavior that is independently useful to every Animus embedder.

Verifiers v1 at reviewed commit `1a997814e85a2e4f409add960aa852737388dbd8` demonstrates a compact separation between immutable task data, task behavior, lazy tasksets, environment control flow, per-agent traces, and multi-agent episodes. Importing its Python classes, decorators, dynamic discovery, or training runtime would conflict with Animus crate, authority, identity, and configuration boundaries. Animus needs a native pure evaluation contract that retains existing owners and lets Harbor become one optional consumer.

## What Changes

- Add a focused non-default `animus-eval-core` `#![no_std]` crate for immutable task descriptors, bounded taskset streams, environment plans, episode identities, recipe planning, reward reduction, resume decisions, and metadata-only receipts. r[agent_harness_runtime.native_evaluation_core.boundary]
- Define typed Nickel evaluation profiles and generated projections for taskset, recipe, reward, resume, redaction, resource, and evidence policy, with strict Rust revalidation and domain-separated BLAKE3 identities. r[agent_harness_runtime.native_evaluation_core.policy]
- Add exact task and taskset contracts with explicit generator, seed, version, finiteness, ordering, and bound semantics. r[agent_harness_runtime.native_evaluation_core.tasks]
- Add a global episode model that references existing canonical Animus ledger branches for local traces instead of creating a second transcript authority. r[agent_harness_runtime.native_evaluation_core.episodes]
- Add deterministic built-in recipe planners for single-agent, best-of-N/pass-at-N, user simulation, and isolated agentic judging. r[agent_harness_runtime.native_evaluation_core.recipes]
- Add finite attributed reward reduction and content-addressed resume decisions that bind every task, model, harness, environment, verifier, rubric, policy, and redaction identity. r[agent_harness_runtime.native_evaluation_core.rewards] r[agent_harness_runtime.native_evaluation_core.resume]
- Add metadata-only receipts, positive and negative fixtures, no-std and Wasm checks, a focused rail, and compatibility migration from `animus-runtime::harbor_eval`. r[agent_harness_runtime.native_evaluation_core.evidence] r[agent_harness_runtime.native_evaluation_core.verification]

## Impact

- **Crates**: new `animus-eval-core`; non-default compatibility re-exports through `animus-runtime` and root `animus`.
- **Existing owners**: the session ledger owns trace history; batch owns execution ordering; agent roles own separation; dataset export owns training admission; Harbor and ATIF remain optional interoperability surfaces.
- **Configuration**: one typed Nickel evaluation profile and checked generated Rust fixture data.
- **Testing**: pure positive and negative task, taskset, episode, recipe, reward, resume, identity, receipt, no-std, Wasm, and dependency-isolation checks.
- **Reference**: PrimeIntellect-ai/verifiers remains an MIT design reference at one immutable reviewed commit, not a runtime dependency.

## Out of Scope

- Provider calls, tool calls, filesystem access, processes, containers, networking, clocks, credentials, schedulers, persistence, or live model evaluation.
- Python decorators, package discovery, Prime CLI, Environments Hub, prime-rl, GEPA, Docker, Modal, or Prime runtime integration.
- A second transcript store, token-level RL tensors, automatic dataset promotion, verifier-soundness claims, benchmark claims, or authoritative completion.

## Affected Specs

- `agent-harness-runtime`: native evaluation boundary, policy, tasks, episodes, recipes, rewards, resume, evidence, and verification.
