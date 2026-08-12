## Why

The native evaluation core, adapter family, and standard host divide behavior by authority. Unit tests inside each crate cannot establish that task identity, episode references, service correlations, durable turn ordering, verifier isolation, artifact transfer, rewards, resume, redaction, and receipts remain coherent across those boundaries.

Animus already has focused rails for Harbor, batch trajectories, roles, provider streams, active-turn recovery, dataset export, safe receipts, and composition. A native evaluation conformance suite must reuse public production entrypoints and those accepted owners. A test-only duplicate evaluator would hide integration drift and could pass while embedders fail.

## What Changes

- Add a versioned native evaluation conformance profile and fixture catalog covering core, adapter, host, ledger, batch, role, Harbor, ATIF, evaluation-feedback, dataset-export, active-turn, provider-stream, artifact, persistence, and receipt boundaries. r[agent_harness_runtime.native_evaluation_conformance.matrix]
- Add deterministic positive scenarios for every built-in recipe, exact resume, isolated verification, model gateway ordering, interruption recovery, and safe downstream projections. r[agent_harness_runtime.native_evaluation_conformance.positive]
- Add deterministic negative scenarios for identity drift, authority widening, malformed outputs, race conditions, verifier compromise, artifact attacks, privacy leaks, stale evidence, and unsupported claims. r[agent_harness_runtime.native_evaluation_conformance.negative]
- Require every fixture to call the same public core, adapter, and host entrypoints used by embedders and forbid a second test-only planner, reducer, gateway, or receipt implementation. r[agent_harness_runtime.native_evaluation_conformance.production_paths]
- Add a focused script, Nix check, component and facade coverage, evidence summary, and explicit live-check non-claims. r[agent_harness_runtime.native_evaluation_conformance.evidence] r[agent_harness_runtime.native_evaluation_conformance.verification]

## Impact

- **Fixtures**: versioned Nickel profiles plus bounded Rust fixtures and expected diagnostics.
- **Checks**: `scripts/cairn/check-evaluation-composition-conformance` and a dedicated Nix check.
- **Existing rails**: native evaluation core, adapter family, std host, Harbor, ATIF, batch, ledger, roles, active-turn, provider stream, dataset export, safe receipts, component catalog, facade inventory, and composition conformance.
- **Runtime behavior**: no new production authority or default feature activation.

## Out of Scope

- Live providers, tools, containers, sandboxes, cloud services, public benchmarks, performance claims, RL training, GEPA, or prompt promotion.
- Duplicating product logic under tests or accepting mock call counts as behavioral evidence.
- Verifier soundness, complete trace capture, host confinement, production readiness, authoritative completion, or release eligibility.

## Affected Specs

- `agent-harness-runtime`: evaluation conformance matrix, positive and negative scenarios, production-path requirement, evidence, and verification.
