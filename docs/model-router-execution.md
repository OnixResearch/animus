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

Fallback and repair execution is policy-bound. Hosts must evaluate failures against `BudgetLedger`, `FallbackChain`, and the selected fallback policy before falling back. Ordinary automatic provider retry is a narrower [same-invocation provider retry contract](provider-retry-execution.md): it cannot select a fallback target and requires fresh router/profile composition for any changed provider, model, authority, prompt, tool, credential, or policy identity. Provider-side fallback, health-check fallback, and ambient model-registry fallback are denied unless the materialized chain explicitly authorizes the target.

## Round-robin rotation

Round-robin model selection is an explicit host-owned policy input, not a provider fallback or live load balancer. Hosts may pass `RouterRotationSnapshot` cursor metadata to `select_recipe_with_rotation` and `materialize_execution_plan_with_rotation`; reusable router logic validates cursor labels and catalog hashes, rotates only already eligible candidates, and returns next-cursor metadata without persisting state, reading clocks, probing health, discovering providers, or touching credentials.

Role rotation happens after role, backend, privacy, budget, duplicate-label, and recipe-family filtering. Fallback rotation only reorders the materialized `FallbackChain` when policy explicitly enables fallback rotation; `evaluate_fallback` still consumes the chain in order and continues to deny provider-side or ambient fallback bypasses.

`RouterRotationReceipt` records safe recipe, route-family, role, policy, cursor class, selected model label, bounded candidate count, catalog hash, next-cursor hash, redaction markers, and a BLAKE3 identity. It rejects raw prompts, provider payloads, health probes, credentials, endpoints, full paths, and unbounded traces.

## Model-evidence advisory inputs

`ModelEvidenceReport` lets hosts feed already-produced verification, conversion, kernel-proof, or benchmark-support reports into routing and admission policy without granting verifier authority to reusable crates. A router `Judge` or verifier-like step is not a semantic agent role: attributed verification or judgment additionally requires a separately admitted `animus_runtime::agent_roles` assignment, exact output capability, immutable subject/check or rubric identity, and the selected separation policy. Semantic role output remains bounded evidence and cannot establish authoritative completion. Validation checks evidence kind, model and corpus identities, verifier identity, property scope, freshness, diagnostics, advisory capability labels, and non-claims. Benchmark-only, stale, mismatched, unsupported-verifier, missing-property, overclaiming, or weakened-non-claim reports fail closed.

Accepted reports are advisory unless a later Cairn change promotes a narrower profile with additional gates. `ModelEvidenceReport::advisory_decision_receipt` preserves non-claims for arbitrary robustness, output correctness, hallucination prevention, verifier soundness, benchmark superiority, release eligibility, GPU correctness, and whole-system safety.

## Verification and non-claims

Fixture-backed evidence currently covers:

- missing, denied, and unavailable service outcomes;
- deterministic fixture execution through the injected service trait;
- single-route and collaborative fan-out dispatch envelopes;
- quorum and over-parallel denial;
- fallback/repair binding and provider-bypass denial;
- round-robin role and fallback rotation with stale-cursor, privacy-filter, and receipt-leak fixtures;
- model-evidence advisory report validation and non-claim preservation;
- safe receipt construction and leak rejection.

Non-claims: this does not implement live OpenAI/Anthropic/local provider clients, provider discovery, credential storage, networked multi-model execution, production latency routing, health checks, verifier execution, GPU access, model safety proofs, benchmark superiority claims, or a default multi-provider agent runtime.
