## Phase 1: Toolchain and pins

- [x] [serial] Move the `flake.lock` nixpkgs pin from `567a49d1913ce81ac6e9582e3553dd90a955875f` (rustc 1.95.0) to `0bb7ec54c8483066ec9d7720e780a5caa71f8612` (rustc 1.96.1). Confirm that `cargo clippy --workspace --all-targets -- -D warnings` passes and that `cargo test --workspace` matches the 1.95.0 baseline: the 3 failures in `animus-std-agent-guardrail`'s `fake_bwrap` test occur on both toolchains. r[agent_harness_runtime.campaign_manager.boundary]
- [x] [serial] Add `animus-campaign-manager` as a non-default `AGPL-3.0-or-later` workspace member that depends on `campaign-core` and `campaign-choregraph` `0.1.0` at Campaign revision `796fe3844c91c3060da5ffc573a6f801a214aba7`. Add the `campaign` and `choregraph` vendoring groups to `flake.nix`. r[agent_harness_runtime.campaign_manager.boundary]

## Phase 2: Manager core

- [x] [serial] Render the advisory brief as handle-only model text with the objective, the reply schema, the rank domain, grants, counters, labelled candidates, and offers. Refuse invalid setup text, too many offers, and impossible rank dimension counts before any model call. r[agent_harness_runtime.campaign_manager.brief] r[agent_harness_runtime.campaign_manager.refusals]
- [x] [serial] Parse one schema-conforming JSON object per reply and refuse anything else with `MalformedReply`. r[agent_harness_runtime.campaign_manager.reply]
- [x] [serial] Map replies onto one `AdvisoryProposal`: guidance for first and raised scores, revisions for lowered scores, a candidate batch for added offers, advisor-scoped reasons, and the whole reply as source facts. r[agent_harness_runtime.campaign_manager.proposal]
- [x] [serial] Run one turn: refuse a stopped campaign, brief, render, call the model once, build the proposal, and submit it to Campaign's review. r[agent_harness_runtime.campaign_manager.turn]
- [x] [serial] Publish admitted events in order through `ControlSession` with host durability, reporting the committed prefix on failure. r[agent_harness_runtime.campaign_manager.publication]

## Phase 3: Evidence

- [x] [parallel] Add positive integration tests over real Choregraph history with a scripted model: scored advice published and steering the planner; a full-manager reply adding, retiring, and stopping; a prune; and rescoring that raises, lowers, and repeats. r[agent_harness_runtime.campaign_manager.verification] r[agent_harness_runtime.campaign_manager.turn] r[agent_harness_runtime.campaign_manager.proposal]
- [x] [parallel] Add negative integration tests: malformed replies; out-of-brief handles and ranks; invalid reasons; ungranted advice; model failure; invalid setups and menus that never reach the model; a stopped campaign; and a store failure that keeps the committed prefix. r[agent_harness_runtime.campaign_manager.refusals] r[agent_harness_runtime.campaign_manager.verification]
- [x] [serial] Add and pass the focused Nix checks `cargo-campaign-manager` (tests and Clippy with `-D warnings`) and `campaign-manager-octet` (Octet deny-all over the library and tests). r[agent_harness_runtime.campaign_manager.verification]

## Phase 4: Documentation and closeout

- [x] [parallel] Document the role, authority boundary, host sequence, reply schema, refusals, pins, toolchain, and non-claims in `docs/campaign-manager.md` and `README.md`. r[agent_harness_runtime.campaign_manager.boundary] r[agent_harness_runtime.campaign_manager.brief]
- [x] [serial] Register the change in `.cairn/roadmap.ncl` as an optional reference and pass the roadmap and Cairn validation rails. r[agent_harness_runtime.campaign_manager.verification]
- [ ] [serial] After review, sync the requirements into `.cairn/specs/agent-harness-runtime/spec.md` and archive the change. r[agent_harness_runtime.campaign_manager.verification]
