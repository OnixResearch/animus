# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: ACP stdio hosting is optional and host owned

r[agent_harness_runtime.acp_stdio_host.boundary] Animus MUST expose ACP agent hosting only through a non-default host-owned adapter pinned to an exact reviewed protocol cohort, and reusable core/runtime, default std, no-std, and Wasm profiles MUST NOT acquire ACP transport, stdio, process, socket, filesystem, terminal, credential, or network authority.

#### Scenario: ACP host feature is selected

- GIVEN an embedder explicitly enables the reviewed ACP host feature and supplies a validated coding-session host plus required services
- WHEN the ACP adapter is constructed
- THEN it MAY advertise only the implemented protocol cohort and feature subset
- AND it MUST NOT discover clients, providers, credentials, sessions, tools, or host resources from ambient state.

#### Scenario: Default build is used

- GIVEN an embedder builds Animus without the ACP host feature
- WHEN dependency and facade checks run
- THEN reusable/default crates MUST remain free of the ACP transport dependency and live server entrypoint
- AND ACP descriptor validation MUST remain data-only.

### Requirement: ACP lifecycle mapping has a functional core

r[agent_harness_runtime.acp_stdio_host.protocol_core] ACP capability negotiation, correlation, session-command mapping, permission planning, event projection, ordering, terminal-state, and failure classification MUST be deterministic reductions over validated decoded messages, explicit connection state, facade observations, and typed host outcomes.

#### Scenario: Identical protocol trace repeats

- GIVEN identical validated ACP messages, connection/session state, coding-facade observations, and host outcomes
- WHEN the protocol reducer runs repeatedly
- THEN it MUST produce identical next state, facade commands, confirmation requests, ordered updates, and transport actions
- AND it MUST perform no stdin/stdout, filesystem, process, network, clock, provider, tool, credential, persistence, or terminal effect.

#### Scenario: Correlation is stale or duplicated

- GIVEN a response or outcome carries an unknown, completed, duplicate, or stale correlation identity
- WHEN protocol reduction runs
- THEN it MUST return a typed correlation/order failure
- AND it MUST NOT mutate facade state or infer a new request.

### Requirement: The initial ACP transport is bounded stdio

r[agent_harness_runtime.acp_stdio_host.transport] The first ACP host adapter MUST use explicit bounded stdin/stdout transport with validated framing, finite buffers, backpressure, flush and cleanup handling, typed EOF/broken-pipe failures, and safe stderr diagnostics, and MUST NOT open sockets or fall back to WebSocket, HTTP, TLS, or network transport.

#### Scenario: Stdio exchange succeeds

- GIVEN a launched ACP host receives valid bounded frames and stdout remains writable
- WHEN the transport shell processes the exchange
- THEN it MUST feed decoded requests through the production protocol reducer and flush ordered encoded responses/updates
- AND transport buffering MUST remain within the admitted profile.

#### Scenario: Output consumer stops reading

- GIVEN outbound updates reach the admitted backpressure or byte bound
- WHEN the client does not drain stdout
- THEN the adapter MUST return a typed backpressure/transport failure and begin bounded cleanup
- AND it MUST NOT buffer without limit, drop required lifecycle updates silently, or switch transports.

### Requirement: ACP sessions delegate to the coding-session facade

r[agent_harness_runtime.acp_stdio_host.session_bridge] ACP initialize, new/load session, prompt, continuation, cancellation, and supported session updates MUST map to the coding-session facade, which remains the sole owner of transcript, queue, active run, tool state, active leaf, durable commits, recovery, and terminal outcome.

#### Scenario: Client submits a prompt

- GIVEN an initialized ACP connection is bound to an admitted idle coding session
- WHEN a valid prompt request arrives
- THEN the adapter MUST issue the corresponding facade command and project its ordered runtime events
- AND it MUST NOT append transcript entries or contact providers/tools independently.

#### Scenario: Overlapping prompt arrives

- GIVEN the bound session already has an active run
- WHEN the ACP client submits another ordinary prompt without an admitted steering or follow-up command
- THEN the facade's single-run decision MUST be preserved in the ACP failure/update
- AND the adapter MUST NOT create a parallel transcript mutation or implicit queue entry.

### Requirement: ACP approval cannot widen authority

r[agent_harness_runtime.acp_stdio_host.permission_bridge] ACP permission and tool flows MUST pass through existing permission-mode, confirmation-broker, Animus/Basalt authorization, session-profile, sandbox, capability, available-tool, and executor-readiness intersections, and a client approval MUST NOT authorize an otherwise denied or unavailable effect.

