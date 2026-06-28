# Animus Cairn roadmap

This document explains the planned Cairn change order. The source of truth is `cairn/roadmap.ncl`; `scripts/cairn/check-roadmap` validates that every active change is classified exactly once, prerequisite references are known, dependency cycles are absent, and product surfaces do not bypass foundational safety contracts.

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
| `foundational` | `adapt-clankers-focused-verification-rails` |
| `core-runtime` | `introduce-steel-runtime-loop`, `adapt-clankers-request-lifecycle-ownership`, `adapt-clankers-runtime-facade-inventory`, `adapt-clankers-runtime-patterns`, `adapt-clankers-injected-effect-services`, `adapt-clankers-steel-runtime-policy`, `adapt-clankers-safe-receipts-artifacts`, `adapt-clankers-neutral-session-ledger`, `adapt-clankers-provenance-ledger`, `adapt-pi-tool-contracts`, `adapt-pi-runtime-event-lifecycle`, `adapt-pi-session-tree-ledger`, `adapt-pi-compaction-branch-summaries`, `adapt-git-ai-vcs-neutral-provenance`, `harden-preserves-boundary-schemas`, `align-clash-patterns-with-ucan`, `adapt-openai-codex-execpolicy-rules`, `adapt-openai-codex-hook-trust-lifecycle`, `adapt-claude-code-lifecycle-hooks`, `adapt-claude-code-permission-modes`, `adapt-hermes-policy-profiles`, `adapt-hermes-retrieval-budgets`, `adapt-pi-skills-resource-discovery`, `adapt-hermes-skill-manifests`, `adapt-openai-codex-guidance-skill-budget`, `adapt-claude-code-layered-memory`, `adapt-openai-codex-opt-in-memories`, `adapt-hermes-plugin-tool-manifests`, `adapt-hermes-review-report-contracts`, `adapt-qualifier-quality-records` |
| `host-adapter` | `adapt-pi-provider-registry-shell`, `add-concrete-std-runtime-adapters`, `adapt-hermes-background-work-queues`, `adapt-sandlock-sandbox-enforcement`, `adapt-branchfs-speculative-workspaces` |
| `product-surface` | `adapt-pi-headless-rpc-protocol`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-headless-surfaces`, `adapt-claude-code-mcp-bridge`, `add-ratatui-tool-host-surface`, `adapt-openai-codex-diff-review-mode`, `adapt-openai-codex-subagent-threads`, `adapt-claude-code-agent-teams`, `adapt-claude-code-workflow-commands`, `adapt-hermes-kanban-orchestration`, `adapt-hermes-researchspec-lifecycle`, `adapt-openai-codex-worktree-automations`, `add-animus-hf-dataset-export` |
| `optional-reference` | `reject-clankers-product-surfaces`, `adapt-trellis-verified-core-boundaries`, `adapt-syndicate-dataspace-adapters` |
| `evaluation-only` | `evaluate-clankers-plugin-manifest-boundary`, `evaluate-clankers-provider-contract-parity`, `evaluate-clankers-release-evidence-index`, `evaluate-clankers-tool-catalog-packs` |

## Overlap owners

| Concept | Owner | Related changes |
| --- | --- | --- |
| Ledger | `adapt-pi-session-tree-ledger` | `adapt-clankers-neutral-session-ledger`, `adapt-clankers-provenance-ledger`, `adapt-pi-compaction-branch-summaries`, `add-animus-hf-dataset-export` |
| Receipt redaction | `adapt-clankers-safe-receipts-artifacts` | `adapt-clankers-provenance-ledger`, `adapt-git-ai-vcs-neutral-provenance`, `adapt-hermes-review-report-contracts`, `add-animus-hf-dataset-export` |
| Provider dispatch | `adapt-clankers-injected-effect-services` | `adapt-pi-provider-registry-shell`, `add-concrete-std-runtime-adapters`, `evaluate-clankers-provider-contract-parity` |
| Hooks | `adapt-pi-runtime-event-lifecycle` | `adapt-openai-codex-hook-trust-lifecycle`, `adapt-claude-code-lifecycle-hooks`, `adapt-openai-codex-jsonl-exec-sdk` |
| Manifest/catalog | `adapt-hermes-plugin-tool-manifests` | `evaluate-clankers-plugin-manifest-boundary`, `evaluate-clankers-tool-catalog-packs`, `adapt-claude-code-mcp-bridge` |
| Tool contracts | `adapt-pi-tool-contracts` | `adapt-clankers-injected-effect-services`, `adapt-hermes-background-work-queues`, `add-ratatui-tool-host-surface`, `add-concrete-std-runtime-adapters` |
| Policy/guidance | `adapt-hermes-policy-profiles` | `adapt-hermes-retrieval-budgets`, `adapt-openai-codex-guidance-skill-budget`, `adapt-pi-skills-resource-discovery`, `adapt-claude-code-layered-memory` |
| UI/headless | `adapt-pi-runtime-event-lifecycle` | `adapt-pi-headless-rpc-protocol`, `adapt-openai-codex-jsonl-exec-sdk`, `adapt-claude-code-headless-surfaces`, `add-ratatui-tool-host-surface` |

## Closeout rule

A dependent change is archive-ready only when its prerequisite requirement IDs are accepted, its prerequisite changes are archived in the same batch, or a waiver names the replacement requirement ID. Product-surface changes must retain a transitive path through foundational lifecycle/policy evidence and reusable runtime safety contracts before they can claim stable behavior.
