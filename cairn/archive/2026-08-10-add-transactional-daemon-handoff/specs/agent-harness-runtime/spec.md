# Agent Harness Runtime

## ADDED Requirements

### Requirement: Daemon handoff starts only from exact strict idle

r[agent_harness_runtime.daemon_handoff.idle] Animus MUST define a pure strict-idle decision that binds worker, session, branch, lease generation, queue generations, active-turn generation, and authority profile. Strict idle MUST require no active provider, tool, repair, confirmation, cancellation, queue, child, steering, durable-commit, or terminal-transition work.

#### Scenario: Worker is strictly idle

r[agent_harness_runtime.daemon_handoff.idle.valid]
- GIVEN all bound work classes are absent or terminal and every supplied generation is current
- WHEN strict-idle validation runs
- THEN Animus MAY produce one handoff-preparation decision for the exact worker and session
- AND a display status or empty primary queue alone MUST NOT establish strict idle.

#### Scenario: Any work remains active

r[agent_harness_runtime.daemon_handoff.idle.busy]
- GIVEN a provider, tool, repair, confirmation, cancellation, primary queue, child queue, steering entry, durable commit, or terminal transition remains active or ambiguous
- WHEN strict-idle validation runs
- THEN handoff MUST be denied before ingress pause or lease mutation
- AND no subset of the idle facts may be treated as sufficient.

### Requirement: Handoff transitions are monotonic and generation-bound

r[agent_harness_runtime.daemon_handoff.transaction] Animus MUST define prepared, ingress-paused, lease-released, replacement-started, claimed, ready, cancelled, lease-restored, and complete states as monotonic transitions. Every transition MUST bind the prior transition identity and exact current host observations.

#### Scenario: Successful transaction advances in order

r[agent_harness_runtime.daemon_handoff.transaction.success]
- GIVEN strict idle passed and each required durable, lease, process, claim, and readiness observation arrives in order
- WHEN handoff reduction runs
- THEN the transaction MUST advance through one canonical state sequence and BLAKE3 identity chain
- AND the old worker MUST exit only after exact replacement readiness commits.

#### Scenario: Transition is skipped or replayed

r[agent_harness_runtime.daemon_handoff.transaction.invalid]
- GIVEN a request skips a state, changes a bound identity, reuses an earlier observation, moves backward, or repeats a non-idempotent transition
- WHEN transition admission runs
- THEN Animus MUST return stale, conflicting, or invalid before the dependent effect
- AND prior committed transition evidence MUST remain authoritative.

### Requirement: Replacement claims are single, current, and authenticated

r[agent_harness_runtime.daemon_handoff.claim] A replacement claim MUST bind one current handoff identity, session identity, released lease generation, replacement process identity, endpoint identity, protocol version, freshness nonce, and host authentication observation. At most one claimant MAY be admitted.

#### Scenario: Current replacement claims the lease

r[agent_harness_runtime.daemon_handoff.claim.allowed]
- GIVEN the released lease, current handoff, replacement process, endpoint, protocol, freshness, and authentication observations match exactly
- WHEN claim admission runs
- THEN Animus MAY bind that claimant to the handoff generation
- AND readiness MUST still require durable session, vault, authority, recovery, and service admission.

#### Scenario: Claimant is stale, duplicate, or unauthenticated

r[agent_harness_runtime.daemon_handoff.claim.denied]
- GIVEN a claim has wrong session, generation, process, endpoint, protocol, nonce, authentication, duplicate claimant, cancelled handoff, or terminal handoff
- WHEN claim admission runs
- THEN Animus MUST deny before lease ownership or readiness
- AND same-user access, endpoint reachability, or process startup MUST NOT replace exact claim evidence.

### Requirement: Cancellation terminates the replacement before lease restoration

r[agent_harness_runtime.daemon_handoff.cancellation] Deadline or operator cancellation MUST commit cancelled state before replacement termination. The old worker MUST NOT restore the lease until terminal evidence covers the complete owned replacement process tree.

#### Scenario: Replacement times out after lease release

r[agent_harness_runtime.daemon_handoff.cancellation.timeout]
- GIVEN a replacement misses readiness after the old worker released the lease
- WHEN the deadline outcome is admitted
- THEN Animus MUST commit cancelled state and request owned process-tree termination
- AND old-worker lease restoration MUST remain blocked until complete terminal process evidence arrives.

#### Scenario: Replacement descendant survives

