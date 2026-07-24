## ADDED Requirements

### Requirement: Authority-free secret-resolution contract

r[agent_harness_runtime.secret_resolution.contract] Animus MUST define an authority-free secret-resolution contract whose requests identify an admitted resolver, declaration or encrypted-document reference, profile, key, safe reason, destination class, policy identity, and explicit bounds without containing plaintext values or granting provider, filesystem, process, environment, network, clock, or persistence authority.

#### Scenario: Valid request produces an effect plan

r[agent_harness_runtime.secret_resolution.contract.admitted]
- GIVEN a typed request references a configured resolver, declared profile and key, admitted purpose and destination, supported capabilities, and positive bounds
- WHEN the pure admission core evaluates explicit policy and availability facts
- THEN it MUST produce one bounded host-service plan containing only safe reference/configuration metadata
- AND it MUST NOT resolve a value, inspect ambient configuration, open a file, spawn a process, contact a provider, or mutate runtime state.

#### Scenario: Missing or unsupported service fails closed

r[agent_harness_runtime.secret_resolution.contract.unavailable]
- GIVEN a request is malformed, denied, unsupported, unbounded, or requires a host secret-resolution service that is absent or unavailable
- WHEN admission runs
- THEN it MUST return a stable typed failure before the dependent model, tool, process, network, file, or workflow effect
- AND it MUST NOT discover an environment variable, provider, manifest, encrypted file, executable, or credential as a fallback.

### Requirement: Secret material remains ephemeral

r[agent_harness_runtime.secret_resolution.material] Resolved secret material MUST cross only the injected host-service delivery boundary in a non-serializable, non-debuggable, non-cloneable, non-hashable ephemeral value and MUST NOT enter reusable runtime state, events, receipts, caches, prompts, logs, diagnostics, or persistence.

#### Scenario: One value reaches one admitted destination

r[agent_harness_runtime.secret_resolution.material.deliver]
- GIVEN a host adapter resolves the exact key in an admitted plan
- WHEN it delivers the value to the caller-owned execution boundary
- THEN the caller MUST receive only that key's ephemeral material for the admitted destination
- AND the adapter MUST drop provider documents and temporary path material after the bounded delivery lifetime.

#### Scenario: Persistable secret representation is rejected

r[agent_harness_runtime.secret_resolution.material.reject_persistence]
- GIVEN code or a fixture attempts to serialize, debug, clone, compare, display, content-hash, cache, or persist resolved secret bytes
- WHEN contract and conformance validation runs
- THEN the attempt MUST be impossible through the public material type or rejected with a stable leak class
- AND no value-derived digest MAY substitute for the plaintext in durable evidence.

### Requirement: Typed Nickel resolver policy

r[agent_harness_runtime.secret_resolution.policy] Resolver kinds, project/profile/provider/key identities, reasons, destinations, document formats, lexical paths, allowed environment-key names, capability requirements, count/byte/deadline bounds, audit posture, and required non-claims MUST be authored in a typed Nickel policy with a deterministic checked runtime projection.

#### Scenario: Valid policy projects deterministically

r[agent_harness_runtime.secret_resolution.policy.valid]
- GIVEN a policy declares unique resolver ids, closed supported kinds, positive named bounds, safe relative paths, explicit environment-key names, and no values
- WHEN policy export and freshness checks run
- THEN the generated runtime projection MUST match the reviewed Nickel source exactly
- AND pure admission MUST consume the projection without evaluating Nickel at runtime.

#### Scenario: Plaintext or ambient policy is rejected

r[agent_harness_runtime.secret_resolution.policy.invalid]
- GIVEN a policy contains a plaintext secret, private key, bearer token, unrestricted inherited environment, absolute or parent-traversing path, unknown resolver kind, zero or excessive bound, or weakened non-claim
- WHEN Nickel and Rust policy validation runs
- THEN validation MUST fail before any host resolver is constructed.

### Requirement: Optional SecretSpec host adapter

