# Provider retry execution

Animus provides a bounded, same-invocation provider retry contract. The contract adapts Tau-style retry behavior without transferring provider, clock, routing, credential, or completion authority into reusable runtime code.

## Ownership boundary

The retry path composes existing owners rather than replacing them:

- `animus-adapters` owns provider-neutral model request, response, and stream-frame contracts.
- provider host adapters own HTTP, SDK, transport, endpoint, credential, and raw diagnostic handling.
- model-runtime profiles own the already-admitted provider/model, prompt, tool, memory, budget, and policy context.
- router fallback owns fresh target selection outside the retry path.
- `animus_runtime::provider_retry` owns pure policy revalidation, safe classification, same-plan admission, semantic-stream tracking, delay calculation, timer-evidence gating, progress projection, and receipt projection.
- `animus-std` owns explicit generated-policy file loading and the injected timer and provider-dispatch calls.
- the selected host owns cancellation observation, event delivery, storage, provider contact, and any later completion decision.

Runtime code does not read retry configuration, evaluate Nickel, inspect environment variables, sleep, poll, read clocks, discover providers, load credentials, or contact a network.

## Generated policy

`config/provider-retry-policy.ncl` is the authoring source for:

- schema and policy identity;
- maximum attempts and delay;
- the complete deterministic delay schedule;
- canonical retryable classes;
- versioned provider-family/native-class mappings;
- required cancellation posture; and
- the deny-after-semantic stream posture.

`scripts/config/export-nickel-configs` emits `config/generated/provider-retry-policy.json`. The `animus-std` parser rejects unknown JSON fields, then `animus-runtime` recomputes the domain-separated BLAKE3 policy identity and validates all bounds, classes, schedules, and mapping uniqueness. There are no built-in runtime fallback values.

Provider-native classification happens at the host boundary. Only a bounded provider-family label, mapping version, and native-class label enter the reusable core. Unknown mappings become `unknown`, which is terminal. Authentication, authorization, entitlement, policy, capability, invalid-request, malformed, unsafe, missing-service, and unsupported failures remain terminal because policy does not list them as retryable.

## Exact invocation identity

Automatic retry is valid only for the identical admitted invocation. The exact plan identity binds:

- provider and model;
- canonical request;
- model-runtime profile plan;
- prompt/system plan;
- tool catalog;
- response format;
- reasoning intent;
- stream intent;
- credential-handle class; and
- retry policy.

A change in any dimension returns `recomposition_required`. Recomposition, provider fallback, model fallback, credential replacement, tool changes, and authority changes run through their original owners as a fresh invocation. Retry never unions or expands authority. The host remains responsible for constructing each upstream canonical content hash correctly; BLAKE3 binding detects identity changes but does not prove the semantic correctness of host canonicalization.

## Semantic stream boundary

Transport start alone does not commit assistant semantics. Accepted content, reasoning, tool-call, structured-output, or other semantic frames permanently move the attempt to committed state. A retryable transport failure after that point becomes `post_commit_terminal`.

The reducer also maps canonical `ModelStreamFrameKind` values into the stream state. Fragmented transport before a semantic delta can remain eligible; any accepted tool-call delta blocks ordinary replay. Provider-native resume protocols require a separate contract and are not inferred here.

## Injected timer and dispatch

Pure planning returns a bounded delay, correlated timer request, and threaded wait state. `ProviderRetryTimerService` is an injected host seam. A host timer returns only completed, cancelled, timed-out, unavailable, or failed evidence bound to the run, next attempt, exact plan, and retry decision. Once a correlated completion, cancellation, timeout, absence, or failure closes the threaded wait, later observations cannot reopen a ready dispatch gate.

`evaluate_provider_retry_timer` admits dispatch only for a completed, exactly correlated observation. Cancellation, timer failure, absence, timeout, or stale correlation returns a blocked gate. The host threads `advance_provider_retry_attempt` through its loop; one request advances only to its immediate next attempt, and replaying that request against the advanced state fails. `dispatch_provider_retry_exact` reuses the existing injected `ModelAdapter` port and recomputes the retained request hash with `retry_model_request_hash` before checking the gate's exact plan identity. The thin shell invokes neither a timer nor a provider when gate, binding, or service admission fails. A returned adapter error remains host-private and must be reduced to a safe versioned observation before any later retry decision or receipt.

The deterministic schedule does not replace a host request deadline or total elapsed-time budget. A host deadline remains an independent authority ceiling and appears to the core only as a typed timer timeout. These seams do not claim timer implementation correctness, remote cancellation, provider cancellation, or exactly-once remote effects. Timer requests and gates are cloneable evidence; hosts must thread the returned wait and attempt states serially, and receipts do not prove that a host did so.

## Active-turn composition

[Active-turn recovery](active-turn-recovery.md) requires durable accepted-work evidence before the first provider dispatch. Provider retry remains a separate same-plan decision after a provider failure.

## Progress events

Retry progress is a separate provider-neutral DTO. It carries only:

- run identity;
- completed or next attempt count and maximum count;
- bounded delay in milliseconds;
- canonical reason class;
- safe retry status; and
- exact plan hash.

Headless, SDK, TUI, and Preserves consumers may render, serialize, or forward this DTO through an admitted event sink. The Preserves runtime bridge round-trips only validated retry metadata and rejects unknown reason/status labels or unsafe identities. It is not an assistant message, tool message, model response, or transcript entry. Raw status bodies, headers, exceptions, endpoints, credentials, prompts, tool bodies, and private paths cannot enter it.

## Receipts

`project_provider_retry_receipt` emits metadata-only evidence binding policy, exact plan, completed attempts, canonical reason sequence, optional timer outcome, terminal status, redaction marker, non-claim, and a domain-separated BLAKE3 receipt identity.

Receipt projection rejects any supplied raw request, response body, response headers, endpoint, credential, SDK exception body, prompt, hidden reasoning, tool body, or private path without echoing that value in the error.

A retry receipt records only supplied and validated local evidence. Its hashes are correlation identities, not an access-control or confidentiality mechanism; hosts still protect receipt stores and logs. It does not prove provider behavior, provider availability, provider acceptance, billing behavior, delivery, timer correctness, remote cancellation, exactly-once effects, semantic completion, or production readiness.

## Snapshot-bound retry

A durable model-request snapshot does not grant retry authority. `ModelRequestRetryOwnerDecision` must represent this existing owner's exact decision.

An admitted retry keeps the committed snapshot identity. Context, profile, tool, output, route, authority, or adapter drift requires a new snapshot. See [durable model-request snapshots](model-request-snapshots.md).

## Verification

Run the focused rail from the repository development shell:

```text
nix develop -c scripts/cairn/check-provider-retry --root .
```

The rail checks Nickel drift and positive/negative fixtures, ambient-authority exclusions, focused runtime and std tests, Clippy, exact-plan and stream behavior, injected timer/dispatch gates, progress projection, receipt safety, and a BLAKE3-bound focused receipt. Live-provider smoke remains optional and is not part of fixture-scoped evidence.
