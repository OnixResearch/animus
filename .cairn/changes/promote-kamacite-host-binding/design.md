## Context

The existing pilot proves a bounded projection over one fake-backed entrypoint. It does not protect a real product composition root.

`animus-live-steel-agent` already has visible model, tool, session, adapter, resource, policy, and host boundaries. This change adds one canonical binding across those boundaries.

## Success Contract

A selected live-agent dispatch cannot start until one exact Kamacite application-host binding is admitted and committed beside the existing runtime authority seal.

The binding must match the current application, entrypoint, schemas, closed effect set, host profile, adapter cohort, resource profile, and policy identity.

## Decisions

### 1. Add a narrow adapter rather than a universal Kamacite dependency

A focused adapter validates canonical bytes and converts the accepted artifact into Animus-owned nominal identity types.

Generic Kamacite values do not enter `animus-core`, adapter contracts, session state, or ordinary runtime APIs.

### 2. Protect one real composition root first

The first product path is `animus-live-steel-agent` behind a non-default feature and explicit configuration selection.

Tests use controlled adapters, but they execute the same composition and pre-effect code used by the product path.

### 3. Use exact semantic operation identities

The closed effect summary binds exact Kamacite identities for admitted model and tool operations. Names and local action labels remain metadata.

Unknown, stale, duplicate, open, or name-only operation sets fail before dispatch preparation.

### 4. Compose with the existing authority seal

Binding admission occurs before `start_runtime_authority_effect`. It cannot bypass the existing seal, capability probes, adapter readiness, credentials, sandbox, cancellation, timeout, or resources.

A current-state mismatch after binding commit still denies the effect.

### 5. Keep private data outside canonical artifacts

The binding carries identities, schema roles, closed operation identities, and non-claims. It excludes prompts, responses, tool bodies, credentials, endpoints, environment values, and private paths.

### 6. Make rollout explicit and removable

The Kamacite-enabled path never silently falls back to the legacy path. Operators select one path through typed configuration.

Removal restores the retained legacy composition without reinterpreting stored session or runtime records.

## Risks and Controls

- The upstream schema can change. Immutable revision pins and negative cohort fixtures block unnoticed drift.
- Identity checks can become duplicate authority logic. The design reuses current runtime seals and does not create another allow decision.
- More startup work can affect latency. Measurements must cover only the selected composition and remain outside semantic claims.
- Private metadata can leak through diagnostics. Safe issue classes and negative leak fixtures block raw values.

## Non-Claims

A passing binding does not prove application correctness, provider behavior, tool safety, host capability, authorization, sandboxing, execution success, or release eligibility.
