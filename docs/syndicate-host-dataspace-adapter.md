# Optional Syndicate host dataspace adapter

`animus-std` provides feature-gated host seams for Syndicate-compatible dataspace coordination behind the non-default `syndicate-host` feature. Reusable `animus-core`, `animus-adapters`, and `animus-runtime` do not construct live Syndicate clients, require daemon availability, read credentials, or add network transport dependencies.

The original `SyndicateHostDataspaceAdapter<T>` remains the typed dataspace-service seam. Hosts that need Preserves network packets can additionally use the capability-free `SyndicateProtocolAdapter<T>` described in [syndicate-turn-protocol-adapter.md](syndicate-turn-protocol-adapter.md). The protocol adapter does not replace the typed service contract or supply a live transport.

The public seam is split into a pure projection core and a thin imperative shell:

- `project_observation` maps approved runtime, headless, tool lifecycle, background-work, session/provenance, agent, and subagent observations into bounded dataspace assertions.
- `build_safe_subscription` constructs raw-payload-free, same-workspace subscriptions.
- `admit_scoped_orchestration` checks cancellation, expiry, acknowledgement, and ownership-transfer events against one expected scope.
- `SyndicateHostDataspaceAdapter<T>` wraps caller-owned `SyndicateTransport` and implements `animus_adapters::AdapterService`.

Projection rejects raw prompt/provider/file/tool/model/log/credential markers and requires explicit redaction evidence for sensitive sources. Live smoke tests are opt-in and host-owned; default release evidence uses deterministic fake or memory transports.

Focused rail:

```sh
cargo test -p animus-std --features syndicate-host dataspace --locked
cargo test -p animus-std --features syndicate-host syndicate --locked
```
