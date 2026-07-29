## ADDED Requirements

### Requirement: Authority-free secret-resolution contract

r[agent_harness_runtime.secret_resolution.contract] Animus MUST define an authority-free secret-resolution contract whose requests identify an admitted resolver, manifest, profile, scope, declaration, allowed provider schemes, safe reason, destination class, policy identity, and explicit bounds without containing plaintext values or granting provider, filesystem, process, environment, network, clock, or persistence authority.

#### Scenario: Valid request produces an effect plan

r[agent_harness_runtime.secret_resolution.contract.admitted]
- GIVEN a typed request references a configured resolver, explicit profile and scope, declared key, admitted purpose and destination, supported capabilities, and positive bounds
- WHEN the pure admission core evaluates explicit policy and availability facts
- THEN it MUST produce one bounded host-service plan containing only safe reference and configuration metadata
- AND it MUST NOT resolve a value, inspect ambient configuration, open a file, spawn a process, contact a provider, or mutate runtime state.

#### Scenario: Missing or unsupported service fails closed

r[agent_harness_runtime.secret_resolution.contract.unavailable]
- GIVEN a request is malformed, denied, unsupported, unbounded, selects a disallowed provider scheme, or requires an absent host service
- WHEN admission runs
- THEN it MUST return a stable typed failure before the dependent model, tool, process, network, file, or workflow effect
- AND it MUST NOT discover an environment variable, provider, manifest, encrypted file, executable, or credential as a fallback.

### Requirement: Secret material remains ephemeral

r[agent_harness_runtime.secret_resolution.material] Resolved secret material MUST cross only the injected host-service delivery boundary in a non-serializable, non-debuggable, non-cloneable, and non-hashable ephemeral value. It MUST NOT enter reusable runtime state, events, receipts, caches, prompts, logs, diagnostics, or persistence.

#### Scenario: One value reaches one admitted destination

r[agent_harness_runtime.secret_resolution.material.deliver]
- GIVEN the host adapter resolves the exact declaration in an admitted plan
- WHEN it delivers the value to the caller-owned execution boundary
- THEN the caller MUST receive only that declaration's ephemeral material for the admitted destination
- AND the adapter MUST drop provider documents and temporary buffers after the bounded delivery lifetime.

#### Scenario: Persistable secret representation is rejected

r[agent_harness_runtime.secret_resolution.material.reject_persistence]
- GIVEN code or a fixture attempts to serialize, debug, clone, display, content-hash, cache, or persist resolved secret bytes
- WHEN contract and conformance validation runs
- THEN the attempt MUST be impossible through the public material type or rejected with a stable leak class
- AND no value-derived digest MAY substitute for plaintext in durable evidence.

### Requirement: Typed Nickel resolver policy

r[agent_harness_runtime.secret_resolution.policy] Resolver kinds, manifest, profile, scope, provider-scheme, declaration, reason, destination, capability, count, byte, deadline, audit, and required non-claim fields MUST be authored in a typed Nickel policy with a deterministic checked runtime projection.

#### Scenario: Valid policy projects deterministically

r[agent_harness_runtime.secret_resolution.policy.valid]
- GIVEN a policy declares unique resolver ids, the closed SecretSpec host kind, positive named bounds, explicit profiles and scopes, allowed provider schemes, and no values
- WHEN policy export and freshness checks run
- THEN the generated runtime projection MUST match the reviewed Nickel source exactly
- AND pure admission MUST consume the projection without evaluating Nickel at runtime.

#### Scenario: Plaintext or ambient policy is rejected

r[agent_harness_runtime.secret_resolution.policy.invalid]
- GIVEN a policy contains plaintext, a private key, a bearer token, provider credentials, unrestricted inherited environment, ambient manifest discovery, unknown resolver kind, zero or excessive bound, or a weakened non-claim
- WHEN Nickel and Rust policy validation runs
- THEN validation MUST fail before any host resolver is constructed.

