# Tasks

## B. Configuration and composition

- [ ] [serial] B1: Record exact prerequisite component revisions, licenses, feature gates, and unresolved compatibility facts. r[agent_harness_runtime.live_steel_agent_shell.verification]
- [ ] [serial] B2: Define typed Nickel application configuration and deterministic Rust revalidation for every selected component and bound. r[agent_harness_runtime.live_steel_agent_shell.configuration]
- [ ] [depends:B2] B3: Add the non-default package and visible composition root without widening reusable or default graphs. r[agent_harness_runtime.live_steel_agent_shell.composition]

## L. Live-shaped loop

- [ ] [depends:B3] L1: Construct the durable store, coding session, provider, Steel bridge, RLM executor, child service, skills, control, events, and receipts from explicit inputs. r[agent_harness_runtime.live_steel_agent_shell.composition]
- [ ] [depends:L1] L2: Drive provider tool calls, committed `steel_repl` dispatch, host results, and provider continuation through the coding-session facade. r[agent_harness_runtime.live_steel_agent_shell.tool_loop]
- [ ] [depends:L2] L3: Preserve message, tool, queue compare-and-swap, kernel, event, cancellation, and terminal ordering. r[agent_harness_runtime.live_steel_agent_shell.session]
- [ ] [depends:L3] L4: Add bounded startup, shutdown, cancellation, active-turn recovery, interrupted-tool repair, and durable kernel restore. r[agent_harness_runtime.live_steel_agent_shell.lifecycle]
- [ ] [depends:L4] L5: Emit safe events and metadata-only run receipts with exact evidence-rung non-claims. r[agent_harness_runtime.live_steel_agent_shell.session]

## T. Tests and closeout

- [ ] [depends:L5] T1: Add one deterministic production-entrypoint scenario from prompt through provider tool call, Steel host result, provider continuation, final commit, and restart. r[agent_harness_runtime.live_steel_agent_shell.verification]
- [ ] [depends:L5] T2: Add negative scenarios for invalid configuration, missing credentials, denied tools, host failure, stale queue edits, child failure, cancellation, bounds, crash windows, restore drift, unsafe evidence, and ambient fallback. r[agent_harness_runtime.live_steel_agent_shell.verification]
- [ ] [depends:T2] T3: Add a separately opt-in live provider smoke and current-head safe receipt path without making it a default gate. r[agent_harness_runtime.live_steel_agent_shell.verification]
- [ ] [depends:T3] T4: Update readiness docs and component metadata, add focused Rust and Nix rails, run relevant checks, and close through Cairn gates. r[agent_harness_runtime.live_steel_agent_shell.verification]
