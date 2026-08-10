## Context

Daemon workers separate client, supervisor, and worker authority. Session restore can describe handoff as a capability, but hosts still own the mechanics.

A live replacement needs stricter rules than detach and reattach. The old worker and replacement must not own the same conversation lease.

Autolith supplies the replacement ordering reference. Tile supplies a hardened local socket and direct-attach surface, but Tile does not own Animus conversations.

This change defines Animus handoff first. A later Tile change can map the accepted state and events.

## Decisions

### Decision: define strict idle as a pure composite decision

**Choice:** Strict idle requires no active provider attempt, tool effect, repair, confirmation, cancellation, primary queue entry, child queue entry, undelivered steering entry, pending durable commit, or unresolved terminal transition.

The decision binds the exact worker, session, branch, lease generation, queue generations, active-turn generation, and authority profile.

A display-only idle status is insufficient.

**Rationale:** Replacement from an incomplete state can duplicate or lose work.

### Decision: use a monotonic handoff transaction

**Choice:** The pure state machine uses `Prepared`, `IngressPaused`, `LeaseReleased`, `ReplacementStarted`, `Claimed`, `Ready`, `Cancelled`, `LeaseRestored`, and `Complete`.

Each transition consumes the prior transition identity and exact current host observation. No state can move backward.

Rollback is a new forward transition to `LeaseRestored`, not mutation of prior evidence.

**Rationale:** A monotonic record makes late claims and repeated cancellation detectable.

### Decision: pause ingress before lease release

**Choice:** After strict-idle admission, the old worker pauses prompt, steer, follow-up, and external message ingress.

It then commits the composer draft, pending-input vault generation, current session head, authority profile, and handoff record.

Only matching durable evidence permits lease release.

**Rationale:** New input after the idle snapshot would invalidate the transfer.

### Decision: bind one replacement claimant

**Choice:** A claim includes handoff identity, session identity, released lease generation, replacement process identity, endpoint identity, protocol version, and freshness nonce.

The local host authenticates the endpoint. Same-user socket access alone does not replace claim binding.

A second claimant, stale generation, cancelled handoff, or mismatched endpoint is rejected.

**Rationale:** Local process identity and session ownership are separate facts.

### Decision: require readiness before old-worker exit

**Choice:** Replacement readiness requires successful lease claim, exact durable session load, pending-vault visibility, authority-profile admission, active-turn recovery admission, and service readiness.

The old worker exits only after matching readiness evidence enters the handoff record.

**Rationale:** Process startup is not session readiness.

### Decision: make timeout cancellation own the process tree

**Choice:** On deadline or operator cancellation, the host marks the handoff cancelled before it requests replacement termination.

The old worker can reclaim the released lease only after terminal evidence covers the complete owned replacement process tree.

A delayed replacement cannot claim the cancelled generation.

**Rationale:** Rollback before termination can produce two live owners.

### Decision: preserve session input state without PTY transfer

**Choice:** Handoff preserves canonical session references, composer draft identity, pending-input vault generation, active-turn generation, and safe attachment references.

It does not transfer a PTY, terminal parser, render cache, file descriptor, provider socket, credential value, or process memory.

**Rationale:** Conversation continuity and terminal-process migration are different claims.

### Decision: keep host-control and Tile roles narrow

**Choice:** Only transport-neutral handoff status, wait, cancellation, and feature DTOs can enter Host Control.

Tile can later present state and route an authorized operator action through its existing managed-agent and direct-attach surfaces.

Tile retains PTYs, workspace state, local socket policy, and rendering. Animus retains conversation and worker lease authority.

**Rationale:** Shared DTOs do not justify moving product authority between repositories.

### Decision: emit bounded handoff evidence

**Choice:** Receipts bind worker, session, lease, authority, handoff, endpoint, process-attempt, transition, cancellation, readiness, vault-generation, and BLAKE3 identities.

They reject drafts, pending input, prompts, transcripts, credentials, socket paths, process arguments, provider data, and raw errors.

**Rationale:** Operators need transaction state without exposing session content.

## Required Transition Order

1. Validate current authority and strict idle.
2. Create the handoff identity.
3. Pause ingress.
4. Commit draft, vault, session-head, and handoff state.
5. Release the current lease.
6. Start one replacement process tree.
7. Authenticate and admit one claimant.
8. Load and validate durable session state.
9. Commit replacement readiness.
10. Exit the old worker.

Cancellation after lease release uses this order:

1. Commit cancelled state.
2. Terminate the owned replacement process tree.
3. Observe complete termination.
4. Revalidate old-worker authority and store head.
5. Acquire the next lease generation.
6. Resume ingress.

## Alternatives Considered

### Start the replacement before lease release

Rejected because both processes can observe or mutate the same conversation generation.

### Use direct attach takeover as handoff

Rejected because attach controls input ownership for a pane. It does not transfer an Animus conversation lease.

### Exit the old worker after process spawn

Rejected because startup does not prove session load, authority admission, or readiness.

### Reclaim the lease immediately after timeout

Rejected because the replacement process can remain alive and claim late.

## Risks and Trade-offs

- Strict idle can delay handoff. This delay prevents duplicate active effects.
- A failed replacement creates an availability gap while teardown completes.
- Host authentication differs by platform. The contract records typed evidence and leaves platform mechanics in the shell.

## Non-Claims

This design does not prove PTY migration, provider-session transfer, process-memory continuity, sandboxing, remote security, crash freedom, power-loss durability, exactly-once effects, production readiness, or release eligibility.
