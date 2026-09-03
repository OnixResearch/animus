## Context

Animus conversation projection already separates canonical ledger history from disposable display caches. Tile's proposed Shareable View contract adds portable application state, presentation profiles, dependencies, semantic actions, and optional live behavior.

The integration is an adapter, not a new conversation model. Animus remains the owner of session, ledger, branch, item, cursor, active-turn, model, tool, and completion meaning.

## Goals

- Export redacted conversation projections as structured application state.
- Preserve exact Animus identity and lineage across the external envelope.
- Support immutable snapshot and bounded live projection modes.
- Route selected semantic actions through existing Animus authority paths.
- Prevent any reverse flow from presentation state into canonical agent state.

## Ownership

| Concern | Owner |
| --- | --- |
| Conversation ledger, branch, item, cursor, and active turn | Animus |
| Conversation projection and redaction | Animus |
| Shareable View product contract and presentation | Tile |
| Canonical cross-language envelope | Optional Kamacite adapter |
| Transport and storage | Host-provided ports |
| Model, tool, follow-up, and steering authority | Animus runtime shell |
| Rendering, viewport, theme, focus, and placement | Tile host |

## Decision

### Separate adapter family

A non-default `animus-adapter-shareable-view` family will own the pure mapping contract. It will use Animus-owned input and output types plus exact pinned Tile schemas and fixture conformance.

The adapter will not import Tile daemon, frontend, renderer, transport, or persistence code. A direct Tile crate dependency is optional only after dependency and license review proves that it is smaller than schema-bound conformance.

A separate optional standard-library shell can publish snapshots and deltas through an injected Shareable View port. The core never opens sockets or writes artifacts.

### Source projection

The adapter accepts one already admitted `ConversationProjectionSnapshot` or exact delta set. Redaction occurs before this boundary.

The application state schema contains bounded conversation items, roles, ordering, active-turn summaries, branch metadata, steering markers, and safe action references. It excludes provider payloads, hidden reasoning, tool bodies, credentials, paths, raw terminal data, and renderer state.

The Shareable View manifest refers to this state artifact. It does not copy the canonical ledger or make the state artifact a replacement ledger.

### Identity mapping

Animus identities remain in nominal Animus fields. Tile logical and snapshot identities remain in nominal Tile fields.

A mapping receipt links these domains:

- session, branch, leaf, ledger generation, and projection policy;
- source projection stream, cursor, item set, and active-turn identity;
- target Shareable View schema, logical object, snapshot, mode, and producer generation;
- adapter profile and external contract revision;
- redaction and non-claims.

The receipt cannot identify transcript truth, delivery, rendering, model context, or action success.

### Snapshot mode

One complete redacted conversation snapshot produces one immutable Shareable View snapshot. Receiver-local scrolling, filtering, selection, and presentation do not mutate Animus state.

Snapshot loss or re-import cannot modify the ledger. The exported state remains a derived projection.

### Live mode

Live mode starts from one complete accepted snapshot. Each delta binds the Animus projection stream, cursor, source revision, target producer generation, Tile snapshot predecessor, and sequence.

A gap, branch change, policy change, cursor drift, producer replacement, or redaction change requests a complete replacement baseline. The adapter never guesses a merge from displayed content.

### Semantic actions

The first action profile can expose bounded follow-up and steering descriptors only when current Animus policy supports them.

Descriptors contain names, schemas, mutability, and capability requirements. They contain no callbacks, provider requests, prompts, tools, shell commands, or host handles.

Invocation passes through the existing message queue, confirmation, runtime-authority seal, session generation, and delivery receipt boundaries. An uncertain mutating outcome is not replayed automatically.

Snapshot-only recipients receive no live action route.

### Reverse-authority firewall

Shareable View snapshots, Tile deltas, renderer output, accessibility output, local selection, and frontend caches cannot become:

- ledger entries;
- model request content;
- tool results;
- branch or active-turn facts;
- completion evidence;
- authority inputs.

A future context-reference adapter can admit selected Shareable View state as external context. That work needs a separate Cairn because it changes model-input authority.

### Functional core and shell

The pure core validates source projection facts, external profile facts, identity mapping, redaction, snapshot and delta order, action descriptors, invocation plans, and metadata-only receipts.

The shell owns external schema loading, artifact publication, transport, capability observations, message delivery, timeouts, cancellation, and output routing.

## Data Flow

```text
admitted Animus ledger branch
  -> conversation projection and redaction
  -> pure Shareable View adapter
  -> state artifact + mapping receipt + publish plan
  -> injected host publication port
  -> Tile consumer
```

Live action flow:

```text
Tile semantic action request
  -> injected host ingress
  -> current session and authority observations
  -> pure action admission
  -> existing Animus message delivery effect
  -> bounded outcome and receipt
```

## Error Model

The adapter distinguishes unsupported external contract, incompatible schema, stale profile, invalid source projection, identity conflict, redaction failure, stream gap, stale action, authority denial, delivery failure, timeout, cancellation, uncertain mutation, and publication failure.

External protocol errors do not become session errors. A conversation policy rejection does not become a transport error.

## Testing Strategy

Positive tests cover snapshot projection, stable identity, reconnect, ordered deltas, no-op top-up, admitted action, and metadata-only receipts.

Negative tests cover equal-text collisions, cursor drift, private data, hidden reasoning, tool-body leakage, unknown schema, callbacks, stale actions, action replay, and reverse authority.

Cross-contract fixtures must use one exact published Tile revision. Fixture agreement is structural and semantic mapping evidence only.

## Alternatives

### Export HTML or Markdown

These formats preserve readable text but lose typed identity, live deltas, action contracts, and application-state boundaries.

### Let Tile read the Animus ledger

This transfers session and privacy authority into the UI host. The design rejects this route.

### Put Tile types in Animus core

This couples reusable agent logic to one product protocol. The design keeps the dependency in an optional adapter.

## Risks

- The external Tile contract can change before a stable revision exists.
- Redacted conversation state can still reveal sensitive content allowed by policy.
- Action descriptors can be mistaken for authority.
- Live reconnect can tempt content-based repair.

Exact revision pinning, redaction-first input, descriptor non-authority, and complete repair address these risks.
