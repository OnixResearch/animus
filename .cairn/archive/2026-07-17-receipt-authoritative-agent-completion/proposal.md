## Why

Animus's bounded Ralph profile currently finalizes a response whenever the previous model message contains the substring `COMPLETE`. The model can therefore terminate the profile without any independent proof that repository-required checks ran or passed. The existing scoped validation planner also reduces `Skipped` results to no follow-up, and continuation policy selects `Complete` whenever no typed work remains rather than when current verification evidence authorizes completion.

The workspace stack supervisor is safer—it ignores model prose and process exit success—but it still recognizes progress from task-count reduction and archive appearance. Those are useful observations, not authoritative proof that every required check passed for the final state.

Animus needs a host-owned completion loop that turns model completion text into a candidate, selects checks from a repository-owned registry, executes them through explicit host services, feeds bounded diagnostics back through typed continuation work, and finalizes only after a fresh passing Cairn completion-validation receipt covers the final subject.

## What Changes

- Add a typed Nickel-authored repository check registry and deterministic runtime export for agent validation profiles, exact argv, affected surfaces, prerequisites, capabilities, budgets, artifacts, reproduction, and positive/negative contracts. r[agent_harness_runtime.authoritative_completion.registry]
- Extend pure change-aware planning so repository-owned rules select the complete required-check set for a host-supplied final change snapshot. r[agent_harness_runtime.authoritative_completion.selection]
- Define host-executed check result and safe receipt contracts with pass-only status semantics and final-subject freshness. r[agent_harness_runtime.authoritative_completion.execution_receipts]
- Replace `COMPLETE` substring authority with a typed completion candidate that cannot bypass Rust admission or verification. r[agent_harness_runtime.authoritative_completion.candidate]
- Require a current passing Cairn completion-validation receipt before embedded or workspace-supervised agent work is represented as complete. r[agent_harness_runtime.authoritative_completion.receipt_gate]
- Feed failed and unavailable verification back through bounded actionable diagnostics and existing continuation work IDs under explicit host policy and budgets. r[agent_harness_runtime.authoritative_completion.feedback]
- Keep check discovery/execution, VCS inspection, timers, output capture, persistence, and Cairn CLI/MCP calls in host shells; keep selection and completion reduction pure. r[agent_harness_runtime.authoritative_completion.boundary]
- Strengthen the workspace-root stack drain consumer contract so task-count reduction remains progress only and archive completion requires fresh receipt-authoritative validation. r[agent_harness_runtime.authoritative_completion.workspace_supervisor]
- Add positive and negative registry, planner, Steel, continuation, receipt, freshness, diagnostic, and supervisor fixtures. r[agent_harness_runtime.authoritative_completion.verification]

## Impact

- **Animus files**: Nickel config/export, validation planner DTOs and pure reducers, continuation state, Steel runtime action/response contracts, host service traits and std-shell orchestration, focused rails, fixtures, and docs.
- **Workspace consumer**: a reviewed handoff for the canonical `../ralph-drain-stack.scm` operator surface after the generic Cairn completion contract is available; no sibling product authority moves into Animus.
- **Compatibility**: `COMPLETE` may remain a compatibility spelling for an exact completion request, but it no longer produces a final response by itself. Existing validation profiles must migrate to explicit pass-only result statuses.
- **Non-claims**: passing completion proves bounded registry selection and supplied receipt conformance for the final subject. It does not prove check-oracle soundness, model correctness, whole-product correctness, production readiness, or unexecuted host-specific tiers.
