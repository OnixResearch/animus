# Proposal: Adapt OpenClaw node capability-host contracts

## Why

Animus has host-control and remote-admission contracts, but future dogfood hosts also need a clear way to model device or machine peripherals that expose bounded commands without becoming the gateway itself. OpenClaw's node model separates the gateway host from paired node hosts, advertises capabilities at connect time, and routes `node.invoke` calls through explicit command and permission checks.

This change adapts that pattern as data-only descriptors, invocation plans, approval requirements, and safe outcomes.

## What Changes

- Define node identity, pairing, advertised capability, command, permission, and metadata descriptors as claims that require server-side policy admission.
- Define `node.invoke`-style plans and result envelopes over injected host services with idempotency, timeout, cancellation, and safe failure classes.
- Require canonical remote exec/system-run plans and per-node approval evidence before a node-host command can be dispatched.
- Preserve foreground, permission-missing, unavailable, denied, timeout, malformed-result, and background-unavailable outcomes as typed data.
- Keep device apps, node daemons, platform APIs, filesystem stores, command execution, network transport, and approval persistence host-owned.

## Impact

- **Files**: future adapter DTOs, effect/permission integration, node invocation tests, safe receipt checks, docs, and facade inventory updates.
- **Testing**: positive node advertisement, pairing metadata, invocation planning, approved remote exec, and typed result tests; negative unpaired node, unsupported command, permission mismatch, missing approval, missing service, timeout, malformed result, and ambient execution fallback tests.
- **Non-claims**: this does not implement iOS, Android, macOS, WebSocket nodes, camera, screen recording, location, SMS, or actual remote command execution.
