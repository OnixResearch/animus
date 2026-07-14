# Optional Iroh agent communication adapter

`animus-std` provides a feature-gated host seam for Iroh-backed agent communication behind the non-default `iroh-host` feature. Reusable `animus-core`, `animus-adapters`, and `animus-runtime` do not construct Iroh endpoints, open network sockets, read tickets, store private keys, or perform service discovery.

The public seam is split into a pure validation/projector core and a thin imperative shell:

- `IrohRouteDescriptor` validates safe node/topic labels, the stable `animus.agent-comms.v1` protocol label, a scope boundary, optional ticket/node-address BLAKE3 hash, remote-admission status, and a maximum frame size.
- The descriptor maps into a neutral `animus_adapters::AgentCommsRoute` with transport family `iroh`, a safe route hash, a derived policy-evidence hash, admission metadata, endpoint/ticket hash metadata, and no raw transport body.
- `project_iroh_frame`, `project_assertion_frame`, and `project_orchestration_event_frame` return neutral `AgentCommsFrame` metadata through the shared agent-comms frame validators.
- `iroh_route_receipt` and `iroh_frame_receipt` return neutral `AgentCommsReceipt` values with safe route/frame hashes, admission/status classes, counts, redaction evidence, and no ticket, relay, key, prompt, provider, file, tool, model, or log bodies.
- `admit_iroh_scope` delegates to the neutral agent-comms admission/scope helper before effects.
- `IrohAgentCommsAdapter<T>` wraps caller-owned `IrohAgentTransport` and implements `animus_adapters::AdapterService` for bounded assertion publish/query/event flows.

Raw Iroh tickets, node-address bodies, relay URLs, private keys, authorization headers, prompts, provider payloads, credentials, file bodies, tool payloads, model outputs, and unbounded logs are rejected before transport effects. Hosts that want live Iroh networking must supply their own endpoint, ticket handling, authentication, relay selection, persistence, retry policy, and smoke-test evidence.

For Syndicate coordination, use the explicit [`iroh-syndicate-host` composition](iroh-syndicate-composition.md). Do not mix this direct assertion/event compatibility seam and Syndicate turn packets on one logical session.

Focused rail:

```sh
cargo test -p animus-std --features iroh-host iroh --locked
```
