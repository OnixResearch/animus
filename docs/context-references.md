# Context references

Animus treats prompt context references as typed inputs, not ambient authority. Reusable crates parse and validate references, decide budgets, and produce safe provenance metadata. Host adapters own all filesystem, VCS, image, and network access.

## Boundary

A raw reference such as `@src/lib.rs:2-4`, `@docs/`, `@diff:working`, `@image:assets/logo.png`, or `@https://example.test/docs` becomes a `ContextReference` with:

- the raw label for audit context;
- a normalized target for policy and resolver dispatch;
- a safe target label for receipts and ledgers;
- a reference kind, optional one-based line range, render mode, and source-trust class.

Parsing never reads files, runs VCS commands, decodes images, or fetches URLs. Local targets must remain relative and cannot traverse through `..`; URL safe labels use BLAKE3 identity labels rather than credential-bearing raw URLs.

## Resolver dispatch

Expansion uses explicit host-injected resolver traits for files, directories, diffs, images, and URLs. `plan_context_resolution` checks policy and resolver presence before any resolver method can run. Denied URL policy and missing resolver services fail closed with typed errors and no fallback authority.

`family:context` records those seams as independent roles through `ContextAdapterRoleRecord` and `ContextAdapterFamilyManifest`: file, directory, diff, image, URL, context-layer source, and external-handle roles can be described or registered independently. A host that provides local file context does not need to provide URL fetching, image decoding, crawlers, credential lookup, VCS diff collection, or global discovery. Unsupported, missing, denied, unavailable, malformed, oversized, unsafe, and leak-rejected outcomes fail closed before reusable code attempts fallback dispatch.

## Budget and provenance

`apply_context_budget` records original byte/line counts and a BLAKE3 content identity before deciding whether the fragment is included, truncated, or rejected. Safe provenance is built through `ContextReferenceProvenance`, which stores only kind, status, safe target label, resolver class, policy decision, truncation state, bounded counts, hashes, and redaction markers.

The provenance validator rejects raw content markers, credential-looking labels, absolute paths, oversized/unbounded labels, and inconsistent unbounded-output counts before review summaries or ledgers are emitted.

## Std shell resolvers

`animus-std` provides explicit opt-in context shells for deterministic embedding:

- `NullContextResolver` implements every resolver role as denied/no-op authority.
- `LocalFileContextResolver` reads policy-admitted relative text files under a configured root.
- `LocalDirectoryContextResolver` returns bounded, sorted, relative directory listings.
- `LocalDiffContextResolver` exposes already-loaded diff text; it does not spawn VCS commands.

These shells are not ambient discovery. They reject path escapes, missing files, binary/NUL-containing bodies, oversized files or listings, denied policy, unavailable roots, and network/image roles unless a separate host-owned adapter is explicitly supplied.
