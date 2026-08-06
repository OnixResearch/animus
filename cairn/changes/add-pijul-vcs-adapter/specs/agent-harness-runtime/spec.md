# Agent Harness Runtime

## ADDED Requirements

### Requirement: Pijul remains a pinned optional CLI boundary

r[agent_harness_runtime.pijul_adapter.boundary] Animus MUST adapt Pijul through a non-default boundary pinned to an exact reviewed `pijul` CLI release, upstream source revision, change-hash format, machine-output format identity, and reviewed GPL-2.0-family license cohort, and reusable, default, no-std, and Wasm surfaces MUST NOT link `libpijul`, `sanakirja`, or any Pijul crate, open `.pijul` databases, acquire process, filesystem, environment, network, credential, key, clock, or persistence dependencies, or discover VCS state.

#### Scenario: Exact read-only CLI cohort is selected

r[agent_harness_runtime.pijul_adapter.boundary.exact_cohort]
- GIVEN a host selects the reviewed `pijul` CLI cohort with its exact release, revision, output-format identity, and license record
- WHEN the optional adapter is composed
- THEN Animus MAY admit bounded query operations such as `log`, `change`, `diff`, and channel listing against explicitly supplied repository paths
- AND default or reusable builds MUST NOT require the `pijul` executable or any Pijul crate.

#### Scenario: Excluded effectful surface is requested

r[agent_harness_runtime.pijul_adapter.boundary.excluded_surfaces]
- GIVEN a caller requests `libpijul` linking, direct Sanakirja database access, `clone`, `pull`, `push`, remote management, Nest integration, identity creation or proving, key custody, or signing ceremonies
- WHEN adapter admission runs
- THEN Animus MUST return a typed unsupported-surface outcome
- AND it MUST NOT silently widen the local CLI adapter's authority or compatibility claim.

### Requirement: Pijul adapter policy is Nickel-authored and revalidated

r[agent_harness_runtime.pijul_adapter.profile] Human-authored Pijul adapter policy MUST use typed Nickel contracts for exact cohort identity, read-only versus protected-mutation operation classes, named log/diff/record/path/channel/diagnostic/resource bounds, channel allowlists, identity and signature-status handling, projection targets, receipt fields, and required non-claims, and generated runtime data MUST be deterministically projected and revalidated by Rust before use.

#### Scenario: Reviewed policy admits a bounded request

r[agent_harness_runtime.pijul_adapter.profile.valid]
- GIVEN the checked profile names the exact CLI cohort, an admitted operation class, finite named bounds, a channel allowlist, and explicit content/redaction policy
- WHEN Rust revalidates the generated profile and a request
- THEN it MUST produce a deterministic admission decision and BLAKE3 policy identity
- AND no host CLI effect may occur before that decision.

#### Scenario: Policy contains drift or unknown semantics

r[agent_harness_runtime.pijul_adapter.profile.drift]
- GIVEN generated policy is stale, names a different CLI release or output format, admits an unknown operation or channel, omits a required bound, admits a network or identity surface, or weakens a required non-claim
- WHEN profile validation runs
- THEN validation MUST fail closed with a typed issue
- AND neither CLI invocation nor observation projection may begin.

### Requirement: Pijul operations cross an injected host service

r[agent_harness_runtime.pijul_adapter.service] Pijul CLI operations MUST cross an implementation-free typed service using explicit repository paths, operation descriptors, and bounded request and result envelopes over an explicitly supplied bounded executor; reusable logic MUST NOT discover executables, search `PATH`, scan ancestor directories for `.pijul`, inspect environment variables, open databases, manage identity keys, spawn processes, or fall back to ambient VCS tooling.

#### Scenario: Injected service returns a bounded result

r[agent_harness_runtime.pijul_adapter.service.injected]
- GIVEN a request passed pure policy admission and the host supplied an exact executable identity plus a compatible injected service
- WHEN an operation is requested
- THEN the host adapter MAY send bounded argv through the service and return a bounded result envelope with exit-status class, stdout, and stderr
- AND raw output, repository contents, and CLI diagnostics MUST remain transient host data until pure validation selects an allowed projection.

#### Scenario: Service is absent or cohort-mismatched

r[agent_harness_runtime.pijul_adapter.service.unavailable]
- GIVEN no compatible service is injected, or the supplied executable identity differs from the admitted cohort
- WHEN an operation is requested
- THEN Animus MUST return a typed unavailable or cohort-mismatch outcome before process or filesystem effects
- AND it MUST NOT search `PATH`, install a package, open a `.pijul` directory, or substitute another VCS tool.

### Requirement: Pijul machine outputs are revalidated by a pure core

r[agent_harness_runtime.pijul_adapter.validation] Animus MUST revalidate CLI results with pure deterministic Rust logic against the exact output contract, including base32 BLAKE3 change-hash syntax and length, unambiguous hash prefixes, channel-name policy, state-identifier shape, output-envelope fields, conflict-status classes, exit-status mapping, record ordering, and configured collection and body bounds.

#### Scenario: Valid CLI output is accepted

