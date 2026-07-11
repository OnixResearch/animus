# Proposal: Split std host adapters into authority-scoped crates

## Why

`animus-std` is the imperative shell, but it currently groups several host-facing concerns behind one crate: runtime config loading, local context resolvers, provider registry metadata, credential-source diagnostics, document/model-evidence ingest planning, observability sinks, optional Iroh/Syndicate seams, and dogfood smoke wiring. Embedders often need one host adapter family without the rest.

Authority-scoped std host crates make host capabilities explicit: local filesystem context, provider registry selection, document ingest, observability sinks, and smoke/demo shells become separate snap-on blocks.

## What Changes

- Define authority-scoped host adapter crates or feature packs under the std boundary while preserving `animus-std` as a compatibility facade.
- Add catalog metadata for each host adapter's authority: environment read, filesystem read, process execution, network transport, credential lookup, persistence, or terminal access.
- Keep deterministic parsing and planning in pure helpers, with actual host effects in thin shells.
- Add fixtures proving missing or disabled host adapters fail closed and do not fall back to ambient discovery.

## Impact

- **Surfaces**: `animus-std` crate features, optional new host crates, component catalog, runtime facade inventory, docs/request lifecycle, README guidance.
- **Testing**: host-adapter unit tests, facade compatibility tests, authority-label checks, negative ambient-fallback fixtures, and Cairn gates.
- **Non-claims**: this does not add live providers, default credential discovery, default persistence, default network transport, daemon behavior, or product CLI behavior.
