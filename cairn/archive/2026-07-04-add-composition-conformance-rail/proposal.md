# Proposal: Add composition conformance rail

## Why

Most Animus tests validate individual contracts well, but composability needs tests that intentionally combine independent modules. The repo should have a focused rail that proves representative surfaces compose safely: provider hardening with model dispatch, tool contracts with provenance receipts, headless protocol with runtime events, middleware denials with failure reports, and capability absence with no ambient fallback.

This change adds a maintained composition conformance rail with positive and negative scenarios across module boundaries.

## What Changes

- Define a composition conformance matrix covering high-value cross-module pairs and triples.
- Add pure in-memory fixtures and fake adapters for representative positive flows.
- Add negative fixtures for malformed responses, unauthorized tools, missing services, oversized output, unsafe receipts, and disabled feature surfaces.
- Add a focused script/check and optional Nix check that runs the conformance rail.
- Record safe evidence summaries without raw prompts, provider payloads, credentials, full paths, or unbounded tool outputs.

## Impact

- **Files**: future conformance tests, fixtures, check script, Nix check, docs, facade inventory evidence links, and Cairn artifacts.
- **Testing**: positive cross-module composition and negative fail-closed cases.
- **Non-claims**: this does not require live providers, real tools, network access, terminal UI, process execution, or persistence backends.
