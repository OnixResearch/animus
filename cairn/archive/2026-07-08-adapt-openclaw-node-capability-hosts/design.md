# Design: Adapt OpenClaw node capability-host contracts

Nodes should be modeled as host-owned peripherals that expose claims. Animus validates claims, plans invocations, and records safe outcomes; only injected host services can actually contact devices or run commands.

## Decisions

### 1. Nodes are peripherals, not gateways

**Choice:** A node descriptor identifies a device or machine capability host associated with a gateway profile, but it does not own model routing, channels, sessions, or provider dispatch.

**Rationale:** This preserves the OpenClaw separation between a gateway control plane and node-local capabilities while keeping Animus host topology explicit.

### 2. Advertised capabilities are claims

**Choice:** Node `caps`, `commands`, and `permissions` are treated as claims that must be intersected with host policy, authority, sandbox, pairing, and approval state before invocation.

**Rationale:** A node saying it can run a command is not authority to run it.

### 3. Invocation plans are deterministic

**Choice:** `node.invoke` planning is pure over node descriptor, command descriptor, params identity, timeout policy, idempotency identity, permission state, and injected-service availability.

**Rationale:** Planning can be tested without a phone, desktop app, WebSocket, or shell.

### 4. Remote exec requires canonical plans

**Choice:** High-impact node commands such as system run require canonical argv/cwd/raw-command/session metadata plus per-node approval evidence before host dispatch.

**Rationale:** This mirrors OpenClaw's `systemRunPlan` guard while keeping approvals and command execution outside reusable crates.

### 5. Platform failures are first-class outcomes

**Choice:** Foreground-required, permission-missing, denied, unavailable, timeout, cancellation, and malformed-result states are typed results and safe receipts rather than stringly host errors.

**Rationale:** UI, headless, and audit layers need consistent failure classes that do not leak platform payloads.
