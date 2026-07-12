## Context

Accepted Animus contracts can classify provider capabilities, project canonical requests, parse streams, and validate same-target retry identity. They intentionally do not decide provider-native retryability, wait on clocks, or repeat network dispatch. Tau places failure classification in provider adapters and exposes retry progress as neutral agent events.

The adaptation will preserve that ownership while adding stricter identity and stream commit rules. Retrying after partial semantic output can duplicate text or tool calls and is therefore not treated like a pre-response transport failure.

## Decisions

### 1. Author retry policy in Nickel

**Choice:** A typed Nickel policy defines schema identity, supported canonical failure classes, provider-family mappings, maximum attempts, deterministic delay schedule bounds, cancellation posture, stream commit policy, and safe event/receipt projection. Rust consumes a checked generated artifact and revalidates it.

**Rationale:** Retry counts, delays, and status mappings are policy, not hidden constants or ambient environment defaults.

### 2. Keep provider-native classification in the host adapter

**Choice:** The host maps HTTP, SDK, transport, and provider error observations into canonical classes such as rate-limited, transient transport, service unavailable, request timeout, conflict, authentication, authorization, invalid request, unsupported feature, and unknown. The pure core sees only validated canonical observations and safe metadata.

**Rationale:** Reusable runtime must not know provider status codes, exception types, response bodies, or SDK internals.

### 3. Denials and caller errors are terminal

**Choice:** Authentication, authorization, entitlement, policy denial, capability denial, invalid request, unsupported feature, malformed canonical request, missing service, missing credential, and unsafe response classes are never automatically retried. Unknown failures default to terminal unless policy explicitly maps a bounded provider-family observation.

**Rationale:** Retrying a permanent or policy failure adds load and can bypass review intent.

### 4. Bind retries to the exact invocation plan

**Choice:** Automatic retry requires the same provider, model, canonical request identity, prompt/system identity, tool catalog identity, response-format identity, reasoning/stream intent, credential-handle class, policy identity, and model-runtime profile plan. Any target or authority change returns a recomposition requirement rather than a retry.

**Rationale:** A retry must not become implicit fallback or authority widening.

### 5. Define a semantic stream commit point

**Choice:** Retry remains eligible only before any assistant-visible content, provider-exposed reasoning, tool-call delta, structured-output delta, or other non-replay-safe semantic frame is accepted. A transport start marker alone need not commit the stream. After commit, failure is terminal unless a separately specified provider resume protocol is admitted.

**Rationale:** Replaying a partially emitted tool call or content stream can duplicate model-visible actions.

### 6. Keep schedule calculation pure and waiting injected

**Choice:** The core returns an explicit retry decision containing next attempt, bounded delay, reason class, exact-plan hash, and cancellation requirement. A host timer service performs the wait and reports completed, cancelled, timed out, or unavailable. The core does not read clocks, sleep, poll, or add random jitter.

**Rationale:** Timer effects and entropy are host authority; deterministic plans are easy to test.

### 7. Cancellation interrupts backoff

**Choice:** A correlated cancellation request during backoff stops the scheduled retry before provider dispatch and emits cancellation events. Cancellation after dispatch remains a request to the provider service and is not proof that billing or remote computation stopped.

**Rationale:** Operators should not wait through a delay after cancelling, but evidence must not overstate remote cancellation.

### 8. Emit provider-neutral progress

**Choice:** Retry progress events contain run and attempt correlation, attempt/max counts, bounded delay class or units, canonical reason class, and safe status. Raw status bodies, headers, SDK exceptions, endpoints, credentials, and hidden reasoning are excluded. Print/TUI policy decides whether to display progress.

**Rationale:** Frontends need progress without coupling to provider internals.

### 9. Separate retry from router fallback

**Choice:** Exhaustion returns the original terminal provider failure plus retry summary metadata. Cross-target fallback is evaluated later by existing router/fallback policy after fresh profile composition; the retry executor never selects another target.

**Rationale:** Retry and fallback have different identity, authority, and evidence requirements.

### 10. Test execution with injected fakes first

**Choice:** Deterministic fixtures cover transient-then-success, rate-limit-then-success, terminal client failure, cancellation during backoff, timer unavailability, exhaustion, stale plan, changed tool catalog, partial semantic stream failure, malformed provider mapping, and receipt leaks. Optional live smoke is separate evidence.

**Rationale:** Retry correctness should not depend on live rate limits or network timing.

## Risks / Trade-offs

- Conservative stream commit rules may forgo retries some providers could safely resume. A provider-specific resume protocol requires a separate contract.
- Deterministic backoff without jitter can synchronize callers. Optional host-supplied jitter can be added later only with explicit bounded policy and evidence.
- Provider documentation about retryable statuses can be wrong or change. Mappings must be versioned and fixture-backed.

## Non-Goals

- Cross-provider or cross-model fallback.
- Retrying tool execution, policy denial, malformed requests, or unsafe output.
- Exactly-once provider billing or remote cancellation proof.
- Randomness, clocks, sleeps, network clients, or credential discovery in reusable crates.
- General partial-stream resume.
