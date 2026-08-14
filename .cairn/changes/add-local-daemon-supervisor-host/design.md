# Design: Local daemon supervisor host

## Context

The daemon contract makes role authority explicit. The transactional handoff host proves strict-idle transfer through fakes. Neither component opens a local endpoint or creates a worker process.

The first concrete host stays local to one machine and one explicit capability-owned runtime directory.

## Success Contract

A local client can request a root session, attach to its bounded event stream, detach without stopping execution, and reattach through a live stream or fresh snapshot.

The supervisor creates or reconnects one generation-bound worker for each root tree. It detects and classifies stale, zombie, stopped, mismatched, and unavailable workers.

The supervisor never executes provider, tool, Steel, or session work. Process separation never produces a sandbox claim.

A process launcher without identity checks, a socket with ambient discovery, or detach that stops session work is false completion.

## Decisions

### 1. Use one exact local transport cohort

The first host uses a versioned local byte-stream protocol over an explicit capability-owned endpoint. Transport framing, authentication, peer facts, message bounds, deadlines, and cancellation are typed.

No TCP listener, remote discovery, or ambient default endpoint is enabled.

### 2. Keep a durable safe worker registry

Registry records bind root identity, worker generation, process identity facts, endpoint identity, session head, lifecycle state, and safe receipt identities.

The registry stores no prompt, transcript, tool body, credential, or private provider payload.

### 3. Launch one worker per root tree

The supervisor launches the reviewed worker binary with explicit argv, environment, root, store, endpoint, policy, and limits. Owned teardown observes the complete process tree.

The worker constructs the live Steel agent service and owns all descendants below that root.

### 4. Make recovery classifications explicit

The supervisor distinguishes:

- healthy and ready
- starting
- stopping
- stopped
- disconnected
- stale registration
- identity mismatch
- zombie or unresponsive process
- failed start
- retry admitted or denied

Stop finalization continues after client disconnect. Explicit `retry_worker` creates a new generation only after current state permits it.

### 5. Keep attach as a subscription

Attach returns a snapshot-first bounded event subscription and a command channel. Detach removes only that client lease.

Worker execution continues unless a separate authorized stop command succeeds.

### 6. Restore through worker-owned services

A replacement worker validates the durable session and kernel heads before continuation. The supervisor routes restore facts but does not interpret session content.

## Authority Flow

```text
local client
  -> authenticated bounded local transport
  -> supervisor routing and health core
  -> explicit worker process shell
  -> live Steel session service
  -> snapshot-first events through supervisor
```

## Alternatives Considered

### Put all sessions in the supervisor process

Rejected because this breaks the accepted role split and root-tree failure containment.

### Discover a socket in a global home directory

Rejected because endpoint authority and lifecycle become ambient.

### Treat a live PID as worker identity

Rejected because PID reuse can connect a registry record to the wrong process.

## Risks and Controls

- The local endpoint can be impersonated. Exact endpoint ownership, authentication, peer facts, and generation binding fail closed.
- Stale records can block startup. Recovery classifies them before replacement.
- A stopped client can abandon cleanup. Stop finalization remains supervisor-owned.
- Worker process separation can be overclaimed. Receipt validation rejects sandbox and confinement claims.

## Non-Claims

This design does not establish remote security, sandboxing, multi-machine coordination, high availability, service-manager integration, crash freedom, production readiness, or release eligibility.
