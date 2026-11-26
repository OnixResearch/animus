# Why

Cross-vault synthesis across `x-bookmarks-mneme` (`papers/arxiv-2611.29709.adoc` *Cache the Knowledge, Not Just the Prompt: Dynamic Datalog as the Action Language for LLM Agents* — 179/180 cached relation reuse and 23%+ token reduction; `papers/protocolis-solid-state-web.adoc` *Solid-State Web Execution*) and `onix-insights-mneme` (`12-artex-dual-graph-and-brakes.adoc`, `15-deloreis-dsl-functional-predicates-and-defeat.adoc`) highlights two high-impact upgrades for `OnixResearch/animus`:

1. Long-horizon Animus loops re-serialize raw tool output into prompt context instead of maintaining bounded, incrementally reused session relations (`certain` vs `shadow` layers, 3-way `Active`/`Dead`/`Absent` deltas).
2. Tool permission and guardrail layers need explicit untrusted-output framing (`UntrustedToolOutput`) and Protocolis-style post-action observed-diff verification so unframed tool text can never promote into authoritative state.

# What Changes

- Add a pure `no_std` bounded session-scoped Dynamic Datalog action contract in `crates/animus-adapter-rlm-kernel/src/lib.rs` (or sibling `animus-adapter-datalog-repl`) supporting insert/retract/query with `certain` vs `shadow` epistemic layer tags and `Active`/`Dead`/`Absent` delta receipts.
- Extend `crates/animus-acp-stdio-host/src/permission.rs` (`plan_acp_permission`, `apply_permission_decision`) and `crates/animus-std-agent-guardrail` with `UntrustedToolOutput` boundary framing and post-action observed-diff verification.
