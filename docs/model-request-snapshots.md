# Durable model-request snapshots

Animus validates one private, provider-neutral snapshot before provider contact.
The snapshot records the exact request supplied to provider projection.

The pure contract is `animus_session_core::model_request_snapshot`.
`animus-runtime` provides the compatibility re-export.
The imperative shell is `animus_std::model_request_snapshot_host`.

## Source reference

The design adapts the model-visible-state invariant from DeepSeek Harness.
The source revision is `47f943859bef60e4160492346772ded9b24f765a`.
The source declares the MIT license and developer-preview status.

The route binding also adapts provider-transport and model-dialect separation from [`digitallyinduced/haskell-agent`](https://github.com/digitallyinduced/haskell-agent). Animus retains its Rust, authority, snapshot, BLAKE3, and receipt contracts.

Animus selects the model-visible-state invariant only.
Animus keeps its existing ledger, request-context, model-profile, authority, retry, and provider owners.
It does not adopt Cordis, TypeScript plugins, ambient lookup, or provider-native storage.

## Private snapshot

`ModelRequestSnapshot` owns these exact values:

- the ordered canonical messages and content parts;
- system and developer sections that exist in the canonical request;
- ordered selected request-context contribution identities;
- native tool declarations and schema values;
- output format, reasoning controls, streaming posture, and budgets;
- the selected provider account, versioned provider transport, model, and versioned model-dialect route;
- the model-invocation plan identity;
- the request-context plan identity;
- the runtime-authority seal identity;
- the provider-adapter cohort identity;
- the session, branch, active leaf, turn generation, and request sequence.

The canonical request remains provider-neutral.
Credentials, transport headers, provider SDK objects, endpoints, and hidden defaults remain outside it.

Schema `animus.model-request-snapshot.v2` adds exact provider-transport and model-dialect fields. Version 1 private snapshots do not receive inferred identities. They fail closed and require a newly composed snapshot.

## Canonical identity

The contract uses versioned BLAKE3 domains.
It preserves message, content-part, context-contribution, tool, stop-sequence, and provenance order.

The validator recomputes these identities:

- model route;
- canonical request;
- tool-schema set;
- output controls;
- budgets;
- complete snapshot;
- private commit plan;
- provider dispatch plan;
- terminal provider observation;
- public receipt.

A changed message order creates a different snapshot.
A changed provider transport, model dialect, tool schema, output control, budget, route, context, authority seal, or adapter cohort blocks reuse.

## Functional core and shell

The core performs no storage, provider, clock, credential, filesystem, process, or network operation.
It returns validated values or safe typed issues.

The shell uses two narrow ports:

- `ModelRequestSnapshotStorePort` commits and loads the private snapshot and terminal observation;
- `ModelRequestSnapshotProviderPort` performs the provider effect.

`dispatch_initial_model_request` runs this order:

1. Plan the private snapshot append.
2. Commit the snapshot.
3. Admit the exact commit observation.
4. Recheck all current request and authority facts.
5. Create the snapshot-bound dispatch plan.
6. Contact the provider.
7. Commit one correlated terminal observation.

A partial or mismatched snapshot append stops before provider contact.
A terminal append failure preserves an uncertain provider outcome.

## Coding-session integration

`CodingSessionProviderRequest` now carries the session, branch, active leaf, durable head, and request sequence.
The reducer derives these values from the committed session state.

`SnapshotBoundCodingSessionProvider` wraps an existing coding-session provider.
A product-owned composer supplies the complete canonical request and current bindings.
The wrapper then enforces snapshot commit ordering before it calls the provider.

The live Steel activation profile requires both snapshot components.
The composition opens a dedicated capability-local snapshot journal through the existing durable-store owner.
It commits the snapshot before each provider stream and closes the explicit lease after the session.
The provider-native projection remains outside the reusable core and journal.

## Retry and recovery

A same-request retry needs an explicit `ModelRequestRetryOwnerDecision::Admitted` value.
`dispatch_same_model_request_retry_from_owner` accepts the concrete existing `ProviderRetryDecision`.
It checks the exact invocation identity and next attempt before it projects that value.
The snapshot shell does not replace the retry owner.

The retry must keep every current binding and the original snapshot identity.
Any drift requires a new request and a new private snapshot.

`DurableModelRequestSnapshotStore` uses the existing private framing, bounds, synchronization, locking, and recovery owner.
It decodes and revalidates the full provider-neutral request after restart.

A committed snapshot without terminal provider evidence becomes `committed_unobserved`.
This state does not imply delivery, failure, cancellation, billing, or replay safety.
Automatic replay is always false.

## Public evidence

The private snapshot can contain prompts and tool schemas.
Public receipts never contain that body.

A public receipt contains only:

- safe BLAKE3 identities;
- safe status classes;
- bounded message, section, contribution, and tool counts;
- one redaction marker;
- explicit non-claims.

Receipt projection rejects raw prompts, tool schemas, contribution bodies, credentials, provider payloads, private paths, and raw errors.

## Verification

Run the focused rail:

```sh
scripts/cairn/check-model-request-snapshots --root .
```

The rail covers exact reconstruction, identity drift, commit order, provider gating, retry ownership, terminal evidence, restart uncertainty, and receipt redaction.
It also checks the no-std Wasm target and related production-owner rails.

The repository-wide formatter baseline has unrelated drift in untouched live Steel files.
Changed Rust files use direct `rustfmt` checks.
This known baseline does not weaken the focused error gates.

## Non-claims

A snapshot identity does not prove provider delivery or wire-byte equivalence.
It does not prove hidden provider defaults, credential safety, response truth, model compliance, task success, production readiness, or release eligibility.