r[agent_harness_runtime.pijul_adapter.validation.valid]
- GIVEN a bounded result contains well-formed change hashes, admitted channels, known envelope fields, coherent state identifiers, and only policy-admitted status classes
- WHEN pure validation runs
- THEN it MUST return typed validated observations, bounded change/conflict/file counts, and projection inputs
- AND repeated validation of identical inputs and policy MUST return the same result.

#### Scenario: Malformed or ambiguous output fails closed

r[agent_harness_runtime.pijul_adapter.validation.invalid]
- GIVEN a result has an unknown field or status class, malformed or wrong-length hash, ambiguous hash prefix, disallowed channel name, malformed state identifier, non-zero exit mapped as success, over-limit output, or prompt-like interactive output
- WHEN pure validation runs
- THEN validation MUST return the matching typed failure without echoing sensitive content
- AND no checkpoint, snapshot, provenance, workspace, persistence, receipt, or model-visible projection may be emitted.

### Requirement: Validated observations project through existing VCS-neutral owners

r[agent_harness_runtime.pijul_adapter.projection] Validated Pijul observations MUST project deterministically into existing Animus checkpoint DTOs, snapshot-ledger records, provenance observations, and isolated-workspace descriptors under explicit content policy, caller-supplied repository identity, and domain-separated BLAKE3 projection identities, while existing rollback, snapshot, provenance, workspace, persistence, and receipt owners remain authoritative.

#### Scenario: Admitted observations become Animus records

r[agent_harness_runtime.pijul_adapter.projection.valid]
- GIVEN validated observations, an admitted repository identity, and policy that admits selected metadata
- WHEN pure projection runs
- THEN channel state identifiers MUST map to snapshot identities, change hashes MUST map to opaque checkpoint IDs, and channel names MUST map to workspace labels in deterministic order with explicit omission and redaction state
- AND change timestamps and authorship MUST remain descriptive rather than becoming identity, freshness, causality, or authority evidence.

#### Scenario: Mutation is requested without grant or confirmation

r[agent_harness_runtime.pijul_adapter.projection.protected_mutations]
- GIVEN a plan names a protected-mutation operation such as `record`, `unrecord`, `apply`, `fork`, `channel switch`, or `reset`
- WHEN policy lacks an explicit grant or host confirmation is missing
- THEN Animus MUST return a typed denied or unconfirmed outcome
- AND no CLI invocation, automatic recording, or background snapshot behavior may be requested by reusable logic.

### Requirement: Pijul adapter evidence is metadata-only

r[agent_harness_runtime.pijul_adapter.evidence] Pijul adapter receipts MUST contain only safe repository class, cohort/policy/executor identities, operation class, status, bounded change/conflict/file counts, confirmation state, omission/redaction/truncation classes, projection target, required non-claims, and a domain-separated BLAKE3 receipt identity, and MUST exclude diffs, file bodies, change descriptions, author names or emails, key material or fingerprints, absolute repository paths, raw CLI output, credentials, and unapproved content-derived fingerprints.

#### Scenario: Safe receipt records bounded adapter facts

r[agent_harness_runtime.pijul_adapter.evidence.safe]
- GIVEN an operation completed under the exact reviewed cohort and policy
- WHEN receipt projection runs
- THEN the receipt MUST bind the cohort, policy, executor, operation class, status, bounded counts, confirmation and redaction state, and required non-claims
- AND it MUST describe only bounded processing of CLI output rather than repository integrity, content correctness, merge safety, authorship, signature validity, or release truth.

#### Scenario: Receipt draft contains sensitive or overclaiming material

r[agent_harness_runtime.pijul_adapter.evidence.rejects_leaks]
- GIVEN a receipt draft includes raw diffs, file bodies, change descriptions, author details, key material, private paths, unbounded CLI output, an unapproved content hash, or wording that claims merge correctness, content correctness, authentic authorship, or verified signatures
- WHEN receipt validation runs
- THEN validation MUST reject the draft with a bounded typed issue without echoing the material
- AND the rejected receipt MUST NOT contribute to lifecycle, completion, provenance, or release evidence.

### Requirement: Pijul adapter adoption has positive and negative evidence

r[agent_harness_runtime.pijul_adapter.verification] The adapter MUST include positive and negative tests for exact cohort/profile binding, every admitted query operation, protected-mutation denial without grant or confirmation, service injection, CLI protocol parsing, hash and channel validation, conflict-status classes, bounds, deterministic projection, content redaction defaults, receipt safety, excluded surfaces, and default dependency/source-shape isolation.

#### Scenario: Deterministic fixture evidence is ready for archive

r[agent_harness_runtime.pijul_adapter.verification.closeout]
- GIVEN maintainers intend to sync and archive the Pijul adapter change
- WHEN closeout validation runs
- THEN focused adapter, runtime, host, Nickel, CLI-parity, no-std/Wasm, component/facade, composition, safe-receipt, Cairn, and relevant Nix checks MUST pass with both positive and negative fixtures
- AND live repositories, network operations, identity custody, automatic recording, libpijul linking, content correctness, merge-safety promotion, production readiness, and release eligibility MUST remain explicit non-claims rather than inferred evidence.
