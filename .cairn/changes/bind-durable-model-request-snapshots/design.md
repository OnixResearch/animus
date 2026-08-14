# Design: Bind durable model-request snapshots

## Context

The coding-session host commits accepted input before provider dispatch. Model-runtime profiles and request-context reduction produce exact invocation and contribution identities. The runtime authority seal binds the effect scope.

These controls prove admission and order. They do not create one durable private artifact for the exact provider-neutral request assembled at that boundary.

## Success Contract

Before every initial provider attempt, the host commits one complete private snapshot. The provider dispatch plan cites the exact committed snapshot identity. Any change to model-visible content or request behavior creates a new snapshot.

Restart can classify an unobserved request without inventing a provider result or automatically repeating the request.

## Decisions

### 1. Snapshot provider-neutral meaning, not provider wire bytes

The snapshot contains the exact canonical request supplied to provider projection. It binds ordered message and content-part values, mandatory system sections, selected request-context payloads, native tool declarations and schemas, output format, reasoning and stream posture, budgets, selected model route, model-invocation identity, authority-seal identity, and provider-adapter cohort.

Credentials, transport headers, provider SDK objects, connection facts, and hidden provider defaults stay outside the snapshot.

### 2. Keep validation in the pure core

`animus-session-core` validates a `ModelRequestSnapshotDraft`, rejects missing or duplicate sections, recomputes canonical BLAKE3 identities, and returns a snapshot or typed issue.

The core performs no storage, provider, clock, credential, or network operation.

### 3. Commit through the existing private durable session boundary

The coding-session shell appends the snapshot as a private durable entry after all request inputs are final and before provider contact. Public transcript projections omit its body.

A successful durable append observation must match the snapshot identity, session lineage, active leaf, turn generation, and request sequence.

### 4. Gate dispatch on exact durable evidence

A `ProviderDispatchPlan` cites one snapshot identity and adapter cohort. The shell rejects missing, stale, substituted, partially committed, or differently projected snapshots before provider contact.

Same-request retry may reuse the snapshot only when the existing provider-retry contract admits the exact invocation. Recomposition, context expiry, profile drift, tool-schema drift, policy drift, or authority drift creates a new request and snapshot.

### 5. Make crash uncertainty explicit

A committed snapshot with no correlated terminal provider observation becomes `committed_unobserved`. Recovery does not infer delivery, failure, billing, or cancellation.

The existing provider-retry owner decides whether an exact bounded retry is permitted. Otherwise, operator or product policy must close the uncertain attempt.

### 6. Separate private truth from public evidence

Private durable state may retain the exact provider-neutral request under session storage policy. Public events and receipts contain only safe labels, counts, identities, omission classes, status, and non-claims.

## Composition Flow

```text
admitted durable session state
  -> model profile and request-context composition
  -> current authority seal
  -> pure complete request snapshot
  -> private durable snapshot commit
  -> exact provider dispatch plan
  -> provider adapter projection and effect
  -> correlated terminal observation or explicit uncertainty
```

## Alternatives Considered

### Reconstruct the request later from separate receipts

Rejected because lifetimes, order, payload selection, schema cohorts, and composition drift can make a later reconstruction ambiguous.

### Store provider-native HTTP bodies

Rejected because provider-native bodies can contain credentials, transport details, SDK defaults, and unstable vendor fields. The contract owns provider-neutral model-visible meaning.

### Persist every stream frame as request evidence

Rejected because stream output is a response concern and full token persistence is not needed to prove the request input.

## Risks and Controls

- Private snapshots can contain sensitive prompt content. Existing private-store authority, restrictive access, retention, and redacted public projection remain mandatory.
- Canonical encoding can drift. Versioned domain tags, exact field ordering, fixtures, and migration denial control this risk.
- A digest can be mistaken for delivery proof. Receipts state that identity proves neither provider receipt nor response correctness.

## Non-Claims

This design does not prove provider delivery, provider wire equivalence, provider defaults, credential safety, model compliance, response truth, task success, production readiness, or release eligibility.
