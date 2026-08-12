# Tasks

- [x] [serial] Define headless eval suite, case, setup summary, expected action, argument matcher, observed action, run score, aggregate score, holdout marker, and safe receipt DTOs. r[agent_harness_runtime.turnstone_headless_eval.dto]
- [x] [depends:dto] Add pure action matching for tool name, exact arguments, bounded regex-like argument patterns, malformed raw arguments, and duplicate-match prevention. r[agent_harness_runtime.turnstone_headless_eval.matching]
- [x] [depends:matching] Add exact, ordered-subset, subset, and contains-any scoring with explicit empty-expectation policy and safe diagnostics. r[agent_harness_runtime.turnstone_headless_eval.scoring]
- [x] [depends:scoring] Add aggregate reducers for pass rate, average score, flaky cases, unmatched actions, extra tools, malformed-tool-call signals, holdout summaries, and non-holdout feedback summaries. r[agent_harness_runtime.turnstone_headless_eval.aggregates]
- [x] [depends:aggregates] Define host-shell boundaries for fixture setup, provider/tool execution, temporary workspaces, timeouts, concurrency, and persistence. r[agent_harness_runtime.turnstone_headless_eval.host_boundary]
- [x] [depends:host_boundary] Add safe eval receipts with suite identity, case labels, run counts, score summaries, policy hashes, fixture/setup hashes, BLAKE3 evidence hashes, and redaction markers only. r[agent_harness_runtime.turnstone_headless_eval.receipts]
- [x] [depends:receipts] Document measurement-vs-proof limits and reject online prompt self-modification, live provider runner, product CLI, and raw transcript storage as non-claims. r[agent_harness_runtime.turnstone_headless_eval.docs]
- [x] [depends:docs] Add positive tests for all match modes and aggregates plus negative malformed specs, invalid patterns, duplicate matches, unsafe receipts, holdout leakage, empty-expectation policy, and impossible score inputs. r[agent_harness_runtime.turnstone_headless_eval.verification]
