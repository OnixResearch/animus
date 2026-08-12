# Agent Harness Runtime

## ADDED Requirements

### Requirement: Active-turn recovery keeps host authority explicit

r[agent_harness_runtime.active_turn_recovery.boundary] Animus MUST model active-turn recovery as pure validation and planning over supplied facts. Hosts MUST retain storage, transaction, clock, provider, tool, cancellation, and cleanup authority.

#### Scenario: Host supplies bounded recovery facts

r[agent_harness_runtime.active_turn_recovery.boundary.supplied]
- GIVEN a host supplies an admitted session, branch, policy, accepted-message sequence, journal snapshot, authority state, and terminal observations
- WHEN pure active-turn recovery validation runs
- THEN Animus MUST return a deterministic validated state, blocked outcome, or recovery plan
- AND reusable code MUST NOT read storage, allocate ambient identifiers, call providers, run tools, inspect clocks, or clean host state.

#### Scenario: Ambient recovery authority is requested

r[agent_harness_runtime.active_turn_recovery.boundary.ambient]
- GIVEN a caller expects reusable recovery code to discover a database, read a global session, use a clock, contact a provider, or execute a tool
- WHEN recovery admission runs
- THEN Animus MUST reject the request before the effect
- AND it MUST NOT substitute a default host service or infer missing authority.

### Requirement: Active-turn recovery policy is typed and bounded

r[agent_harness_runtime.active_turn_recovery.policy] Human-authored recovery policy MUST use typed Nickel contracts. Rust MUST revalidate generated policy before journal admission, recovery planning, provider continuation, or receipt projection.

#### Scenario: Complete recovery policy is admitted

r[agent_harness_runtime.active_turn_recovery.policy.valid]
- GIVEN generated policy names supported schemas, finite entry and aggregate bounds, replacement rules, corruption posture, retention, terminal classes, receipt fields, and non-claims
- WHEN Rust validates the policy
- THEN it MUST return a deterministic decision and domain-separated BLAKE3 policy identity
- AND later plans MUST bind that exact identity.

#### Scenario: Recovery policy is stale or incomplete

r[agent_harness_runtime.active_turn_recovery.policy.invalid]
- GIVEN policy is stale, malformed, unbounded, permits row skipping, omits transaction evidence, or weakens receipt and authority boundaries
- WHEN policy validation runs
- THEN validation MUST fail closed with bounded diagnostics
- AND no journal mutation, recovery continuation, provider dispatch, tool dispatch, cleanup, or receipt success may occur.

### Requirement: Durable acceptance precedes provider and steering delivery

r[agent_harness_runtime.active_turn_recovery.journal] A host MUST commit accepted user work before provider dispatch. It MUST commit steering work before runtime delivery and replace checkpoints through one evidence-bound atomic transaction.

#### Scenario: Accepted message becomes provider-eligible

r[agent_harness_runtime.active_turn_recovery.journal.accepted]
- GIVEN a valid user message has an admitted session, branch, turn sequence, policy, and expected prior journal generation
- WHEN the host durably commits the accepted message and active marker in one transaction
- THEN the host MAY request provider dispatch for the exact resulting turn identity
- AND provider dispatch MUST remain blocked without matching durable acceptance evidence.

#### Scenario: Checkpoint replacement is prepared and committed

r[agent_harness_runtime.active_turn_recovery.journal.replace]
- GIVEN every bounded replacement entry is fully prepared and its manifest matches the expected active turn and prior generation
- WHEN the host executes the compare-and-swap replacement transaction
- THEN it MUST expose one complete new generation or a typed blocked result
- AND it MUST NOT delete the prior valid checkpoint before replacement preparation completes.

#### Scenario: Steering message becomes delivery-eligible

r[agent_harness_runtime.active_turn_recovery.journal.steering]
- GIVEN an active turn accepts a valid steering message with the next monotonic steering sequence
- WHEN the host durably appends the exact steering identity
- THEN the runtime MAY attempt one correlated delivery under current policy
- AND missing, stale, duplicate, or mismatched append evidence MUST block delivery.

### Requirement: Stable recovery identities bind typed facts

r[agent_harness_runtime.active_turn_recovery.identity] Animus MUST derive domain-separated BLAKE3 identities for turns, checkpoints, steering work, recovery plans, and receipts from typed admitted facts and explicit monotonic sequences.

#### Scenario: Repeated content remains distinct

r[agent_harness_runtime.active_turn_recovery.identity.distinct]
- GIVEN two accepted messages have equal text or equal timestamps but distinct admitted turn sequences
- WHEN canonical turn identities are derived
- THEN the identities MUST remain distinct and deterministic
- AND timestamps, insertion order, random identifiers, or physical row locations MUST NOT serve as canonical content identity.

#### Scenario: Recovery identity drifts

r[agent_harness_runtime.active_turn_recovery.identity.drift]
- GIVEN supplied session, branch, sequence, policy, generation, entry manifest, or terminal facts differ from the bound recovery plan
- WHEN continuation admission runs
- THEN Animus MUST return a stale or mismatched outcome
- AND it MUST authorize no provider, tool, steering, cleanup, or completion effect.

