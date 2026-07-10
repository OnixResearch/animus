# Agent Harness Runtime Specification

## Purpose

Define Animus ast-grep structural audits for agent/runtime adapter boundaries and safe receipt reporting.

## Requirements

### Requirement: Agent-boundary audit profiles
r[animus.ast_grep_agent_audits.profile] Animus MUST define ast-grep audit profiles for syntax-level checks across core, adapter, runtime, std-shell, Steel, TUI, and generated-config surfaces.

#### Scenario: Runtime profile is discovered
- GIVEN an ast-grep runtime-boundary profile declares scan scope and severity posture
- WHEN Animus conformance tooling enumerates checks
- THEN the profile MUST be reported as structural boundary evidence with non-claims.

### Requirement: Inventory-first scans
r[animus.ast_grep_agent_audits.inventory] Animus SHOULD start ast-grep rules as inventory-only for ambient authority, unsafe blocks, panics, direct provider calls, and host effects inside reusable crates.

#### Scenario: Ambient filesystem call is found
- GIVEN an inventory rule matches a filesystem call inside a reusable runtime crate
- WHEN the scan report is produced
- THEN Animus SHOULD record a candidate boundary finding and MUST NOT claim the crate is unsafe solely from the match.

### Requirement: Scan identity receipts
r[animus.ast_grep_agent_audits.identity] Animus MUST bind ast-grep version, rule bundle BLAKE3 identity, scan scope, findings summary, and non-claim labels into audit receipts.

#### Scenario: Rule bundle changes
- GIVEN a safe receipt names an ast-grep rule bundle identity
- WHEN the rule bundle changes
- THEN Animus MUST require a fresh scan before treating findings as current.

### Requirement: Positive and negative fixtures
r[animus.ast_grep_agent_audits.fixtures] Animus MUST require positive and negative rule fixtures before an ast-grep audit rule can become warning or blocking.

#### Scenario: Rule misses allowed adapter shell pattern
- GIVEN a rule flags an allowed std-shell adapter effect as a violation
- WHEN negative fixture validation runs
- THEN the rule MUST fail promotion until the false positive is addressed or explicitly scoped.

### Requirement: Conformance reporting preserves non-claims
r[animus.ast_grep_agent_audits.conformance] Animus MUST report ast-grep findings through safe receipt or conformance-index surfaces without claiming adapter correctness, runtime safety, or provider behavior.

#### Scenario: Provider call candidate is reported
- GIVEN ast-grep finds a direct provider call outside an adapter boundary
- WHEN Animus records the conformance finding
- THEN the report MUST preserve source span and remediation guidance without asserting provider behavior or exploitability.

### Requirement: Audit validation evidence
r[animus.ast_grep_agent_audits.validation] Animus MUST validate ast-grep agent-boundary audits with rule tests, fixture scans, Cairn validation/gates, and focused conformance rails.

#### Scenario: Audit profile changes
- GIVEN an ast-grep agent-boundary profile changes
- WHEN validation evidence is assembled
- THEN the evidence MUST include positive fixtures, negative fixtures, scan receipts, lifecycle gates, and focused Animus rail output.
