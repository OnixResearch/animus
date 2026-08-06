## Context

Prime Agent's architecture doc states the ownership split plainly. The client owns rendering, keyboard input, and local UI preferences; it does not own execution. The supervisor owns discovery, routing, attachments, worker health, and cross-agent message delivery. Each worker owns one root runtime, its scheduler, kernels, and all descendants below that root. Interactive, print, JSON, and RPC clients all reach the same execution path through the supervisor. Workers and kernels are separate processes for lifecycle and failure containment, not security sandboxes.

Animus owns each piece as a contract but not the composition rule. `host-control-protocol` defines UI-neutral host topology and bounded read/event/wait contracts. `session-restore-contract` defines live-process, snapshot, and replay restore claims. `animus-std-control` owns pause, resume, cancellation, and bounded subscriptions. `background-work-queues` owns long-running queue service contracts. What is missing is the invariant that binds them: execution authority lives in the worker role, and clients of every kind are rendering and input shells.

Completion requires the three-role split as typed contracts, detach-safe reattach semantics, supervisor duties, and the containment non-claim. A client with execution authority, a reattach that loses state, or a sandbox claim is false completion.

## Decisions

### Decision: role authority is typed, not conventional

**Choice:** A focused contract module defines the three roles and their authority sets as types. The client role cannot express prompt execution, queue ownership, or kernel access. The supervisor role cannot express session execution. The worker role cannot express rendering. Cross-role communication is versioned command and event envelopes.

**Rationale:** Prime-agent's split works because it is architectural. Animus gets the same property by making out-of-role authority unrepresentable in the contract types, then checking conformance in the focused rail.

### Decision: detach is the default, attach is a subscription

**Choice:** Session execution never depends on an attached client. Attach creates a bounded event subscription plus command channel through the supervisor. Detach ends the subscription only. Reattach admits either a live stream or a recovery snapshot per the existing session-restore contract.

**Rationale:** Detach-safe execution is the property that makes heartbeats, schedules, goals, and autonomous mode meaningful. Modeling attach as a subscription keeps it inside existing bounded-subscription contracts.

### Decision: one worker owns exactly one root session tree

**Choice:** The worker role binds one root session, its prompt queue, schedules, kernel handles, and all descendants. Cross-session traffic routes through the supervisor with the messaging-receipts contract. No worker reaches into another worker's tree.

**Rationale:** Single-ownership is what makes failure containment real: a crashed worker damages exactly one session tree, and recovery scope is explicit.

### Decision: containment is lifecycle-only, by contract

**Choice:** The containment contract states what process isolation provides (independent crash domains, teardown, restart) and receipt validation rejects any sandbox, confinement, or security claim. Sandboxing remains a separate concern for `sandlock`-style descriptors and host policy.

**Rationale:** Prime-agent documents the same boundary. Writing it into receipt validation prevents containment evidence from being laundered into security claims downstream.

## Alternatives Considered

### In-process sessions with careful API discipline

Rejected because process boundaries are what give detach, crash containment, and independent teardown their force. API discipline alone decays.

### A supervisor that also executes sessions

Rejected because mixing routing authority with execution authority recreates the monolith the split exists to avoid, and it makes supervisor failure fatal to all sessions.

### Claim sandboxing from process isolation

Rejected explicitly. Process isolation without a sandbox mechanism (Landlock, seccomp, Bubblewrap-class) is containment for lifecycle, not a security boundary.

## Risks / Trade-offs

- Three process roles add operational complexity over in-process sessions. Mitigation: the same runtime must run in-process for SDK and test use, with role contracts unchanged.
- Recovery snapshots can go stale between detach and crash. Mitigation: snapshot freshness is typed in the session-restore contract, and receipts bind snapshot identity.
- Supervisor becomes a single point of failure for routing. Mitigation: worker execution continues when the supervisor is down; only routing and attachment degrade.

## Non-Claims

This design does not establish security sandboxing, crash-freedom, durability under arbitrary host failure, multi-machine supervision, remote client support, production readiness, or release eligibility.
