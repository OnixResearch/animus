# Agent Harness Runtime

## ADDED Requirements

### Requirement: Product control descriptors are exact and effect-free

r[agent_harness_runtime.live_terminal_agent_host.product_commands] The live terminal MUST expose dedicated `kernel_clear` and `state_inspection` client command descriptors with correlation identities. It MUST NOT execute kernel, journal, provider, tool, session, queue, or persistence effects.

#### Scenario: Operator requests an explicit kernel clear

r[agent_harness_runtime.live_terminal_agent_host.product_commands.clear]
- GIVEN a terminal control requests an explicit kernel clear
- WHEN the pure command reducer constructs the descriptor
- THEN it MUST emit the exact `kernel_clear` command class with a BLAKE3 correlation identity
- AND it MUST perform no kernel effect.

#### Scenario: Operator requests bounded state inspection

r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect]
- GIVEN a terminal control requests state inspection
- WHEN the pure command reducer constructs the descriptor
- THEN it MUST emit the exact `state_inspection` command class with a BLAKE3 correlation identity
- AND it MUST read no session, kernel, provider, credential, or private path data.

#### Scenario: Worker returns bounded state metadata

r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result]
- GIVEN the worker authorizes a state-inspection request
- WHEN it emits the ordered result through the bounded event stream
- THEN the event MUST contain only request and observation identities, worker and queue generations, restore status, BLAKE3 state heads, and bounded counts
- AND its observation identity MUST bind the request, root, sequence, and complete metadata payload
- AND direct and daemon connections MUST project the same typed event
- AND the terminal MUST validate the event before rendering it
- AND terminal receipts MUST NOT add the inspected state.

#### Scenario: Product control descriptor is malformed

r[agent_harness_runtime.live_terminal_agent_host.product_commands.denied]
- GIVEN a descriptor has an unknown command kind or malformed correlation identity
- WHEN terminal validation or connection routing runs
- THEN it MUST reject the descriptor before worker delivery
- AND it MUST perform no protected effect.

#### Scenario: State result contains private or malformed fields

r[agent_harness_runtime.live_terminal_agent_host.product_commands.inspect_result_denied]
- GIVEN a state event contains an unknown field, blocked restore, stale generation, regressed queue generation, unordered sequence, or malformed identity
- WHEN the supervisor or terminal validates the event
- THEN it MUST reject the event before view projection
- AND it MUST NOT expose prompt, Steel source, provider result, credential, or private path data.
