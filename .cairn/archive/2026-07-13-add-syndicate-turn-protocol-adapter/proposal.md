## Why

Animus already exposes a Syndicate-inspired dataspace host seam and stable Preserves dataspace DTO records, but it does not encode or transport Syndicate Protocol turn packets. Hosts must currently invent their own wire mapping, which risks losing turn boundaries, assertion-handle lifecycle, scope validation, and the existing redaction guarantees.

A bounded adapter should map approved Animus dataspace DTOs onto the capability-free `A`, `R`, and `M` subset of Syndicate Protocol v1 turns, preserve the protocol's sequence boundaries, and delegate reliable authenticated byte transport to an injected host service. Full WireRef membranes, embedded capabilities, attenuation caveats, discovery, and live networking remain separate security work.

## What Changes

- Add a stable Preserves Syndicate-turn schema family and exact text/binary encoding for bounded `Turn = [TurnEvent ...]` packets containing `A`, `R`, and `M` events. r[agent_harness_runtime.syndicate_turn_adapter.wire]
- Project validated Animus assertions and orchestration events into capability-free turn bodies while revalidating decoded dataspace DTOs. r[agent_harness_runtime.syndicate_turn_adapter.projection]
- Add a non-default `syndicate-host` protocol adapter over a caller-owned reliable, ordered, authenticated packet transport. r[agent_harness_runtime.syndicate_turn_adapter.host]
- Track inbound and outbound assertion handles fail-closed, preserve one transport packet per turn, and bound packet sizes before and after transport effects. r[agent_harness_runtime.syndicate_turn_adapter.lifecycle]
- Reject embedded capabilities, `S` synchronization events, unknown packet/event shapes, malformed Preserves, stale retractions, duplicate handles, unsafe payloads, and oversized packets. r[agent_harness_runtime.syndicate_turn_adapter.fail_closed]
- Add positive and negative codec, lifecycle, fake-transport, feature-isolation, and documentation evidence. r[agent_harness_runtime.syndicate_turn_adapter.verification]

## Impact

- **Rust**: `animus-preserves` gains a Syndicate turn codec; `animus-std` gains a feature-gated protocol adapter and injected byte-transport port.
- **Wire**: the first profile supports the capability-free `A`, `R`, and `M` turn subset with bounded Animus dataspace records as assertion/message bodies.
- **Transport**: Iroh, local IPC, TLS streams, or other hosts may carry packets only when they satisfy reliable, in-order, confidentiality, integrity, authenticity, replay-resistance, and session-lifecycle requirements.
- **Claims**: this is not a complete Syndicate relay, WireRef membrane, capability attenuation implementation, daemon client, discovery service, or production network stack.
