## Phase 1: Wire contract and pure lifecycle

- [x] [serial] Add the Syndicate-turn schema family, profile label, typed `A`/`R`/`M` turn DTOs, and exact Preserves text/binary codecs. r[agent_harness_runtime.syndicate_turn_adapter.wire]
- [x] [serial] Add assertion, retraction, and orchestration-message projection plus owning dataspace DTO revalidation on decode. r[agent_harness_runtime.syndicate_turn_adapter.projection]
- [x] [serial] Add the pure directional handle-transition planner for unused-assert and live-retract invariants. r[agent_harness_runtime.syndicate_turn_adapter.lifecycle]

## Phase 2: Host adapter

- [x] [depends:wire,lifecycle] Add the non-default `syndicate-host` packet transport port and protocol adapter with separate inbound/outbound state. r[agent_harness_runtime.syndicate_turn_adapter.host]
- [x] [depends:host] Bound outbound and inbound packet bytes, preserve one packet per turn, and commit handle state only after successful effects. r[agent_harness_runtime.syndicate_turn_adapter.lifecycle]
- [x] [depends:projection,host] Expose typed publish, retract, orchestration-message, send-turn, and receive-turn methods through `animus-std` and the root facade. r[agent_harness_runtime.syndicate_turn_adapter.host]

## Phase 3: Fail-closed evidence

- [x] [parallel] Add positive text/binary fixtures for exact `A`, `R`, and `M` turn shapes, multi-event order, DTO revalidation, and fake-transport round trips. r[agent_harness_runtime.syndicate_turn_adapter.verification]
- [x] [parallel] Add negative fixtures for malformed packets, unsupported `S`/extension/error packets, embedded capability markers, unsafe bodies, duplicate handles, stale retractions, oversized packets, unavailable transport, and no state commit after failure. r[agent_harness_runtime.syndicate_turn_adapter.fail_closed] r[agent_harness_runtime.syndicate_turn_adapter.verification]
- [x] [serial] Verify default builds remain independent of the protocol adapter and no live transport dependency enters reusable core/runtime/adapter crates. r[agent_harness_runtime.syndicate_turn_adapter.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document the layer split, protocol profile, secure transport requirements, Iroh composition, supported events, capability non-claims, and operator-owned session reset behavior. r[agent_harness_runtime.syndicate_turn_adapter.host] r[agent_harness_runtime.syndicate_turn_adapter.fail_closed]
- [x] [serial] Run focused Preserves, std-host, root-feature, formatting, clippy, boundary-schema, component-catalog, workspace, Tiger Style, and Cairn validation rails. The runtime-facade inventory contains no Syndicate omissions; its remaining failures belong to the separate untracked durable-session-store change. r[agent_harness_runtime.syndicate_turn_adapter.verification]
- [x] [serial] Sync accepted requirements and archive only after all deterministic gates pass; full WireRef/capability interoperability and live production transport remain explicit non-claims. r[agent_harness_runtime.syndicate_turn_adapter.verification]
