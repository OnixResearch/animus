## ADDED Requirements

### Requirement: Scoped live authority view
r[agent_harness_runtime.scoped_authority.view] The optional Basalt shell MUST offer an owner-created borrowed view of the actual holder authorization, decision, receipt, and commit identity inside dispatch. The view MUST NOT have public construction, Clone, serde, or Debug. Its borrowed authority MUST NOT escape the callback result.

#### Scenario: Actual verified binding
- GIVEN a valid direct or delegated raw holder invocation
- WHEN the scoped effect runs
- THEN it receives the verified operation and effective time bounds, not receipt-derived authority

#### Scenario: Attempted escape
- GIVEN the scoped callback signature
- WHEN a consumer returns the view or a reference to its live holder authorization
- THEN compilation fails

### Requirement: One shared guarded dispatch path
r[agent_harness_runtime.scoped_authority.order] Both dispatch APIs MUST share verification, gate intersection, and durable commit ordering. The callback MUST run only after successful commit and MUST NOT cause a second replay admission. The existing API MUST retain its behavior and receipt schema.

#### Scenario: Denied dispatch
- GIVEN a provider, replay, local-gate, or commit denial
- WHEN either dispatch API runs
- THEN it does not call the effect and preserves applicable denial evidence

#### Scenario: Successful scoped dispatch
- GIVEN valid holder authority and all required gates
- WHEN the commit succeeds
- THEN the callback runs once after exactly one replay admission and receives the actual commit identity
