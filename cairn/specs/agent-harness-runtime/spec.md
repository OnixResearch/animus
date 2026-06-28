# Agent Harness Runtime Specification

## Purpose

Defines the accepted baseline for Animus as a modular Rust agent-harness framework with explicit crate barriers, typed adapter contracts, a Steel decision fabric, and standard-library host shells.

## Requirements

### Requirement: Crate boundary baseline

r[agent_harness_runtime.baseline.crate_boundaries] The workspace MUST preserve explicit crate boundaries between pure core state, implementation-free adapter contracts, optional Preserves boundary serialization, Steel turn decisions, standard-library host orchestration, optional terminal UI hosting, and the root compatibility facade.

#### Scenario: Embedders choose the smallest surface

r[agent_harness_runtime.baseline.crate_boundaries.smallest_dependency]
- GIVEN an embedder needs only deterministic state logic, adapter DTOs, serialization, Steel scripting, std shell orchestration, or TUI hosting
- WHEN they select workspace crates
- THEN each reusable crate exposes only its owned boundary
- AND concrete host, provider, filesystem, process, network, and UI behavior stays outside unrelated reusable crates.

### Requirement: Functional core and imperative shell split

r[agent_harness_runtime.baseline.functional_core] Non-trivial Animus behavior MUST be decomposed into pure deterministic core logic over explicit inputs and thin imperative shells for filesystem, process, environment, network, terminal, credential, clock, and persistence effects.

#### Scenario: Core logic is testable without standing up the world

r[agent_harness_runtime.baseline.functional_core.testable]
- GIVEN runtime, policy, adapter, prompt, provenance, or serialization logic contains branch decisions
- WHEN maintainers add or change that behavior
- THEN the branch decisions are represented as deterministic functions over in-memory values
- AND tests can exercise positive and negative cases without real providers, shells, terminals, credentials, or storage backends.

### Requirement: Adapter-first integration boundary

r[agent_harness_runtime.baseline.adapters] Model, tool, memory, ledger, queue, policy, confirmation, provider, workspace, and host integrations MUST cross typed adapter or service contracts rather than being constructed by reusable runtime core code.

#### Scenario: Missing host services fail closed

r[agent_harness_runtime.baseline.adapters.missing_service]
- GIVEN reusable runtime logic requires a host capability
- WHEN the corresponding adapter or service is absent, denied, unavailable, malformed, or cancelled
- THEN runtime returns a typed failure before claiming completed effects
- AND it does not discover ambient config, read credentials, spawn commands, or contact providers as a fallback.

### Requirement: Steel decision fabric baseline

r[agent_harness_runtime.baseline.steel_fabric] Animus SHOULD use Steel Scheme as the default scriptable turn-decision fabric while Rust boundary code MUST revalidate every selected model, tool, memory, response, or host action before dispatch.

#### Scenario: Script decisions are revalidated

r[agent_harness_runtime.baseline.steel_fabric.revalidated]
- GIVEN a Steel script returns a loop decision
- WHEN Rust runtime code receives that decision
- THEN the decision is decoded through typed boundary DTOs
- AND malformed payloads, unsupported actions, unauthorized effects, or invalid text fail closed before adapter dispatch.

### Requirement: Nickel-owned repository configuration

r[agent_harness_runtime.baseline.nickel_config] Repo-owned tool and policy configuration MUST use Nickel as the source of truth when Animus owns the config shape, while generated tool-native artifacts are kept fresh for consumers that require JSON, TOML, YAML, or other native formats.

#### Scenario: Generated config drift is detectable

r[agent_harness_runtime.baseline.nickel_config.drift]
- GIVEN maintainers change repo-owned Nickel configuration
- WHEN generated native config files are checked
- THEN drift is reported deterministically
- AND stale generated config is not treated as authoritative evidence.

### Requirement: Verification safety rails

r[agent_harness_runtime.baseline.verification] The workspace MUST maintain focused positive and negative tests, formatting, linting, Tiger Style, generated-config drift checks, feature checks, and lifecycle validation evidence for behavior that crosses public API, adapter, policy, serialization, or host boundaries.

#### Scenario: Boundary changes include reviewable evidence

r[agent_harness_runtime.baseline.verification.evidence]
- GIVEN a boundary-affecting change is ready for review
- WHEN maintainers inspect its evidence
- THEN the evidence names the focused checks that cover the changed boundary
- AND unavailable checks are recorded with a blocker and next best validation path.
