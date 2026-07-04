# Proposal: Generalize adapter capability composition

## Why

Runtime adapter wiring currently centers on model and tool services, while surrounding domains use their own injected services and middleware seams. That keeps individual boundaries typed, but it makes larger compositions harder: multi-model routing, optional event sinks, policy gates, queue services, provenance stores, and cancellation services need a consistent way to declare required capabilities and fail closed when absent.

This change defines a small typed capability composition layer that lets runtime plans declare required host capabilities without discovering ambient services.

## What Changes

- Define typed capability identities and required-capability descriptors for runtime decisions and service-backed helpers.
- Add a host-supplied capability registry or service bundle that resolves explicit ports such as model, tool, event sink, policy gate, queue, cancellation, confirmation, and provenance receipt sink.
- Preserve existing simple `RuntimeAdapters::new(model, tool)` ergonomics as a compatibility constructor over the new representation.
- Add fail-closed outcomes for absent, denied, unavailable, malformed, cancelled, and timed-out capabilities.
- Add tests for multi-adapter composition, missing capability denial, duplicate capability registration, and no ambient fallback.

## Impact

- **Files**: future adapter service bundle/registry DTOs, runtime shell wiring, middleware integration, tests, docs, facade inventory, and Cairn artifacts.
- **Testing**: positive single-service and multi-service composition; negative missing/duplicate/denied/unavailable/malformed/cancelled/timeout cases.
- **Non-claims**: this does not add concrete provider discovery, plugin loading, process execution, or global service locators to reusable runtime code.
