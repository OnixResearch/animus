# Clankers Provider Contract Parity Evaluation

Animus adopts Clankers-style provider projection parity as reusable conformance fixtures. The separate non-default `animus-std-openai` crate adds one narrow live host without moving network or credential authority into reusable contracts.

## Source patterns evaluated

Reference material inspected in the sibling Clankers repo:

- `docs/src/reference/provider-contracts.md`: required shared `CompletionRequest` fields, constructor inventory, shared-field serde parity, provider-native message conversion, stream parser entrypoint tests, and fail-closed provider-prefix checks.
- `AGENTS.md` provider notes: `extra_params`/`_session_id` drift risks, constructor-count inventory, provider-vs-router shared-field projection, explicit literal fixtures, and raw SSE parser-entrypoint coverage.
- `cairn/archive/1970-01-01-decouple-current-coupling-hotspots/evidence/provider-router-boundary-rails.md`: evidence pattern for request-shape, provider-native message JSON, and known-prefix fail-closed tests.

## Activation rule

Provider projection fixture rails are active for claimed provider families once a family/API style is named in the conformance index. Live provider/router rails activate when Animus has more than one concrete provider request shape for the same dispatch path. Examples:

- an adapter DTO plus a routed provider DTO;
- multiple provider-specific request DTOs;
- a local provider path and a router/RPC provider path;
- streaming provider backends with normalized event DTOs.

Until live provider/router DTOs exist, provider registry work stays shell-owned and runtime dispatch consumes injected `ModelAdapter` services only after `plan_model_dispatch` admits the selected capability, request contract, prompt budget, stream policy, receipt policy, cancellation/timeout posture, and deny-by-default fallback policy.

## Current provider projection conformance rails

The reusable fixture harness lives in `crates/animus-adapters/src/model/provider_conformance.rs` and is indexed by `cairn/provider-projection-conformance-index.ncl`. It covers OpenAI Responses, OpenAI-compatible chat/local HTTP, Anthropic Messages, and deterministic mock families with literal request projection fixtures, lossy projection rejection, provider-native response normalization, response leak sentinels, and streaming parser-entrypoint fixtures.

Expected request payloads are reviewed literal fixture strings, including ordered roles, text/image content, tool declarations, structured-output intent, reasoning controls, streaming intent, and safe provenance metadata. Response fixtures normalize assistant text, structured payloads, tool calls, refusals, safety blocks, malformed-output diagnostics, usage, and finish reasons into `CanonicalModelResponseEnvelope`. Hidden reasoning, credentials, raw provider payloads, missing finish reasons, malformed usage, impossible response combinations, truncated streams, streams without terminals, and out-of-order terminal events fail closed.

## Future shared request fields

When provider/router DTOs exist, Animus should define a shared request-field matrix. Candidate fields are:

- model identifier;
- ordered messages/content blocks;
- optional system prompt;
- output limit;
- temperature or sampling controls;
- tool declarations and tool-choice metadata;
- provider/session metadata;
- cache policy and TTL;
- reasoning/thinking controls;
- extra provider parameters.

A parity test should project each provider DTO into the shared matrix and fail when any required cross-cutting metadata is dropped.

## Constructor inventory rail

If a concrete provider request struct has multiple construction sites, add an inventory rail that counts real constructors and checks required fields in each constructor snippet. The inventory must fail when a new constructor omits a shared field. Maintainers should update the accepted constructor count only when reviewing a real new construction site.

## Provider-native message conversion

Provider routing adapters must use provider-native message conversion fixtures when backend message shape matters. Tests should pin explicit literal fixtures for representative user, assistant, tool, summary, and metadata content. They must not build the expected value by calling the same body builder under test, and they must reject lossy generic JSON conversion that drops backend-required content.

## Streaming parser-entrypoint coverage

Future streaming claims need at least one parser-entrypoint test per stream family. The test should feed raw `text/event-stream` bytes or equivalent provider stream bytes through the real parser entrypoint and assert normalized stream events. Helper-only state-machine tests are not enough evidence.

## Known provider prefix fail-closed behavior

If Animus adopts provider-qualified model IDs, known-but-unavailable providers must fail closed. A request for a known prefix with missing credentials, disabled backend, failed entitlement, unavailable route, or denied fallback policy must not silently fall back to another provider. The current provider-neutral plan already records fallback denial as a fail-closed planning issue before adapter execution.

## Non-claims

This evaluation does not prove live-provider behavior, OAuth, credential storage, router daemons, provider discovery, or broad provider parity. The separate OpenAI host remains text-only and non-default. Its ignored live smoke is not evidence until an explicit run is indexed.
