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

### Requirement: Private runtime events use an opaque bounded relay

r[agent_harness_runtime.live_terminal_agent_host.private_events] The worker MUST bind each private runtime event to its root, sequence, class, payload size, and exact payload bytes with BLAKE3. The supervisor MUST route the bounded payload without decoding, retaining, logging, or adding it to receipts. The terminal MUST decode only the exact versioned runtime-event contract after delivery.

#### Scenario: Worker receives completed run events

r[agent_harness_runtime.live_terminal_agent_host.private_events.source]
- GIVEN a bounded live Steel run completes
- WHEN the worker reads the run output
- THEN the output MUST expose the exact ordered runtime-event snapshot
- AND the output event count MUST match the metadata-only receipt count
- AND the receipt MUST omit all event payload content.

#### Scenario: Private runtime events reach the terminal

r[agent_harness_runtime.live_terminal_agent_host.private_events.render]
- GIVEN an attached client and ordered worker runtime events
- WHEN the worker emits bounded private payloads for conversation, tool, and session status events
- THEN direct and daemon connections MUST produce the same typed runtime events
- AND the terminal MUST render the decoded events through the existing reducers
- AND supervisor metadata and receipts MUST omit the payload bytes.

#### Scenario: Private runtime event is invalid

r[agent_harness_runtime.live_terminal_agent_host.private_events.denied]
- GIVEN a private event has an unknown class, empty or oversized payload, malformed codec, forged identity, stale root, or unordered sequence
- WHEN the supervisor or terminal validates the event
- THEN it MUST reject the event before terminal projection
- AND it MUST NOT include the private payload in debug output or receipts.

### Requirement: Live Steel sessions support bounded incremental worker control

r[agent_harness_runtime.live_steel_agent_shell.persistent_session] The live Steel shell MUST keep provider, tool, snapshot, session, and kernel authority inside one persistent session owner. It MUST let an authorized worker submit exact commands, drive at most one pending effect, read bounded private event batches, and close all leases.

#### Scenario: Worker drives one live prompt incrementally

r[agent_harness_runtime.live_steel_agent_shell.persistent_session]
- GIVEN a validated live composition with explicit provider, executor, and persistence services
- WHEN the worker starts one correlated prompt and drives one pending effect at a time
- THEN the shell MUST preserve normal `CodingSessionHost` ordering and authority checks
- AND it MUST return the same private runtime events and metadata-only completion receipt as the bounded synchronous path.

#### Scenario: Worker interleaves queue and cancellation controls

r[agent_harness_runtime.live_steel_agent_shell.persistent_session.controls]
- GIVEN an active run with an effect pending
- WHEN the worker queues a follow-up, applies an exact generation-and-identity queue edit or removal, or requests cancellation
- THEN the shell MUST reduce the command through the existing coding-session core without driving unrelated effects
- AND stale queue guards, malformed correlations, missing runs, or reducer conflicts MUST fail closed
- AND cancellation MUST use the existing coding-session cancellation effect.

#### Scenario: Incremental private event request is invalid

r[agent_harness_runtime.live_steel_agent_shell.persistent_session.failure]
- GIVEN an event cursor or maximum is stale, zero, or over the fixed batch bound
- WHEN the worker requests incremental private events
- THEN the shell MUST reject the request without exposing event content in receipts or debug output
- AND closing an idle, cancelled, or failed session MUST still attempt every owned lease release.

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
