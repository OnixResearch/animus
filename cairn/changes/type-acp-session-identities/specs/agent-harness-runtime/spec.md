# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: ACP host identities are nominal

r[agent_harness_runtime.acp_nominal_identities.types] Animus MUST represent admitted ACP correlation, ACP session, Animus session, connection, workspace, resource, run, entry, authority-profile, session, and receipt identities with distinct private Rust types.

#### Scenario: Same-role ACP identity is accepted

r[agent_harness_runtime.acp_nominal_identities.types.same_role]
- GIVEN a valid typed ACP session ID
- WHEN an ACP state API requires an ACP session ID
- THEN the API MUST accept it without another expected-role string.

#### Scenario: Cross-role ACP identity does not compile

r[agent_harness_runtime.acp_nominal_identities.compile_time]
- GIVEN ACP session and Animus session IDs use distinct types
- WHEN source passes an Animus session ID to an API that requires an ACP session ID
- THEN the source MUST fail compilation with a type mismatch.

#### Scenario: Malformed identity fails before state use

r[agent_harness_runtime.acp_nominal_identities.validation]
- GIVEN an untrusted identity is empty, oversized, malformed, or uses unsupported digest spelling
- WHEN ACP admission constructs the typed value
- THEN construction MUST fail with deterministic diagnostics
- AND the invalid identity MUST NOT enter the connection state machine.

### Requirement: ACP identity migration follows a semantic inventory

r[agent_harness_runtime.acp_nominal_identities.inventory] Animus MUST classify selected ACP strings as external ID, Animus identity, reference, protocol constant, display text, diagnostic text, or deferred before migration.

#### Scenario: Inventory excludes protocol and display text

r[agent_harness_runtime.acp_nominal_identities.inventory.exclusions]
- GIVEN the ACP model contains method names, protocol versions, safe labels, failure text, and redaction text
- WHEN the identity inventory runs
- THEN those values MUST remain outside nominal identity enforcement unless a separate domain is approved.

### Requirement: Raw ACP frames admit typed requests

r[agent_harness_runtime.acp_nominal_identities.wire_boundary] Animus MUST decode bounded ACP JSON-RPC frames into raw DTOs and MUST construct typed supported requests before state transitions or facade dispatch.

#### Scenario: Raw frame admits a typed request

r[agent_harness_runtime.acp_nominal_identities.wire_boundary.valid]
- GIVEN a supported ACP frame contains bounded valid correlation, session, workspace, resource, run, or entry data
- WHEN request admission runs
- THEN Animus MUST construct the matching typed request variant.

#### Scenario: Malformed frame identity fails before state use

r[agent_harness_runtime.acp_nominal_identities.wire_boundary.invalid]
- GIVEN a frame has a malformed, oversized, wrong-role, or conflicting identity
- WHEN request admission runs
- THEN Animus MUST return a typed failure before changing connection or session state.

### Requirement: ACP session state preserves identity roles

r[agent_harness_runtime.acp_nominal_identities.state] ACP connection and session state MUST preserve distinct connection, ACP session, Animus session, session, authority-profile, run, and correlation roles.

#### Scenario: Session binding preserves ACP and Animus roles

r[agent_harness_runtime.acp_nominal_identities.state.binding]
- GIVEN an ACP session binds to one Animus session and one authority profile
- WHEN prompt, load, continue, or cancel transitions run
- THEN each transition MUST compare the exact typed role and reject a valid value from another role.

### Requirement: ACP facade inputs remain typed

r[agent_harness_runtime.acp_nominal_identities.facade] ACP facade commands, observations, updates, permission requests, cancellation, failures, and receipt construction MUST retain typed identities until protocol or receipt projection.

#### Scenario: Facade commands keep run and correlation roles

r[agent_harness_runtime.acp_nominal_identities.failure_matrix]
- GIVEN a command carries a valid correlation ID but a stale or wrong-session run ID
- WHEN facade admission runs
- THEN the command MUST fail without provider, tool, permission, or persistence effects.

### Requirement: ACP wire and receipt compatibility is preserved

r[agent_harness_runtime.acp_nominal_identities.compatibility] The nominal-type migration MUST preserve the reviewed ACP protocol cohort, JSON-RPC shape, safe event projection, receipt canonical material, and BLAKE3 receipt identity.

#### Scenario: Frame and receipt outputs remain stable

r[agent_harness_runtime.acp_nominal_identities.compatibility.golden]
- GIVEN a reviewed positive or negative fixture-client scenario runs before and after migration
- WHEN frame JSON, safe events, receipt bytes, and receipt refs are compared
- THEN they MUST remain unchanged unless a separate versioned protocol or receipt change approves the difference.

### Requirement: Octet guards ACP identity scopes

r[agent_harness_runtime.acp_nominal_identities.octet] After the policy becomes available, Animus MUST declare migrated ACP identity domains through Nickel config to the reviewed Octet nominal-domain policy.

#### Scenario: Raw ACP identity regression is rejected

r[agent_harness_runtime.acp_nominal_identities.octet.guard]
- GIVEN a migrated ACP state or facade field reintroduces a declared identity as `String`
- WHEN Octet checks the configured scope
- THEN the selected nominal-domain check MUST fail.

### Requirement: Typed authority identity does not grant permission

r[agent_harness_runtime.acp_nominal_identities.docs] Animus documentation and receipts MUST state that typed ACP identities prove local category separation and checked syntax only.

#### Scenario: Boundary remains visible

r[agent_harness_runtime.acp_nominal_identities.final_checks]
- GIVEN typed ACP identities pass request, state, facade, and receipt checks
- WHEN Animus states the supported claim
- THEN it MUST NOT claim client authorization, sandbox enforcement, provider correctness, replay equivalence, arbitrary ACP compatibility, or release eligibility.
