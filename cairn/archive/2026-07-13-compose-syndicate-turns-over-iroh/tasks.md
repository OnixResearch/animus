## Phase 1: Composition contract

- [x] [serial] Add the explicit `iroh-syndicate-host` composition feature without changing default or independent feature behavior. r[agent_harness_runtime.syndicate_iroh_bridge.feature]
- [x] [serial] Add pure route-admission, exact body-scope, and directional packet-size validation for the composition boundary. r[agent_harness_runtime.syndicate_iroh_bridge.framing] r[agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
- [x] [depends:feature] Add the caller-owned Iroh Syndicate packet-session port and transport bridge implementing `SyndicatePacketTransport`. r[agent_harness_runtime.syndicate_iroh_bridge.transport]

## Phase 2: Public composition and evidence

- [x] [depends:transport] Re-export the bridge through `animus-std` and root `animus`, and register its host/component/facade inventory entries. r[agent_harness_runtime.syndicate_iroh_bridge.feature] r[agent_harness_runtime.syndicate_iroh_bridge.transport]
- [x] [parallel] Add positive fake-session tests for exact one-packet send/receive and outer Syndicate adapter handle lifecycle. r[agent_harness_runtime.syndicate_iroh_bridge.verification]
- [x] [parallel] Add negative tests for denied admission, cross-scope assertion/message bodies, zero/oversized route bounds, unavailable session send/receive, safe errors, and handle non-commit. r[agent_harness_runtime.syndicate_iroh_bridge.fail_closed] r[agent_harness_runtime.syndicate_iroh_bridge.verification]
- [x] [serial] Repair the pre-existing combined-feature Iroh receipt fixture borrow mismatch found by the baseline test, without broadening behavior. The baseline failed before bridge edits because `AgentCommsReceipt::new` now borrows its draft. r[agent_harness_runtime.syndicate_iroh_bridge.verification]

## Phase 3: Documentation and closeout

- [x] [parallel] Document the composition feature, route/profile binding, no-double-envelope rule, session security contract, no-mixed-protocol rule, reset behavior, and live-network non-claims. r[agent_harness_runtime.syndicate_iroh_bridge.transport] r[agent_harness_runtime.syndicate_iroh_bridge.fail_closed]
- [x] [serial] Focused combined-feature tests, independent feature checks, root facade checks, workspace tests/checks/clippy, feature clippy, formatting, component catalog, Tiger Style, and Cairn validation pass. Runtime-facade inventory reports no bridge omissions; its remaining 29 issues are pre-existing durable-session-store exports outside this change. r[agent_harness_runtime.syndicate_iroh_bridge.verification]
- [x] [serial] Sync accepted requirements and archive only after deterministic gates pass; live Iroh endpoint interoperability remains an explicit non-claim. r[agent_harness_runtime.syndicate_iroh_bridge.verification]
