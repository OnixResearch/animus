# Goose remote server admission

Animus adapts Goose-style remote server ergonomics as data-only admission contracts. Reusable crates validate descriptors, host-supplied handshake summaries, policy inputs, fail-closed outcomes, and safe receipts. Hosts own sockets, TLS probes, certificate stores, credential lookup, backend selection, fallback wiring, and persistence.

## Contract map

- `r[agent_harness_runtime.goose_remote_server_admission.descriptors]`: `RemoteServerDescriptorDraft` and `RemoteServerDescriptor` describe backend identity, endpoint identity hash, transport kind, protocol role, platform role, TLS requirement, fingerprint policy, safe secret-source label/class, scope, capability ceilings, expected audience hash, and provenance hash.
- `r[agent_harness_runtime.goose_remote_server_admission.handshake]`: `RemoteServerHandshakeSummaryDraft` and `RemoteServerHandshakeSummary` carry host-owned probe summaries for TLS state, fingerprint status, auth outcome, protocol version status, accepted feature labels, remote audience hash, and safe failure class.
- `r[agent_harness_runtime.goose_remote_server_admission.admission]`: `evaluate_remote_server_admission` is pure validation over descriptor, optional handshake, and `RemoteServerAdmissionPolicy` inputs.
- `r[agent_harness_runtime.goose_remote_server_admission.fail_closed]`: admission denies missing handshakes, missing TLS, fingerprint mismatch, TOFU pending without approval, secret unavailable, unauthorized auth, unsupported protocol, stale pins, scope denial, audience mismatch, capability widening, and fallback attempts.
- `r[agent_harness_runtime.goose_remote_server_admission.receipts]`: `RemoteServerAdmissionReceipt` renders metadata-only receipts with hashes, classes, scope, capability ceilings, policy hash, denial class, failure class, and a BLAKE3 receipt hash. Raw endpoints, secrets, headers, tokens, credential bodies, certificate bodies, provider payloads, prompt bodies, and full paths are rejected.

## Non-claims

This surface does not implement a TCP server, ACP transport, TLS stack, socket client, certificate store, background daemon, Goose Desktop setting, credential persistence, or automatic fallback. Those remain host adapter responsibilities that must pass their own admission and receipt rails.

## Focused verification

Run:

```sh
cargo test -p animus-adapters gateway --locked
```

The focused fixtures include positive pinned-TLS and approved-TOFU cases plus negative HTTP, missing-handshake, fingerprint mismatch, TOFU-pending, secret-unavailable, unauthorized, unsupported-protocol, wrong-audience, capability-widening, fallback-denial, raw-endpoint, and receipt-leak cases.
