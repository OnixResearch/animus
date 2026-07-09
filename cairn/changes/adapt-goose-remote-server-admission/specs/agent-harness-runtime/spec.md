# agent-harness-runtime Change Spec: Goose remote server admission

## Requirements

### Requirement: Remote backend descriptors

r[agent_harness_runtime.goose_remote_server_admission.descriptors] Animus MUST define remote backend descriptors for backend identity, endpoint identity or hash, transport kind, protocol role, platform role, TLS requirement, certificate fingerprint policy, secret source label, workspace or session scope, capability ceilings, and provenance.

#### Scenario: Pinned HTTPS backend descriptor is valid

r[agent_harness_runtime.goose_remote_server_admission.descriptors.pinned]
- GIVEN a host provides a remote backend descriptor for an HTTPS headless-control endpoint with a fingerprint pin and secret source label
- WHEN descriptor validation runs
- THEN validation accepts endpoint identity, TLS requirement, fingerprint policy, protocol role, capability ceilings, scope, and provenance
- AND the descriptor does not include raw shared secrets, headers, certificate bodies, or transport handles.

### Requirement: Host-supplied handshake summaries

r[agent_harness_runtime.goose_remote_server_admission.handshake] Remote backend admission MUST consume host-supplied handshake summaries for observed TLS state, certificate fingerprint status, auth outcome, protocol version, accepted features, remote audience, and safe failure class, and MUST NOT open network connections or verify certificates inside reusable crates.

#### Scenario: Network probe remains host-owned

r[agent_harness_runtime.goose_remote_server_admission.handshake.host_owned]
- GIVEN a descriptor names a remote endpoint and the host has not supplied a handshake summary
- WHEN reusable admission validation runs
- THEN validation returns a missing-handshake diagnostic
- AND no socket, TLS handshake, HTTP request, ACP stream, or credential lookup is attempted by reusable logic.

### Requirement: Pure remote admission validation

r[agent_harness_runtime.goose_remote_server_admission.admission] Remote backend admission MUST be pure deterministic validation over descriptors, handshake summaries, policy, credential-source availability, certificate pin state, workspace or session scope, capability ceilings, audience, and fallback policy.

#### Scenario: Matching pin and valid secret source admits backend

r[agent_harness_runtime.goose_remote_server_admission.admission.pinned_success]
- GIVEN a remote descriptor requires TLS and a fingerprint pin
- AND the host-supplied handshake summary reports TLS enabled, matching fingerprint, accepted protocol version, available secret source, authorized auth outcome, and matching audience
- WHEN admission validation runs
- THEN the backend is admitted for the declared scope and capability ceilings
- AND the result records no provider, tool, or session dispatch side effect.

### Requirement: Remote admission fails closed

r[agent_harness_runtime.goose_remote_server_admission.fail_closed] Remote backend admission MUST fail closed for missing TLS when TLS is required, fingerprint mismatch, trust-on-first-use pending without approval, secret source unavailable, unauthorized auth outcome, unsupported protocol version, stale pin state, scope denial, audience mismatch, capability widening, and fallback attempts not explicitly admitted by policy.

#### Scenario: Fingerprint mismatch blocks connection use

r[agent_harness_runtime.goose_remote_server_admission.fail_closed.fingerprint]
- GIVEN a remote descriptor has a pinned certificate fingerprint
- AND the host-supplied handshake summary reports a different fingerprint
- WHEN admission validation runs
- THEN the backend is denied with a fingerprint-mismatch diagnostic
- AND no fallback backend, local provider, or remote session is selected implicitly.

### Requirement: Safe remote admission receipts

r[agent_harness_runtime.goose_remote_server_admission.receipts] Remote descriptor validation, handshake admission, TLS pinning, auth result, denial, unavailable backend, and fallback-denial outcomes MUST produce safe receipts with backend identity hash, endpoint identity class, TLS status, fingerprint status, auth status, protocol version class, audience hash, secret source class, scope label, capability ceiling labels, policy hash, denial class, and BLAKE3 receipt hash.

#### Scenario: Receipt rejects secret and endpoint leaks

r[agent_harness_runtime.goose_remote_server_admission.receipts.rejects_leaks]
- GIVEN a remote admission receipt candidate contains raw endpoint URLs, shared secrets, authorization headers, raw tokens, credential values, certificate bodies, provider payloads, prompt bodies, or full paths
- WHEN receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND durable remote-admission evidence does not echo the raw material.

### Requirement: Remote server admission verification

r[agent_harness_runtime.goose_remote_server_admission.verification] Remote server admission adoption MUST include positive pinned-TLS and approved trust-on-first-use fixtures plus negative HTTP, missing-handshake, fingerprint mismatch, TOFU-pending, secret-unavailable, unauthorized, unsupported-protocol, wrong-audience, capability-widening, fallback-denial, no ambient network, and receipt leak fixtures.

#### Scenario: Closeout evidence covers remote admission

r[agent_harness_runtime.goose_remote_server_admission.verification.closeout]
- GIVEN maintainers intend to archive the Goose remote server admission adaptation
- WHEN closeout validation runs
- THEN evidence names descriptor, handshake, admission, fail-closed, receipt, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
