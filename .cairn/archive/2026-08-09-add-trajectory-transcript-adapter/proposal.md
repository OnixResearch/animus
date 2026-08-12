## Why

Animus already owns neutral session-ledger replay, batch trajectory ordering, dataset export, and Harbor ATIF projection, but it has no reviewed boundary for normalizing native transcripts from other agent runtimes. `@letta-ai/trajectory` provides a focused Apache-2.0 normalizer for Claude Code, Codex, Hermes, Letta Code, OpenClaw, OpenHands, and Pi transcripts, with one versioned trajectory record shape and explicit cleanup diagnostics.

Importing that TypeScript package into reusable Rust crates, scanning local agent stores, or accepting its output without revalidation would add ambient Node/filesystem authority and could silently move raw reasoning, tool arguments, tool results, paths, or repaired data into Animus ledgers and exports. Animus needs a pinned, non-default adapter that preserves existing Rust ownership and treats normalization as an external host effect followed by pure validation and policy-controlled projection.

## What Changes

- Add a non-default transcript-normalizer adapter profile pinned to `@letta-ai/trajectory` `0.2.0` and reviewed revision `a665556089d6a3c59f5806ee109b4d36f6c2ac83`. r[agent_harness_runtime.letta_trajectory.boundary]
- Author typed Nickel policy for the supported transcript sources, input/output bounds, diagnostic dispositions, reasoning/tool inclusion, redaction, and exact interoperability cohort. r[agent_harness_runtime.letta_trajectory.profile]
- Define an implementation-free normalization service and thin std host adapter over an explicitly injected bounded executor; reusable crates do not discover Node, scan transcript stores, read SQLite, or spawn processes. r[agent_harness_runtime.letta_trajectory.service]
- Add pure Rust validation for exact trajectory-v1 records, ordered roles, timestamps, tool-call argument objects, tool-result correlation, configured bounds, and known diagnostics. r[agent_harness_runtime.letta_trajectory.validation]
- Project validated records into existing Animus session-ledger and batch-trajectory contracts without replacing their identity, replay, persistence, redaction, dataset-export, or ATIF owners. r[agent_harness_runtime.letta_trajectory.projection]
- Emit metadata-only BLAKE3 receipts that bind the reviewed cohort, policy, service, diagnostic classes, counts, omissions, and projection status without transcript bodies or private source metadata. r[agent_harness_runtime.letta_trajectory.evidence]
- Add positive and negative fixtures, dependency-isolation guards, a focused verification rail, integration docs, component/facade metadata, and a bounded README reference. r[agent_harness_runtime.letta_trajectory.verification]

## Impact

- **Reusable crates**: `animus-adapters` gains implementation-free DTOs and a service port; `animus-runtime` gains pure validation and projection only.
- **Host shell**: a dedicated non-default std adapter owns JSON/process protocol plumbing around a caller-injected executor and an exact normalizer executable identity.
- **Configuration**: Nickel remains the source of truth; generated JSON is checked and revalidated by Rust.
- **Existing owners**: neutral ledger, batch runner, persistence, dataset export, ATIF, reasoning policy, process-job profiles, and safe receipts remain authoritative.
- **Testing**: focused adapter/runtime/host tests, exact-cohort fixture parity, positive and negative source fixtures, Nickel drift checks, source-shape guards, Cairn validation/gates, and a Nix check for the opt-in helper.

## Out of Scope

- Reimplementing Trajectory's source decoders in Rust.
- `listTrajectories()`, home-directory scanning, Deep Agents checkpoint/SQLite normalization, the Python wrapper, or cloud-worker ingestion behavior.
- The canonical-v1/ClickHouse record lane, chunked-upload identity, source listing, automatic persistence, or automatic dataset/training export.
- Proving transcript completeness, source authenticity, timestamp truth, tool execution truth, privacy, semantic equivalence, model correctness, evaluation validity, or production readiness.

## Affected Specs

- `agent-harness-runtime`: exact Trajectory cohort, typed policy, injected service, pure validation, ledger/batch projection, safe evidence, and verification.
