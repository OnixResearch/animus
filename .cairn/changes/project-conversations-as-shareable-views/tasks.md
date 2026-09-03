## Phase 1: External contract and baseline

- [ ] [serial] `[shareable-view-external-contract]` Record the exact published Tile Shareable View revision, schemas, fixture identities, bounds, license facts, non-claims, and source-admission result. r[agent_harness_runtime.shareable_view_projection.boundary.external]
  - Verify: a branch, sibling path, draft schema, screenshot, or undocumented DTO cannot satisfy the dependency.
- [ ] [serial] `[shareable-view-projection-baseline]` Run and record current conversation projection, session ledger, redaction, messaging, authority-seal, headless, TUI, and Wasm baselines. r[agent_harness_runtime.shareable_view_projection.validation.baseline]
  - Verify: existing failures and external-contract blockers remain visible.
- [ ] [parallel] `[shareable-view-projection-inventory]` Inventory every source field, target field, identity domain, redaction rule, action path, effect owner, and reverse-authority risk. r[agent_harness_runtime.shareable_view_projection.boundary.ownership]
  - Verify: Animus, Tile, optional Kamacite, and host responsibilities remain separate.

## Phase 2: Policy and pure adapter

- [ ] [serial] `[shareable-view-projection-policy]` Add typed Nickel policy for external profiles, source fields, redaction, item and delta bounds, action classes, receipts, and non-claims. r[agent_harness_runtime.shareable_view_projection.boundary.mapping] r[agent_harness_runtime.shareable_view_projection.non_authority.redaction]
  - Verify: unknown profiles, unbounded items, private fields, callbacks, commands, hidden authority, and overclaims fail export and Rust validation.
- [ ] [serial] `[shareable-view-projection-fixtures]` Freeze positive snapshot, live, reconnect, action, and receipt fixtures plus identity, redaction, gap, stale-action, and reverse-authority negatives. r[agent_harness_runtime.shareable_view_projection.validation.matrix]
  - Verify: fixtures use synthetic content and exact external contract identities.
- [ ] [serial] `[shareable-view-projection-core]` Add a non-default host-effect-free adapter for source validation, state mapping, identity linkage, snapshot and delta planning, action descriptors, and receipts. r[agent_harness_runtime.shareable_view_projection.snapshot.export] r[agent_harness_runtime.shareable_view_projection.identity.linkage]
  - Verify: direct tests need no Tile daemon, renderer, transport, storage, clock, provider, tool, or process.
- [ ] [serial] `[shareable-view-live-reducer]` Add snapshot-first live transitions, exact cursor and predecessor checks, repair plans, and producer-generation fencing. r[agent_harness_runtime.shareable_view_projection.live.stream] r[agent_harness_runtime.shareable_view_projection.live.repair]
  - Verify: branch, policy, redaction, cursor, sequence, or producer drift cannot apply an incremental update.

## Phase 3: Host publication and actions

- [ ] [serial] `[shareable-view-publication-port]` Add narrow injected snapshot, delta, repair, withdrawal, and outcome ports plus a non-default recording shell. r[agent_harness_runtime.shareable_view_projection.snapshot.publication] r[agent_harness_runtime.shareable_view_projection.live.stream]
  - Verify: reusable crates gain no network, filesystem, Tile frontend, renderer, cache, or persistent-state authority.
- [ ] [serial] `[shareable-view-action-bridge]` Map admitted follow-up and steering descriptors into existing message, confirmation, authority-seal, session-generation, and receipt paths. r[agent_harness_runtime.shareable_view_projection.actions.dispatch]
  - Verify: descriptor visibility, snapshot possession, Tile focus, or transport success grants no model, tool, message, branch, or session authority.
- [ ] [parallel] `[shareable-view-action-negatives]` Add stale, denied, disconnected, timed-out, duplicate, uncertain, wrong-session, wrong-generation, and over-broad action tests. r[agent_harness_runtime.shareable_view_projection.actions.uncertain]
  - Verify: an uncertain mutating action is not replayed automatically and late completion cannot cross generations.
- [ ] [serial] `[shareable-view-reverse-authority-guard]` Add structural and runtime guards against using exported or rendered view data as ledger, model-context, tool, branch, active-turn, completion, or authority input. r[agent_harness_runtime.shareable_view_projection.non_authority.reverse]
  - Verify: every prohibited reverse flow fails before model or tool dispatch.

## Phase 4: Conformance and closeout

- [ ] [parallel] `[shareable-view-projection-positive-tests]` Add positive snapshot, identity, reconnect, delta, no-op, action, withdrawal, and receipt tests. r[agent_harness_runtime.shareable_view_projection.validation.matrix]
  - Verify: repeated identical inputs produce identical state artifacts, mappings, plans, and metadata-only receipts.
- [ ] [parallel] `[shareable-view-projection-negative-tests]` Add malformed profile, equal-text collision, stale cursor, gap, private data, callback, command, identity swap, reverse-authority, and overclaim tests. r[agent_harness_runtime.shareable_view_projection.validation.matrix]
  - Verify: each negative fails at its declared boundary without changing the source ledger or host state.
- [ ] [serial] `[shareable-view-projection-dogfood]` Publish one synthetic conversation snapshot and one bounded live update to a recording Tile consumer fixture. r[agent_harness_runtime.shareable_view_projection.validation.dogfood]
  - Verify: structured state and stable identities cross the adapter without pixel data or transcript-authority transfer.
- [ ] [serial] `[shareable-view-projection-closeout]` Document the adapter and run focused runtime, adapter, std-host, redaction, messaging, Wasm, Cairn, Clippy, and relevant Nix gates. r[agent_harness_runtime.shareable_view_projection.validation.closeout]
  - Verify: accepted requirements sync before archive; import into model context remains outside this change.
