# Turnstone headless eval scorer

The headless eval scorer is a deterministic measurement reducer over already-collected safe action ledgers. It models suites, cases, setup hashes, expected actions, bounded argument matchers, observed actions, score policies, aggregates, holdout markers, and safe receipts.

Supported score modes are exact, ordered subset, subset, and contains-any with an explicit empty-expectation policy. Aggregates separate holdout summaries from optimizer feedback labels.

Scores measure sampled behavior and do not prove semantic correctness. Non-claims: online prompt self-modification, live provider execution, tool execution, temporary workspaces, product CLI behavior, optimizer loops, timeouts, concurrency, persistence, and raw transcript storage.
