# Agent Harness Runtime

## ADDED Requirements

### Requirement: Pending-input vault planning keeps host authority explicit

r[agent_harness_runtime.pending_input_vault.boundary] Animus MUST model pending-input vault validation, ordering, recovery, and disposition as pure deterministic decisions over supplied facts. Hosts MUST retain storage, transaction, body, clock, synchronization, operator, and cleanup authority.

#### Scenario: Host supplies one conversation vault

r[agent_harness_runtime.pending_input_vault.boundary.supplied]
- GIVEN a host supplies one admitted conversation, branch, store, authority profile, policy, vault generation, and bounded pending-entry set
- WHEN pure vault validation runs
- THEN Animus MUST return a deterministic no-vault, review-required, blocked, or disposition state
- AND reusable code MUST NOT read storage, inspect clocks, submit input, dispatch providers, or delete host state.

#### Scenario: Ambient vault discovery is requested

r[agent_harness_runtime.pending_input_vault.boundary.ambient]
- GIVEN a caller omits the conversation or explicit store and expects recovery to discover global pending input
- WHEN vault admission runs
- THEN Animus MUST reject before any effect
- AND it MUST NOT substitute a default store or another conversation.

### Requirement: Vault contents preserve class, order, provenance, and identity

r[agent_harness_runtime.pending_input_vault.contents] A pending-input vault MUST preserve active accepted input, in-flight steering, queued steering, follow-up, command, and bounded attachment-reference classes with exact monotonic order, provenance, conversation scope, and domain-separated BLAKE3 identities.

#### Scenario: Mixed pending input is projected

r[agent_harness_runtime.pending_input_vault.contents.ordered]
- GIVEN durable facts contain valid entries from multiple pending-input classes under one conversation and generation
- WHEN vault projection runs
- THEN the projection MUST retain each original class, identity, sequence, provenance, and attachment reference
- AND it MUST NOT convert entries into transcript messages or reorder them by timestamp, body, or storage location.

#### Scenario: Entry identity or sequence conflicts

r[agent_harness_runtime.pending_input_vault.contents.conflict]
- GIVEN two entries reuse an identity with different facts or contain duplicate, missing, reversed, or cross-conversation sequences
- WHEN vault validation runs
- THEN Animus MUST return a typed conflict or corruption outcome
- AND it MUST expose no partial ordered projection as recoverable input.

### Requirement: Recovered input requires explicit review and new admission

r[agent_harness_runtime.pending_input_vault.recovery] Restart recovery MUST quarantine pending input for explicit inspect, restore, or discard action. It MUST NOT submit, dispatch, inject, or make recovered input model-visible automatically.

#### Scenario: Valid vault requires review

r[agent_harness_runtime.pending_input_vault.recovery.review]
- GIVEN restart validation finds a complete non-empty vault with current identities and authority profile
- WHEN startup recovery completes
- THEN Animus MUST return `ReviewRequired` with bounded metadata inspection
- AND no provider, queue, transcript, tool, or messaging effect may result.

#### Scenario: Operator restores selected entries

r[agent_harness_runtime.pending_input_vault.recovery.restore]
- GIVEN an authorized operator selects current vault entries for restore
- WHEN the disposition transaction commits
- THEN Animus MAY emit normal coding-session admission requests that reference the original entry identities
- AND every restored entry MUST pass current queue, authority, policy, durable-commit, and provider gates before model visibility.

### Requirement: Vault dispositions are generation-fenced and idempotent

r[agent_harness_runtime.pending_input_vault.transactions] Restore, discard, repair, and cleanup plans MUST bind the exact vault, conversation, branch, store, authority, expected generation, selected entries, and disposition identity. Hosts MUST record disposition evidence before vault deletion.

#### Scenario: Disposition repeats after a crash

r[agent_harness_runtime.pending_input_vault.transactions.repeat]
- GIVEN a completed disposition is retried with the same identity and exact facts after a restart
- WHEN transaction validation runs
- THEN Animus MUST return the prior completed outcome without creating new admission or deletion work
- AND no pending entry may be restored or discarded twice.

#### Scenario: Disposition facts drift

