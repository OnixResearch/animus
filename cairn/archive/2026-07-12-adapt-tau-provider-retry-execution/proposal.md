## Why

Animus has provider dispatch plans, canonical stream frames, same-target retry identity checks, and fixture-backed provider parsers, but no host retry executor. Retry policy remains an explicit non-claim of the concrete streaming and dispatch seams.

Tau demonstrates two useful runtime behaviors: provider-specific adapters classify transient failures before translating them into provider-neutral retry progress, and cancellation can interrupt backoff. Animus should adapt those behaviors with Nickel-authored bounds, a pure retry decision core, an injected timer shell, exact invocation-plan binding, and stricter partial-stream safety.

## What Changes

- Add a typed Nickel retry policy for bounded attempts, canonical retry classes, delay schedule, cancellation posture, and provider-family mappings. r[agent_harness_runtime.tau_provider_retry.policy]
- Classify retryable and terminal provider failures at the host adapter boundary without exposing raw HTTP or SDK payloads. r[agent_harness_runtime.tau_provider_retry.classification]
- Permit automatic retry only for the same exact target and invocation plan; target changes continue to require fresh composition. r[agent_harness_runtime.tau_provider_retry.same_target]
- Forbid automatic retry after a semantic content, reasoning, tool-call, or other non-replay-safe stream frame has been emitted. r[agent_harness_runtime.tau_provider_retry.stream_safety]
- Use an injected host timer for bounded cancellable backoff and emit provider-neutral retry progress events. r[agent_harness_runtime.tau_provider_retry.backoff] r[agent_harness_runtime.tau_provider_retry.events]
- Add metadata-only retry receipts and positive/negative execution fixtures. r[agent_harness_runtime.tau_provider_retry.receipts] r[agent_harness_runtime.tau_provider_retry.verification]

## Impact

- **Policy**: Nickel is the source of truth for retry classes and bounds; runtime Rust consumes a checked projection.
- **Runtime core**: decides retry, stop, or cancellation over explicit observations without clocks, sleeps, network, credentials, or provider clients.
- **Host shell**: owns provider-native error mapping, timer waits, cancellation polling, and repeated dispatch through the existing provider service.
- **Claims**: fixture-backed execution proves policy and event ordering only. Live reliability, provider availability, rate-limit correctness, cross-provider fallback, exactly-once billing, and partial-response resumability remain non-claims unless separately evidenced.
