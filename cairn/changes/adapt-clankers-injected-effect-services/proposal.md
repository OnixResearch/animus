# Proposal: Adapt Clankers injected effect services for Animus

## Why

Animus should not let reusable runtime code construct provider clients, read credentials, discover host config, open network connections, launch tools, or mutate sessions directly. Clankers' runtime facade and effect-gating patterns keep those capabilities behind injected services that fail closed when absent or denied. Animus should adopt that boundary before adding richer model/tool/provider execution.

## What Changes

- Define injected service contracts for model/provider dispatch, tool execution, host effects, capability decisions, cancellation, confirmation, and runtime policy checks.
- Ensure missing, unavailable, denied, cancelled, malformed-output, or timed-out services return typed failures before side effects are claimed.
- Add fail-closed confirmation behavior with a default deny broker.
- Add tests proving runtime does not read credentials, discover provider config, open network, construct provider clients, execute product tools, or persist sessions except through injected services.
- Add safe effect receipts that record outcome classes without raw payload leakage.

## Impact

- **Files**: adapter/service traits, std-shell wiring, tests, docs, Cairn artifacts.
- **Testing**: positive injected-service tests and negative missing/denied/unavailable/cancelled/malformed-output fixtures.
- **Non-claims**: this does not implement real providers, plugin runtimes, OAuth, daemon sessions, or product tool catalogs.