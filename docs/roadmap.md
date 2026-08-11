# Animus Cairn roadmap

This document explains the planned Cairn change order. The source of truth is `cairn/roadmap.ncl`; `scripts/cairn/check-roadmap` validates that every active change is classified exactly once, prerequisite references are known, dependency cycles are absent, and product surfaces do not bypass foundational safety contracts. The checker also emits safe DAG explanations: deterministic active-change topological batches, active prerequisite blockers, cycle paths, product-safety marker paths, and a bounded review-projection marker.

## Roles

| Role | Meaning |
| --- | --- |
| `foundational` | Lifecycle, local policy, roadmap, or verification infrastructure needed before behavior changes. |
| `core-runtime` | Reusable no-ambient runtime contracts, DTOs, policies, ledgers, receipts, events, and pure validation. |
| `host-adapter` | Concrete std-shell or host-provided adapters that own effects behind injected contracts. |
| `product-surface` | User- or client-facing modes such as TUI, headless, SDK, workflow, subagent, review, MCP, or export surfaces. |
| `optional-reference` | Bounded non-default reference adaptation or dependency-isolation guard. |
| `evaluation-only` | Study/adoption-matrix work that must not claim runtime behavior. |

## Tier order

| Tier | Lands after | Owns |
| --- | --- | --- |
| `policy-wiring` | — | Repo-local Cairn policy and default validation. |
| `baseline-lifecycle` | `policy-wiring` | Accepted specs, lifecycle dogfood, and focused rails. |
| `roadmap-sequencing` | `baseline-lifecycle` | Roadmap taxonomy, prerequisites, overlap owners, and checks. |
| `runtime-loop` | `roadmap-sequencing` | Runtime loop, lifecycle ownership, and public facade inventory. |
| `injected-services` | `runtime-loop` | Model/tool/provider/effect service seams. |
| `receipt-provenance` | `injected-services` | Safe receipts, artifacts, provenance, and boundary schemas. |
| `session-ledger` | `receipt-provenance` | Neutral replay, branches, compaction, and summaries. |
| `event-hooks` | `session-ledger` | Events, hook chains, middleware, and trust lifecycle. |
| `policy-guidance` | `event-hooks` | Capabilities, permissions, guidance, skills, memories, and retrieval budgets. |
| `concrete-adapters` | `policy-guidance` | Std shell providers, tools, queues, sandbox, workspace, and dataspace adapters. |
| `ui-headless` | `concrete-adapters` | TUI, headless RPC, SDK, review, subagent, workflow, and MCP surfaces. |
| `export-surfaces` | `ui-headless` | Dataset, evidence, and lifecycle export surfaces. |
| `optional-references` | `roadmap-sequencing` | Non-default reference adapters and dependency isolation. |
| `evaluation` | `roadmap-sequencing` | Evaluation-only adoption matrices. |

## Current classification

