# Agent Harness Runtime

## ADDED Requirements

### Requirement: Conversation Shareable View projection is an optional adapter [r[agent_harness_runtime.shareable_view_projection.boundary]]

Animus MUST project conversations into Shareable Views only through a non-default adapter over one admitted redacted conversation projection. Animus MUST retain session, ledger, branch, item, cursor, model, tool, and completion ownership.

#### Scenario: Exact external contract is selected [r[agent_harness_runtime.shareable_view_projection.boundary.external]]

- GIVEN a host selects Shareable View projection
- WHEN adapter activation runs
- THEN Animus MUST require one exact published Tile contract revision, schema profile, fixture identity, bounds set, and compatibility result
- AND an ambient sibling path, draft branch, screenshot, or undocumented DTO MUST NOT satisfy activation

#### Scenario: Source projection maps to target state [r[agent_harness_runtime.shareable_view_projection.boundary.mapping]]

- GIVEN one admitted redacted conversation snapshot and compatible external profile
- WHEN pure mapping runs
- THEN Animus MUST return bounded application state, target manifest facts, identity links, and a publication plan
- AND the adapter MUST NOT render, transport, persist, contact providers, execute tools, or mutate the source ledger

#### Scenario: Boundary ownership is reviewed [r[agent_harness_runtime.shareable_view_projection.boundary.ownership]]

- GIVEN the projection contract changes
- WHEN architecture validation runs
- THEN each source field, target field, identity, effect, authority input, and receipt MUST have one owner
- AND Tile presentation or transport facts MUST NOT redefine Animus domain meaning

### Requirement: Projection preserves separate identity domains [r[agent_harness_runtime.shareable_view_projection.identity]]

Animus MUST preserve session, branch, leaf, ledger-generation, projection-policy, stream, cursor, item, turn, and action identities as nominal Animus facts. It MUST link them to nominal Tile object and snapshot identities without substitution.

#### Scenario: Mapping receipt links exact domains [r[agent_harness_runtime.shareable_view_projection.identity.linkage]]

- GIVEN a valid source projection produces one Shareable View snapshot
- WHEN mapping receipt construction runs
- THEN the receipt MUST bind exact source and target identities, adapter profile, external revision, redaction, counts, and non-claims
- AND it MUST NOT claim transcript truth, delivery, rendering, accessibility, model context, or action success

#### Scenario: Equal text or timestamps appear

- GIVEN distinct conversation items have equal visible text or timestamps
- WHEN Shareable View state is derived
- THEN the items MUST retain distinct canonical Animus identities and deterministic order
- AND content equality, timestamp equality, Tile placement, or display order MUST NOT merge them

### Requirement: Immutable conversation snapshots can be published [r[agent_harness_runtime.shareable_view_projection.snapshot]]

Animus MUST map one complete redacted conversation projection snapshot into one immutable Shareable View state artifact and metadata-only mapping receipt.

#### Scenario: Snapshot export succeeds [r[agent_harness_runtime.shareable_view_projection.snapshot.export]]

- GIVEN one bounded admitted projection snapshot and current export policy
- WHEN pure snapshot export runs
- THEN Animus MUST preserve item order, branch lineage, active-turn summary, steering markers, safe action references, and redaction facts
- AND it MUST exclude provider payloads, hidden reasoning, tool bodies, credentials, paths, terminal bytes, and renderer state

#### Scenario: Host publishes the snapshot [r[agent_harness_runtime.shareable_view_projection.snapshot.publication]]

- GIVEN the pure adapter returns an admitted publication plan
- WHEN an injected host port publishes the snapshot
- THEN the host MUST report a bounded success, denial, failure, timeout, cancellation, or uncertain outcome
- AND publication success MUST NOT become ledger, model, tool, completion, or authority evidence

### Requirement: Live conversation views remain snapshot-first [r[agent_harness_runtime.shareable_view_projection.live]]

Animus MUST start each live Shareable View producer generation with one complete snapshot. It MUST bind each later delta to exact source and target predecessor facts.

#### Scenario: Live delta is current [r[agent_harness_runtime.shareable_view_projection.live.stream]]

- GIVEN an accepted baseline and one exact next conversation projection delta
- WHEN live transition validation runs
- THEN Animus MUST produce one bounded target delta under the same session, branch, policy, redaction, stream, and producer generation
- AND the delta MUST bind the source cursor, source revision, target predecessor, and next sequence

#### Scenario: Live delta cannot continue [r[agent_harness_runtime.shareable_view_projection.live.repair]]

