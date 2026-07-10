# agent-harness-runtime Change Spec: Turnstone headless eval scorer

## Requirements

### Requirement: Headless eval DTOs describe suites and observations safely

r[agent_harness_runtime.turnstone_headless_eval.dto] Animus MUST represent headless eval suites, cases, setup summaries, expected actions, argument matchers, observed actions, run scores, aggregate scores, holdout markers, and receipts with typed DTOs.

#### Scenario: Suite DTO omits raw fixture bodies

r[agent_harness_runtime.turnstone_headless_eval.dto.safe_suite]
- GIVEN a host supplies a headless eval suite with case labels and setup hashes
- WHEN suite DTO validation runs
- THEN Animus accepts safe case identities, expected action specs, holdout markers, and setup summaries
- AND it excludes raw prompts, raw fixture bodies, provider payloads, full paths, credentials, and raw tool output.

### Requirement: Action matching is pure and bounded

r[agent_harness_runtime.turnstone_headless_eval.matching] Animus MUST match observed actions to expected actions by tool name, exact arguments, bounded regex-like argument patterns, malformed raw-argument markers, and duplicate-match prevention using pure deterministic logic.

#### Scenario: Expected action matches by bounded pattern

r[agent_harness_runtime.turnstone_headless_eval.matching.pattern]
- GIVEN an expected action includes a bounded pattern for a command label
- AND an observed action has a matching safe argument summary
- WHEN action matching runs
- THEN Animus marks the observed action as matched once
- AND duplicate use of the same observed action for multiple expectations is rejected or reported according to policy.

### Requirement: Scoring supports explicit match modes

r[agent_harness_runtime.turnstone_headless_eval.scoring] Animus MUST score expected and observed action ledgers with exact, ordered-subset, subset, and contains-any modes and an explicit empty-expectation policy.

#### Scenario: Ordered subset allows extra tools between matches

r[agent_harness_runtime.turnstone_headless_eval.scoring.ordered_subset]
- GIVEN expected actions appear in order within a longer observed tool ledger
- WHEN ordered-subset scoring runs
- THEN Animus reports all expected actions matched with a passing score
- AND extra tools are reported as safe metadata rather than ignored silently.

### Requirement: Aggregates separate holdout and feedback data

r[agent_harness_runtime.turnstone_headless_eval.aggregates] Animus MUST reduce multi-run eval observations into pass rate, average score, flaky case labels, unmatched-action summaries, extra-tool counts, malformed-tool-call signals, holdout summaries, and non-holdout feedback summaries without leaking holdout-only details into feedback outputs.

#### Scenario: Holdout case is excluded from optimizer feedback

r[agent_harness_runtime.turnstone_headless_eval.aggregates.holdout]
- GIVEN an eval suite contains holdout and non-holdout cases
- WHEN aggregate reduction runs
- THEN holdout scores are reported in holdout summaries
- AND optimizer feedback summaries exclude holdout-only failure details.

### Requirement: Host shell owns execution effects

r[agent_harness_runtime.turnstone_headless_eval.host_boundary] Headless eval execution MUST keep fixture setup, provider calls, tool execution, temporary workspaces, timeouts, concurrency, clocks, filesystem access, and persistence in host shells or injected services.

#### Scenario: Reusable scorer receives observed ledger only

r[agent_harness_runtime.turnstone_headless_eval.host_boundary.no_effects]
- GIVEN a host has already executed a headless eval run and collected a safe observed ledger
- WHEN reusable scoring runs
- THEN Animus computes deterministic scores from the in-memory ledger
- AND it does not create temp directories, read files, call providers, run tools, spawn processes, or write persistence.

### Requirement: Eval receipts are safe summaries

r[agent_harness_runtime.turnstone_headless_eval.receipts] Headless eval receipts MUST contain suite identity, case labels, run counts, score summaries, policy hashes, fixture/setup hashes, BLAKE3 evidence hashes, redaction markers, and safe diagnostics only.

#### Scenario: Receipt rejects raw transcript

r[agent_harness_runtime.turnstone_headless_eval.receipts.no_transcript]
- GIVEN a receipt draft includes a raw model transcript, raw prompt, or raw tool output
- WHEN receipt validation runs
- THEN Animus rejects the receipt with a leak diagnostic
- AND durable evidence keeps only safe labels, counts, hashes, and score summaries.

### Requirement: Headless eval docs distinguish measurement from proof

r[agent_harness_runtime.turnstone_headless_eval.docs] Documentation for the headless eval scorer MUST state that tool-sequence scoring is measurement, not proof, and that online prompt self-modification, live provider runner, product CLI, and raw transcript storage are non-claims.

#### Scenario: Documentation names scorer limits

r[agent_harness_runtime.turnstone_headless_eval.docs.measurement]
- GIVEN users read the headless eval scorer documentation
- WHEN they inspect the limitations section
- THEN it states that scores measure sampled behavior and do not prove semantic correctness
- AND it identifies optimizer loops, live execution, product CLI behavior, and raw transcript storage as outside reusable scope.

### Requirement: Headless eval verification covers scoring and safety failures

r[agent_harness_runtime.turnstone_headless_eval.verification] The headless eval scorer change MUST include positive tests for all match modes and aggregate reducers plus negative malformed specs, invalid patterns, duplicate matches, unsafe receipts, holdout leakage, empty-expectation policy, and impossible score inputs before archive.

#### Scenario: Closeout evidence names scorer fixtures

r[agent_harness_runtime.turnstone_headless_eval.verification.closeout]
- GIVEN maintainers prepare to archive headless eval scorer work
- WHEN closeout evidence is reviewed
- THEN it names DTOs, matching, scoring, aggregation, host-boundary docs, receipts, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records Turnstone optimizer behavior and live provider execution as non-claims.
