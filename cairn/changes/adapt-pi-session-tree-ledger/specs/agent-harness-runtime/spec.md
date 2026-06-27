# agent-harness-runtime Change Spec: Pi session tree ledger adaptation

## Requirements

### Requirement: Neutral session tree ledger contract

r[agent_harness_runtime.pi_session_tree.ledger_contract] Animus MUST define a neutral append-only session tree ledger contract for runtime-visible entries, parent links, active leaf selection, labels, summaries, replay metadata, and typed ledger errors without depending on Pi session files, a TUI, a database, provider crates, filesystem traversal, or process-global state.

#### Scenario: Ledger snapshot validates tree shape

r[agent_harness_runtime.pi_session_tree.ledger_contract.validate]
- GIVEN an in-memory ledger snapshot with entry IDs, parent links, and an active leaf
- WHEN the runtime validates the snapshot before replay
- THEN duplicate IDs, missing parents, parent cycles, invalid active leaves, and unsupported entry kinds fail with typed ledger errors
- AND no model or tool adapter is called.

### Requirement: Active-leaf replay context

r[agent_harness_runtime.pi_session_tree.replay_context] Animus MUST rebuild model-visible context by walking from the selected active leaf to the root, preserving root-to-leaf ordering and supported content semantics.

#### Scenario: Replay follows the active branch only

r[agent_harness_runtime.pi_session_tree.replay_context.branch]
- GIVEN a ledger with two branches from a shared ancestor
- WHEN runtime rebuilds context for one active leaf
- THEN only entries on that active branch contribute model-visible messages
- AND sibling branch messages remain available as ledger history but are not sent to the model.

### Requirement: Summary entries preserve bounded context

r[agent_harness_runtime.pi_session_tree.summary_entries] Animus MUST represent compaction summaries and branch handoff summaries as first-class ledger entries with bounded model-visible summary text and safe replay metadata.

#### Scenario: Summary entry replaces older model-visible span

r[agent_harness_runtime.pi_session_tree.summary_entries.replay]
- GIVEN a branch contains earlier messages, a compaction summary, and later kept messages
- WHEN replay context is rebuilt
- THEN the model-visible context includes the summary and the later kept messages in order
- AND it does not include the raw older span replaced by the summary.

### Requirement: Labels remain replay-neutral metadata

r[agent_harness_runtime.pi_session_tree.labels] Animus labels MUST attach operator-facing names to ledger entries without changing model-visible replay by default.

#### Scenario: Label does not steer model context

r[agent_harness_runtime.pi_session_tree.labels.replay_neutral]
- GIVEN a ledger entry has a label used for navigation
- WHEN runtime rebuilds model-visible context
- THEN the label is preserved in metadata queries
- AND the label text is not included in the model request unless an explicit host policy injects it.

### Requirement: Ledger service absence fails closed

r[agent_harness_runtime.pi_session_tree.missing_service] Runtime requests that require resume or replay semantics MUST fail closed with typed missing-ledger, missing-session, or unavailable-ledger-service errors when no suitable ledger service is injected.

#### Scenario: Resume without ledger service halts

r[agent_harness_runtime.pi_session_tree.missing_service.halts]
- GIVEN a runtime input requests resume from a prior session
- WHEN no ledger service is available
- THEN runtime emits a typed missing-ledger-service failure
- AND no Steel fabric, model adapter, or tool adapter is invoked.
