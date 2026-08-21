## Phase 1: Command contracts

- [x] [serial] [task:descriptors] Add exact kernel-clear and state-inspection command and control variants in the pure terminal core. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:docs] [after:descriptors] Document descriptor-only authority and exact stable classes. r[agent_harness_runtime.live_terminal_agent_host.product_commands]

## Phase 2: Metadata projection

- [x] [serial] [task:state-event] [after:descriptors] Add a typed worker state event with exact generations, restore status, BLAKE3 heads, and bounded counts. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]
- [x] [serial] [task:terminal-projection] [after:state-event] Route the state event through direct and daemon modes into the terminal view. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]

## Phase 3: Private runtime relay

- [x] [serial] [task:private-event-contract] [after:terminal-projection] Add a bounded opaque worker event with ordered BLAKE3 payload binding and redacted debug output. r[agent_harness_runtime.live_terminal_agent_host.private_events]
- [x] [serial] [task:private-event-terminal] [after:private-event-contract] Route private payloads through direct and daemon modes, then decode exact versioned runtime events in the terminal. r[agent_harness_runtime.live_terminal_agent_host.private_events]
- [x] [serial] [task:private-event-tests] [after:private-event-terminal] Cover conversation, tool, and status rendering plus unknown, malformed, forged, overbound, and debug-redaction failures. r[agent_harness_runtime.live_terminal_agent_host.private_events]

## Phase 4: Verification

- [x] [serial] [task:positive-tests] [after:terminal-projection] Add positive construction, serialization, routing, reduction, and rendering tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [serial] [task:negative-tests] [after:terminal-projection] Add negative malformed-correlation, unknown-command, no-effect, malformed-metadata, blocked-restore, stale-generation, unordered-event, forged-identity, queue-regression, and private-field tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [ ] [serial] [task:gates] [after:docs] [after:positive-tests] [after:negative-tests] [after:private-event-tests] Run focused terminal, strict Clippy, formatting, Octet, Cairn, and relevant Nix gates. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
