# Agent Harness Runtime

## ADDED Requirements

### Requirement: Conversation views remain derived projections

r[agent_harness_runtime.conversation_view_projection.boundary] Animus MUST project conversation views from one selected admitted ledger branch. Render caches and UI hosts MUST NOT gain model-context, ledger-mutation, tool-correlation, authority, or completion ownership.

#### Scenario: Admitted ledger snapshot is projected

r[agent_harness_runtime.conversation_view_projection.boundary.projected]
- GIVEN a host supplies one admitted session branch, leaf, ledger generation, ordered entries, projection policy, and optional active-turn state
- WHEN pure projection validation runs
- THEN Animus MUST return a deterministic snapshot, delta, rebuild plan, heal plan, no-op, or blocked outcome
- AND reusable code MUST NOT read storage, render a UI, inspect clocks, mutate the ledger, or contact a transport.

#### Scenario: Render cache is offered as model context

r[agent_harness_runtime.conversation_view_projection.boundary.no_reverse_authority]
- GIVEN a caller offers rendered items, UI cache rows, transport frames, or healed metadata as provider context or ledger truth
- WHEN projection or runtime admission runs
- THEN Animus MUST reject the reverse flow before model or tool dispatch
- AND it MUST NOT infer canonical conversation facts from display state.

### Requirement: Conversation projection identities bind canonical source facts

r[agent_harness_runtime.conversation_view_projection.identity] Animus MUST derive domain-separated BLAKE3 identities for projection streams, cursors, items, actions, snapshots, deltas, rebuilds, heals, and receipts from typed canonical source facts.

#### Scenario: Equal content and timestamps remain distinct

r[agent_harness_runtime.conversation_view_projection.identity.distinct]
- GIVEN two source entries have equal visible content or equal timestamps but different canonical entry or turn identities
- WHEN view-item identities are derived
- THEN their identities MUST remain distinct and deterministic
- AND content comparison, timestamps, row locations, random values, or parsed counts MUST NOT replace canonical source identity.

#### Scenario: Cursor source facts drift

r[agent_harness_runtime.conversation_view_projection.identity.cursor_drift]
- GIVEN a cursor differs in schema, policy, session, branch, leaf, generation, source anchor, sequence, or active-turn identity
- WHEN top-up admission runs
- THEN Animus MUST return a rebuild plan or blocked outcome
- AND it MUST NOT continue incrementally from the stale cursor.

### Requirement: Conversation projection streams are snapshot-first

r[agent_harness_runtime.conversation_view_projection.stream] A projection stream MUST emit one accepted redaction-safe snapshot before incremental updates. Every update MUST bind the stream, source revision, projection sequence, item identity, and mutation class.

#### Scenario: Snapshot starts a projection stream

r[agent_harness_runtime.conversation_view_projection.stream.snapshot]
- GIVEN one valid bounded source snapshot and admitted projection policy
- WHEN a host starts a projection stream
- THEN the first accepted record MUST be the complete redaction-safe snapshot for that stream identity
- AND later updates MUST start after the snapshot sequence and bind the same source lineage.

#### Scenario: Incremental stream order is invalid

r[agent_harness_runtime.conversation_view_projection.stream.invalid_order]
- GIVEN an update arrives before a snapshot, repeats or skips a sequence, names another stream, uses a stale revision, or carries an unsupported mutation
- WHEN stream validation runs
- THEN Animus MUST reject the update with a bounded typed diagnostic
- AND the update MUST NOT mutate cache, ledger, model context, tool state, or completion state.

### Requirement: One active turn maps to one stable assistant item

r[agent_harness_runtime.conversation_view_projection.active_turn] All admitted assistant entries for one exact active-turn identity MUST fold into one stable visible assistant item in canonical order. Interposed steering work MUST retain explicit ordered markers.

#### Scenario: Multiple provider turns fold into one item

r[agent_harness_runtime.conversation_view_projection.active_turn.fold]
- GIVEN one agent turn contains multiple ordered assistant entries with the same canonical turn and branch identity
- WHEN snapshot or delta projection runs
- THEN Animus MUST fold their admitted visible parts into one assistant item with one stable identity
- AND replay, reconnect, recovery, or terminal projection MUST NOT create duplicate items for that turn.

#### Scenario: Steering occurs between assistant entries

r[agent_harness_runtime.conversation_view_projection.active_turn.steering]
- GIVEN a valid steering entry occurs between two assistant entries of the same turn
- WHEN conversation projection runs
- THEN it MUST place one typed steering marker at the exact canonical position
- AND it MUST NOT convert the steering fact into assistant text, drop it during rebuild, or merge it by timestamp.

#### Scenario: Unrelated writer targets the same visible content

