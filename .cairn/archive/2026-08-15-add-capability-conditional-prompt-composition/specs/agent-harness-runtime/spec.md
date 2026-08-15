# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Availability is an explicit composition input

r[agent_harness_runtime.capability_conditional_prompt.availability] Prompt composition MUST receive an explicit resolved availability set of tools and skills available for the current turn, derived from extension defaults, model compatibility, policy rules, and the selected role, and MUST NOT infer availability from prompt text.

#### Scenario: Composition consumes only the explicit availability set

- GIVEN a prompt composition request carries a resolved availability set
- WHEN composition runs
- THEN every availability decision uses that set
- AND no fragment availability is inferred from instruction text.

### Requirement: Fragments render only when available

r[agent_harness_runtime.capability_conditional_prompt.conditional] A prompt fragment MUST render only when every tool or skill it names is in the resolved availability set for that turn.

#### Scenario: Unavailable fragment is dropped

- GIVEN a fragment names a tool that is absent from the availability set
- WHEN prompt composition runs
- THEN the fragment is dropped from the composed prompt
- AND the omission is recorded in the composition receipt.

### Requirement: Missing-capability invariant with a negative fixture

r[agent_harness_runtime.capability_conditional_prompt.missing] The composition invariant MUST include a mandatory negative fixture in which a role names a missing tool or skill and MUST NOT produce instructions for it, and capability-conditional composition MUST reject dangling capability references with a typed diagnostic.

#### Scenario: Missing-tool instructions are a correctness failure

- GIVEN a role names a tool that is not in the availability set
- WHEN a negative composition test runs
- THEN the test asserts no instructions are produced for the missing tool
- AND the composition surface treats the dangling reference as an error.

### Requirement: Composition validation fails closed

r[agent_harness_runtime.capability_conditional_prompt.fail_closed] Composition validation MUST reject unresolvable capability inputs and dangling capability references with bounded safe error classes before any prompt is emitted.

#### Scenario: Unresolvable capability denies the composition

- GIVEN a composition request contains an unresolvable capability input
- WHEN composition validation runs
- THEN validation rejects the composition with a typed diagnostic
- AND no prompt is emitted from the invalid composition.

### Requirement: Metadata-only composition receipts

r[agent_harness_runtime.capability_conditional_prompt.receipts] Composition receipts MUST include included fragment identities, omitted fragment identities, the availability-set identity, and rejection classes, and MUST NOT include raw prompt bodies or provider payloads.

#### Scenario: Receipt rejects raw prompt body

- GIVEN a composition receipt fixture includes a raw prompt body or provider payload
- WHEN receipt validation runs
- THEN validation rejects the fixture with a typed leak diagnostic.

### Requirement: Capability-conditional composition verification evidence

r[agent_harness_runtime.capability_conditional_prompt.verification] The adaptation MUST include focused positive and negative tests for availability resolution, conditional rendering, the missing-capability negative fixture, fail-closed validation, receipt leak rejection, and feature isolation.

#### Scenario: Closeout evidence covers composition fixtures

- GIVEN maintainers intend to archive the capability-conditional composition adaptation
- WHEN closeout validation runs
- THEN evidence names focused profile, role, workspace, receipt, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