#### Scenario: Client approves an admitted request

- GIVEN an ACP permission request is correlated to a current confirmation whose action remains authorized, sandbox-compatible, profile-bound, and executor-ready
- WHEN the client returns an allow decision
- THEN the adapter MAY return that decision to the existing admission flow
- AND final effect dispatch MUST still require the existing authority and enforcement checks.

#### Scenario: Client approves a denied tool

- GIVEN active policy, session profile, sandbox, capability ceiling, or executor readiness denies a requested tool
- WHEN the ACP client returns allow
- THEN the action MUST remain denied with distinct approval and authority outcomes
- AND no tool or fallback command may execute.

### Requirement: ACP updates preserve event visibility and bounds

r[agent_harness_runtime.acp_stdio_host.events] The ACP adapter MUST project the coding-session facade's ordered runtime events into bounded protocol updates while preserving assistant-visible content, tool/confirmation/queue lifecycle, cancellation, durable boundaries, terminal state, hidden-reasoning policy, and redaction.

#### Scenario: Provider emits visible and hidden channels

- GIVEN facade events contain assistant-visible content plus redacted thought or provider-native metadata
- WHEN ACP update projection runs
- THEN visible content MAY be streamed according to protocol and bounds
- AND hidden reasoning, raw provider payloads, credentials, raw tool arguments/output, physical paths, and unbounded logs MUST be omitted or represented only by approved safe metadata.

#### Scenario: Update cannot be represented safely

- GIVEN a required facade event is unsupported by the negotiated protocol subset or exceeds safe projection bounds
- WHEN update projection runs
- THEN the adapter MUST return a typed unsupported/projection failure or bounded diagnostic according to profile
- AND it MUST NOT silently promote, truncate, or reinterpret authoritative lifecycle state.

### Requirement: ACP resume passes durable recovery and profile admission

r[agent_harness_runtime.acp_stdio_host.resume] ACP load or resume MUST resolve an explicit safe session reference and pass durable-store integrity, active-leaf replay, interrupted-tool repair, and runtime authority-profile admission before prompt or continuation, while transport correlation and external session labels remain non-authoritative metadata.

#### Scenario: Durable session resumes

- GIVEN the host resolves a unique fresh safe reference whose store, replay, repair, and authority profile all validate
- WHEN the ACP client requests load/resume
- THEN the adapter MAY bind the connection to the reconstructed coding-session facade
- AND subsequent continuation MUST use the facade's committed state and current host admission.

#### Scenario: Authority profile changed

- GIVEN the durable session profile differs from current admitted profile inputs
- WHEN ACP load/resume runs
- THEN automatic continuation MUST be blocked with a safe profile-drift outcome
- AND reconnect, client approval, or a new transport correlation ID MUST NOT bypass explicit reauthorization and durable rebind.

### Requirement: ACP host receipts are metadata only

r[agent_harness_runtime.acp_stdio_host.receipts] ACP host receipts MUST contain only safe protocol/cohort/feature labels, operation and status classes, hashed connection/session/correlation/profile identities, bounded frame/update/tool/permission counts, transport and cleanup status, redaction markers, non-claims, and a domain-separated BLAKE3 receipt identity.

#### Scenario: Transport failure receipt is built

- GIVEN the ACP session ends because of malformed input, EOF, broken pipe, backpressure, cancellation, or facade failure
- WHEN receipt projection runs
- THEN it MUST record the safe failure class, bounded counts, cleanup status, and relevant identities
- AND it MUST omit raw frames, prompts, tool payloads, provider payloads, credentials, environment values, physical paths, hidden reasoning, and unbounded diagnostics.

### Requirement: ACP stdio behavior has conformance evidence

r[agent_harness_runtime.acp_stdio_host.verification] The ACP stdio host change MUST include a deterministic fixture client and positive and negative tests for protocol negotiation, facade mapping, prompts, events, permissions, tools, cancellation, durable resume, transport failures, redaction, dependency isolation, and explicit non-claims.

#### Scenario: Change is ready to archive

- GIVEN maintainers intend to sync and archive the ACP stdio host change
- WHEN closeout validation runs
- THEN focused protocol, transport, facade, store/profile, permission, tool, event, receipt, Cairn, and relevant Nix checks MUST pass
- AND live editor, WebSocket, remote, and live-provider evidence MUST remain separately classified.