### Requirement: Optional SecretSpec host adapter

r[agent_harness_runtime.secret_resolution.secretspec] An opt-in `std` host adapter MUST use SecretSpec `v0.17.0` at upstream tag commit `a8794e46ec9664a0e1a3869cc3105d0853937e48` to resolve one admitted declaration from an explicit manifest, profile, scope, and provider policy. Provider access MUST remain outside reusable Animus cores.

#### Scenario: Declared key resolves without global injection

r[agent_harness_runtime.secret_resolution.secretspec.resolve]
- GIVEN an explicit SecretSpec manifest, selected profile and scope, allowed provider scheme, and declared key
- WHEN the injected SecretSpec adapter executes an admitted plan
- THEN it MUST deliver only that key through the ephemeral material boundary
- AND it MUST NOT perform whole-profile environment injection, generation, migration, import, provider login, write operations, or plaintext caching.

#### Scenario: Provider failure remains safe

r[agent_harness_runtime.secret_resolution.secretspec.failure]
- GIVEN the manifest, profile, scope, provider, declaration, provider credentials, executable, or provider operation is missing, invalid, denied, unavailable, timed out, cancelled, or oversized
- WHEN the adapter handles the outcome
- THEN it MUST return a fixed typed failure without arbitrary provider text or secret material
- AND the dependent effect MUST NOT run.

#### Scenario: SecretSpec audit is advisory

r[agent_harness_runtime.secret_resolution.secretspec.audit]
- GIVEN SecretSpec records metadata-only audit events but its audit sink is capped, truncating, unavailable, or unwritable
- WHEN Animus constructs resolution evidence
- THEN Animus MAY retain safe correlation metadata but MUST NOT treat the local audit as complete or authoritative
- AND audit write failure MUST NOT be rewritten as proof that access did or did not occur.

### Requirement: SOPS runs through the bounded SecretSpec worker

r[agent_harness_runtime.secret_resolution.sops] Animus MUST access SOPS-encrypted values through the pinned SecretSpec Rust SDK provider only. Because that provider can launch the `sops` executable, resolution MUST run inside an Animus-owned worker with explicit request, deadline, output, cancellation, and process-tree teardown bounds.

#### Scenario: SOPS declaration resolves inside bounds

r[agent_harness_runtime.secret_resolution.sops.resolve]
- GIVEN an allowed SecretSpec SOPS profile, a valid encrypted provider file, valid provider credentials, and an admitted declaration
- WHEN the bounded worker executes SDK resolution
- THEN it MUST return only the selected value through the private bounded channel
- AND it MUST retain no decrypted document in diagnostics, receipts, events, caches, or runtime state.

#### Scenario: Unsafe or uncertain provider execution fails closed

r[agent_harness_runtime.secret_resolution.sops.failure]
- GIVEN SOPS input is malformed, the executable is missing, provider credentials are unavailable, output is oversized, execution times out, cancellation occurs, the child is signalled, exit is nonzero, or teardown is uncertain
- WHEN the bounded worker handles resolution
- THEN it MUST terminate the owned process tree and return a stable redaction-safe failure before delivery
- AND it MUST NOT use partial output or fall back to a direct SOPS parser, ambient file, executable, environment, or plaintext value.

### Requirement: Native SecretSpec SOPS cohort is conformance gated

r[agent_harness_runtime.secret_resolution.sops_compatibility] Animus MUST enable the native SecretSpec SOPS provider only for the exact pinned released cohort after provider identity, address, read behavior, bounded-worker containment, failure, and plaintext-redaction compatibility fixtures pass.

#### Scenario: Released compatible provider is admitted

r[agent_harness_runtime.secret_resolution.sops_compatibility.released]
- GIVEN SecretSpec `v0.17.0` exposes the reviewed native SOPS provider and every required fixture passes
- WHEN provider configuration is validated
- THEN Animus MAY admit the `sops` scheme with the source identity and known child-process limitation recorded
- AND every process-capable resolution MUST still use the bounded worker.

