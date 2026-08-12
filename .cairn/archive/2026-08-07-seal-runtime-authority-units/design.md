## Context

Animus already has a metadata-only `SessionRuntimeAuthorityProfile`. The durable host commits profile changes and rechecks realized enforcement before provider, tool, or summarizer effects.

The missing invariant is local to one dispatch. After the host prepares an exact dispatch, that dispatch needs a final authority boundary. Later discovery, fallback, rebind, or approval must not change the active unit.

Redox uses a related acquire-then-seal pattern for service authority. Animus adapts the transition rule through existing injected services and host shells. It does not copy the Redox namespace implementation.

## Goals

- Freeze one dispatch unit before its dependent effect.
- Keep the transition logic pure and deterministic.
- Preserve current Animus authorization and enforcement owners.
- Make post-seal widening a typed failure.
- Record safe evidence without claiming operating-system isolation.

## Decisions

### 1. Seal one dispatch unit, not the whole session

A runtime-authority unit covers one bounded provider, tool, summarizer, or equivalent dispatch. Its states are `preparing`, `sealed`, `running`, and one terminal state.

A resumable session can create later units. It cannot rewrite or widen a sealed unit.

This scope preserves explicit session reauthorization. A new grant or profile binding produces a new unit after policy permits the current unit to finish or cancel.

### 2. Use an exact immutable input set

The pure seal plan consumes safe identities for:

- session lineage and durable authority binding;
- current authority profile;
- current realized-enforcement observation;
- admitted operation and resource classes;
- selected adapter cohort;
- declared resource profile and budgets; and
- observed runtime-view classes and identities.

Canonical validated metadata produces a domain-separated BLAKE3 unit identity. Clocks, randomness, physical paths, secrets, payloads, and ambient process state do not enter the identity.

### 3. Compare declared and observed runtime views

The host supplies an observation for the authority classes that the selected profile claims it can inspect. The pure core compares the expected and observed sets.

Missing required entries, unexpected entries, duplicate identities, stale observations, unknown classes, or an unsupported required observation deny sealing.

The first fake-backed pilot observes injected Animus services and adapter selections. It does not claim that all operating-system descriptors or host resources were observed.

### 4. Make sealing monotonic

After sealing, rebind, replacement, adapter discovery, capability addition, and ambient fallback are invalid transitions for that unit.

A fresh recheck can narrow or deny execution. It cannot widen the sealed set. Widening requires a new admitted unit.

This rule applies even when new authority is valid for the session. Valid session authority does not mutate prior unit identity.

### 5. Commit the seal before the effect

The shell prepares services and observations. The core returns a seal plan. The shell commits the metadata-only seal receipt before it calls the dependent adapter.

If commit fails, dispatch does not occur. The shell reports the terminal failure through existing safe receipt paths.

### 6. Preserve functional core and imperative shell

`animus-runtime` owns pure validation, transitions, comparison, issue ordering, and BLAKE3 identity material.

`animus-std` owns service preparation, current probes, durable commit, effect dispatch, cancellation, cleanup, and safe output.

The core does not read files, inspect the environment, open handles, contact providers, or execute effects.

### 7. Keep evidence bounded

The receipt records safe identities, class labels, seal state, issue codes, and explicit non-claims. It excludes raw grants, descriptors, paths, credentials, prompts, provider payloads, tool bodies, and model output.

A passing receipt proves only that the admitted Animus shell followed this bounded transition over supplied observations. It does not prove operating-system confinement or the absence of hidden host authority.

## Test Design

Positive tests cover canonical preparation, exact view matching, deterministic identity, commit-before-effect ordering, one successful fake-backed dispatch, and a later separately sealed unit.

Negative tests cover missing and extra view entries, stale observations, duplicate identities, post-seal capability addition, profile rebind, adapter replacement, ambient fallback, effect-before-seal, commit failure, repeated seal, stale unit reuse, unsafe receipt fields, and isolation overclaims.

Property tests cover deterministic order normalization and monotonic transitions. The focused host test uses recording services and needs no live provider or network.

## Risks and Trade-offs

- A metadata seal can look stronger than it is. Types, receipts, and documentation keep the host-isolation non-claim explicit.
- Per-dispatch seals add records. Receipts stay metadata-only and use bounded fields.
- Dynamic approvals need a new unit. This is intentional because approval changes effective authority.
- Incomplete host observation can hide authority. Required unknown or unsupported classes block strict sealing instead of becoming empty evidence.
