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

#### Scenario: Product control descriptor is malformed

r[agent_harness_runtime.live_terminal_agent_host.product_commands.denied]
- GIVEN a descriptor has an unknown command kind or malformed correlation identity
- WHEN terminal validation or connection routing runs
- THEN it MUST reject the descriptor before worker delivery
- AND it MUST perform no protected effect.