r[agent_harness_runtime.secret_resolution.secretspec] An opt-in std host adapter MUST use an exact reviewed released SecretSpec SDK cohort to resolve one admitted key from an explicit manifest, profile, and provider while keeping provider clients, machine configuration, credentials, filesystem/network effects, temporary files, and audit writes outside reusable Animus crates.

#### Scenario: Declared key resolves without global injection

r[agent_harness_runtime.secret_resolution.secretspec.resolve]
- GIVEN an explicit SecretSpec manifest and selected profile/provider declare the requested key and the host provider returns it
- WHEN the injected SecretSpec adapter executes an admitted plan
- THEN it MUST deliver only that key through the ephemeral material boundary
- AND it MUST NOT call whole-profile environment injection, generation, migration/import, provider login, or persist the resolved value.

#### Scenario: Provider failure remains safe

r[agent_harness_runtime.secret_resolution.secretspec.failure]
- GIVEN the manifest, profile, provider, reference coordinates, provider credentials, required key, temporary path materialization, or provider operation is missing, invalid, denied, or unavailable
- WHEN the adapter handles the outcome
- THEN it MUST return a fixed typed failure without arbitrary provider text or secret material
- AND the dependent effect MUST NOT run.

#### Scenario: SecretSpec audit is advisory

r[agent_harness_runtime.secret_resolution.secretspec.audit]
- GIVEN SecretSpec records local metadata-only audit events but its audit sink is capped, truncating, unavailable, or unwritable
- WHEN Animus constructs resolution evidence
- THEN Animus MAY retain safe correlation metadata but MUST NOT treat the local audit as complete or authoritative
- AND audit write failure MUST NOT be rewritten as proof that access did or did not occur.

### Requirement: Optional bounded SOPS host adapter

r[agent_harness_runtime.secret_resolution.sops] An opt-in std host adapter MUST resolve an admitted key from a SOPS-encrypted file through an exact configured executable, capability-confined encrypted-file access, explicit argv and environment-key names, positive process and output bounds, duplicate-rejecting in-memory parsing, and no decrypted-file persistence.

#### Scenario: Encrypted key resolves inside bounds

r[agent_harness_runtime.secret_resolution.sops.resolve]
- GIVEN one supported SOPS document under an admitted root contains the requested lexical key and the configured executable decrypts it within every bound
- WHEN the injected SOPS adapter executes the admitted plan
- THEN it MUST extract and deliver only the selected value through the ephemeral material boundary
- AND it MUST retain no decrypted document on disk, in diagnostics, receipts, events, caches, or runtime state.

#### Scenario: Unsafe or uncertain decryption fails closed

r[agent_harness_runtime.secret_resolution.sops.failure]
- GIVEN the root or path escapes, the executable is unavailable or ambiguous, a format or key path is unsupported, output is malformed, duplicated, missing, oversized, truncated, timed out, cancelled, signalled, nonzero, stderr-sensitive, or cleanup remains uncertain
- WHEN SOPS resolution runs
- THEN the adapter MUST return a stable redaction-safe failure before delivery or dependent effects
- AND it MUST NOT fall back to an ambient file, executable, environment, plaintext value, or weaker parser.

### Requirement: Native SecretSpec SOPS compatibility is release gated

r[agent_harness_runtime.secret_resolution.sops_compatibility] Animus MUST enable a combined native SecretSpec SOPS provider mode only after an exact released SecretSpec cohort passes provider identity, address, read behavior, failure, and plaintext/redaction compatibility fixtures; unreleased documentation, branches, or pull requests MUST NOT satisfy this gate.

#### Scenario: Released compatible provider is admitted

r[agent_harness_runtime.secret_resolution.sops_compatibility.released]
- GIVEN a pinned released SecretSpec SDK exposes a native SOPS provider and passes every required compatibility fixture
- WHEN combined-mode configuration is validated
- THEN Animus MAY admit that exact cohort with its provider identity and limitations recorded
- AND direct SOPS mode MUST remain a separately identified mechanism.