r[agent_harness_runtime.pending_input_vault.transactions.drift]
- GIVEN a request changes the generation, selected entries, conversation, store, authority profile, or disposition facts
- WHEN disposition admission runs
- THEN Animus MUST return stale or conflicting before mutation
- AND the current vault and prior disposition evidence MUST remain authoritative.

#### Scenario: Cleanup fails after disposition commit

r[agent_harness_runtime.pending_input_vault.transactions.cleanup]
- GIVEN restore or discard evidence committed but host cleanup did not complete
- WHEN restart recovery runs
- THEN Animus MUST preserve the completed disposition and return cleanup-required state
- AND it MUST NOT repeat admission, change disposition, or report vault deletion.

### Requirement: Corruption blocks ordinary ingress but retains recovery controls

r[agent_harness_runtime.pending_input_vault.corruption] Malformed, missing, reordered, duplicated, mixed-generation, over-budget, cross-conversation, or uncertain vault state MUST block ordinary input for the affected conversation without hiding bounded inspect, quarantine, exact repair, or authorized discard controls.

#### Scenario: Vault is corrupt during startup

r[agent_harness_runtime.pending_input_vault.corruption.blocked]
- GIVEN vault records are incomplete, ambiguous, unsupported, mismatched, or over budget
- WHEN startup validation runs
- THEN Animus MUST return `BlockedCorrupt` and deny prompt, continue, steer, follow-up, and automatic dispatch
- AND it MUST NOT skip invalid entries or import a valid-looking suffix.

#### Scenario: Recovery control targets corrupt state

r[agent_harness_runtime.pending_input_vault.corruption.control]
- GIVEN an authorized recovery request names the exact blocked vault and current generation
- WHEN inspect, quarantine export, repair import, or destructive discard admission runs
- THEN Animus MAY return one bounded recovery plan under separate policy
- AND the plan MUST grant no provider, transcript, tool, messaging, or unrelated-conversation authority.

### Requirement: Pending-input vault evidence is metadata-only

r[agent_harness_runtime.pending_input_vault.evidence] Vault receipts MUST contain bounded vault, conversation, branch, store, authority, policy, generation, count, class, disposition, corruption, cleanup, redaction, non-claim, and BLAKE3 identity metadata only.

#### Scenario: Safe vault receipt is projected

r[agent_harness_runtime.pending_input_vault.evidence.safe]
- GIVEN a vault state or disposition passes exact validation
- WHEN receipt projection runs
- THEN the receipt MUST bind the validated identities, counts, classes, outcome, and mandatory non-claims
- AND it MUST not assert provider delivery, exactly-once effects, store correctness, or power-loss durability.

#### Scenario: Receipt contains private input or overclaims

r[agent_harness_runtime.pending_input_vault.evidence.rejects]
- GIVEN a receipt candidate contains input bodies, attachment content, prompts, tool data, provider data, credentials, environment values, physical paths, raw store errors, or unsupported durability claims
- WHEN receipt validation runs
- THEN Animus MUST reject the candidate without echoing the private value
- AND the candidate MUST NOT enter session, completion, lifecycle, or release evidence.

### Requirement: Pending-input recovery includes positive and negative evidence

r[agent_harness_runtime.pending_input_vault.verification] The change MUST include deterministic positive and negative tests for every pending-input class, ordering rule, restart state, disposition transaction, corruption class, conversation boundary, no-auto-submit invariant, and receipt boundary.

#### Scenario: Complete restart corpus passes

r[agent_harness_runtime.pending_input_vault.verification.complete]
- GIVEN fixtures cover mixed-class recovery, inspection, selected restore, discard, repeated crashes, cleanup retry, and empty vaults
- WHEN focused production core and host checks run
- THEN allowed cases MUST return exact deterministic states, identities, and ordered plans
- AND denied cases MUST produce no provider, transcript, queue, tool, messaging, or unrelated-store effect.

#### Scenario: Closeout evidence is incomplete

r[agent_harness_runtime.pending_input_vault.verification.incomplete]
- GIVEN maintainers intend to archive without corruption, duplicate-recovery, cross-conversation, leak, feature-isolation, Cairn, or relevant Nix evidence
- WHEN closeout validation runs
- THEN archive readiness MUST remain blocked with the missing evidence or exact unavailable-rail blocker
- AND production readiness, power-loss durability, and exactly-once behavior MUST remain explicit non-claims.
