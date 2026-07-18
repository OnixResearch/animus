# Bounded ACP stdio host

Animus provides an **optional, non-default, local-stdio-only** Agent Client Protocol host in `animus-acp-stdio-host`. Enable the root `acp-stdio-host` feature or depend on that crate explicitly. Default, reusable, no-std, and Wasm surfaces do not include the ACP dependency cohort or a live server entrypoint.

## Reviewed cohort and advertised subset

The reviewed protocol cohort is pinned exactly:

- `agent-client-protocol = 0.10.4`
- `agent-client-protocol-schema = 0.11.4`
- license: `Apache-2.0`
- protocol version: `1`
- stable SDK features only; the broad `unstable` feature is disabled

The advertised subset is initialize, text prompt, resource-link identity, new session, load session, ordered session updates, client permission requests, and cancellation. ACP MCP-server arguments fail closed because this host does not use them to widen tool or remote-server authority. Images, audio, embedded resources, terminals, filesystem RPC, sockets, WebSocket, HTTP, TLS, endpoint discovery, provider discovery, credential discovery, implicit reconnect, retry, and network fallback are outside this profile.

Any cohort or advertised-feature change requires a separate review. The SDK supplies reviewed schema types and method names; it is **not** transport evidence. Animus does not use the SDK's unbounded connection loop as its production transport.

## Ownership map

| Direction / concern | Sole owner | ACP bridge responsibility |
| --- | --- | --- |
| JSON-RPC bytes to typed request | `animus-acp-stdio-host::{codec,transport}` | Bound newline-delimited frames, preserve request IDs, classify malformed/oversized/EOF/read/write/flush/broken-pipe failures. |
| Connection and correlation state | `animus-acp-stdio-host::core` | Pure initialize, ordering, terminal, and response-action reduction. |
| Transcript, queue, run, active leaf, durable commits | `animus_std::coding_session_host::CodingSessionHost` | Delegate typed new/load/prompt/cancel commands; never maintain a second transcript or run ledger. |
| Durable load, replay, repair | durable session store and coding-session host | Call the existing `resume` path before continuation; ACP does not bypass store validation or interrupted-tool repair. |
| Runtime authority profile | coding-session host plus injected authority service | Preserve the durable profile identity and require a fresh current-enforcement observation. Equality is continuity evidence, not enforcement evidence. |
| Tool policy, capability, sandbox, executor | existing injected coding-session services | ACP approval is an additional gate only. A client allow cannot override any existing denial or unavailability. |
| Runtime lifecycle facts | `animus_runtime::RuntimeEvent` | Project ordered bounded updates from shared events; do not mutate transcript state. |
| Headless and TUI views | their existing reducers | Consume the same shared event order independently. |
| Transport/session receipt | `animus-acp-stdio-host::receipt` | Emit counts, safe labels, BLAKE3 identities, cleanup status, redaction markers, and non-claims only. |

## Functional core and imperative shell

`model.rs`, `core.rs`, `permission.rs`, `events.rs`, and `receipt.rs` are deterministic cores. They do not read files, inspect the environment, contact providers, launch processes, open sockets, or access stdio.

`codec.rs` maps the exact reviewed schema cohort to Animus-owned typed envelopes. Workspace paths and resource URIs are domain-separated BLAKE3 identities before they enter reducer state or evidence. Unsupported content fails closed.

`facade.rs` is the explicit bridge to one coding-session host. The factory owns fresh/open durable-session construction, the prompt resolver owns resource resolution, and the admission service supplies current policy, permission mode, authorization, capability, sandbox, authority-profile, and executor observations.

`transport.rs` is the thin imperative shell. It owns bounded line framing, frame and permission counts, write/flush behavior, cancellation/deadline observation, safe stderr diagnostics, and cleanup. `run_acp_stdio` returns an `AcpTransportReport`; failures expose the same report through `AcpTransportError::report`. Cleanup failure is recorded alongside, rather than replacing, the primary transport failure.

## Permission rule

The client decision is evaluated only after current host-owned intersections are observed. The pure permission planner produces one of:

- `pre_approved`: all host checks admit and no client confirmation is required;
- `client_decision_required`: all host checks admit and the client must choose a bounded option;
- `denied`: at least one existing check denies or is unavailable.

Absent, malformed, stale, cancelled, EOF, or unknown permission responses fail closed. Client `allow_once` or `allow_always` has no authority to widen policy, permission mode, authorization, capability, sandbox, runtime profile, or executor admission.

## Event and evidence safety

Visible final assistant text may be projected as an ACP message chunk. Tool names, safe correlation identities, queue counts, durable boundaries, authority status, cancellation, and terminal metadata may be projected as bounded metadata updates. Hidden reasoning, raw provider requests, raw tool arguments/results, credentials, environment values, physical paths, resource URIs, and raw diagnostics are omitted.

Receipts are domain-separated BLAKE3-bound metadata. Receipt projection rejects any supplied raw frame, prompt, tool argument/output, provider payload, credential, environment value, physical path, hidden reasoning, or raw diagnostic. A successful receipt cannot also contain a failure, and cleanup status remains explicit.

## Verification

Run:

```text
scripts/cairn/check-acp-stdio-host --root .
nix build .#checks.x86_64-linux.acp-stdio-host --no-link -L
```

The focused rail checks the exact dependency cohort, production codec/reducer/facade/transport fixtures, additive permissions, durable resume seams, shared headless/TUI/ACP event ordering, transport failures, safe receipts, and dependency/source-shape isolation.

## Non-claims

This rail does **not** prove arbitrary-editor interoperability, remote transport security, MCP server hosting, plugin hosting, terminal multiplexing, live provider correctness, tool correctness, durable-store kernel/filesystem/power-loss correctness, sandbox containment, runtime-authority oracle soundness, whole-product correctness, production readiness, or release eligibility. Live editor smoke evidence, if performed, is recorded separately and does not widen these claims.
