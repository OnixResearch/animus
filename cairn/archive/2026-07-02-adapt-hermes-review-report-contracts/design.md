# Design: Adapt Hermes review report contracts for Animus

## Context

Hermes' review-gate plugin and helper scripts share a report grammar for final reviews, OpenSpec gates, and metrics. The useful part for Animus is the stable boundary: a report can be read by humans and parsed by deterministic tooling.

Reference material inspected:

- `/home/brittonr/git/hermes/_global/extensions/lib/review-contract.ts`
- `/home/brittonr/git/hermes/tools/hermes-review-gate-plugin/scripts/done_review_helper.py`
- `/home/brittonr/git/hermes/tools/hermes-review-gate-plugin/scripts/review_metrics.py`
- `/home/brittonr/git/hermes/openspec/specs/agent-orchestration/spec.md`

## Decisions

### 1. Report parsing is a pure core

**Choice:** Implement report parsing over in-memory markdown text and return a typed parsed report plus diagnostics. Filesystem reads, appends, and storage remain in std/host adapters.

**Rationale:** Review report validation is deterministic and easy to test without standing up the world.

### 2. Findings are section-scoped

**Choice:** Parse metric-bearing findings only from the `## Findings` section. Checklist bullets, task lists, and bracketed text elsewhere remain prose.

**Rationale:** This avoids accidentally converting process checklists into metrics records.

### 3. Metrics writes are explicit mutations

**Choice:** Summary and promotion-candidate views are read-only. Appending a metrics record requires an explicit record action and a valid parsed report.

**Rationale:** Reading evidence should not mutate durable state.

### 4. Tags are a stable vocabulary

**Choice:** Require `class`, `scope`, `route`, and `promoted` tags for metric-bearing findings, with `promoted` limited to `yes` or `no`.

**Rationale:** Repeated-finding analysis needs normalized dimensions, and malformed tags should fail before records are appended.

## Risks / Trade-offs

- A strict grammar can reject useful prose. Keep report bodies flexible and make only the machine-readable pieces strict.
- Metric storage may differ by host. Keep storage interfaces abstract and make append-only JSONL an optional std adapter shape.
- Promotion summaries are heuristics. They should suggest deterministic rails, not silently create tasks or mutate specs.
