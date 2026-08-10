## Context

Active-turn recovery protects accepted user work before provider dispatch. The coding-session host also keeps steering and follow-up entries outside the transcript until admission.

Those contracts do not define one restart surface for every accepted pending-input class. They also do not define an operator disposition transaction.

The pending-input vault closes that gap. It does not replace the session ledger, active-turn journal, queue reducer, or host store.

Autolith's recovery vault is the behavior reference. Its useful properties are no automatic submission, exact ordering, idempotent import, visible corruption, and conversation isolation.

## Decisions

### Decision: model vault state as references plus host-owned bodies

**Choice:** The pure core receives validated entries with conversation, branch, input class, monotonic sequence, body identity, optional attachment identities, provenance, and store-generation metadata.

The reusable types do not contain persistence implementations. Host storage can retain the admitted input body under the existing privacy and retention policy.

**Rationale:** Recovery needs exact identity and order. Receipts and lifecycle state do not need raw input bodies.

### Decision: preserve all pending-input classes in one ordered projection

**Choice:** The projection covers active accepted input, in-flight steering, queued steering, follow-up input, host command input, and bounded attachment references.

Each class retains its original identity and sequence. Projection never converts a class or moves an entry into transcript state.

**Rationale:** Separate recovery lists can reorder work and hide the source route.

### Decision: quarantine recovered input until explicit disposition

**Choice:** Startup recovery returns `NoVault`, `ReviewRequired`, `BlockedCorrupt`, or `DispositionComplete`.

`ReviewRequired` permits bounded inspection and an explicit restore or discard request. It never emits a provider or queue-dispatch effect.

Restore creates new admission requests that reference the original vault entries. The normal coding-session and authority gates decide if each request can proceed.

**Rationale:** A crash changes the execution context. The host must not assume that old intent remains safe to submit.

### Decision: make disposition generation-fenced and idempotent

**Choice:** Restore and discard bind the exact vault identity, conversation identity, expected generation, selected entry identities, and disposition identity.

A repeated request for the same completed disposition returns the prior result. A request with changed entry selection or generation returns stale or conflicting.

The host records disposition evidence before it removes vault bodies or indexes.

**Rationale:** A second crash during recovery must not duplicate input or erase unresolved work.

### Decision: keep recovery controls available during corruption

**Choice:** Corrupt or ambiguous state blocks prompt, continue, steer, follow-up, and automatic queue dispatch for that conversation.

The host can still offer bounded metadata inspection, quarantine export, exact repair import, or explicit destructive discard under separate authorization.

Reusable code never skips malformed entries or imports a valid-looking suffix.

**Rationale:** Blocking all controls can trap the operator. Continuing around corrupt state can lose accepted work.

### Decision: isolate vaults by conversation and authority profile

**Choice:** Every plan binds one conversation, branch, store descriptor, runtime authority profile, policy identity, and generation.

Cross-conversation selectors, reused disposition identities, and store-profile drift fail before mutation.

**Rationale:** A host-global recovery list would recreate the global-store behavior that Animus rejects.

### Decision: emit metadata-only evidence

**Choice:** Receipts contain vault, conversation, branch, policy, generation, entry-count, class-count, disposition, corruption, redaction, and BLAKE3 identities.

They reject message text, attachment content, credentials, paths, provider data, tool data, and raw store errors.

**Rationale:** Evidence must prove ordering and disposition without becoming a second transcript.

## Transition Model

| Current state | Input | Result |
| --- | --- | --- |
| `NoVault` | startup inspection | `NoVault` |
| `ReviewRequired` | inspect | same state plus bounded metadata view |
| `ReviewRequired` | restore plan | `DispositionPending` |
| `ReviewRequired` | discard plan | `DispositionPending` |
| `DispositionPending` | matching durable evidence | `DispositionComplete` |
| `DispositionPending` | failure before commit | `ReviewRequired` |
| `DispositionPending` | uncertain commit evidence | `BlockedCorrupt` |
| `BlockedCorrupt` | ordinary input | denied |
| `BlockedCorrupt` | exact repair or discard | separately authorized plan |

## Composition Order

1. Load the explicit store and authority profile.
2. Validate the active-turn journal, pending queues, and terminal state.
3. Project one vault candidate from the supplied durable facts.
4. Validate ordering, identities, bounds, and conversation scope.
5. Return review or blocked state without provider dispatch.
6. Admit an explicit disposition request.
7. Commit disposition evidence before deletion.
8. Pass restored entries through normal coding-session admission.

## Alternatives Considered

### Resume every pending item automatically

Rejected because recovery cannot infer current intent or current authority.

### Store vault entries in transcript history

Rejected because steering and follow-up entries are not model-visible until normal admission commits them.

### Skip only corrupt entries

Rejected because a corrupt entry can be the only accepted active input or the ordering link for later entries.

### Use host timestamps for ordering

Rejected because timestamps can collide or move. Existing monotonic sequences and typed identities are authoritative.

## Risks and Trade-offs

- Explicit review can slow recovery. This cost prevents silent duplicate submission.
- Host stores must retain pending bodies under explicit policy. The reusable core cannot prove store privacy or durability.
- A corrupt vault can block one conversation. The operator receives bounded repair and discard controls instead of silent continuation.

## Non-Claims

This design does not prove power-loss durability, storage correctness, operator intent, provider acceptance, queue delivery, exactly-once effects, transcript completeness, encryption, production readiness, or release eligibility.