r[agent_harness_runtime.conversation_view_projection.active_turn.unrelated]
- GIVEN another source item has equal text but a different turn, branch, generation lineage, or policy identity
- WHEN cache action validation runs
- THEN it MUST receive a distinct item identity or a conflict outcome
- AND it MUST NOT replace the active-turn item through content equality.

### Requirement: Conversation caches rebuild through exact cursor evidence

r[agent_harness_runtime.conversation_view_projection.rebuild] Hosts MUST treat conversation caches as disposable projections. Top-up, rebuild, and heal actions MUST use exact revision, cursor, action, and resulting-state evidence.

#### Scenario: Matching cursor permits idempotent top-up

r[agent_harness_runtime.conversation_view_projection.rebuild.top_up]
- GIVEN a cache cursor exactly matches the projection schema, policy, source lineage, last entry, sequence, and active-turn state
- WHEN pure top-up planning runs
- THEN it MAY emit deterministic actions after the exact anchor
- AND applying an already applied action MUST be an exact no-op with no duplicate visible item.

#### Scenario: Stale or partial cache requires rebuild

r[agent_harness_runtime.conversation_view_projection.rebuild.stale]
- GIVEN the cursor is stale, an anchor is missing, a prior action is partial, cache content diverges, or source generation changed
- WHEN cache reconciliation runs
- THEN Animus MUST plan a full bounded rebuild or return a blocked outcome
- AND rebuild execution MUST clear only the declared cache after preflight without altering the source ledger.

#### Scenario: Legacy metadata has one exact source join

r[agent_harness_runtime.conversation_view_projection.rebuild.heal]
- GIVEN a legacy item lacks admitted metadata and exactly one validated source identity matches under current migration policy
- WHEN pure heal planning runs
- THEN Animus MAY add only admitted metadata while preserving item identity, role, visible parts, unknown safe metadata, and source ledger bytes
- AND ambiguous timestamps, equal content, missing identity, or active-turn races MUST produce no heal action.

### Requirement: Conversation projection evidence is metadata-only

r[agent_harness_runtime.conversation_view_projection.evidence] Projection receipts MUST contain bounded source revision, stream, cursor, action, item-count, fold, steering, rebuild, heal, cache-outcome, redaction, and non-claim metadata only.

#### Scenario: Safe projection receipt is emitted

r[agent_harness_runtime.conversation_view_projection.evidence.safe]
- GIVEN source validation, projection planning, and supplied cache evidence pass exact checks
- WHEN receipt projection runs
- THEN the receipt MUST bind policy, source, stream, cursor, projection, cache outcome, and receipt identities with bounded counts and classes
- AND it MUST describe only the supplied and validated projection facts.

#### Scenario: Projection receipt contains private data or an overclaim

r[agent_harness_runtime.conversation_view_projection.evidence.rejects]
- GIVEN a receipt draft includes transcript text, reasoning, tool bodies, provider payloads, credentials, environment values, physical paths, raw UI state, or unsupported truth and delivery claims
- WHEN receipt validation runs
- THEN it MUST reject the draft without echoing the private value
- AND the rejected receipt MUST NOT contribute to model context, completion, lifecycle, or release evidence.

### Requirement: Conversation projection has reconnect and rebuild evidence

r[agent_harness_runtime.conversation_view_projection.verification] Conversation projection MUST include deterministic positive and negative tests for identity, snapshot, delta, reconnect, active-turn folding, steering, cursor, cache, rebuild, heal, redaction, and receipt behavior.

#### Scenario: Positive projection matrix passes

r[agent_harness_runtime.conversation_view_projection.verification.positive]
- GIVEN fixtures cover initial snapshot, matching-cursor top-up, reconnect no-op, active-turn folding, steering, terminal replay, rebuild, and exact healing
- WHEN the focused projection rail runs
- THEN production reducers and host-cache entrypoints MUST return the expected deterministic actions and outcomes
- AND repeated runs over identical source facts MUST return identical identities, sequences, and ordered diagnostics.

#### Scenario: Adversarial projection attempt fails

r[agent_harness_runtime.conversation_view_projection.verification.negative]
- GIVEN a fixture attempts identity collision, incremental-before-snapshot, stale cursor, sequence gap, partial cache write, divergent writer, ambiguous heal, private projection, ledger mutation, or overclaiming
- WHEN the focused projection rail runs
- THEN the attempt MUST fail before the prohibited effect or claim
- AND ledger, model, tool, authority, completion, host-cache, and consumer boundaries MUST remain intact.

#### Scenario: Change is ready to archive

r[agent_harness_runtime.conversation_view_projection.verification.closeout]
- GIVEN maintainers intend to sync and archive the conversation-view projection change
- WHEN closeout validation runs
- THEN focused runtime, adapter, std-host, headless, TUI, Nickel, redaction, reconnect, rebuild, Cairn, and relevant Nix checks MUST pass
- AND transcript truth, UI delivery, cache durability, renderer correctness, task completion, and production readiness MUST remain explicit non-claims.
