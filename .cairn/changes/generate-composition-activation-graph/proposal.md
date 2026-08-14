# Proposal: Generate the composition activation graph

## Why

Animus has a typed component catalog, request-lifecycle ownership map, runtime facade inventory, and composition conformance rail. The catalog records component dependencies and authority posture, but it does not fully record application-owned ports, providers, direct consumers, event producers, event consumers, lifecycle owners, and required activation state as one checked graph.

This gap makes it possible for a composition root to select cataloged components while leaving a required provider unresolved, using an undeclared consumer, or starting effects before required components report readiness.

DeepSeek Harness commit `47f943859bef60e4160492346772ded9b24f765a` provides useful capability-seam, event-matrix, and fail-loud boot patterns. Animus should adapt those patterns as generated metadata and pure preflight decisions. It should not adopt Cordis contexts, ambient dependency injection, or the rule that every policy owner is a plugin.

## What Changes

- Extend the Nickel component catalog with typed port, provider, consumer, event, scope, lifecycle, multiplicity, and activation records. r[agent_harness_runtime.composition_activation_graph.catalog]
- Generate human-readable dependency, capability-seam, and event-ownership maps from the Nickel source. r[agent_harness_runtime.composition_activation_graph.projections]
- Add a pure activation preflight that validates one selected composition and current host-supplied readiness facts before effects. r[agent_harness_runtime.composition_activation_graph.preflight]
- Fail startup for unresolved required providers, undeclared consumers, invalid multiplicity, missing lifecycle owners, stale readiness, and required event-route gaps. r[agent_harness_runtime.composition_activation_graph.fail_loud]
- Expose a bounded read-only safe activation snapshot for later Host Control inspection mapping. r[agent_harness_runtime.composition_activation_graph.inspection]
- Add positive and negative catalog, projection, preflight, and composition fixtures. r[agent_harness_runtime.composition_activation_graph.verification]

## Prerequisites

- `agent_harness_runtime.component_catalog.source`
- `agent_harness_runtime.component_catalog.metadata`
- `agent_harness_runtime.component_catalog.generated_docs`
- `agent_harness_runtime.composition_conformance.boundary`
- `agent_harness_runtime.request_lifecycle.ownership`

## Impact

- **Configuration**: `.cairn/component-catalog.ncl` remains the Nickel source of truth.
- **Pure core**: activation requests, readiness observations, deterministic decisions, issue classes, and safe snapshots.
- **Shells**: product composition roots must run preflight before provider, process, filesystem, network, credential, persistence, terminal, or daemon effects.
- **Documentation**: generated component, seam, dependency, and event maps.
- **Existing active work**: live Steel, daemon, and terminal hosts must declare their selected activation graph before closeout.

## Out of Scope

- Runtime plugin discovery, dynamic code loading, service locators, global registries, arbitrary event payloads, provider startup, or product effects.
- Replacing Rust ownership, authority policy, request lifecycle, or focused conformance tests with metadata.

## Affected Specs

- `agent-harness-runtime`: catalog completeness, generated projections, activation preflight, fail-loud startup, safe inspection, and verification.
