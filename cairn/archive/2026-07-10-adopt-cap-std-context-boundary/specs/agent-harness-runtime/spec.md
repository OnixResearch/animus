# Agent Harness Runtime Specification

## Purpose

Adds capability-bounded local filesystem roots for Animus standard-library context and host-store shell boundaries.

## Requirements

### Requirement: Animus std-shell context filesystem authority is capability-rooted
r[agent_harness_runtime.cap_std_context_boundary.root_wrappers] Animus MUST represent workspace, context-file, artifact, and host-store filesystem authority in `animus-std` as typed capability roots opened by the standard-library shell.

#### Scenario: Valid context path stays under the declared root
r[agent_harness_runtime.cap_std_context_boundary.tests.positive]
- GIVEN `animus-std` has opened an operator-declared workspace or context root
- WHEN context discovery, layering, provenance, or host-store code reads a valid relative path through the typed root
- THEN the operation MUST remain confined to that declared root and preserve deterministic context precedence.

#### Scenario: Invalid context path cannot escape the root
r[agent_harness_runtime.cap_std_context_boundary.tests.negative]
- GIVEN input names `../` traversal, an absolute path, a missing capability root, a symlink escape attempt, or hidden host-file exposure outside the declared root
- WHEN `animus-std` resolves the path through the typed root
- THEN Animus MUST fail closed before loading that file into context or host-store evidence.

### Requirement: Capability dependency stays in the std shell
r[agent_harness_runtime.cap_std_context_boundary.dependency] Animus MUST add `cap-std` only to `animus-std` shell/adaptor code that owns filesystem effects.

#### Scenario: Reusable crates stay filesystem-neutral
r[agent_harness_runtime.cap_std_context_boundary.conversion]
- GIVEN `animus-core`, `animus-adapters`, `animus-steel`, `animus-runtime`, `animus-tui`, and root facade contracts are reviewed
- WHEN cap-std adoption is complete
- THEN those crates MUST operate on in-memory DTOs, adapter contracts, loaded context values, or validated relative locators and MUST NOT depend on `cap-std` or open ambient filesystem paths directly.

### Requirement: Capability boundary is documented without host-effect overclaims
r[agent_harness_runtime.cap_std_context_boundary.docs] Animus MUST document that capability roots bound local filesystem reads and writes only and do not sandbox model/tool behavior, provider trust, process execution, credentials, network access, or TUI safety.

#### Scenario: Closeout evidence includes std-shell boundary validation
r[agent_harness_runtime.cap_std_context_boundary.validation]
- GIVEN the cap-std context-boundary change is ready to archive
- WHEN focused `animus-std` tests, crate-boundary scans, Cairn validation, and gate checks run
- THEN the evidence MUST include positive and negative filesystem-boundary coverage and visible host-effect non-claim documentation.
