# Agent Spec-Contract Rail Specification

## Purpose

Define a harness rail that runs a bounded model check after an agent edits a behavioral model, admits only fresh, matching, receipted verdicts, and fails handoff closed when the receipt is missing or stale.

## ADDED Requirements

### Requirement: The rail runs after edits and before handoff

r[animus.spec_contract.rail]

Animus MUST provide a spec-contract stage that runs after a behavioral-model edit and before action handoff, and MUST admit the edited model only with a fresh, matching verdict receipt.

#### Scenario: Edited model is receipted
- GIVEN an agent edits a behavioral model and a matching verdict receipt is produced
- WHEN the spec-contract stage evaluates
- THEN the loop MAY accept the action result.

#### Scenario: No check ran
- GIVEN an agent edits a behavioral model and no verdict receipt exists
- WHEN the spec-contract stage evaluates
- THEN the loop MUST deny handoff.

### Requirement: The checker is a pinned adapter

r[animus.spec_contract.adapter]

Animus MUST consume the external model checker through a pinned adapter that implements the bounded checking contract, and MUST admit only an exact checker identity and version.

#### Scenario: Admitted adapter is used
- GIVEN an adapter identity and version admitted by the rail
- WHEN model-check invocation is requested
- THEN the adapter MUST return verdict, identity, and bound facts through the contract.

#### Scenario: Checker is unsupported
- GIVEN the checker identity or version is not admitted
- WHEN model-check invocation is requested
- THEN the rail MUST deny handoff with an unsupported-checker outcome.

### Requirement: Receipts bind the edited model exactly

r[animus.spec_contract.receipt]

The loop receipt MUST bind the post-edit model hash, checker identity and version, declared bounds, and verdict class, and MUST classify a receipt whose model hash differs as stale for the current action.

#### Scenario: Receipt matches the edited model
- GIVEN the receipt hash equals the post-edit model hash
- WHEN receipt admission runs
- THEN the loop MAY proceed to handoff.

#### Scenario: Receipt is stale
- GIVEN the receipt binds a different model hash, checker version, or bounds
- WHEN receipt admission runs
- THEN the rail MUST deny handoff and MUST report the mismatched layer.

### Requirement: Handoff fails closed

r[animus.spec_contract.fail_closed]

Animus MUST deny handoff deterministically for missing, stale, mismatched, or unsupported-checker outcomes, and MUST produce a denial receipt that names the failed layer.

#### Scenario: All completion paths are defined
- GIVEN any of the denied outcome classes occurs
- WHEN the spec-contract stage terminates
- THEN it MUST produce one deterministic denial outcome with a named failed layer.

#### Scenario: Denial receipt validates
- GIVEN a denial outcome
- WHEN the denial receipt is validated
- THEN it MUST bind the model hash and the failed layer identity.

### Requirement: Suite has positive and negative fixtures

r[animus.spec_contract.fixtures]

The spec-contract suite MUST include positive fixtures for receipted pass-and-handoff and negative fixtures for missing receipts, stale receipts, mismatched model hashes, and unsupported checker identities.

#### Scenario: Suite runs
- GIVEN all fixtures are committed with their declared identities
- WHEN the harness executes
- THEN positive fixtures MUST pass and each negative fixture MUST fail at its declared boundary.

### Requirement: Rail changes run full validation

r[animus.spec_contract.validation]

Changes to the rail, adapter contract, receipts, or fixtures MUST run focused core tests, clippy with warnings denied, lifecycle validation, Cairn gates, and relevant flake checks.

#### Scenario: Change is validated
- GIVEN a spec-contract change passes all focused checks
- WHEN lifecycle validation is assembled
- THEN it MUST include both positive and negative results and MUST identify the exact checker identity and rail version.
