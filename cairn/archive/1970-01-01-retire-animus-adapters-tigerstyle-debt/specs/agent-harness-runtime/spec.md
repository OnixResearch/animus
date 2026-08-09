## ADDED Requirements

### Requirement: Adapter public paths remain compatible

r[agent_harness_runtime.adapters_tigerstyle.compatibility] Animus SHALL retain the existing public adapter module paths and public contract names while canonical implementations move to neutral searchable module paths.

#### Scenario: Existing downstream path still compiles

r[agent_harness_runtime.adapters_tigerstyle.compatibility.path]
- GIVEN downstream code imports a reviewed host-authority, messaging, Pijul, SecretSpec, skill-source, or trajectory contract through its current public module
- WHEN the Tiger Style refactor is compiled
- THEN the existing path resolves to the same public contract
- AND no new ambient authority or dependency is introduced.

### Requirement: Adapter Tiger Style scan has zero findings

r[agent_harness_runtime.adapters_tigerstyle.zero_findings] The checked Octet profile MUST report zero Tiger Style findings for `animus-adapters` without disabling lint families or adding broad module-wide suppression.

#### Scenario: Octet scans library and test targets

r[agent_harness_runtime.adapters_tigerstyle.zero_findings.scan]
- GIVEN the immutable Octet toolchain, policy, and Cargo vendor inputs
- WHEN the focused adapter scan runs over library and test targets
- THEN every enabled Tiger Style lint reports zero findings
- AND an integration failure is not accepted as a passing result.

### Requirement: Refactoring preserves adapter behavior

r[agent_harness_runtime.adapters_tigerstyle.behavior] Tiger Style refactoring MUST preserve existing DTO fields, discriminants, validation outcomes, receipts, non-claims, effect boundaries, and positive and negative test behavior.

#### Scenario: Invalid adapter input still fails closed

r[agent_harness_runtime.adapters_tigerstyle.behavior.negative]
- GIVEN an existing malformed, stale, unauthorized, oversized, or privacy-unsafe adapter input
- WHEN the refactored contract validates that input
- THEN it returns the same typed rejection class
- AND it performs no added effect.

### Requirement: Adapter quality debt has focused verification

r[agent_harness_runtime.adapters_tigerstyle.verification] The change SHALL include positive compatibility checks, negative behavior checks, Clippy, no-default, Wasm, focused Octet, Cairn, and relevant broad Nix evidence.

#### Scenario: Focused verification passes

r[agent_harness_runtime.adapters_tigerstyle.verification.focused]
- GIVEN the refactored adapter source and compatibility modules
- WHEN focused verification runs
- THEN public compatibility and positive and negative behavior tests pass
- AND Octet reports zero adapter findings.
