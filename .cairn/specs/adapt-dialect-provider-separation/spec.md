# Adapt Dialect Provider Separation Specification

## Purpose

Defines the `adapt-dialect-provider-separation` capability.

## Requirements

### Requirement: Model targets resolve transport and dialect independently

r[animus.dialect.target] Every model target MUST resolve to one versioned provider-transport identity and one versioned model-dialect identity. Unknown or missing identities MUST fail closed.

#### Scenario: One transport serves distinct dialects

r[animus.dialect.target.independent]
- GIVEN one provider transport serves models from two behavioral families
- WHEN each model target resolves
- THEN each target MUST retain its exact dialect identity
- AND both targets MAY share the provider-transport identity

#### Scenario: Unknown identity fails

r[animus.dialect.target.unknown]
- GIVEN a target names an unknown provider or dialect identity
- WHEN target resolution runs
- THEN resolution MUST fail before request assembly or provider contact

### Requirement: Provider and dialect owners remain separate

r[animus.dialect.ownership] Provider hosts MUST own authentication, billing, transport, streamed wire decoding, and provider-native continuation. Dialect profiles MUST own prompt assembly, tool surfaces, schema conventions, project-instruction formatting, and subagent protocol.

#### Scenario: Provider effects stay in the host

r[animus.dialect.ownership.provider]
- GIVEN a target resolves to a provider and a dialect
- WHEN the request runs
- THEN authentication, billing, transport, and continuation effects MUST remain in the provider host

#### Scenario: Model-facing projection stays in the dialect

r[animus.dialect.ownership.dialect]
- GIVEN a target resolves to a provider and a dialect
- WHEN the request is assembled
- THEN prompt, tool, schema, instruction, and subagent projections MUST derive from the dialect profile

### Requirement: Request snapshots bind both identities

r[animus.dialect.binding] The complete provider-neutral request snapshot MUST bind the provider identity, dialect identity, exact profile inputs, and compatibility decision before provider contact.

#### Scenario: Shared transport preserves family binding

r[animus.dialect.binding.shared_transport]
- GIVEN two model families use one transport
- WHEN their request snapshots are created
- THEN each snapshot MUST bind its exact dialect and shared transport separately

#### Scenario: Cross-family leakage fails

r[animus.dialect.binding.leak]
- GIVEN a target receives prompt, tool, schema, instruction, or subagent content from another family
- WHEN the snapshot is inspected
- THEN the inspection MUST report the family-binding violation

### Requirement: Provider transitions preserve dialect compatibility

r[animus.dialect.transition] A provider transition MUST retain a dialect only when the new transport explicitly admits that exact dialect cohort. An incompatible transition MUST fail closed and MUST NOT substitute a generic dialect silently.

#### Scenario: Incompatible transition fails

r[animus.dialect.transition.incompatible]
- GIVEN a fallback transport does not admit the current dialect
- WHEN provider transition is planned
- THEN the transition MUST fail before provider contact
- AND MUST NOT substitute another dialect

### Requirement: Dialect separation verification is complete

r[animus.dialect.verification] Positive, negative, and boundary fixtures MUST cover independent resolution, owner boundaries, request-snapshot binding, shared transports, cross-family leakage, and incompatible transitions.

#### Scenario: Dialect separation matrix passes

r[animus.dialect.verification.matrix]
- GIVEN the dialect rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming targets MUST behave as declared
- AND every unknown-identity, ownership, leakage, or compatibility violation MUST fail as declared
