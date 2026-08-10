# Agent harness runtime delta

## ADDED Requirements

### Requirement: Real-check receipts expose exact index metadata

r[agent_harness_runtime.real_check_evidence_index.receipt] The real-check completion smoke receipt MUST include the exact selected check identifiers and pinned Cairn binding without exposing commands, output, subject data, credentials, or paths.

#### Scenario: Live receipt names the bounded evidence

r[agent_harness_runtime.real_check_evidence_index.receipt.live]

- **GIVEN** the two reviewed quick checks pass through pinned Cairn on a clean head
- **WHEN** the live smoke writes its metadata-only receipt
- **THEN** the receipt SHALL name `nickel-config,cairn-validate` and `pinned-cairn-core`

### Requirement: Release index has a separate real-check claim

r[agent_harness_runtime.real_check_evidence_index.claim] The release evidence index MUST represent `bounded-real-check-completion` separately from current-head pure evidence, packaged dogfood, and release readiness.

#### Scenario: Bounded claim remains narrow

r[agent_harness_runtime.real_check_evidence_index.claim.boundary]

- **GIVEN** a valid real-check receipt
- **WHEN** the index accepts the bounded claim
- **THEN** the claim SHALL NOT imply release readiness, production readiness, check sufficiency, executable authenticity, sandboxing, hermeticity, or check-oracle soundness

### Requirement: Generator reads explicit receipt evidence

r[agent_harness_runtime.real_check_evidence_index.generator] The current-head generator MUST read only an explicit `--real-check-receipt` path, normalize its safe metadata into one typed index record, and MUST NOT run the smoke or hidden validation suites.

#### Scenario: Explicit receipt creates one record

r[agent_harness_runtime.real_check_evidence_index.generator.record]

- **GIVEN** a valid metadata-only live receipt path and the bounded claim
- **WHEN** the generator renders an index
- **THEN** it SHALL emit one normalized real-check receipt record bound to the receipt head

### Requirement: Real-check index admission fails closed

r[agent_harness_runtime.real_check_evidence_index.validation] The index checker MUST require an exact schema, rail, live mode, pass status, clean matching head, 64-hex source BLAKE3, exact selected check identifiers, and pinned Cairn binding for `bounded-real-check-completion`.

#### Scenario: Complete evidence passes

r[agent_harness_runtime.real_check_evidence_index.validation.pass]

- **GIVEN** one exact live receipt record for the clean indexed head
- **WHEN** the bounded claim is checked
- **THEN** the index SHALL pass without requiring unrelated pure modes

#### Scenario: Stale evidence fails

r[agent_harness_runtime.real_check_evidence_index.validation.stale]

- **GIVEN** a receipt head different from the indexed head
- **WHEN** the bounded claim is checked
- **THEN** the index SHALL report stale receipt evidence

#### Scenario: Wrong selected checks fail

r[agent_harness_runtime.real_check_evidence_index.validation.checks]

- **GIVEN** a receipt that names a different or incomplete selected-check set
- **WHEN** the bounded claim is checked
- **THEN** the index SHALL reject the real-check receipt

#### Scenario: Wrong Cairn binding fails

r[agent_harness_runtime.real_check_evidence_index.validation.cairn]

- **GIVEN** a receipt without the exact pinned Cairn binding
- **WHEN** the bounded claim is checked
- **THEN** the index SHALL reject the real-check receipt

#### Scenario: Generic pass row cannot self-attest

r[agent_harness_runtime.real_check_evidence_index.validation.self_attestation]

- **GIVEN** a passing generic check row without a typed real-check receipt record
- **WHEN** the bounded claim is checked
- **THEN** the index SHALL reject the claim as missing receipt evidence

### Requirement: Real-check index behavior has paired fixtures

r[agent_harness_runtime.real_check_evidence_index.verification] The release-index rails MUST contain positive and negative fixtures for the real-check claim and its exact receipt bindings.

#### Scenario: Focused rails cover rejection paths

r[agent_harness_runtime.real_check_evidence_index.verification.fixtures]

- **GIVEN** valid and adversarial synthetic receipt records
- **WHEN** the schema and generated-index rails run
- **THEN** they SHALL accept the exact case and reject dirty, stale, malformed, fixture, non-pass, wrong-check, wrong-Cairn, missing, duplicate, and conflicting cases

### Requirement: Documentation states the indexed claim

r[agent_harness_runtime.real_check_evidence_index.docs] Animus documentation MUST show how to generate the bounded claim from an explicit receipt and MUST list its stronger non-claims.

#### Scenario: Operator can distinguish claim levels

r[agent_harness_runtime.real_check_evidence_index.docs.boundary]

- **GIVEN** the release evidence index documentation
- **WHEN** an operator reviews the real-check command
- **THEN** the text SHALL distinguish bounded real-check completion from release readiness

### Requirement: Closeout binds final evidence

r[agent_harness_runtime.real_check_evidence_index.closeout] The change MUST pass focused, dogfood, Nix, Cairn, clean-current-head, sync, and archive checks before publication.

#### Scenario: Published head retains the archived change

r[agent_harness_runtime.real_check_evidence_index.closeout.published]

- **GIVEN** all implementation tasks and checks pass
- **WHEN** the change is archived and published
- **THEN** the final head SHALL contain the accepted requirements and archived evidence
