# Codebase graph host

Animus exposes a non-default Rust-first syntactic graph through `animus_adapters::codegraph` and the `animus-std/codebase-graph-host` feature.

The reusable adapter surface owns bounded profile, contribution, index, event, query, cache-identity, and metadata-only receipt DTOs. Its reducers operate only on already-loaded bytes, parsed contributions, prior graph state, and explicit events. It has no filesystem, watcher, parser, thread, mmap, process, network, VCS, or cache-file authority.

The optional std host owns capability-scoped reads and the exact `tree-sitter-rust-0.24` parser cohort. It rejects traversal, symlinks, special files, binary/NUL or malformed UTF-8 input, malformed Rust, excess bytes/files/results, duplicate paths, ambiguous renames, and stale queries. Create, modify, delete, and rename events are explicit. Overflow, event gaps, and unsupported events mark the graph stale until the caller performs a bounded full rebuild.

`config/codebase-graph-profile.ncl` is the typed source for language/query cohorts, capability-root label, exclusions, budgets, stale-event policy, disabled-or-disposable cache posture, and redaction/non-claim policy. Runtime Rust revalidates the checked JSON and never evaluates Nickel.

The initial cache posture is disabled. The pure cache identity binds workspace, source, profile, parser, query, schema, and graph identities so a later capability-scoped persistence adapter can quarantine mismatches, but this change does not persist or authorize cache files.

Callers may compose `CodeGraphQueryService` with existing context-reference, retrieval-budget, and read-only tool admission. The service itself grants no file mutation, formatting, refactoring, VCS, process, network, model, or tool authority.

Run:

```text
scripts/cairn/check-codebase-graph --root .
cargo test -p animus-std --features codebase-graph-host codegraph
```

Results are syntactic evidence only. They do not establish type correctness, name-resolution completeness, semantic equivalence, compiler behavior, supported languages beyond the checked Rust cohort, production readiness, or release eligibility.