| Role | Changes |
| --- | --- |
| `foundational` | `define-dogfoodable-host-readiness-roadmap` |
| `core-runtime` | `adapt-clankers-runtime-patterns`, `adapt-clankers-injected-effect-services`, `adapt-clankers-confirmation-broker-kit`, `adapt-clankers-steel-runtime-policy`, `adapt-clankers-safe-receipts-artifacts`, `adapt-clankers-observability-audit-receipts`, `adapt-clankers-neutral-session-ledger`, `adapt-clankers-provenance-ledger`, `adapt-clankers-controller-continuation-policy`, `adapt-clankers-context-references`, `adapt-clankers-prompt-assembly-kit`, `adapt-clankers-cost-model-routing`, `adapt-clankers-agent-definition-trust`, `add-per-model-runtime-profiles`, `add-capability-bound-agent-roles`, `receipt-authoritative-agent-completion`, `adapt-clankers-remote-admission-receipts`, `adapt-session-restore-contract`, `adapt-plugin-action-manifests`, `ensure-model-token-thought-streaming`, `adapt-pi-tool-contracts`, `adapt-pi-runtime-event-lifecycle`, `adapt-pi-session-tree-ledger`, `adapt-pi-compaction-branch-summaries`, `adapt-zerostack-midturn-compaction`, `adapt-git-ai-vcs-neutral-provenance`, `harden-preserves-boundary-schemas`, `align-clash-patterns-with-ucan`, `adapt-openai-codex-execpolicy-rules`, `adapt-openai-codex-hook-trust-lifecycle`, `adapt-claude-code-lifecycle-hooks`, `adapt-claude-code-permission-modes`, `adapt-zerostack-permission-kernel`, `adapt-hermes-policy-profiles`, `adapt-hermes-retrieval-budgets`, `adapt-pi-skills-resource-discovery`, `adapt-hermes-skill-manifests`, `adapt-openai-codex-guidance-skill-budget`, `adapt-zerostack-prompt-mode-profiles`, `adapt-zerostack-context-file-layering`, `adapt-claude-code-layered-memory`, `adapt-openai-codex-opt-in-memories`, `adapt-hermes-plugin-tool-manifests`, `adapt-hermes-review-report-contracts`, `adapt-qualifier-quality-records`, `adapt-zerostack-hashed-edit-protocol`, `adapt-ornith-scaffold-profiles`, `adapt-ornith-trust-boundary-monitor` |
| `host-adapter` | `adapt-pi-provider-registry-shell`, `adapt-zerostack-provider-registry-hardening`, `add-concrete-std-runtime-adapters`, `adapt-hermes-background-work-queues`, `adapt-clankers-process-job-profiles`, `adapt-clankers-tool-gateway-delivery`, `adapt-clankers-checkpoint-rollback-ux`, `adapt-sandlock-sandbox-enforcement`, `adapt-branchfs-speculative-workspaces`, `add-sandlock-linux-host-adapter`, `add-branchfs-speculative-host-adapter`, `add-multimodal-image-payloads`, `document-ingest-adapter-boundary`, `ml-evidence-adapter-boundary` |
| `product-surface` | `adapt-pi-headless-rpc-protocol`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-headless-surfaces`, `adapt-claude-code-mcp-bridge`, `adapt-host-control-protocol`, `add-ratatui-tool-host-surface`, `adapt-openai-codex-diff-review-mode`, `adapt-openai-codex-subagent-threads`, `adapt-zerostack-fresh-context-tasks`, `adapt-claude-code-agent-teams`, `adapt-claude-code-workflow-commands`, `adapt-hermes-kanban-orchestration`, `adapt-hermes-researchspec-lifecycle`, `adapt-openai-codex-worktree-automations`, `adapt-clankers-batch-trajectory-runner`, `add-animus-hf-dataset-export` |
| `optional-reference` | `adapt-trellis-verified-core-boundaries`, `adapt-syndicate-dataspace-adapters`, `extract-host-control-protocol` |
| `evaluation-only` | `evaluate-clankers-provider-contract-parity`, `evaluate-clankers-tool-catalog-packs`, `adapt-ornith-eval-feedback-loop` |

## Overlap owners

| Concept | Owner | Related changes |
| --- | --- | --- |
| Ledger | `adapt-pi-session-tree-ledger` | `adapt-clankers-neutral-session-ledger`, `adapt-clankers-provenance-ledger`, `adapt-pi-compaction-branch-summaries`, `adapt-zerostack-midturn-compaction`, `adapt-clankers-context-references`, `add-animus-hf-dataset-export` |
| Receipt redaction | `adapt-clankers-safe-receipts-artifacts` | `adapt-clankers-provenance-ledger`, `adapt-clankers-observability-audit-receipts`, `adapt-clankers-remote-admission-receipts`, `adapt-git-ai-vcs-neutral-provenance`, `adapt-hermes-review-report-contracts`, `adapt-clankers-checkpoint-rollback-ux`, `adapt-clankers-tool-gateway-delivery`, `adapt-clankers-batch-trajectory-runner`, `adapt-ornith-trust-boundary-monitor`, `adapt-ornith-eval-feedback-loop`, `add-animus-hf-dataset-export` |
| Audit observability | `adapt-clankers-observability-audit-receipts` | `adapt-pi-runtime-event-lifecycle`, `adapt-clankers-safe-receipts-artifacts` |
| Receipt-authoritative completion | `receipt-authoritative-agent-completion` | `adapt-hermes-scoped-validation-planner`, `add-current-head-release-evidence-index` |
| Dogfood current-head evidence binding | `bind-dogfood-readiness-current-head-evidence` | `add-current-head-release-evidence-index`, `add-packaged-headless-dogfood-smoke` |
| Standalone host-control protocol | `extract-host-control-protocol` | `split-adapter-family-crates` |
| Sandlock Linux host enforcement | `add-sandlock-linux-host-adapter` | `split-std-host-adapter-crates` |
| BranchFS speculative host workspaces | `add-branchfs-speculative-host-adapter` | `split-std-host-adapter-crates` |
| Per-model runtime composition | `add-per-model-runtime-profiles` | `adapt-clankers-prompt-assembly-kit`, `adapt-hermes-skill-manifests`, `adapt-openai-codex-opt-in-memories`, `adapt-hermes-plugin-tool-manifests`, `adapt-clankers-agent-definition-trust`, `adapt-clankers-cost-model-routing`, `add-capability-bound-agent-roles`, `receipt-authoritative-agent-completion` |
| Capability-bound semantic roles | `add-capability-bound-agent-roles` | `adapt-openai-codex-subagent-threads`, `adapt-claude-code-agent-teams`, `add-per-model-runtime-profiles`, `receipt-authoritative-agent-completion` |
| Model streaming | `ensure-model-token-thought-streaming` | `adapt-pi-runtime-event-lifecycle`, `adapt-pi-headless-rpc-protocol`, `adapt-openai-codex-jsonl-exec-sdk`, `add-ratatui-tool-host-surface`, `adapt-pi-provider-registry-shell`, `adapt-clankers-cost-model-routing`, `add-multimodal-image-payloads`, `evaluate-clankers-provider-contract-parity` |
| Provider dispatch | `adapt-clankers-injected-effect-services` | `adapt-pi-provider-registry-shell`, `adapt-zerostack-provider-registry-hardening`, `add-concrete-std-runtime-adapters`, `adapt-clankers-cost-model-routing`, `add-multimodal-image-payloads`, `evaluate-clankers-provider-contract-parity` |
| Hooks | `adapt-pi-runtime-event-lifecycle` | `adapt-openai-codex-hook-trust-lifecycle`, `adapt-claude-code-lifecycle-hooks`, `adapt-openai-codex-jsonl-exec-sdk` |
| Manifest/catalog | `adapt-hermes-plugin-tool-manifests` | `adapt-clankers-tool-gateway-delivery`, `adapt-plugin-action-manifests`, `evaluate-clankers-tool-catalog-packs`, `adapt-claude-code-mcp-bridge` |
| Tool contracts | `adapt-pi-tool-contracts` | `adapt-clankers-injected-effect-services`, `adapt-zerostack-hashed-edit-protocol`, `adapt-hermes-background-work-queues`, `adapt-clankers-tool-gateway-delivery`, `add-ratatui-tool-host-surface`, `add-concrete-std-runtime-adapters` |
| Policy/guidance | `adapt-hermes-policy-profiles` | `adapt-clankers-prompt-assembly-kit`, `adapt-zerostack-permission-kernel`, `adapt-zerostack-prompt-mode-profiles`, `adapt-clankers-cost-model-routing`, `adapt-clankers-agent-definition-trust`, `adapt-clankers-process-job-profiles`, `adapt-hermes-retrieval-budgets`, `adapt-openai-codex-guidance-skill-budget`, `adapt-pi-skills-resource-discovery`, `adapt-claude-code-layered-memory`, `adapt-ornith-scaffold-profiles`, `adapt-ornith-trust-boundary-monitor` |
| Prompt assembly | `adapt-clankers-prompt-assembly-kit` | `adapt-hermes-policy-profiles`, `adapt-openai-codex-guidance-skill-budget`, `adapt-zerostack-prompt-mode-profiles`, `adapt-clankers-context-references`, `adapt-zerostack-context-file-layering`, `add-multimodal-image-payloads`, `adapt-clankers-agent-definition-trust`, `adapt-claude-code-layered-memory`, `adapt-openai-codex-opt-in-memories`, `adapt-ornith-scaffold-profiles` |
| Context references | `adapt-clankers-context-references` | `adapt-hermes-retrieval-budgets`, `adapt-pi-skills-resource-discovery`, `adapt-clankers-prompt-assembly-kit`, `adapt-zerostack-context-file-layering`, `adapt-zerostack-fresh-context-tasks`, `add-multimodal-image-payloads`, `adapt-clankers-batch-trajectory-runner`, `add-animus-hf-dataset-export` |
| Batch/export | `adapt-clankers-batch-trajectory-runner` | `add-animus-hf-dataset-export`, `adapt-hermes-researchspec-lifecycle`, `adapt-ornith-eval-feedback-loop` |
| Scaffold feedback | `adapt-ornith-scaffold-profiles` | `adapt-ornith-eval-feedback-loop`, `adapt-ornith-trust-boundary-monitor`, `adapt-clankers-batch-trajectory-runner`, `adapt-clankers-prompt-assembly-kit` |
| Workspace checkpoints | `adapt-git-ai-vcs-neutral-provenance` | `adapt-clankers-checkpoint-rollback-ux`, `adapt-branchfs-speculative-workspaces`, `adapt-openai-codex-worktree-automations` |
| Artifact delivery | `adapt-clankers-tool-gateway-delivery` | `adapt-pi-tool-contracts`, `adapt-hermes-background-work-queues`, `add-ratatui-tool-host-surface` |
| Multimodal input | `add-multimodal-image-payloads` | `adapt-clankers-context-references`, `adapt-clankers-prompt-assembly-kit`, `adapt-pi-provider-registry-shell`, `evaluate-clankers-provider-contract-parity`, `add-animus-hf-dataset-export` |
| Document ingest adapter boundary | `document-ingest-adapter-boundary` | `adopt-cap-std-context-boundary` |
| ML evidence adapter boundary | `ml-evidence-adapter-boundary` | `define-model-router-execution-service`, `add-concrete-provider-dispatch-adapter` |
| Background work | `adapt-hermes-background-work-queues` | `adapt-clankers-process-job-profiles`, `adapt-clankers-tool-gateway-delivery`, `adapt-hermes-kanban-orchestration` |
| Agent definitions | `adapt-clankers-agent-definition-trust` | `adapt-hermes-skill-manifests`, `adapt-clankers-prompt-assembly-kit`, `adapt-claude-code-agent-teams`, `adapt-openai-codex-subagent-threads`, `adapt-zerostack-fresh-context-tasks` |
| Confirmation | `adapt-clankers-confirmation-broker-kit` | `adapt-clankers-injected-effect-services`, `adapt-clankers-checkpoint-rollback-ux`, `adapt-clankers-tool-gateway-delivery`, `adapt-clankers-remote-admission-receipts` |
| Continuation policy | `adapt-clankers-controller-continuation-policy` | `adapt-pi-runtime-event-lifecycle`, `adapt-pi-session-tree-ledger`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-workflow-commands` |
| Session restore | `adapt-session-restore-contract` | `adapt-pi-session-tree-ledger`, `adapt-pi-headless-rpc-protocol`, `adapt-host-control-protocol` |
| Transactional daemon handoff | `add-transactional-daemon-handoff` | `add-durable-pending-input-vault`, `adapt-session-restore-contract`, `adapt-host-control-protocol`, `extract-host-control-protocol` |
| Revision-gated workspace edit host | `add-revision-gated-workspace-edit-host` | `add-tool-executor-kit`, `add-host-tool-executor-capability-packs`, `adapt-zerostack-hashed-edit-protocol` |
| Remote admission | `adapt-clankers-remote-admission-receipts` | `align-clash-patterns-with-ucan`, `adapt-pi-headless-rpc-protocol`, `adapt-claude-code-mcp-bridge` |
| Host control | `adapt-host-control-protocol` | `adapt-pi-headless-rpc-protocol`, `add-ratatui-tool-host-surface`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-headless-surfaces`, `adapt-session-restore-contract` |
| UI/headless | `adapt-pi-runtime-event-lifecycle` | `adapt-pi-headless-rpc-protocol`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-headless-surfaces`, `add-ratatui-tool-host-surface`, `adapt-host-control-protocol` |

