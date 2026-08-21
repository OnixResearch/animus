## Phase 1: Command contracts

- [x] [serial] [task:descriptors] Add exact kernel-clear and state-inspection command and control variants in the pure terminal core. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [task:docs] [after:descriptors] Document descriptor-only authority and exact stable classes. r[agent_harness_runtime.live_terminal_agent_host.product_commands]

## Phase 2: Metadata projection

- [x] [task:state-event] [after:descriptors] Add a typed worker state event with exact generations, restore status, BLAKE3 heads, and bounded counts. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]
- [x] [task:terminal-projection] [after:state-event] Route the state event through direct and daemon modes into the terminal view. r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]

## Phase 3: Verification

- [x] [task:positive-tests] [after:terminal-projection] Add positive construction, serialization, routing, reduction, and rendering tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [task:negative-tests] [after:terminal-projection] Add negative malformed-correlation, unknown-command, no-effect, malformed-metadata, blocked-restore, stale-generation, unordered-event, forged-identity, queue-regression, and private-field tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [ ] [task:gates] [after:docs] [after:positive-tests] [after:negative-tests] Run focused terminal, strict Clippy, formatting, Octet, Cairn, and relevant Nix gates. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
