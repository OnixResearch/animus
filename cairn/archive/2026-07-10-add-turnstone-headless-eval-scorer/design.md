# Design: Turnstone-inspired headless eval scorer

## Context

Animus has batch trajectory contracts and eval feedback planning, but not a focused pure scorer for expected tool-call sequences. Turnstone's evaluation substrate is valuable because it separates measurement from optimization: a run produces an observed tool ledger, and scoring compares that ledger with expected actions under an explicit match mode.

## Decisions

### 1. Scoring is pure over observed ledgers

**Choice:** The scorer consumes typed observed action records and expected action specs. It returns matched indices, unmatched indices, extra tools, score numerator/denominator, pass status, and safe details.

**Rationale:** Hosts own provider execution, temporary directories, timeouts, and tool effects. Reusable code owns deterministic scoring.

### 2. Match modes are explicit policy

**Choice:** Suites select exact, ordered-subset, subset, or contains-any matching. Empty expected-action behavior is policy-selected rather than implicit.

**Rationale:** Different evals measure different properties, and implicit success on empty expectations can hide broken tests.

### 3. Aggregation separates training feedback from holdouts

**Choice:** Reducers compute pass rate, average score, flaky case labels, unmatched summaries, extra-tool counts, malformed-tool-call signals, and holdout/non-holdout summaries separately.

**Rationale:** Optimization feedback must not leak holdout-only evidence into proposed prompt/tool changes.

### 4. Receipts are safe summaries

**Choice:** Eval receipts record suite/case labels, run counts, score summaries, policy hash, fixture/setup hashes, and redaction markers. Raw prompts, provider payloads, raw tool output, full paths, and model transcripts are excluded.

**Rationale:** Eval artifacts often contain sensitive task data and should remain host-owned.

## Risks / Trade-offs

- Sequence scoring can overfit to tool choreography. Docs should frame it as measurement, not proof of task correctness.
- Regex-like argument matching is useful but can become unsafe. Compile/validate patterns in a pure core and record only safe diagnostics.
- Multi-run evals can be expensive. The reusable scorer should not own execution concurrency or scheduling.