## Phase 1: Retry policy and observations

- [x] [serial] Inventory existing provider dispatch, stream parser, canonical failure, model-profile retry binding, router fallback, RuntimeEvent, cancellation, and receipt owners and define the retry executor's exact non-overlap. r[agent_harness_runtime.tau_provider_retry.policy] r[agent_harness_runtime.tau_provider_retry.verification]
- [x] [serial] Author a typed Nickel retry policy for schema/version identity, canonical retry classes, provider-family mappings, attempt bounds, delay schedule bounds, cancellation posture, stream commit policy, and safe event/receipt fields; generate and revalidate the Rust projection. r[agent_harness_runtime.tau_provider_retry.policy]
- [x] [depends:agent_harness_runtime.tau_provider_retry.policy] Define provider-native observation input, canonical failure observation, retry decision, timer request/outcome, attempt state, stream commit state, progress event metadata, and receipt DTOs. r[agent_harness_runtime.tau_provider_retry.classification] r[agent_harness_runtime.tau_provider_retry.backoff] r[agent_harness_runtime.tau_provider_retry.events]

## Phase 2: Pure retry core

- [x] [depends:agent_harness_runtime.tau_provider_retry.classification] Implement provider-family mapping validation and pure canonical classification, defaulting unknown, authentication, authorization, entitlement, policy, capability, invalid-request, malformed, unsafe, and missing-service failures to terminal. r[agent_harness_runtime.tau_provider_retry.classification]
- [x] [depends:agent_harness_runtime.tau_provider_retry.policy] Implement attempt-budget and deterministic delay-plan reduction with named bounds, overflow-safe arithmetic, and no clocks, sleeps, entropy, environment, or network access. r[agent_harness_runtime.tau_provider_retry.backoff]
- [x] [depends:agent_harness_runtime.tau_provider_retry.classification] Bind retry admission to the exact provider/model/request/profile/prompt/tool/format/reasoning/stream/credential/policy identities and require recomposition for any target or authority change. r[agent_harness_runtime.tau_provider_retry.same_target]
- [x] [depends:agent_harness_runtime.tau_provider_retry.same_target] Implement semantic stream commit tracking and deny automatic retry after content, thought, tool-call, structured-output, or any configured non-replay-safe frame. r[agent_harness_runtime.tau_provider_retry.stream_safety]
- [x] [depends:agent_harness_runtime.tau_provider_retry.backoff] Implement provider-neutral retry progress event projection and metadata-only retry receipt validation. r[agent_harness_runtime.tau_provider_retry.events] r[agent_harness_runtime.tau_provider_retry.receipts]

## Phase 3: Injected execution shell

- [x] [depends:agent_harness_runtime.tau_provider_retry.backoff] Add an injected timer service boundary and thin host wait loop that accepts correlated cancellation, reports timer unavailability/failure, and never sleeps or polls inside reusable core. r[agent_harness_runtime.tau_provider_retry.backoff]
- [x] [depends:agent_harness_runtime.tau_provider_retry.same_target] Wire repeated dispatch through the existing host provider service only after exact-plan revalidation; keep router fallback and fresh target composition outside the retry executor. r[agent_harness_runtime.tau_provider_retry.same_target]
- [x] [depends:agent_harness_runtime.tau_provider_retry.events] Stream progress to headless/SDK/TUI consumers without persisting raw provider diagnostics or treating retry events as assistant transcript messages. r[agent_harness_runtime.tau_provider_retry.events]

## Phase 4: Positive and negative evidence

- [x] [parallel] Add positive fixtures for transient transport then success, rate-limit then success, service-unavailable then success, zero-delay admitted retry, bounded exponential or policy schedule, cancellation during backoff, stable exact-plan reuse, ordered events, and deterministic receipts. r[agent_harness_runtime.tau_provider_retry.verification]
- [x] [parallel] Add negative fixtures for authentication, authorization, entitlement, invalid request, unsupported feature, policy/capability denial, unknown failure, exhausted attempts, delay overflow, timer absence/failure, stale run, changed target/profile/prompt/tool catalog/credential policy, partial semantic stream, malformed mapping, cancellation race, cross-target fallback attempt, and raw payload/header/credential/path/reasoning leakage. r[agent_harness_runtime.tau_provider_retry.verification]
- [x] [depends:agent_harness_runtime.tau_provider_retry.stream_safety] Add fragmented-stream tests proving pre-semantic transport failure may retry while any accepted semantic delta makes the same failure terminal. r[agent_harness_runtime.tau_provider_retry.stream_safety] r[agent_harness_runtime.tau_provider_retry.verification]

## Phase 5: Documentation and closeout

- [x] [serial] Update provider dispatch/streaming, model profile, router fallback, event lifecycle, request lifecycle, component catalog, runtime facade, dogfood readiness, policy, and README reference docs with retry ownership and non-claims. r[agent_harness_runtime.tau_provider_retry.policy] r[agent_harness_runtime.tau_provider_retry.receipts]
- [x] [serial] Add a focused retry-execution rail for Nickel drift, mapping fixtures, exact-plan identity, stream commit, timer/cancellation, attempts/delays, events, receipts, and default dependency isolation. r[agent_harness_runtime.tau_provider_retry.verification]
- [x] [serial] Run focused provider/runtime/std tests, formatting, Clippy, Nickel checks, parser/parity rails, composition conformance, safe receipts, dogfood claim hygiene, Cairn validation/gates, and relevant Nix checks; keep live smoke optional and separately indexed. r[agent_harness_runtime.tau_provider_retry.verification]
