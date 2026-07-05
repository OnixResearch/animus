# Hermes review report contracts

Animus adapts the Hermes review-report pattern as an Animus-owned, pure adapter contract. The parser operates over in-memory markdown only; hosts own file reads, storage, timestamps, and append-only persistence.

## Grammar

A report must contain a `VERDICT: PASS | WARN | FAIL` line. Optional `##` sections are parsed as bounded section metadata with line counts and BLAKE3 body hashes.

Metric-bearing findings are parsed only inside `## Findings` from bullets shaped like:

```md
- [class=lint scope=adapter route=test promoted=no] Short safe summary.
```

The required tags are `class`, `scope`, `route`, and `promoted`. `promoted` is limited to `yes` or `no`. Checklist bullets in other sections remain human-readable prose and do not become metric records.

## Metrics boundary

Parsing, summary, and promotion-candidate analysis are read-only. Durable writes require an explicit host append action using `append_review_metric_record`. The resulting record contains safe metadata, counts, tags, verdict, report hash, finding summary hashes, and a BLAKE3 record hash. It excludes raw prompts, provider payloads, credentials, full host paths, and unbounded tool/model output.

Repeated-finding promotion candidates group unpromoted findings with the same `class`, `scope`, and `route`. The grouping suggests a durable prevention route; it does not mark records promoted, edit tasks, mutate specs, or create lifecycle changes automatically.

## Validation

Focused validation:

```sh
nix develop -c cargo test -p animus-adapters --locked review::tests
nix develop -c cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
```

Full workspace clippy/flake closeout remains subject to the repo-wide `animus-adapters` clippy debt tracked in Cairn evidence.
