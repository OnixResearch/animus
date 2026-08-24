# Generation-Scoped Response Ownership

## ADDED Requirements

### Requirement: One exchange generation owns a reusable response connection

r[animus.ownership.scope] Every reusable streaming response connection MUST carry one generation-scoped owner before frame handling begins. The runtime MUST reject a second active owner for the same connection.

#### Scenario: One generation claims ownership

r[animus.ownership.scope.claim]
- GIVEN a reusable response connection has no active owner
- WHEN one exchange generation claims the connection
- THEN ownership MUST be recorded before frames are accepted

#### Scenario: Concurrent owner fails

r[animus.ownership.scope.concurrent]
- GIVEN a response connection has an active generation owner
- WHEN another generation attempts to claim it
- THEN the second claim MUST fail closed

### Requirement: Uncertain exchanges poison the connection

r[animus.ownership.poison] Interruption, malformed framing, ownership mismatch, or return before the admitted terminal frame MUST poison the reusable connection. A poisoned connection MUST NOT be offered to a later exchange.

#### Scenario: Interrupted exchange poisons the connection

r[animus.ownership.poison.interrupted]
- GIVEN an owned streaming exchange is interrupted before terminal completion
- WHEN the ownership reducer handles the interruption
- THEN the connection MUST become poisoned
- AND the host MUST discard or close it

#### Scenario: Late frame from an old generation fails

r[animus.ownership.poison.stale_frame]
- GIVEN a connection has moved to a later generation
- WHEN a frame carries the old generation identity
- THEN the frame MUST be rejected
- AND the connection MUST NOT continue as reusable

### Requirement: Only terminal completion releases ownership

r[animus.ownership.release] Generation ownership MUST be released only after the adapter observes its admitted terminal frame. End-of-read, timeout, cancellation, or parser error MUST NOT imply terminal completion.

#### Scenario: Terminal frame permits reuse

r[animus.ownership.release.terminal]
- GIVEN an exchange receives its admitted terminal frame
- WHEN the ownership reducer completes the exchange
- THEN ownership MUST be released
- AND the connection MAY become reusable

#### Scenario: Early return cannot release ownership

r[animus.ownership.release.early]
- GIVEN an exchange returns before its admitted terminal frame
- WHEN the host requests connection reuse
- THEN reuse MUST be denied and the connection MUST be poisoned

### Requirement: Response-ownership verification is complete

r[animus.ownership.verification] Positive, negative, and boundary fixtures MUST cover ownership claims, concurrent claims, interruption, malformed frames, stale frames, terminal release, and early return.

#### Scenario: Response-ownership matrix passes

r[animus.ownership.verification.matrix]
- GIVEN the ownership rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming exchanges MUST behave as declared
- AND every concurrent-owner, stale-frame, or early-reuse input MUST fail as declared
