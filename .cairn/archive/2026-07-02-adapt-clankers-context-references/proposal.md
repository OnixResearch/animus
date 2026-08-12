# Proposal: Adapt Clankers context references for Animus

## Why

Clankers has a useful `@` reference pattern for bringing bounded local files, directories, diffs, images, and policy-enabled URLs into a turn while recording safe provenance metadata. Animus needs the same capability shape before richer prompt assembly, dataset export, and replay work can claim that model-visible context is deterministic and auditable.

The adaptation must stay embeddable. Reusable Animus crates should parse, validate, budget, and record context references, but host adapters must own filesystem reads, diff generation, image loading, URL fetching, and any network policy.

## What Changes

- Define Animus-owned context-reference DTOs for raw references, normalized targets, reference kind, policy decision, expansion status, and model-visible context fragments.
- Add pure parsing, normalization, budget, and provenance validation for local files, line ranges, directories, diffs, images, and URL references.
- Route expansion through injected host resolver services that fail closed when filesystem, VCS, image, or network authority is absent or denied.
- Record safe ledger/receipt metadata such as kind, status, byte/line counts, truncation state, safe target labels, and BLAKE3 content hashes without persisting raw expanded content by default.
- Add focused positive and negative fixtures for valid local references, unsupported URL policy, path escapes, oversized references, missing resolver services, malformed ranges, and receipt leak attempts.

## Impact

- **Files**: future context-reference DTOs, resolver traits, prompt assembly docs, policy fixtures, tests, and Cairn artifacts.
- **Testing**: positive context expansion tests and negative denied/malformed/oversized/missing-service/leak fixtures.
- **Non-claims**: this does not implement a browser, unrestricted web fetcher, ambient project-file discovery for embedders, or Clankers session-artifact reference compatibility.