- GIVEN branch, policy, redaction, cursor, sequence, source generation, or producer generation changed incompatibly
- WHEN live transition validation runs
- THEN Animus MUST request a complete replacement snapshot or withdraw the live producer
- AND it MUST NOT merge by displayed text, timestamp, item count, or Tile cache state

### Requirement: Shareable View actions use existing Animus authority [r[agent_harness_runtime.shareable_view_projection.actions]]

Animus MAY expose selected follow-up or steering descriptors for live views. Every invocation MUST pass existing message, confirmation, runtime-authority, session-generation, and delivery boundaries.

#### Scenario: Current action dispatches [r[agent_harness_runtime.shareable_view_projection.actions.dispatch]]

- GIVEN a live recipient invokes one current descriptor with exact object, snapshot, producer, session, catalog, action, input, capability, and client-sequence facts
- WHEN Animus action admission runs
- THEN it MAY return one typed message-delivery effect through the existing host port
- AND descriptor visibility, Tile focus, or snapshot possession MUST NOT grant model, tool, branch, session, or message authority

#### Scenario: Mutating outcome is uncertain [r[agent_harness_runtime.shareable_view_projection.actions.uncertain]]

- GIVEN transport or worker loss leaves a follow-up or steering outcome uncertain
- WHEN reconnect or recovery runs
- THEN Animus MUST preserve the uncertain outcome and MUST NOT replay the action automatically
- AND late completion MUST NOT cross session or producer generations

### Requirement: Shareable View data has no reverse authority [r[agent_harness_runtime.shareable_view_projection.non_authority]]

Animus MUST NOT use exported manifests, target deltas, Tile caches, rendered output, accessibility output, selection, placement, or action metadata as canonical session or agent input.

#### Scenario: Redaction occurs before export [r[agent_harness_runtime.shareable_view_projection.non_authority.redaction]]

- GIVEN a conversation projection contains data excluded by current export policy
- WHEN Shareable View mapping runs
- THEN Animus MUST remove or reject that data before target state and identity construction
- AND receipts and diagnostics MUST NOT echo the excluded value

#### Scenario: Target state is offered as canonical input [r[agent_harness_runtime.shareable_view_projection.non_authority.reverse]]

- GIVEN a caller offers exported or rendered Shareable View data as ledger truth, model context, tool result, branch state, active-turn state, completion evidence, or authority
- WHEN runtime admission runs
- THEN Animus MUST reject the reverse flow before model or tool dispatch
- AND a future context import MUST require a separate typed context-reference contract and Cairn change

### Requirement: Shareable View projection has positive and negative evidence [r[agent_harness_runtime.shareable_view_projection.validation]]

Animus MUST provide deterministic tests for external compatibility, mapping, identities, snapshots, live transitions, actions, redaction, reverse authority, publication, and receipts.

#### Scenario: Baseline is recorded [r[agent_harness_runtime.shareable_view_projection.validation.baseline]]

- GIVEN core or adapter implementation is about to change
- WHEN focused baseline validation runs
- THEN current conversation projection, ledger, redaction, messaging, authority, headless, TUI, and Wasm results MUST be recorded
- AND existing failures and external blockers MUST remain visible

#### Scenario: Conformance matrix runs [r[agent_harness_runtime.shareable_view_projection.validation.matrix]]

- GIVEN positive and adversarial fixtures cover every adapter boundary
- WHEN the focused Shareable View projection rail runs
- THEN valid snapshot, live, reconnect, action, withdrawal, and receipt cases MUST pass
- AND malformed, stale, private, identity-swapped, callback-bearing, replayed, over-authorized, and reverse-flow cases MUST fail before prohibited effects

#### Scenario: Dogfood projection runs [r[agent_harness_runtime.shareable_view_projection.validation.dogfood]]

- GIVEN a synthetic admitted conversation branch and recording Tile consumer fixture
- WHEN one snapshot and one live update are published
- THEN the consumer MUST receive structured state and stable identity links without pixel data
- AND the receipt MUST retain transcript-truth, delivery, rendering, accessibility, model-context, completion, production, and release-readiness non-claims

#### Scenario: Change is ready to archive [r[agent_harness_runtime.shareable_view_projection.validation.closeout]]

- GIVEN maintainers intend to accept the adapter
- WHEN closeout validation runs
- THEN focused runtime, adapter, std-host, redaction, messaging, Wasm, Cairn, Clippy, and relevant Nix gates MUST pass
- AND the exact published Tile contract revision MUST remain pinned and reviewable
