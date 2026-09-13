# Animus Live-Action Receipt

## ADDED Requirements

### Requirement: Receipt identity

r[animus.live_action_receipt.identity] Every live-action receipt MUST carry a BLAKE3 identity over a canonical projection of action kind, adapter identity, request snapshot, response selection, and gate verdicts. The projection MUST exclude ambient session state that does not change the action.

#### Scenario: Equal actions project to equal identities

r[animus.live_action_receipt.identity.scenario.equal]
- GIVEN two actions with equal kind, adapter, request, selection, and verdicts
- WHEN the core projects both receipts
- THEN the identities MUST be equal

#### Scenario: Distinct actions project to distinct identities

r[animus.live_action_receipt.identity.scenario.distinct]
- GIVEN two actions whose kind, adapter, request, selection, or verdicts differ
- WHEN the core projects both receipts
- THEN the identities MUST differ

### Requirement: Universal publication

r[animus.live_action_receipt.universality] When a session records, the live loop MUST publish a receipt for every admitted action across model calls, tool calls, response selection, retries, cancellations, and authority intersections. The loop MUST NOT leave an admitted action without a receipt.

#### Scenario: Recording session publishes all actions

r[animus.live_action_receipt.universality.scenario.all]
- GIVEN a session with recording enabled
- WHEN the loop admits any covered action
- THEN the loop MUST publish exactly one receipt for it

#### Scenario: Non-recording session stays silent

r[animus.live_action_receipt.universality.scenario.off]
- GIVEN a session without recording
- WHEN the loop admits an action
- THEN the loop MUST NOT require a receipt

### Requirement: Claim boundary

r[animus.live_action_receipt.boundary] A receipt MUST claim only observed binding facts. A receipt MUST NOT claim model honesty, tool success, or system correctness.

#### Scenario: Receipt over-claim fails

r[animus.live_action_receipt.boundary.scenario.overclaim]
- GIVEN a receipt that claims tool success
- WHEN the claim boundary is verified
- THEN the claim MUST fail unless the tool owner makes it

#### Scenario: Metadata-only receipt passes

r[animus.live_action_receipt.boundary.scenario.metadata]
- GIVEN a receipt that records only exact adapter and verdict identities
- WHEN the claim boundary is verified
- THEN the receipt MUST pass

### Requirement: Verifiability without a store

r[animus.live_action_receipt.verifiable] Every receipt MUST verify through the deterministic verifier with no store access. The verifier MUST reject tampered or malformed receipts.

#### Scenario: Valid receipt verifies

r[animus.live_action_receipt.verifiable.scenario.valid]
- GIVEN an untampered receipt
- WHEN the verifier checks it
- THEN the verifier MUST accept it

#### Scenario: Tampered receipt fails

r[animus.live_action_receipt.verifiable.scenario.tampered]
- GIVEN a receipt with a mutated identity or payload
- WHEN the verifier checks it
- THEN the verifier MUST reject it

### Requirement: Authority-gate identity

r[animus.live_action_receipt.authority] Each receipt MUST record the exact authority-gate identities applied to the action. A receipt MUST NOT assert authority that no gate verified.

#### Scenario: Missing gate identity fails

r[animus.live_action_receipt.authority.scenario.missing]
- GIVEN a receipt with no gate identity
- WHEN the authority check runs
- THEN the check MUST fail

#### Scenario: Exact gate identity passes

r[animus.live_action_receipt.authority.scenario.exact]
- GIVEN a receipt that names one exact verified gate
- WHEN the authority check runs
- THEN the check MUST pass

### Requirement: Fixture coverage

r[animus.live_action_receipt.verification] Positive, negative, and boundary fixtures MUST cover identity, universality, claim boundary, verifiability, and authority identity.

#### Scenario: Complete focused matrix passes

r[animus.live_action_receipt.verification.scenario.matrix]
- GIVEN decisions, fixtures, documentation, and shell wiring are complete
- WHEN focused workspace, Clippy, Cairn, and Nix verification runs
- THEN in-bounds inputs MUST behave as declared
- AND every over-claim, tampered, missing-gate, or malformed input MUST fail as declared