## Dogfoodability readiness rule

Successor Cairns that claim live providers, built-in tools, session persistence, TUI dogfood, daemon/attach, plugin/MCP runtime, release harness behavior, production readiness, or Clankers parity must cite the accepted dogfood readiness prerequisites. They must name the highest completed evidence rung from [dogfoodable-host-readiness-roadmap.md](dogfoodable-host-readiness-roadmap.md). They must also list omitted optional surfaces as non-claims. The bounded `packaged-headless-dogfood` claim requires one clean, passing, head-matched packaged-smoke receipt. Implementation code alone is not sufficient evidence.

## Roadmap DAG explanation evidence

`scripts/cairn/check-roadmap --root .` is the focused rail for roadmap sequencing. Its default output is safe for closeout summaries and includes only counts, change IDs, tier IDs, role labels, prerequisite IDs, issue codes, redaction markers, and product-safety marker IDs. Use `--review-markdown` on the compiled checker only when a reviewer needs a Markdown projection; keep any generated artifact under ignored local output such as `target/cairn/focused-rails/`.

Closeout evidence may cite:

- `DAG_BATCH <n> <change-ids>` lines for deterministic active-change order;
- `BLOCKED <change> waits_for <prerequisite> status active-prerequisite` lines for active blockers;
- `CYCLE_PATH <graph> <node> waits_for <node>` lines for cycle diagnostics;
- `SAFETY_PATH <change> foundational <marker> runtime <marker>` lines for product-surface safety paths;
- the `ROADMAP_REVIEW ... redaction=applied` marker to show the projection stayed bounded.

Do not copy proposal/design/task bodies, raw command transcripts, raw diffs, private full paths, credentials, provider payloads, prompts, environment values, or unbounded logs into roadmap closeout evidence. If a review projection is unavailable, record the blocker and cite the next-best `scripts/cairn/check-roadmap --root .` result.

## Closeout rule

A dependent change is archive-ready only when its prerequisite requirement IDs are accepted, its prerequisite changes are archived in the same batch, or a waiver names the replacement requirement ID. Product-surface changes must retain a transitive path through foundational lifecycle/policy evidence, dogfoodability readiness gates when they make readiness claims, and reusable runtime safety contracts before they can claim stable behavior.
