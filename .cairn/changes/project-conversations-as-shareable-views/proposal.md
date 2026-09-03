## Why

Animus already derives snapshot-first conversation views from one admitted session ledger branch. The projection has stable identities, redaction, rebuild behavior, and disposable caches.

Current image attachments and terminal renderings flatten this structure into pixels or text. They cannot preserve item identity, live updates, accessibility, semantic actions, or reusable presentation data.

Animus needs one optional adapter that projects an admitted conversation view into Tile's published Shareable View contract. The adapter must not make Tile presentation state a transcript, model-context, tool-correlation, completion, or authority source.

## What Changes

- Add a non-default, host-effect-free Shareable View projection adapter over canonical conversation snapshots and deltas.
- Bind one exact published Tile contract revision, schema profile, fixture corpus, and compatibility result.
- Project redacted conversation items, branch facts, active-turn state, and bounded presentation data into application-owned state artifacts.
- Preserve Animus stream, cursor, item, branch, policy, and redaction identities without reinterpreting them as Tile identities.
- Publish optional live updates only after a complete accepted snapshot.
- Map selected follow-up or steering actions through current Animus messaging and runtime-authority gates.
- Keep received Shareable View data outside model context unless a separate context-reference adapter admits it.
- Add positive and negative fixtures for snapshot export, live updates, reconnect, stale actions, redaction, and reverse-authority attempts.

## Capabilities

### New Capabilities

- `agent_harness_runtime.shareable_view_projection.boundary`: Animus can project one admitted conversation view into a pinned external Shareable View contract.
- `agent_harness_runtime.shareable_view_projection.identity`: Animus and Tile identity domains remain separate and linked.
- `agent_harness_runtime.shareable_view_projection.snapshot`: Redacted conversation state can produce one immutable Shareable View snapshot.
- `agent_harness_runtime.shareable_view_projection.live`: Admitted conversation deltas can drive one snapshot-first live Shareable View.
- `agent_harness_runtime.shareable_view_projection.actions`: Selected semantic actions route through existing Animus authority and messaging contracts.
- `agent_harness_runtime.shareable_view_projection.non_authority`: UI state cannot become session, model, tool, or completion authority.
- `agent_harness_runtime.shareable_view_projection.validation`: Cross-contract and adversarial evidence covers the adapter.

## Success Criteria

- A Tile fixture presents an Animus conversation snapshot without pixel data.
- Equal text and timestamps do not merge distinct conversation items.
- A reconnect uses exact cursor and producer-generation facts or requests a complete baseline.
- A visible action descriptor grants no model or tool effect by itself.
- A current authorized follow-up routes once through the existing messaging boundary.
- A stale or uncertain mutating action is not replayed automatically.
- Rendered or received view state cannot enter model context without separate admission.

## False Completion

A screenshot, HTML transcript, terminal dump, or copied renderer tree is not a Shareable View projection.

A successful serialization round trip does not prove Tile rendering, live delivery, action authority, accessibility, or transcript truth.

A Tile object identity must not replace Animus session, branch, item, cursor, policy, or ledger identity.

## Impact

- **Session core**: Pure projection planning over existing conversation snapshots and deltas.
- **Adapter**: A focused non-default Shareable View mapping and compatibility boundary.
- **Runtime shell**: Optional live publication and semantic action dispatch through injected ports.
- **Policy**: Typed Nickel schema, redaction, action, size, live, and external-contract admission.
- **Evidence**: Metadata-only receipts and pinned external contract identity.
- **Testing**: Positive, negative, reconnect, redaction, stale-action, and reverse-authority fixtures.

## Dependencies

Implementation requires a published Tile Shareable View contract with exact schemas, fixtures, bounds, and non-claims. An optional Kamacite envelope adapter requires its own published revision.

The reusable Animus core must not depend on Tile daemon, renderer, transport, filesystem, network, clock, or credential types.

## Out of Scope

- Importing arbitrary Shareable Views into prompts or model context.
- Making UI caches, Tile snapshots, or renderer output authoritative conversation history.
- Embedding Tile scenes, callbacks, closures, shell commands, or executable renderer code.
- Granting follow-up, steering, tool, model, branch, or session authority from visible metadata.
- Browser, remote transport, arbitrary frontend, production, or release-readiness claims.
