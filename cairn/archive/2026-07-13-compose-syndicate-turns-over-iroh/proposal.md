## Why

Animus now has a bounded Syndicate turn adapter and a separate host-owned Iroh route seam, but callers still need to invent the bridge between them. Ad hoc composition can accidentally wrap a turn in the legacy redacted frame metadata model, split one turn across effects, skip route admission, or mix direct agent-comms semantics and Syndicate coordination on one session.

A narrow composition adapter should bind an admitted `animus.agent-comms.v1` Iroh route to the `syndicate.protocol.v1.animus-safe` packet contract while leaving endpoints, streams, tickets, keys, relays, retries, and discovery in a caller-owned session implementation.

## What Changes

- Add an explicit non-default `iroh-syndicate-host` composition feature that requires both existing host seams without adding an Iroh crate dependency. r[agent_harness_runtime.syndicate_iroh_bridge.feature]
- Add a caller-implemented Iroh Syndicate packet-session port and a bridge that implements `SyndicatePacketTransport` over one admitted Iroh route. r[agent_harness_runtime.syndicate_iroh_bridge.transport]
- Preserve one complete binary Syndicate turn packet per session effect and enforce the Iroh route's frame ceiling and exact DTO scope in both directions. r[agent_harness_runtime.syndicate_iroh_bridge.framing]
- Fail closed before session effects for denied routes and oversized outbound packets, return bounded errors for session failures, and leave Syndicate handle-state commit semantics in the existing protocol adapter. r[agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
- Add combined-feature positive and negative fake-session evidence, facade/catalog entries, and composition documentation. r[agent_harness_runtime.syndicate_iroh_bridge.verification]

## Impact

- **Rust**: `animus-std` gains one combined-feature module with a pure route/packet admission core and a thin caller-owned session shell; root `animus` re-exports the composition seam.
- **Features**: default, `iroh-host`, and `syndicate-host` remain independently usable; `iroh-syndicate-host` opts into their composition.
- **Transport**: live Iroh construction remains outside Animus. The new session port represents already-established, reliable, ordered, confidential, integral, authentic, replay-resistant packet effects.
- **Claims**: fake-session conformance does not establish production Iroh interoperability, network availability, peer identity correctness, relay behavior, retry safety, or full Syndicate capability support.
