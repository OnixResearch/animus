# Tasks: Type ACP session identities

## Phase 1: Inventory and baseline

- [x] [serial] I1 Inventory identity, reference, protocol, display, and diagnostic strings in the ACP model and facade boundary. r[agent_harness_runtime.acp_nominal_identities.inventory]
- [x] [serial] V1 Capture current JSON-RPC frames, safe events, state transitions, receipt canonical material, and fixture-client outputs. r[agent_harness_runtime.acp_nominal_identities.compatibility]
- [x] [serial] I2 Add private external-ID and marker-role BLAKE3 types with bounds, checked constructors, explicit accessors, and typed diagnostics. r[agent_harness_runtime.acp_nominal_identities.types]
- [x] [parallel] V2 Add positive and negative constructor tests for empty, oversized, malformed, and unsupported values. r[agent_harness_runtime.acp_nominal_identities.validation]

## Phase 2: Request and state migration

- [x] [serial] I3 Split raw frame DTO decoding from typed supported-request admission. r[agent_harness_runtime.acp_nominal_identities.wire_boundary]
- [x] [serial] I4 Migrate connection identity, session binding, pending and completed correlations, and connection state. r[agent_harness_runtime.acp_nominal_identities.state]
- [x] [serial] I5 Migrate facade commands, observations, updates, permission requests, cancellation, failures, and safe receipt inputs. r[agent_harness_runtime.acp_nominal_identities.facade]
- [x] [parallel] I6 Add explicit JSON-RPC and receipt projection adapters that preserve existing strings. r[agent_harness_runtime.acp_nominal_identities.compatibility]

## Phase 3: Evidence

- [x] [parallel] V3 Add compile-pass same-role fixtures and compile-fail ACP/Animus session, session/connection, correlation/run, run/entry, workspace/resource, and authority/session fixtures. r[agent_harness_runtime.acp_nominal_identities.compile_time]
- [x] [parallel] V4 Add wrong-session, unknown-correlation, stale-run, authority-mismatch, duplicate-completion, unsafe-frame, and cancellation negative tests. r[agent_harness_runtime.acp_nominal_identities.failure_matrix]
- [x] [parallel] V5 Prove reviewed frame JSON, safe event shape, receipt canonical material, and BLAKE3 receipt refs remain unchanged. r[agent_harness_runtime.acp_nominal_identities.compatibility]
- [x] [parallel] V6 Run focused ACP core, fixture-client, facade, permission, cancellation, and receipt suites. r[agent_harness_runtime.acp_nominal_identities.facade]

## Phase 4: Enforcement and closeout

- [x] [serial] I7 Add ACP identity domains to Animus Nickel config and the future Octet nominal-domain policy. r[agent_harness_runtime.acp_nominal_identities.octet]
- [x] [serial] I8 Document typed ACP boundaries, external-ID limits, receipt compatibility, and authority non-claims. r[agent_harness_runtime.acp_nominal_identities.docs]
- [x] [serial] V7 Run config freshness, focused Cargo checks, Clippy, ACP Cairn rails, lifecycle validation, change gates, and relevant Nix checks. r[agent_harness_runtime.acp_nominal_identities.final_checks]

## Verification coverage

- `Scenario: Inventory excludes protocol and display text` -> I1
- `Scenario: Same-role ACP identity is accepted` -> I2, V2, V3
- `Scenario: Cross-role ACP identity does not compile` -> I2, V3
- `Scenario: Raw frame admits a typed request` -> I3, V4
- `Scenario: Malformed frame identity fails before state use` -> I3, V4
- `Scenario: Session binding preserves ACP and Animus roles` -> I4, V4
- `Scenario: Facade commands keep run and correlation roles` -> I5, V4
- `Scenario: Frame and receipt outputs remain stable` -> V1, I6, V5
- `Scenario: Existing ACP cohort retains behavior` -> V6
- `Scenario: Octet rejects raw identity regression` -> I7, V7
- `Scenario: Typed authority identity does not grant permission` -> I8, V4, V7
