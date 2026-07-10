## Tasks

- [x] [serial] r[animus.ast_grep_agent_audits.profile] Define ast-grep agent-boundary audit profiles and scan scopes.
- [x] [depends:ast-grep-agent-profile] r[animus.ast_grep_agent_audits.inventory] Add inventory-only scans for ambient authority, unsafe blocks, panics, direct provider calls, and host effects in reusable crates.
- [x] [depends:ast-grep-agent-profile] r[animus.ast_grep_agent_audits.identity] Bind ast-grep version, rule bundle BLAKE3 identity, scope, findings, and non-claims into Animus receipts.
- [x] [depends:ast-grep-agent-inventory] r[animus.ast_grep_agent_audits.fixtures] Add positive and negative rule fixtures before any audit rule becomes warning or blocking.
- [x] [depends:ast-grep-agent-fixtures] r[animus.ast_grep_agent_audits.conformance] Report findings through safe receipt or conformance-index surfaces without proof overclaims.
- [x] [depends:ast-grep-agent-conformance] r[animus.ast_grep_agent_audits.validation] Run ast-grep rule tests, boundary fixture scans, Cairn validation/gates, and focused Animus conformance rails.
