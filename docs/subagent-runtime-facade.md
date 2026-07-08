# Subagent runtime facade

`animus_runtime::subagents` is the reusable functional core for subagent child-run validation and coordination. It does not execute children, discover services, read credentials, open networks, spawn processes, read files, import raw child logs, or apply mutations.

## Boundary

The facade normalizes recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks into validated child-run admissions. Shared context packets carry selected safe items, deterministic BLAKE3 identities, and budget summaries. Typed capability conversion keeps stable adapter labels at public boundaries while runtime decisions operate over catalog-validated typed values.

Host execution remains injected. `SubagentService` is a caller-supplied port for enqueue, status, cancel, and collect operations. Missing or denied services fail closed and never cause a fallback scheduler, shell, provider, or external CLI to be constructed by reusable runtime code.

## Pure reducers

The module includes deterministic reducers for:

- capability catalog conversion and per-ceiling denial reporting;
- shared context-packet validation and child-kind projection;
- child-run facade validation and admission identity creation;
- coordinator state transitions and safe lifecycle event projection;
- fan-out plan validation and fan-in aggregation with mutation arbitration;
- lineage record validation and replay influence checks;
- UI-neutral status projections and command descriptors;
- offline ACP/MCP external-subagent conformance fixtures.

## Safety rules

Subagent receipts, lineage records, UX projections, and conformance reports contain only safe labels, status classes, requirement ids, BLAKE3 hashes, bounded evidence references, and budget summaries. Validation rejects raw transcripts, raw child logs, provider payload markers, credential-like material, full-path markers, unbounded output markers, wildcard authority, duplicate normalized labels, stale handles, unbounded detail import, mutation bypass attempts, and missing expected failure classes.

## Verification

Focused verification lives in `crates/animus-runtime/src/subagents.rs` unit tests and covers positive and negative cases for every subagent surface. Run:

```sh
cargo test -p animus-runtime subagents --locked
cargo clippy -p animus-runtime --all-targets --no-deps -- -D warnings
```

The broader `cargo clippy -p animus-runtime --all-targets -- -D warnings` rail currently reaches pre-existing `animus-router` lint failures before checking runtime; keep using the no-deps runtime Clippy pass for focused subagent validation until that dependency rail is clean.
