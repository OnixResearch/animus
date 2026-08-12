# Tasks

## Phase 1: Packaged smoke

- [x] [serial] Add the non-default `animus-tui-dogfood-smoke` feature and thin binary shell. r[agent_harness_runtime.packaged_tui_dogfood_smoke.contract]
- [x] [depends:agent_harness_runtime.packaged_tui_dogfood_smoke.contract] Add pure bounded headless-receipt projection and fixed TUI view planning. r[agent_harness_runtime.packaged_tui_dogfood_smoke.projection]
- [x] [depends:agent_harness_runtime.packaged_tui_dogfood_smoke.projection] Add Ratatui `TestBackend` rendering plus terminal and cleanup gates. r[agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate]
- [x] [depends:agent_harness_runtime.packaged_tui_dogfood_smoke.host_gate] Add metadata-only BLAKE3 evidence projection and explicit non-claims. r[agent_harness_runtime.packaged_tui_dogfood_smoke.receipt]

## Phase 2: Verification and documentation

- [x] [depends:agent_harness_runtime.packaged_tui_dogfood_smoke.receipt] Add positive tests and negative terminal-loss, capability, receipt-limit, cleanup, and leak tests. r[agent_harness_runtime.packaged_tui_dogfood_smoke.verification]
- [x] [depends:agent_harness_runtime.packaged_tui_dogfood_smoke.verification] Add the focused script, Nix check, claim fixture, roadmap entry, and operator docs. r[agent_harness_runtime.packaged_tui_dogfood_smoke.docs]
- [x] [serial] Run focused Cargo, Clippy, readiness, Nix, Cairn validation, and all Cairn gates before sync and archive. r[agent_harness_runtime.packaged_tui_dogfood_smoke.verification]
