# agent-harness-runtime Change Spec: Clankers focused verification rails

## Requirements

### Requirement: Focused rail convention

r[agent_harness_runtime.clankers_focused_rails.convention] Animus MUST define a focused verification rail convention where each rail validates one boundary, uses deterministic fixtures, emits safe versioned evidence, and avoids live credentials or host-specific state by default.

#### Scenario: Rails have bounded evidence

r[agent_harness_runtime.clankers_focused_rails.convention.evidence]
- GIVEN a focused rail validates an Animus boundary
- WHEN the rail writes evidence
- THEN evidence includes schema, status, fixture names, hashed artifacts, and safe metadata
- AND it omits raw prompts, credentials, provider payloads, full paths, raw tool output, and live host state.

### Requirement: Lifecycle documentation rail

r[agent_harness_runtime.clankers_focused_rails.lifecycle] Animus MUST provide a lifecycle-doc marker rail that checks boundary names, owner markers, forbidden dependency sections, and validation command references.

#### Scenario: Lifecycle rail catches drift

r[agent_harness_runtime.clankers_focused_rails.lifecycle.drift]
- GIVEN the lifecycle document loses a required boundary, owner, forbidden dependency section, or validation marker
- WHEN the lifecycle rail runs
- THEN it fails with an actionable diagnostic
- AND it does not require provider credentials or live services.

### Requirement: Steel policy rail

r[agent_harness_runtime.clankers_focused_rails.steel_policy] Animus MUST provide focused Steel policy rails for Nickel/export drift, valid activation, invalid policy fixtures, and enforcement-before-evaluation behavior.

#### Scenario: Steel rail covers positive and negative activation

r[agent_harness_runtime.clankers_focused_rails.steel_policy.fixtures]
- GIVEN valid and invalid Steel policy fixtures exist
- WHEN the Steel policy rail runs
- THEN valid policy activates safely
- AND invalid policies fail before Steel evaluation or adapter dispatch.

### Requirement: Receipt, artifact, ledger, and service rails

r[agent_harness_runtime.clankers_focused_rails.receipts_artifacts] Animus MUST provide focused rails for safe receipt validation, leak rejection, BLAKE3 artifact determinism, and redaction-class display behavior.

r[agent_harness_runtime.clankers_focused_rails.ledger] Animus MUST provide focused rails for neutral ledger replay, unsupported-entry failure, safe metadata counts, and missing ledger service behavior.

r[agent_harness_runtime.clankers_focused_rails.injected_services] Animus MUST provide focused rails or tests for injected provider/tool service boundaries, absence of ambient provider behavior, and fail-closed missing/denied/cancelled/malformed outcomes.

#### Scenario: Boundary rails cover negative cases

r[agent_harness_runtime.clankers_focused_rails.boundary_negative]
- GIVEN receipt, artifact, ledger, or injected-service boundary fixtures include unsafe or unavailable cases
- WHEN focused rails run
- THEN each unsafe or unavailable case returns a typed failure or redacted result
- AND no rail accepts leaked sensitive material or false success evidence.

### Requirement: Stable rails wire into Nix checks

r[agent_harness_runtime.clankers_focused_rails.nix_wiring] Stable credential-free focused rails SHOULD be wired into named Nix checks or the default flake check once deterministic; rails not wired into Nix MUST document the blocker.

#### Scenario: Closeout separates evidence from non-claims

r[agent_harness_runtime.clankers_focused_rails.closeout]
- GIVEN a Clankers-pattern adoption is ready for archive
- WHEN closeout evidence is prepared
- THEN it names successful focused rails, workspace tests, clippy, Nickel/config drift checks when applicable, Cairn validation, and Cairn gates
- AND it explicitly identifies skipped rails and non-claims such as live providers, TUI dogfood, daemon tests, VM checks, or production readiness when not covered.
