# Tasks

- [ ] [serial] Document the Animus focused-rail convention: one boundary per rail, deterministic fixtures, safe evidence schema, target receipt path, and closeout use. r[agent_harness_runtime.clankers_focused_rails.convention]
- [ ] [depends:clankers_focused_rails.convention] Add a lifecycle-doc marker rail for boundary names, owner markers, forbidden dependency sections, and validation command references. r[agent_harness_runtime.clankers_focused_rails.lifecycle]
- [ ] [depends:clankers_focused_rails.convention] Add Steel policy drift and activation rails with valid and invalid fixtures. r[agent_harness_runtime.clankers_focused_rails.steel_policy]
- [ ] [depends:clankers_focused_rails.convention] Add safe receipt/artifact rails with leak fixtures, BLAKE3 determinism, and redaction-class checks. r[agent_harness_runtime.clankers_focused_rails.receipts_artifacts]
- [ ] [depends:clankers_focused_rails.convention] Add neutral ledger replay and unsupported-entry rails. r[agent_harness_runtime.clankers_focused_rails.ledger]
- [ ] [depends:clankers_focused_rails.convention] Add injected-service boundary rails for provider/tool ambient behavior and fail-closed outcomes. r[agent_harness_runtime.clankers_focused_rails.injected_services]
- [ ] [depends:clankers_focused_rails.lifecycle,clankers_focused_rails.steel_policy,clankers_focused_rails.receipts_artifacts,clankers_focused_rails.ledger,clankers_focused_rails.injected_services] Wire stable credential-free rails into Nix checks or document why they remain focused-only. r[agent_harness_runtime.clankers_focused_rails.nix_wiring]
- [ ] [depends:clankers_focused_rails.nix_wiring] Add closeout evidence indexing guidance that separates covered rails from non-claims. r[agent_harness_runtime.clankers_focused_rails.closeout]
