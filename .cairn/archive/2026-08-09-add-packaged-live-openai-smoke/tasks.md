## Phase 1: Packaged smoke contract

- [x] [serial] Add pure smoke plan construction and metadata-only evidence projection over the accepted OpenAI host. r[agent_harness_runtime.packaged_live_openai_smoke.contract]
- [x] [depends:contract] Add the non-default binary with explicit model input and bounded credential standard input. r[agent_harness_runtime.packaged_live_openai_smoke.binary]
- [x] [depends:binary] Add positive evidence tests and negative missing, duplicate, unknown, oversized, malformed, and leak tests. r[agent_harness_runtime.packaged_live_openai_smoke.verification]

## Phase 2: Current-head evidence and closeout

- [x] [depends:verification] Reject live-provider claims from stale optional receipts and retain absent live runs as non-claims. r[agent_harness_runtime.packaged_live_openai_smoke.current_head_evidence]
- [x] [depends:current_head_evidence] Add docs, focused Rust and Nix rails, and the explicit credential blocker. r[agent_harness_runtime.packaged_live_openai_smoke.evidence]
- [x] [depends:evidence] Run focused checks and Cairn gates, sync the accepted spec, archive the change, and record bounded evidence. r[agent_harness_runtime.packaged_live_openai_smoke.closeout]
