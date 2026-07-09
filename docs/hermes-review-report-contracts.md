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

## Corrective memory boundary

Corrective records are optional host-supplied DTOs. `validate_corrective_record` accepts safe labels, trust scope, freshness, prompt budget, redaction state, and BLAKE3 body identity only; raw napkin or memory bodies stay in the host store. `admit_corrective_record` requires explicit host policy for trusted source labels, allowed scopes, freshness, and prompt budget before a record can influence prompt assembly. Always-read records, untrusted sources, expired records, secret records, and over-budget records fail closed or are omitted with safe reasons.

`corrective_promotion_candidates` routes repeated review metrics into advisory targets such as deterministic checks, policy guidance, spec rules, memory guidance, or human checkpoints. Append and promote operations use explicit `validate_corrective_append_request` and `validate_corrective_promote_request` DTO validation against a host-owned store status. Safe receipts from `validate_corrective_host_action_receipt` contain labels, status, issue code, and BLAKE3 hashes only.

Non-claims: Animus does not load `.agent/napkin.md`, create global corrective memory, persist records by default, edit Cairn tasks/specs/policy automatically, or make review metrics a scoring authority.

## Validation

Focused validation:

```sh
nix develop -c cargo test -p animus-adapters --locked review::tests
nix develop -c cargo test -p animus-adapters --locked
nix run .#cairn -- validate --root .
```

Full workspace clippy/flake closeout remains subject to the repo-wide `animus-adapters` clippy debt tracked in Cairn evidence.
