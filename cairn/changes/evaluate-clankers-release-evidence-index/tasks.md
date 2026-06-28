# Tasks

- [ ] [serial] Add an architecture note evaluating Clankers release evidence indexing and defining Animus non-claims. r[agent_harness_runtime.clankers_evidence_index.scope]
- [ ] [depends:clankers_evidence_index.scope] Define a future evidence index schema for focused rails, workspace tests, clippy, flake checks, Nickel drift checks, Cairn validation, and Cairn gates. r[agent_harness_runtime.clankers_evidence_index.schema]
- [ ] [depends:clankers_evidence_index.schema] Define current-head metadata fields for commit, branch, dirty status, receipt/head match, stale receipts, and missing modes, with VCS collection kept in the shell. r[agent_harness_runtime.clankers_evidence_index.current_head]
- [ ] [depends:clankers_evidence_index.schema] Define pure-vs-optional check separation and non-claims for live providers, TUI/daemon dogfood, VM checks, remote services, plugin runtime, and production readiness. r[agent_harness_runtime.clankers_evidence_index.non_claims]
- [ ] [depends:clankers_evidence_index.current_head,clankers_evidence_index.non_claims] Add fixture-based validation for matching current-head receipts, stale receipts, dirty state, missing modes, and overclaim rejection. r[agent_harness_runtime.clankers_evidence_index.fixtures]
- [ ] [depends:clankers_evidence_index.fixtures] Run docs/fixture validation, workspace tests if touched, and Cairn validation/gates before archive. r[agent_harness_runtime.clankers_evidence_index.closeout]
