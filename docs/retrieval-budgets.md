# Retrieval budgets

Animus treats retrieval budgets as typed policy metadata for retrieval-capable skills, tools, and prompt resources. A budget describes the retrieval purpose, first action, repeat conditions, stop conditions, missing-evidence behavior, source/citation expectations, progress guidance, provenance, and optional host retrieval limits.

The default pattern is: make one targeted retrieval first, repeat only when a required fact, artifact, owner, date, parameter, source, contradiction, or evidence gap remains, and stop when the evidence is sufficient for the requested output. If the budget is exhausted, the runtime should ask for the smallest missing input instead of browsing for polish.

Retrieval-capable resources include web search, browser automation, documentation lookup, repository browsing, multi-lens analysis, and property discovery. Host policy can require such resources to carry a validated budget before they are advertised in prompt context.

Reports and receipts may record budget decisions, source labels, safe references, counts, truncation status, and BLAKE3 content hashes. They must not embed raw fetched bodies, credentials, full host paths, or unbounded retrieved content by default.

The optional [codebase graph host](codebase-graph-host.md) consumes the same finite retrieval posture for direct syntactic queries. Profile and request result ceilings apply before graph dispatch; stale indexes block answers rather than spending additional retrieval budget implicitly.

Non-claims: this contract does not implement web search, browser automation, a documentation provider, or live retrieval. It defines deterministic DTOs and policy decisions that hosts and future adapters can consume.
