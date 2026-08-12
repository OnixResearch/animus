# Credentials Specification

## Purpose

Defines the `credentials` capability.

## Requirements

### Requirement: credentials.secretspec.explicit_resolution

r[credentials.secretspec.explicit_resolution]

The Animus host SHALL resolve a declared credential through the pinned SecretSpec Rust SDK only after pure validation of an explicit manifest, profile, scope, declaration, and provider policy.

#### Scenario: Allowed declaration resolves

- **GIVEN** a valid manifest, explicit profile and scope, an allowed provider, and an existing declaration
- **WHEN** the host requests that declaration
- **THEN** the pure core SHALL authorize a typed resolution request
- **AND** the imperative shell SHALL return the provider-owned value to the requesting adapter

#### Scenario: Implicit or undeclared selection is rejected

- **GIVEN** a missing profile, missing scope, unknown declaration, or disallowed provider
- **WHEN** the host validates the request
- **THEN** validation SHALL fail before provider access
- **AND** the host SHALL NOT fall back to an ambient value

### Requirement: credentials.secretspec.functional_boundary

r[credentials.secretspec.functional_boundary]

SecretSpec manifest access, provider access, environment reads, file reads, and process execution SHALL remain in an optional `std` imperative shell. Reusable cores SHALL accept typed, already-resolved credential inputs.

#### Scenario: Core logic is tested without providers

- **GIVEN** typed declaration metadata and an opaque test credential
- **WHEN** adapter core behavior is tested
- **THEN** the test SHALL require no filesystem, environment, provider, process, clock, or network access

#### Scenario: Provider access is attempted from a core

- **GIVEN** a `no_std` or pure core crate
- **WHEN** dependency checks inspect its graph and source
- **THEN** SecretSpec, filesystem, environment, and process dependencies SHALL be absent

### Requirement: credentials.secretspec.bounded_provider_execution

r[credentials.secretspec.bounded_provider_execution]

A SecretSpec provider path that can launch a child process SHALL run inside an Animus-owned bounded worker with explicit deadline, input, output, cancellation, and teardown limits.

#### Scenario: SOPS resolution succeeds within limits

- **GIVEN** an allowed SOPS profile, valid encrypted input, and valid provider credentials
- **WHEN** the host resolves a declaration
- **THEN** the bounded worker SHALL return only the requested value
- **AND** the worker SHALL terminate before the host uses the value

#### Scenario: Provider process exceeds a bound

- **GIVEN** a provider process that times out, ignores cancellation, or emits oversized output
- **WHEN** the worker reaches a configured bound
- **THEN** the host SHALL terminate the owned process tree
- **AND** the host SHALL return a stable redacted failure category
- **AND** the host SHALL NOT use partial output

### Requirement: credentials.secretspec.no_durable_secret_material

r[credentials.secretspec.no_durable_secret_material]

Animus SHALL NOT place plaintext credentials, decrypted provider files, provider credentials, or value-derived hashes in configuration, persistence, logs, diagnostics, transcripts, snapshots, receipts, or lifecycle evidence.

#### Scenario: Resolution is recorded

- **GIVEN** a successful credential resolution
- **WHEN** Animus emits diagnostics or evidence
- **THEN** it MAY record public declaration, profile, scope, provider-scheme, and policy metadata
- **AND** it SHALL omit the value, provider credentials, decrypted content, and every value-derived hash

#### Scenario: Provider fails with secret-bearing output

- **GIVEN** provider stderr that contains the requested value or a provider credential
- **WHEN** Animus maps the failure
- **THEN** the durable error SHALL contain only a stable redacted category
- **AND** raw stderr SHALL NOT enter durable output

### Requirement: credentials.secretspec.read_only

r[credentials.secretspec.read_only]

The Animus SecretSpec adapter SHALL expose read operations only. It SHALL NOT mutate provider state or encrypted files.

#### Scenario: Caller requests a write

- **GIVEN** a caller that requests credential creation, update, or deletion
- **WHEN** the adapter validates the operation
- **THEN** the adapter SHALL reject it before provider access
- **AND** provider state SHALL remain unchanged

### Requirement: credentials.secretspec.provider_conformance

r[credentials.secretspec.provider_conformance]

Animus SHALL maintain positive and negative conformance fixtures for every enabled SecretSpec provider class and for the bounded worker boundary.

#### Scenario: Supported providers pass

- **GIVEN** valid environment, systemd credential, and SOPS fixtures
- **WHEN** the conformance suite runs
- **THEN** each enabled provider SHALL resolve the expected declaration under its explicit profile and scope

#### Scenario: Unsafe provider behavior fails

- **GIVEN** malformed input, missing credentials, a missing executable, timeout, cancellation, oversized output, or an attempted write
- **WHEN** the conformance suite runs
- **THEN** each case SHALL fail closed with its expected typed outcome
- **AND** no fixture output SHALL contain secret material
