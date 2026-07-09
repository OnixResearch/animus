# Model-router execution service

Animus router execution is a host-owned service seam. `animus-router` owns pure plan materialization, budget/fallback policy, route labels, and safe receipt metadata. Host shells own concrete provider clients, credentials, concurrency, network I/O, latency telemetry, synthesis execution, and any live multi-model fan-out.

The reusable contract lives in `animus-adapters::router::execution` so runtimes and shells can share typed dispatch envelopes without gaining provider authority. A host passes a validated `RouterExecutionPlan`, a safe request handle, host-approved model catalog bindings, cancellation/deadline metadata, and `BudgetLedger` state. Missing or denied services fail closed; no ambient provider registry, fallback model, shell command, network request, or credential lookup is created by reusable crates.

## Dispatch and outcomes

A dispatch envelope records only safe execution metadata:

- recipe ID and route family;
- ordered role steps and candidate labels authorized by the materialized plan;
- per-step parallelism cap and quorum;
- selected output contract, cancellation posture, deadline class, budget state, and safe correlation metadata;
- explicit fallback policy.

Step outcomes and receipts record status classes, bounded counts, selected safe labels, BLAKE3 identities, duration buckets, fallback/repair classes, output-contract status, policy hash, and redaction markers. They must not contain raw prompts, provider-native request or response JSON, worker outputs, judge rationales, hidden reasoning, credentials, full paths, or unbounded traces.

## Fallback and repair

Fallback and repair execution is policy-bound. Hosts must evaluate failures against `BudgetLedger`, `FallbackChain`, and the selected fallback policy before retrying or falling back. Provider-side fallback, health-check fallback, and ambient model-registry fallback are denied unless the materialized chain explicitly authorizes the target.

## Verification and non-claims

Fixture-backed evidence currently covers:

- missing, denied, and unavailable service outcomes;
- deterministic fixture execution through the injected service trait;
- single-route and collaborative fan-out dispatch envelopes;
- quorum and over-parallel denial;
- fallback/repair binding and provider-bypass denial;
- safe receipt construction and leak rejection.

Non-claims: this does not implement live OpenAI/Anthropic/local provider clients, provider discovery, credential storage, networked multi-model execution, production latency routing, health checks, or a default multi-provider agent runtime.
