# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Durable session storage is an explicit host authority

r[agent_harness_runtime.durable_session_store.boundary] Animus MUST expose the local durable session store only through a non-default authority-scoped host block requiring an explicit capability location, store/profile identity, session and branch scope, bounds, durability policy, lock policy, private-data posture, encryption posture, and receipt policy.

#### Scenario: Explicit local store is admitted

- GIVEN a host supplies a capability-scoped location and a complete supported store descriptor
- WHEN store admission runs
- THEN the host MAY open the selected local store under the declared policies
- AND reusable crates MUST receive only validated store data and typed outcomes.

#### Scenario: Ambient store fallback is requested

- GIVEN no capability location is supplied or the request relies on home, current-directory, environment, or global-session discovery
- WHEN store admission runs
- THEN it MUST fail before filesystem access
- AND Animus MUST NOT create or select a default global session store.

### Requirement: Private replay content is separate from safe ledger evidence

r[agent_harness_runtime.durable_session_store.private_journal] The durable host MUST separate replay-private model-visible payload records from metadata-only safe ledger and receipt projections, bind the channels with BLAKE3 identities, and prevent private bodies from entering receipts, diagnostics, or default exports.

#### Scenario: Private message is committed

- GIVEN an admitted model-visible message must survive restart
- WHEN the host commits it
- THEN the private journal MAY store the bounded admitted body under the declared private-data posture
- AND the safe ledger sidecar MUST contain only kind, order, counts, redaction class, safe identities, and receipt references.

#### Scenario: Encryption is required but unavailable

- GIVEN store policy requires encryption and no explicit admitted encryption service is supplied
- WHEN the host opens or appends to the private journal
- THEN it MUST fail closed
- AND it MUST NOT discover keys or silently downgrade to unencrypted storage.

### Requirement: Session records use versioned bounded framing

r[agent_harness_runtime.durable_session_store.framing] Durable session records MUST use explicit versioned framing with record kind, logical sequence, branch and parent identity, bounded payload length, payload BLAKE3 hash, prior committed-frame hash, and a validated frame identity.

#### Scenario: Valid frame round trips

- GIVEN a supported record kind and bounded canonical payload
- WHEN pure frame encoding and decoding run
- THEN the decoded frame MUST preserve every validated field and identity
- AND repeated encoding MUST produce identical bytes.

#### Scenario: Frame length or hash is malformed

- GIVEN a frame declares an unsupported version, zero or excessive length, truncated payload, invalid BLAKE3 hash, wrong prior hash, or trailing bytes outside its frame
- WHEN frame validation runs
- THEN it MUST return a typed framing or integrity error
- AND the payload MUST NOT become replay-visible.

### Requirement: Record identities are deterministic and clock-independent

r[agent_harness_runtime.durable_session_store.identity] Frame, transaction, branch, snapshot, active-leaf, and receipt identities MUST use domain-separated BLAKE3 over canonical validated inputs and MUST NOT depend on wall clocks, random UUIDs, ambient paths, locale, or process-global state.

#### Scenario: Identical transaction inputs repeat

- GIVEN identical validated frames, parent head, branch, profile, and commit inputs
- WHEN identity construction runs in separate processes or at different host times
- THEN the transaction and commit identities MUST match
- AND optional host time observations MUST NOT affect replay ordering or identity.

### Requirement: Message and active-leaf updates commit atomically at the logical layer

r[agent_harness_runtime.durable_session_store.commit] The durable store MUST group session entries and their proposed active-leaf transition into bounded transactions, recognize only validated commit frames as durable heads, and satisfy the declared synchronization policy before reporting a durable commit.

#### Scenario: Message and leaf commit succeeds

- GIVEN a valid message frame group and proposed leaf are appended under the durable profile
- WHEN the commit frame is written and the declared data synchronization completes
- THEN readers MUST select the committed leaf and all frames in the group
- AND dependent provider or tool effects MAY proceed only after that success.

#### Scenario: Crash occurs before commit frame

- GIVEN some transaction frames are present but no valid commit frame binds them
- WHEN the store is reopened
- THEN the frames MUST remain uncommitted and non-model-visible
- AND the prior committed active leaf MUST remain authoritative.

### Requirement: Local store admits only one writer

r[agent_harness_runtime.durable_session_store.locking] The local durable host MUST require one exclusive writer lease per store and MUST reject conflicting writers, stale lease identities, unsupported locking semantics, and last-writer-wins merge behavior.

#### Scenario: Second writer opens the store

- GIVEN one valid writer lease is active
- WHEN another writer requests mutation authority for the same store
- THEN the request MUST fail before append
- AND neither writer's branch head or hash chain may be merged implicitly.

#### Scenario: Reader opens committed snapshot

- GIVEN a writer has a valid committed head
- WHEN a bounded reader requests a snapshot
- THEN the reader MAY observe the validated committed prefix
- AND it MUST NOT treat uncommitted trailing data as part of the snapshot.

### Requirement: Recovery distinguishes tails from committed corruption

r[agent_harness_runtime.durable_session_store.recovery] Recovery MUST classify clean end, incomplete trailing frame, incomplete trailing transaction, stale uncommitted head, sequence gap, committed hash mismatch, unsupported schema, and mid-file corruption, and MAY ignore or quarantine only uncommitted trailing fragments while failing closed on committed corruption or ambiguous heads.

#### Scenario: Final frame is torn

- GIVEN the file ends with an incomplete frame after a valid committed head
- WHEN recovery runs
- THEN it MAY quarantine or ignore only that uncommitted tail with a safe diagnostic
- AND replay MUST stop at the prior committed head without silently claiming the tail durable.

#### Scenario: Committed middle frame is corrupt

- GIVEN a frame inside the committed hash chain has a sequence or hash mismatch
- WHEN recovery runs
- THEN store opening MUST fail closed
- AND the host MUST NOT skip, truncate, rewrite, or compact past the corruption automatically.

### Requirement: Durable-store receipts are metadata-only

r[agent_harness_runtime.durable_session_store.receipts] Durable-store receipts MUST contain only safe store/profile labels or hashes, session/branch/head identities, operation and durability classes, frame/transaction counts, lock and recovery status, redaction markers, non-claims, and domain-separated BLAKE3 receipt identity.

#### Scenario: Successful commit receipt is built

- GIVEN a transaction is durably committed under a declared profile
- WHEN receipt projection runs
- THEN the receipt MUST bind the prior and new head, committed counts, durability class, and redaction state
- AND it MUST not imply encryption, power-loss proof, filesystem correctness, backup, or replication.

#### Scenario: Receipt input leaks private journal data

- GIVEN receipt input contains a message body, tool result, provider payload, credential, capability path, store path, raw frame payload, lock debug dump, or unbounded error text
- WHEN receipt validation runs
- THEN it MUST reject the receipt without echoing the private value.

### Requirement: Durable store behavior has fake and file evidence

r[agent_harness_runtime.durable_session_store.verification] The durable session store MUST run a shared fake/file conformance corpus plus file-specific positive and negative tests for framing, grouped commits, replay, branches, repair, restart, tails, corruption, locks, permissions, synchronization, encryption posture, bounds, and receipt safety.

#### Scenario: Durable profile is ready for archive

- GIVEN maintainers intend to archive the durable-store change
- WHEN closeout checks run
- THEN focused fake and file tests, restart fault injection, crate isolation, safe-receipt and neutral-ledger rails, dogfood claim hygiene, Cairn gates, and relevant Nix checks MUST pass
- AND filesystem-specific checks that cannot run MUST remain explicit blockers for the corresponding claim.