#### Scenario: Unreviewed provider cohort stays unsupported

r[agent_harness_runtime.secret_resolution.sops_compatibility.proposed]
- GIVEN SOPS behavior comes from another release, branch, pull request, feature set, or incompatible API
- WHEN provider configuration is validated
- THEN validation MUST return `unsupported_provider_cohort`
- AND it MUST NOT silently route through a direct SOPS mechanism or another SecretSpec provider.

### Requirement: Secret-resolution receipts are value free

r[agent_harness_runtime.secret_resolution.receipts] Resolution admission and observation receipts MUST contain only safe resolver, manifest, profile, scope, declaration, provider-scheme, policy, reason, destination, bound, capability, outcome, count, and redaction facts. BLAKE3 MUST apply only to public non-secret reference and configuration bytes.

#### Scenario: Successful receipt omits value and provider text

r[agent_harness_runtime.secret_resolution.receipts.safe]
- GIVEN a SecretSpec provider resolves and delivers a value
- WHEN the pure receipt core constructs evidence
- THEN the receipt MUST identify the admitted request and completed delivery class without the value, value hash, provider credential, full host path, decrypted document, stdout, stderr, or arbitrary provider text.

#### Scenario: Leak candidate is rejected

r[agent_harness_runtime.secret_resolution.receipts.reject_leak]
- GIVEN a receipt candidate contains plaintext or value-derived bytes, a private key, bearer token, provider payload, decrypted document, full path, process output, environment value, or unrestricted diagnostic text
- WHEN receipt validation runs
- THEN it MUST fail with a stable leak class and MUST NOT persist the candidate.

### Requirement: Secret-resolution authority remains host owned

r[agent_harness_runtime.secret_resolution.authority] SecretSpec configuration discovery, provider credentials, KMS, age, or PGP keys, filesystem and process access, network calls, clocks, cancellation, cleanup, material delivery, persistence, rotation, revocation, access control, and durable audit retention MUST remain caller-owned host behavior outside reusable Animus cores.

#### Scenario: Optional features preserve existing boundaries

r[agent_harness_runtime.secret_resolution.authority.isolation]
- GIVEN an Animus consumer does not select the secret-resolution feature or inject the service
- WHEN default, `no_std`, Wasm, core, adapter, runtime, Steel, TUI, and unrelated `std` builds run
- THEN they MUST acquire no SecretSpec SDK, SOPS executable, provider client, manifest search, credential read, filesystem, process, network effect, or ambient fallback.

#### Scenario: Resolution claims remain bounded

r[agent_harness_runtime.secret_resolution.authority.non_claim]
- GIVEN a resolution and its safe receipt complete
- WHEN the result is documented or consumed
- THEN it MUST NOT claim value correctness or freshness, global authorization, SOPS or key-service correctness, SecretSpec policy sufficiency, audit completeness, arbitrary confidentiality, host-compromise resistance, downstream effect uniqueness, release eligibility, or production readiness.

### Requirement: Secret-resolution verification includes failures

r[agent_harness_runtime.secret_resolution.verification] Adoption MUST pair positive SecretSpec environment, systemd credential, and SOPS delivery fixtures with negative policy, availability, capability, scope, provider, declaration, bound, worker, process, material-leak, receipt-leak, audit, feature-isolation, and unreviewed-cohort fixtures.

#### Scenario: Closeout evidence is complete

r[agent_harness_runtime.secret_resolution.verification.closeout]
- GIVEN maintainers intend to archive the change and unblock a downstream consumer
- WHEN closeout validation runs
- THEN evidence MUST name the exact Animus and SecretSpec cohorts, focused positive and negative tests, Nickel freshness, `no_std` and Wasm isolation, adapter, component, and contract rails, Cairn validation and gates, and the immutable consumable Animus revision
- AND every skipped rail MUST include an exact blocker and next-best check.
