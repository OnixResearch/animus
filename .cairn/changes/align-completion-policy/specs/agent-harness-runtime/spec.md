# Agent harness runtime delta

## ADDED Requirements

### Requirement: Current completion policy coherence

r[agent_harness_runtime.completion_policy.coherence]

The registry and runtime MUST bind the canonical identity of the reviewed current repository policy.
Verification MUST reject a changed policy or an old policy binding without an explicit reviewed update.
The identity computation MUST use Cairn's published operation, not a copied algorithm.

#### Scenario: Current policy agrees

GIVEN the reviewed current repository policy and its checked registry
WHEN the coherence rail computes the policy identity
THEN it equals the runtime and registry identities.

#### Scenario: Policy drifts

GIVEN the old policy identity or a changed current policy
WHEN the coherence or completion rail validates existing bindings
THEN validation fails rather than accepting internally matching stale inputs.

### Requirement: Core and CLI completion parity

r[agent_harness_runtime.completion_policy.parity]

The maintained completion rail MUST compare complete reports from the pinned core and selected CLI.
It MUST check a positive response and rejection of stale subjects and old policy bindings.
It MUST distinguish actual CLI validation from saved fixture replay and check execution.

#### Scenario: Current positive evidence

GIVEN a current synthetic contract and matching passed results
WHEN the pinned core and CLI validate the same inputs and policy
THEN their complete completion reports agree and the verdict is PASS.

#### Scenario: Invalid evidence cannot borrow success

GIVEN stale subject or old policy inputs, or an altered CLI report
WHEN the rail compares exact completion reports
THEN it rejects the invalid completion or mismatched response, even when the CLI exits zero.
