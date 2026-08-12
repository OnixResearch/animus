# agent-harness-runtime Change Spec: OpenClaw node capability hosts

## Requirements

### Requirement: Nodes are host-owned peripherals

r[agent_harness_runtime.openclaw_nodes.scope] Animus MUST treat OpenClaw-style nodes as bounded references for host-owned peripheral capability hosts, and MUST NOT make nodes own gateway control, model routing, channel connectors, provider sessions, durable session stores, or reusable runtime execution authority.

#### Scenario: Node cannot become a gateway by descriptor

- GIVEN a node descriptor claims messaging, model, or provider routing authority
- WHEN node descriptor validation runs
- THEN validation rejects those gateway-owned claims
- AND the node remains limited to declared host capability surfaces admitted by policy.

### Requirement: Node advertisements are claims

r[agent_harness_runtime.openclaw_nodes.advertisement] Node advertisements MUST represent identity, pairing metadata, platform label, capability claims, command allowlist, permission claims, safe version metadata, and optional metadata-pinning evidence as untrusted claims until host policy and admission approve them.

#### Scenario: Unsupported command is denied

- GIVEN a paired node advertises camera capability but not a screen-record command
- WHEN a node invocation requests screen recording
- THEN invocation planning returns an unsupported-command diagnostic
- AND no host node service is called.

### Requirement: Node invocation uses injected services

r[agent_harness_runtime.openclaw_nodes.invoke] Node invocation planning MUST produce typed requests over injected host services with node identity, command name, parameter identity, timeout posture, idempotency identity, cancellation handle when available, and safe result expectations, and MUST fail closed when the selected node service is missing, denied, unavailable, timed out, cancelled, or malformed.

#### Scenario: Missing node service fails closed

- GIVEN an admitted node command requires a host node service
- WHEN no matching service is injected
- THEN invocation planning or dispatch returns node-service-unavailable
- AND reusable runtime does not open a socket, spawn a process, contact a device, or execute a fallback command.

### Requirement: Node-host exec requires approvals

r[agent_harness_runtime.openclaw_nodes.approvals] High-impact node commands, including system-run or system-which equivalents, MUST require canonical argv, cwd, raw-command, agent/session metadata, selected node identity, policy posture, and per-node approval evidence before dispatch, and approval receipts MUST exclude raw credentials, environment values, full paths, and unbounded command output.

#### Scenario: Missing system-run plan is rejected

- GIVEN a node-host exec request includes a raw command string but omits canonical argv and session metadata
- WHEN approval validation runs
- THEN validation rejects the request with a missing-plan diagnostic
- AND no node-host executor is invoked.

### Requirement: Node pairing and metadata drift are explicit

r[agent_harness_runtime.openclaw_nodes.pairing] Node pairing descriptors MUST bind safe device identity, role, approved scopes or command ceilings, token identity class, and pinned metadata, and MUST require repair or re-approval when pinned metadata materially changes.

#### Scenario: Device metadata drift requires repair

- GIVEN a paired node reconnects with the same device identity but a different platform family or command ceiling
- WHEN pairing validation compares pinned metadata
- THEN validation returns a metadata-drift diagnostic requiring host repair or re-approval
- AND widened node commands remain unavailable until admitted.

### Requirement: Node capability-host verification

r[agent_harness_runtime.openclaw_nodes.verification] The node capability-host adaptation MUST include positive advertisement, pairing, invocation, approval, safe result, and receipt tests plus negative unpaired node, metadata drift, unsupported command, permission mismatch, missing approval, missing service, timeout, malformed result, and ambient execution fallback tests before archive.

#### Scenario: Closeout evidence covers node host safety

- GIVEN maintainers intend to archive the OpenClaw node adaptation
- WHEN closeout validation runs
- THEN evidence names node descriptor, invocation, approval, pairing, receipt, documentation, Cairn validation, and Cairn gate checks
- AND omitted device-app, platform API, WebSocket, camera, screen, location, SMS, and real command execution surfaces are recorded as non-claims.
