# Persist-Before-Process Gateway Discipline

## ADDED Requirements

### Requirement: Gateway work is persisted before effects

r[animus.gateway.persist] The gateway MUST commit each incoming update before agent processing begins. It MUST commit each pending reply intent before transport delivery begins. Each record MUST bind stable conversation, work, and content identities.

#### Scenario: Incoming update is admitted first

r[animus.gateway.persist.incoming]
- GIVEN a remote update arrives for one conversation
- WHEN the gateway admits the update
- THEN durable admission MUST complete before the agent turn starts

#### Scenario: Reply intent is admitted first

r[animus.gateway.persist.reply]
- GIVEN an agent turn produces a reply
- WHEN the gateway prepares delivery
- THEN the pending reply intent MUST be durable before the transport send begins

### Requirement: Gateway ordering is per conversation and bounded

r[animus.gateway.order] The gateway MUST process admitted updates in order within one conversation. Separate conversations MAY progress concurrently only within an explicit host capacity bound.

#### Scenario: Same-conversation order is preserved

r[animus.gateway.order.same]
- GIVEN two admitted updates belong to one conversation
- WHEN the gateway schedules processing
- THEN the earlier update MUST complete, park, or cancel before the later update starts

#### Scenario: Separate conversations progress within the bound

r[animus.gateway.order.different]
- GIVEN admitted updates belong to separate conversations
- WHEN host capacity is available
- THEN the gateway MAY process them concurrently without exceeding the named bound

### Requirement: Restart recovery is idempotent and uncertainty-preserving

r[animus.gateway.resume] Restart recovery MUST reconstruct pending updates and reply intents from durable records. It MUST prevent duplicate processing and MUST preserve uncertain delivery as uncertain.

#### Scenario: Pending work resumes after restart

r[animus.gateway.resume.restart]
- GIVEN an admitted update or reply intent remains pending when the worker stops
- WHEN the worker restarts
- THEN recovery MUST return the pending item to its declared next state without losing identity

#### Scenario: Duplicate processing and false delivery fail

r[animus.gateway.resume.duplicate]
- GIVEN recovery proposes duplicate processing or marks an uncertain reply as delivered
- WHEN the recovery plan is admitted
- THEN admission MUST fail and report the conflicting identity or state

### Requirement: Remote mutation is denied by default

r[animus.gateway.authority] A remote gateway session MUST expose read-only tools by default. A mutating tool MUST require exact current authority for the session, remote-user binding, tool, and request.

#### Scenario: Mutation requires exact authority

r[animus.gateway.authority.mutation]
- GIVEN a remote gateway session requests a mutating tool
- WHEN exact current authority is absent
- THEN the request MUST fail closed
- AND transport authentication alone MUST NOT grant mutation authority

### Requirement: Gateway verification is complete

r[animus.gateway.verification] Positive, negative, and boundary fixtures MUST cover persist-before-process admission, reply intent, per-conversation order, bounded concurrency, restart recovery, duplicate denial, delivery uncertainty, and mutation authority.

#### Scenario: Gateway discipline matrix passes

r[animus.gateway.verification.matrix]
- GIVEN the gateway rails, fixtures, and documentation are complete
- WHEN the focused validation runs
- THEN conforming records and plans MUST behave as declared
- AND every admission, ordering, recovery, delivery, or authority violation MUST fail as declared