### Requirement: Recovery composes without rerunning interrupted tools

r[agent_harness_runtime.active_turn_recovery.recovery] Active-turn recovery MUST preserve completed tool results and delegate dangling model-visible calls to the existing interrupted tool repair contract. It MUST NOT rerun tools.

#### Scenario: Failure occurs before first provider event

r[agent_harness_runtime.active_turn_recovery.recovery.pre_provider]
- GIVEN durable acceptance exists and no provider event or terminal turn record exists
- WHEN recovery evaluates the exact active checkpoint
- THEN it MUST preserve the accepted user message and MAY plan continuation under current authority and exact provider-plan gates
- AND it MUST retain the same canonical turn identity.

#### Scenario: Tool result already exists

r[agent_harness_runtime.active_turn_recovery.recovery.real_result]
- GIVEN the journal contains one valid model-visible tool call and its exact completed real result
- WHEN recovery planning runs
- THEN it MUST preserve that result and MAY plan model continuation
- AND it MUST NOT construct, authorize, or dispatch another tool request.

#### Scenario: Tool result is missing

r[agent_harness_runtime.active_turn_recovery.recovery.interrupted_tool]
- GIVEN the journal contains one valid model-visible tool call without a compatible terminal result
- WHEN recovery composes interrupted tool repair
- THEN only the existing evidence-bound synthetic failure plan MAY close the call before provider continuation
- AND active-turn recovery MUST NOT infer success, infer cancellation, or invoke the tool executor.

#### Scenario: Recovery state is corrupt or ambiguous

r[agent_harness_runtime.active_turn_recovery.recovery.corrupt]
- GIVEN records are malformed, missing, reordered, duplicated, mixed across generations, over budget, unsupported, stale, or terminally conflicting
- WHEN recovery validation runs
- THEN Animus MUST return a typed blocked outcome without skipping or guessing records
- AND no partial transcript, provider continuation, tool effect, steering delivery, cleanup, completion, or success receipt may occur.

### Requirement: Active-turn recovery evidence is metadata-only

r[agent_harness_runtime.active_turn_recovery.evidence] Recovery receipts MUST contain bounded identity, generation, count, crash-window, action, repair, terminal, redaction, and non-claim metadata only.

#### Scenario: Safe recovery receipt is projected

r[agent_harness_runtime.active_turn_recovery.evidence.safe]
- GIVEN a recovery plan and supplied host observations pass exact validation
- WHEN receipt projection runs
- THEN the receipt MUST bind policy, turn, checkpoint, plan, repair, terminal, and receipt identities with bounded status metadata
- AND it MUST describe only the validated supplied recovery facts.

#### Scenario: Receipt contains private data or an overclaim

r[agent_harness_runtime.active_turn_recovery.evidence.rejects]
- GIVEN a receipt draft includes messages, prompts, reasoning, tool bodies, provider data, credentials, environment values, physical paths, raw errors, or unsupported durability and completion claims
- WHEN receipt validation runs
- THEN it MUST reject the draft without echoing the private value
- AND the rejected receipt MUST NOT contribute to continuation, completion, lifecycle, or release evidence.

### Requirement: Active-turn recovery has positive and negative crash evidence

r[agent_harness_runtime.active_turn_recovery.verification] Active-turn recovery MUST include deterministic positive and negative tests for every declared crash window, transaction boundary, corruption class, no-rerun invariant, and receipt boundary.

#### Scenario: Declared crash windows pass

r[agent_harness_runtime.active_turn_recovery.verification.positive]
- GIVEN fixtures cover accepted work, checkpoint replacement, real tool results, interrupted calls, steering work, terminal commit, and cleanup interruption
- WHEN the focused recovery rail runs
- THEN production core and host-adapter entrypoints MUST return the expected deterministic plans and outcomes
- AND repeated runs over identical supplied facts MUST return identical canonical identities and ordered diagnostics.

#### Scenario: Adversarial recovery attempt fails

r[agent_harness_runtime.active_turn_recovery.verification.negative]
- GIVEN a fixture attempts pre-acceptance dispatch, partial replacement, stale replay, corruption, tool rerun, silent truncation, authority expansion, private receipt data, or overclaiming
- WHEN the focused recovery rail runs
- THEN the attempt MUST fail before the prohibited effect or claim
- AND existing session, repair, retry, tool, completion, and host authority boundaries MUST remain intact.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.active_turn_recovery.verification.closeout]
- GIVEN maintainers intend to sync and archive the active-turn recovery change
- WHEN closeout validation runs
- THEN focused runtime, adapter, std-host, Nickel, crash-window, composition, no-std, Wasm, Cairn, and relevant Nix checks MUST pass
- AND store correctness, power-loss durability, provider acceptance, exactly-once effects, task completion, and production readiness MUST remain explicit non-claims.
