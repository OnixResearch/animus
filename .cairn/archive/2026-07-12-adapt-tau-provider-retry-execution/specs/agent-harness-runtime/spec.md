# Agent Harness Runtime Specification Delta

## ADDED Requirements

### Requirement: Provider retry policy is typed and bounded

r[agent_harness_runtime.tau_provider_retry.policy] Animus MUST author provider retry classes, provider-family mappings, attempt bounds, deterministic delay bounds, cancellation posture, semantic stream commit policy, and safe event/receipt fields as typed Nickel policy with a checked Rust projection.

#### Scenario: Valid retry policy is loaded

- GIVEN a generated retry policy matches its Nickel source, schema, and content identities and all attempt and delay bounds are valid
- WHEN the host loads and Rust revalidates the projection
- THEN the policy MAY govern retry planning
- AND runtime code MUST NOT evaluate Nickel or read ambient retry environment variables.

#### Scenario: Retry policy has invalid bounds

- GIVEN a policy has zero allowed initial attempts, excessive attempts, negative or excessive delay, unknown failure class, duplicate mapping, or unsupported stream posture
- WHEN policy validation runs
- THEN it MUST fail before provider dispatch
- AND no built-in fallback constants may silently replace it.

### Requirement: Provider-native failures map to canonical retry classes

r[agent_harness_runtime.tau_provider_retry.classification] Provider host adapters MUST map admitted HTTP, SDK, transport, and provider observations into validated canonical retry classes, while reusable runtime receives no raw status bodies, headers, exception values, endpoints, credentials, or SDK types.

#### Scenario: Rate limit is mapped by selected provider family

- GIVEN the selected provider adapter observes a versioned provider-native rate-limit response covered by current policy
- WHEN classification runs
- THEN it MAY produce the canonical `rate_limited` class with safe bounded metadata
- AND the raw response body and headers MUST remain host-private.

#### Scenario: Permanent or unknown failure is observed

- GIVEN authentication, authorization, entitlement, policy, capability, invalid-request, unsupported-feature, malformed, unsafe, missing-service, or unmapped unknown failure is observed
- WHEN retry classification runs
- THEN the decision MUST be terminal by default
- AND the executor MUST NOT repeat provider contact automatically.

### Requirement: Automatic retry reuses one exact invocation plan

r[agent_harness_runtime.tau_provider_retry.same_target] Automatic provider retry MUST preserve the exact provider, model, canonical request, model-runtime profile, prompt/system, tool catalog, response format, reasoning and stream intent, credential-handle class, and policy identities, and any target or authority change MUST require fresh composition outside the retry executor.

#### Scenario: Same-target transient retry is admitted

- GIVEN a retryable pre-commit failure and every invocation identity matches the prior admitted plan
- WHEN retry admission runs within budget
- THEN it MAY return a same-target retry decision bound to the exact plan hash
- AND it MUST not add capabilities, tools, prompts, memory, or provider parameters.

#### Scenario: Tool catalog changes between attempts

- GIVEN a retryable failure but the next attempt carries a different tool-catalog identity or another changed invocation identity
- WHEN retry admission runs
- THEN it MUST reject exact-plan reuse
- AND it MUST not reinterpret the change as provider fallback.

### Requirement: Partial semantic streams are not automatically replayed

r[agent_harness_runtime.tau_provider_retry.stream_safety] Automatic retry MUST remain eligible only before any accepted assistant content, reasoning, tool-call, structured-output, or policy-declared non-replay-safe semantic frame, and a post-commit failure MUST be terminal unless a separate provider resume contract is admitted.

#### Scenario: Transport fails before semantic content

- GIVEN provider contact started but no semantic frame was accepted
- WHEN a retryable transport failure occurs
- THEN the stream state MAY remain retry-eligible under policy
- AND the next attempt MUST still pass exact-plan and budget checks.

#### Scenario: Tool-call delta was accepted

- GIVEN any part of a provider tool call was accepted into canonical stream state
- WHEN the transport fails
- THEN automatic retry MUST be denied as post-commit
- AND Animus MUST not risk duplicating or extending that tool call through ordinary retry.

### Requirement: Backoff uses an injected cancellable timer

r[agent_harness_runtime.tau_provider_retry.backoff] Retry schedule calculation MUST be pure and bounded, while waiting MUST use an injected host timer service that reports completion, cancellation, timeout, or unavailability without clocks, sleeps, polling, or entropy in reusable core.

#### Scenario: Timer completes the admitted delay

- GIVEN retry planning returns a bounded delay and the injected timer completes for the correlated run and attempt
- WHEN the host reports completion
- THEN the reducer MAY request the next exact-plan provider attempt
- AND attempt state MUST advance exactly once.

#### Scenario: Cancellation arrives during backoff

- GIVEN a retry is waiting and a correlated cancellation request is accepted
- WHEN the timer reports cancellation
- THEN no next provider dispatch may occur
- AND the run MUST emit a typed cancellation outcome rather than wait for the full delay.

### Requirement: Retry progress is provider-neutral

r[agent_harness_runtime.tau_provider_retry.events] Retry execution MUST emit ordered provider-neutral progress events containing only correlated run/attempt identity, attempt and maximum counts, bounded delay class or units, canonical reason class, and safe status.

#### Scenario: Frontend receives retry progress

- GIVEN an admitted retry waits before another provider request
- WHEN progress is emitted
- THEN headless, SDK, and TUI consumers MAY render attempt and delay status
- AND the event MUST NOT become assistant transcript content or expose provider-native diagnostics.

### Requirement: Retry receipts are metadata-only

r[agent_harness_runtime.tau_provider_retry.receipts] Retry receipts MUST bind policy, exact invocation plan, attempt sequence, canonical reason classes, timer/cancellation outcomes, terminal status, redaction state, and domain-separated BLAKE3 identity without raw request or provider material.

#### Scenario: Exhausted retry receipt is projected

- GIVEN all admitted attempts are consumed and the provider remains unavailable
- WHEN receipt projection runs
- THEN it MUST preserve the original terminal failure class plus bounded attempt metadata
- AND it MUST not claim cross-provider fallback, exactly-once billing, remote cancellation, or live-provider reliability.

#### Scenario: Receipt contains provider payload

- GIVEN receipt input contains a raw response body, header, endpoint, credential, SDK exception, prompt, hidden reasoning, tool body, or private path
- WHEN receipt validation runs
- THEN it MUST reject the receipt without echoing the private material.

### Requirement: Retry execution has positive and negative evidence

r[agent_harness_runtime.tau_provider_retry.verification] Provider retry execution MUST include positive and negative tests for policy projection, classification, exact-plan binding, attempt and delay bounds, semantic stream commit, timer outcomes, cancellation, progress events, exhaustion, terminal failures, fallback separation, and receipt safety.

#### Scenario: Retry executor is ready for archive

- GIVEN maintainers intend to sync and archive the retry change
- WHEN closeout validation runs
- THEN focused provider, runtime, std, parser, parity, profile, composition, policy, receipt, dogfood-hygiene, Cairn, and relevant Nix checks MUST pass
- AND optional live-provider smoke MUST remain separately indexed and absent live evidence MUST not block fixture-scoped claims.
