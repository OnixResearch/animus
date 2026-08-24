## Phase 1: Command contracts

- [x] [serial] [task:descriptors] Add exact kernel-clear and state-inspection command and control variants in the pure terminal core. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:docs] [after:descriptors] Document descriptor-only authority and exact stable classes. r[agent_harness_runtime.live_terminal_agent_host.product_commands]

## Phase 2: Metadata projection

- [x] [serial] [task:state-event] [after:descriptors] Add a typed worker state event with exact generations, restore status, BLAKE3 heads, and bounded counts. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]
- [x] [serial] [task:terminal-projection] [after:state-event] Route the state event through direct and daemon modes into the terminal view. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]

## Phase 3: Private runtime relay

- [x] [serial] [task:private-event-source] [after:terminal-projection] Return the exact ordered live-run event snapshot to the worker without adding payload content to receipts. r[agent_harness_runtime.live_terminal_agent_host.private_events.source]
- [x] [serial] [task:private-event-contract] [after:private-event-source] Add a bounded opaque worker event with ordered BLAKE3 payload binding and redacted debug output. r[agent_harness_runtime.live_terminal_agent_host.private_events]
- [x] [serial] [task:private-event-terminal] [after:private-event-contract] Route private payloads through direct and daemon modes, then decode exact versioned runtime events in the terminal. r[agent_harness_runtime.live_terminal_agent_host.private_events]
- [x] [serial] [task:private-event-tests] [after:private-event-terminal] Cover conversation, tool, and status rendering plus unknown, malformed, forged, overbound, and debug-redaction failures. r[agent_harness_runtime.live_terminal_agent_host.private_events]

## Phase 4: Persistent live session

- [x] [serial] [task:persistent-session] [after:private-event-source] Move provider, tool, snapshot, session, and kernel resources into one persistent live-session owner. r[agent_harness_runtime.live_steel_agent_shell.persistent_session]
- [x] [serial] [task:persistent-controls] [after:persistent-session] Add one-effect driving, prompt, follow-up, steering, queue-CAS, cancellation, bounded event cursors, and explicit close. r[agent_harness_runtime.live_steel_agent_shell.persistent_session.controls]
- [x] [serial] [task:persistent-session-tests] [after:persistent-controls] Add positive incremental completion and interleaving tests plus malformed, stale, zero, and over-bound failures. r[agent_harness_runtime.live_steel_agent_shell.persistent_session.failure]

## Phase 5: Product control authority

- [x] [serial] [task:executor-controls] [after:persistent-controls] Add explicit persistent executor approval and child-control delegation without moving authority into the terminal. r[agent_harness_runtime.live_steel_agent_shell.persistent_session.controls]
- [x] [serial] [task:lifecycle-routing] [after:terminal-projection] Route refresh, detach, and stop through explicit connection APIs instead of worker-private command bytes. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:daemon-loop] [after:lifecycle-routing] Add a bounded authenticated local daemon request loop with deterministic disconnect cleanup. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:control-tests] [after:executor-controls] [after:daemon-loop] Cover accepted controls and stale, missing, malformed, over-bound, and disconnect failures. r[agent_harness_runtime.live_terminal_agent_host.product_commands]

## Phase 6: Verification

- [x] [serial] [task:positive-tests] [after:terminal-projection] Add positive construction, serialization, routing, reduction, and rendering tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:negative-tests] [after:terminal-projection] Add negative malformed-correlation, unknown-command, no-effect, malformed-metadata, blocked-restore, stale-generation, unordered-event, forged-identity, queue-regression, and private-field tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:gates] [after:docs] [after:positive-tests] [after:negative-tests] [after:private-event-tests] [after:persistent-session-tests] [after:control-tests] Run focused terminal, strict Clippy, formatting, Octet, Cairn, and relevant Nix gates. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
