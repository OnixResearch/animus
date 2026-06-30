# Clankers Provider Contract Parity Evaluation

Animus should adopt Clankers-style provider/router parity rails only when Animus grows multiple provider or router request DTOs. Today Animus has a small model adapter contract plus shell-owned provider registry metadata, so this note records future adoption criteria without adding provider clients, routers, OAuth, credential lookup, network transport, or live provider checks.

## Source patterns evaluated

Reference material inspected in the sibling Clankers repo:

- `docs/src/reference/provider-contracts.md`: required shared `CompletionRequest` fields, constructor inventory, shared-field serde parity, provider-native message conversion, stream parser entrypoint tests, and fail-closed provider-prefix checks.
- `AGENTS.md` provider notes: `extra_params`/`_session_id` drift risks, constructor-count inventory, provider-vs-router shared-field projection, explicit literal fixtures, and raw SSE parser-entrypoint coverage.
- `cairn/archive/1970-01-01-decouple-current-coupling-hotspots/evidence/provider-router-boundary-rails.md`: evidence pattern for request-shape, provider-native message JSON, and known-prefix fail-closed tests.

## Activation rule

Provider parity rails activate when Animus has more than one provider request shape for the same dispatch path. Examples:

- an adapter DTO plus a routed provider DTO;
- multiple provider-specific request DTOs;
- a local provider path and a router/RPC provider path;
- streaming provider backends with normalized event DTOs.

Until then, provider registry work stays shell-owned and runtime dispatch consumes injected `ModelAdapter` services only.

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

If Animus adopts provider-qualified model IDs, known-but-unavailable providers must fail closed. A request for a known prefix with missing credentials, disabled backend, failed entitlement, or unavailable route must not silently fall back to another provider.

## Non-claims

This evaluation does not add live providers, network clients, OAuth, credential storage, router daemons, provider discovery, streaming parsers, or live checks. It defines the future rails that must exist before those surfaces are treated as stable.