r[agent_harness_runtime.daemon_handoff.cancellation.survivor]
- GIVEN the replacement leader exits but an owned descendant remains live or termination evidence is incomplete
- WHEN rollback admission runs
- THEN Animus MUST keep the handoff blocked and the old lease unrecovered
- AND it MUST not infer complete teardown from leader exit.

#### Scenario: Delayed replacement claims after cancellation

r[agent_harness_runtime.daemon_handoff.cancellation.late_claim]
- GIVEN cancelled state committed before a delayed replacement sends its claim
- WHEN claim admission runs
- THEN the claim MUST be rejected as terminally stale
- AND it MUST not reactivate the handoff or acquire a later lease generation.

### Requirement: Handoff preserves canonical session input state without PTY transfer

r[agent_harness_runtime.daemon_handoff.session_state] Before lease release, the host MUST commit the exact session head, composer-draft identity, pending-input vault generation, active-turn generation, authority profile, and handoff record. Replacement readiness MUST validate those facts without claiming PTY, file-descriptor, provider-socket, credential, process-memory, or render-cache transfer.

#### Scenario: Replacement validates durable session state

r[agent_harness_runtime.daemon_handoff.session_state.ready]
- GIVEN the claimant loads the exact committed session head, visible vault generation, authority profile, and active-turn recovery state
- WHEN readiness admission runs
- THEN Animus MAY commit ready state for the claimed replacement
- AND the old worker MAY exit only after that matching commit.

#### Scenario: Durable state differs from the handoff record

r[agent_harness_runtime.daemon_handoff.session_state.drift]
- GIVEN the session head, draft, vault, active-turn, authority, policy, or service readiness differs from the committed handoff record
- WHEN readiness admission runs
- THEN readiness MUST fail closed and cancellation or operator recovery MUST remain explicit
- AND the host MUST NOT claim conversation continuity from process or endpoint identity alone.

### Requirement: Daemon handoff evidence is metadata-only

r[agent_harness_runtime.daemon_handoff.evidence] Handoff receipts MUST contain bounded worker, session, lease, authority, handoff, endpoint, process-attempt, transition, cancellation, readiness, vault-generation, redaction, non-claim, and BLAKE3 identity metadata only.

#### Scenario: Safe handoff receipt is projected

r[agent_harness_runtime.daemon_handoff.evidence.safe]
- GIVEN a handoff transition passes exact validation
- WHEN receipt projection runs
- THEN the receipt MUST bind the validated transition and mandatory non-claims
- AND it MUST not claim PTY migration, process-memory continuity, sandboxing, remote security, or production readiness.

#### Scenario: Receipt contains session or process secrets

r[agent_harness_runtime.daemon_handoff.evidence.rejects]
- GIVEN a receipt candidate contains composer text, pending input, prompts, transcripts, credentials, socket paths, process arguments, provider data, terminal data, or unrestricted errors
- WHEN receipt validation runs
- THEN Animus MUST reject it without echoing the value
- AND the candidate MUST NOT enter session, completion, lifecycle, or release evidence.

### Requirement: Transactional handoff includes adversarial process evidence

r[agent_harness_runtime.daemon_handoff.verification] The change MUST include positive and negative production-path tests for strict idle, monotonic transitions, lease exclusivity, claim authentication, readiness, stale claims, cancellation, process-tree termination, rollback, session-state drift, safe receipts, feature isolation, and a non-default local smoke.

#### Scenario: Complete handoff corpus passes

r[agent_harness_runtime.daemon_handoff.verification.complete]
- GIVEN fixtures cover success, every busy class, duplicate claims, stale generations, wrong endpoints, readiness failure, timeouts, surviving descendants, rollback drift, and leaks
- WHEN focused core, daemon-adapter, host-shell, process, and packaged-smoke checks run
- THEN allowed cases MUST produce exact deterministic transitions and denied cases MUST fail before duplicate ownership or unsupported claims
- AND the smoke MUST use local fake services without remote transport.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.daemon_handoff.verification.incomplete]
- GIVEN maintainers intend to archive without process-tree, late-claim, lease-restoration, state-drift, leak, dependency-isolation, Cairn, or relevant Nix evidence
- WHEN closeout validation runs
- THEN archive readiness MUST remain blocked with the missing evidence or exact unavailable-rail blocker
- AND Tile integration, PTY migration, remote handoff, sandboxing, and production readiness MUST remain explicit non-claims.
