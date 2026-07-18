## Phase 1: Protocol ownership and feature boundary

- [x] [serial] Inventory ACP descriptor/conformance, coding-session facade, durable store/profile, confirmation, permission, tool executor, runtime events, headless protocol, host control, and receipt owners and document the direction-specific no-duplication mapping. r[agent_harness_runtime.acp_stdio_host.boundary] r[agent_harness_runtime.acp_stdio_host.verification]
- [x] [serial] Select and record one reviewed ACP protocol/library cohort, license, advertised feature subset, upgrade posture, and default/no-std dependency-isolation contract. r[agent_harness_runtime.acp_stdio_host.boundary]
- [x] [serial] Define connection/session/correlation state, validated request/outcome envelopes, facade command mappings, bounded update projections, transport actions, failure classes, and receipt drafts. r[agent_harness_runtime.acp_stdio_host.protocol_core] r[agent_harness_runtime.acp_stdio_host.receipts]

## Phase 2: Functional protocol core

- [x] [depends:agent_harness_runtime.acp_stdio_host.protocol_core] Implement pure initialize/capability negotiation, correlation validation, session command mapping, ordering, terminal-state, and error reduction over decoded messages and typed facade outcomes. r[agent_harness_runtime.acp_stdio_host.protocol_core]
- [x] [depends:agent_harness_runtime.acp_stdio_host.protocol_core] Implement pure new/load session, prompt, continuation, cancellation, and safe session-reference mapping without transcript or active-leaf ownership. r[agent_harness_runtime.acp_stdio_host.session_bridge]
- [x] [depends:agent_harness_runtime.acp_stdio_host.protocol_core] Implement pure permission/confirmation and tool pass-through planning that preserves existing capability, policy, sandbox, profile, and executor intersections. r[agent_harness_runtime.acp_stdio_host.permission_bridge]
- [x] [depends:agent_harness_runtime.acp_stdio_host.protocol_core] Implement ordered bounded RuntimeEvent-to-ACP update projection, visibility filtering, backpressure decisions, and hidden-reasoning/redaction handling. r[agent_harness_runtime.acp_stdio_host.events]
- [x] [depends:agent_harness_runtime.acp_stdio_host.receipts] Implement metadata-only transport/session/permission receipt projection and leak rejection. r[agent_harness_runtime.acp_stdio_host.receipts]

## Phase 3: Thin stdio shell and facade bridge

- [x] [depends:agent_harness_runtime.acp_stdio_host.boundary] Add the non-default ACP host crate/feature with bounded production framing/codec entrypoints and prove reusable/default/Wasm dependency isolation. r[agent_harness_runtime.acp_stdio_host.boundary] r[agent_harness_runtime.acp_stdio_host.transport]
- [x] [depends:agent_harness_runtime.acp_stdio_host.transport] Implement explicit stdin/stdout transport, bounded buffers, flushing, EOF/broken-pipe classification, cancellation/deadline observation, safe stderr diagnostics, and cleanup with no socket or network fallback. r[agent_harness_runtime.acp_stdio_host.transport]
- [x] [depends:agent_harness_runtime.acp_stdio_host.session_bridge] Wire protocol commands and facade events through one coding-session host without duplicating transcript, queue, run, tool, active-leaf, or commit state. r[agent_harness_runtime.acp_stdio_host.session_bridge] r[agent_harness_runtime.acp_stdio_host.events]
- [x] [depends:agent_harness_runtime.acp_stdio_host.permission_bridge] Wire confirmation requests/responses and tool requests through existing authority, sandbox, tool admission, cancellation, and receipt services before effects. r[agent_harness_runtime.acp_stdio_host.permission_bridge]
- [x] [depends:agent_harness_runtime.acp_stdio_host.resume] Wire explicit session load/resume through durable store validation, active-leaf replay, interrupted-tool repair, and runtime authority-profile admission before continuation. r[agent_harness_runtime.acp_stdio_host.resume]

## Phase 4: Positive and negative conformance

- [x] [parallel] Add a deterministic fixture client that drives production initialize, feature negotiation, new/load session, prompt streaming, tool/permission flow, cancellation, durable resume, terminal completion, EOF, and clean shutdown paths. r[agent_harness_runtime.acp_stdio_host.verification]
- [x] [parallel] Add negative fixtures for malformed/oversized frames, unknown required features, unsupported methods, duplicate/out-of-order correlation, stale session/run IDs, permission widening, denied tools, sandbox/profile mismatch, hidden-reasoning or payload leak, buffer exhaustion, client EOF during confirmation, broken stdout, store/facade failure, cancellation ambiguity, and implicit reconnect/retry attempts. r[agent_harness_runtime.acp_stdio_host.protocol_core] r[agent_harness_runtime.acp_stdio_host.transport] r[agent_harness_runtime.acp_stdio_host.permission_bridge] r[agent_harness_runtime.acp_stdio_host.verification]
- [x] [depends:agent_harness_runtime.acp_stdio_host.verification] Prove identical facade event fixtures produce compatible headless, TUI-reducer, and ACP lifecycle outcomes without frontend-owned transcript mutation. r[agent_harness_runtime.acp_stdio_host.events] r[agent_harness_runtime.acp_stdio_host.verification]
- [x] [depends:agent_harness_runtime.acp_stdio_host.verification] Add source-shape guards proving reusable crates and default features do not read stdio, launch ACP processes, listen on sockets, inspect environment credentials, contact providers/tools, or own transport retries. r[agent_harness_runtime.acp_stdio_host.boundary] r[agent_harness_runtime.acp_stdio_host.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update ACP/MCP interop, external-subagent, coding-session, durable resume/profile, permission, confirmation, tools, event lifecycle, headless/TUI, component catalog, runtime facade, safety, dogfood, and README reference documentation. r[agent_harness_runtime.acp_stdio_host.boundary] r[agent_harness_runtime.acp_stdio_host.receipts]
- [x] [serial] Add a focused ACP stdio rail covering protocol cohort, codec/reducer fixtures, facade mapping, permissions/tools, event visibility, resume, transport failures, dependency isolation, and safe receipts. r[agent_harness_runtime.acp_stdio_host.verification]
- [x] [depends:agent_harness_runtime.acp_stdio_host.verification] Run focused protocol/host/facade/store/permission/tool/event tests, formatting, Clippy, composition and safe-receipt rails, dogfood claim hygiene, Cairn validation and gates, and relevant Nix checks; record any live editor smoke separately. r[agent_harness_runtime.acp_stdio_host.verification]
