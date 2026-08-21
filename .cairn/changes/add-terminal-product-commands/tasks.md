## Phase 1: Command contracts

- [x] [serial] [task:descriptors] Add exact kernel-clear and state-inspection command and control variants in the pure terminal core. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [task:docs] [after:descriptors] Document descriptor-only authority and exact stable classes. r[agent_harness_runtime.live_terminal_agent_host.product_commands]

## Phase 2: Verification

- [x] [task:positive-tests] [after:descriptors] Add positive construction, serialization, and routing tests for both descriptors. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [x] [task:negative-tests] [after:descriptors] Add negative malformed-correlation, unknown-command, and no-effect tests. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
- [ ] [task:gates] [after:docs] [after:positive-tests] [after:negative-tests] Run focused terminal, strict Clippy, formatting, Octet, Cairn, and relevant Nix gates. r[agent_harness_runtime.live_terminal_agent_host.product_commands]
