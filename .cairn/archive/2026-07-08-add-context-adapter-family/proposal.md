# Proposal: Add context adapter family

## Why

Animus already has typed context-reference parsing, host-injected resolver traits, context-file layering DTOs, fresh-context task descriptors, and prompt assembly sources. Those pieces prove the authority model, but context is not yet named as a maintained adapter family in the contracts index or adapter conformance matrix. Without a formal family, embedders may treat context as ad hoc host glue, add ambient filesystem or network access to reusable crates, or ship concrete std resolvers without the same positive/negative conformance coverage as model, tool, workspace, policy, receipt, and memory adapters.

A context adapter family makes the existing design explicit: hosts own all authority for file, directory, diff, image, URL, layered-context discovery, and external context handles; reusable crates own pure parsing, planning, budgeting, admission, prompt assembly metadata, and safe provenance. This gives embedders a discoverable seam while avoiding a broad monolithic adapter that mixes unrelated authorities.

## What Changes

- Formalize `family:context` in the contracts index and adapter conformance index with ownership, authority boundaries, receipt behavior, focused surfaces, and validation rails.
- Preserve narrow role-specific resolver/discovery seams rather than adding a single omnipotent `ContextAdapter`.
- Add adapter-family DTOs or registry wrappers only where they clarify availability, capability, outcome, and conformance evidence for file, directory, diff, image, URL, context-layer source, and external-context-handle roles.
- Add standard-library no-op/null context services plus policy-gated local file, directory, and diff resolver implementations as host-owned shells.
- Add focused positive and negative tests for missing resolvers, denied policy, unavailable services, malformed resolver output, oversized/truncated context, unsafe targets, schema mismatches, leak rejection, and no ambient fallback.
- Update docs so embedders know when to use context references, context layering, fresh-context task descriptors, and std-shell resolvers.

## Impact

- **Files**: future `animus-adapters` context-family registry/outcome DTOs if needed, `animus-std` resolver implementations, `docs/contracts-index.md`, `cairn/adapter-conformance-index.ncl`, context docs, tests, and Cairn artifacts.
- **Testing**: focused positive tests for supported reference kinds, context-layer descriptors, std no-op/local resolvers, and safe provenance; negative tests for absent/denied/unavailable services, unsafe paths/URLs, oversized context, malformed output, raw-body leakage, and unsupported schema/capability claims.
- **Non-claims**: this does not add default network fetching, default image decoding, a repository-wide crawler, implicit VCS access, global context-file discovery, credentialed URL support, or a monolithic adapter with filesystem/network/VCS authority.