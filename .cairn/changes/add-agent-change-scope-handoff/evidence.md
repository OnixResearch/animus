# Evidence: add-agent-change-scope-handoff

## Phase 1 baselines (before core changes)

Executed 2026-09-04 from a clean head (`ce96874`) plus pre-existing unrelated worktree changes that are preserved untouched:

- `scripts/cairn/check-tool-execution-stages --root .` — PASS (in `nix develop`).
- `scripts/cairn/check-runtime-authority-seal --root .` — PASS (in `nix develop`).
- `scripts/cairn/check-safe-receipts-artifacts --root .` — PASS (in `nix develop`).

These rails stay authoritative for tool stages (`animus_runtime::tool_execution_stages`), runtime authority seals, and safe receipts. This change must not weaken or broaden them.

## Accepted non-claim boundaries (kept unchanged)

The accepted agent-receipt non-claims that this change must preserve:

- `animus_runtime::tool_execution_stages` public evidence: `TOOL_EXECUTION_STAGE_NON_CLAIMS` — not tool correctness, not effect rollback, not handler honesty, not sandboxing, not task success, not provider correctness, not production readiness, not release eligibility.
- Accepted spec `agent-harness-runtime`: passing evidence MUST NOT claim live providers, TUI/daemon dogfood, VM checks, remote services, plugin runtime, policy enforcement, or production readiness unless those checks explicitly ran (`r[agent_harness_runtime.clankers_evidence_index.non_claims]`, `r[agent_harness_runtime.current_head_release_evidence_index.docs]`).
- This change adds no new authority: handoff receipts inherit those non-claims and add their own (`not-change-correctness`, `not-vcs-completion`, `not-policy-enforcement`, `not-tool-correctness`, `not-production-readiness`, `not-release-eligibility`).

## Pre-implementation gates

- `nix run .#cairn -- gate proposal add-agent-change-scope-handoff --root .` — PASS.
- `nix run .#cairn -- gate design add-agent-change-scope-handoff --root .` — PASS.
- `nix run .#cairn -- gate tasks add-agent-change-scope-handoff --root .` — PASS.
- `nix run .#cairn -- validate --root .` — valid, `change next` ranks this change first with no blockers.