#### Scenario: Proposed provider stays unsupported

r[agent_harness_runtime.secret_resolution.sops_compatibility.proposed]
- GIVEN native SecretSpec SOPS support exists only in documentation, an issue, a pull request, an unreleased branch, or an incompatible release
- WHEN combined-mode configuration is validated
- THEN validation MUST return `unsupported_provider_cohort`
- AND it MUST NOT silently route through direct SOPS or another SecretSpec provider.

### Requirement: Secret-resolution receipts are value free

r[agent_harness_runtime.secret_resolution.receipts] Resolution admission and observation receipts MUST contain only safe resolver/declaration/reference/policy identities, reason and destination classes, configured bounds, capability and outcome classes, counts, and redaction evidence, with BLAKE3 applied only to non-secret reference/configuration bytes.

#### Scenario: Successful receipt omits value and provider text

r[agent_harness_runtime.secret_resolution.receipts.safe]
- GIVEN a SecretSpec or SOPS adapter resolves and delivers a value
- WHEN the pure receipt core constructs evidence
- THEN the receipt MUST identify the admitted request and completed delivery class without the value, value hash, provider credential, full host path, decrypted document, stdout, stderr, or arbitrary provider text.

#### Scenario: Leak candidate is rejected

r[agent_harness_runtime.secret_resolution.receipts.reject_leak]
- GIVEN a receipt candidate contains plaintext or value-derived bytes, a private key, bearer token, provider payload, decrypted document, full path, process output, environment value, or unrestricted diagnostic text
- WHEN receipt validation runs
- THEN it MUST fail with a stable leak class and MUST NOT persist the candidate.

### Requirement: Secret-resolution authority remains host owned

r[agent_harness_runtime.secret_resolution.authority] SecretSpec/SOPS configuration discovery, provider credentials, KMS/age/PGP keys, filesystem and process access, network calls, clocks, cancellation, cleanup, material delivery, persistence, rotation, revocation, access control, and durable audit retention MUST remain caller-owned host behavior outside reusable Animus cores.

#### Scenario: Optional features preserve existing boundaries

r[agent_harness_runtime.secret_resolution.authority.isolation]
- GIVEN an Animus consumer does not select secret-resolution features or inject the service
- WHEN default, no-std, Wasm, core, adapter, runtime, Steel, TUI, and unrelated std builds run
- THEN they MUST acquire no SecretSpec SDK, SOPS executable, provider client, manifest search, credential read, filesystem/process/network effect, or new ambient fallback.

#### Scenario: Resolution claims remain bounded

r[agent_harness_runtime.secret_resolution.authority.non_claim]
- GIVEN a resolution and its safe receipt complete
- WHEN the result is documented or consumed
- THEN it MUST NOT claim value correctness or freshness, global authorization, SOPS or key-service correctness, SecretSpec policy sufficiency, audit completeness, arbitrary confidentiality, host-compromise resistance, downstream effect uniqueness, release eligibility, or production readiness.

### Requirement: Secret-resolution verification includes failures

r[agent_harness_runtime.secret_resolution.verification] Adoption MUST pair positive SecretSpec and SOPS delivery fixtures with negative policy, availability, capability, path, format, key, bound, process, parser, provider, material-leak, receipt-leak, audit, feature-isolation, and unreleased-cohort fixtures.

#### Scenario: Closeout evidence is complete

r[agent_harness_runtime.secret_resolution.verification.closeout]
- GIVEN maintainers intend to archive the secret-resolution change and unblock a downstream consumer
- WHEN closeout validation runs
- THEN evidence MUST name the exact dependency cohorts, focused positive and negative tests, Nickel freshness, no-std/Wasm isolation, adapter/component/contract rails, Cairn validation and gates, and immutable consumable Animus revision
- AND every skipped rail MUST include an exact blocker and next-best check.
