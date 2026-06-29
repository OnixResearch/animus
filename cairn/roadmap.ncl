# Roadmap sequencing is data, not prose. This Nickel file is the source
# of truth consumed by scripts/cairn/check-roadmap.
let Tier = {
  id | String,
  name | String,
  requires | Array String,
}
in
let Change = {
  id | String,
  role | String,
  tier | String,
  prerequisites | Array String,
  related | Array String,
}
in
let Overlap = {
  concept | String,
  owner | String,
  related | Array String,
}
in

let foundation_role = "foundational" in
let core_role = "core-runtime" in
let host_role = "host-adapter" in
let optional_role = "optional-reference" in
let product_role = "product-surface" in
let evaluation_role = "evaluation-only" in

let tier = fun tier_id tier_name required_tiers =>
  {
    id = tier_id,
    name = tier_name,
    requires = required_tiers,
  }
in
let change = fun change_id change_role change_tier change_prerequisites related_changes =>
  {
    id = change_id,
    role = change_role,
    tier = change_tier,
    prerequisites = change_prerequisites,
    related = related_changes,
  }
in
let overlap = fun overlap_concept owner_change related_changes =>
  {
    concept = overlap_concept,
    owner = owner_change,
    related = related_changes,
  }
in
let csv = fun values => std.string.join "," values in
let tier_line = fun entry => "tier|%{entry.id}|%{csv entry.requires}|%{entry.name}" in
let change_line = fun entry => "change|%{entry.id}|%{entry.role}|%{entry.tier}|%{csv entry.prerequisites}|%{csv entry.related}" in
let overlap_line = fun entry => "overlap|%{entry.concept}|%{entry.owner}|%{csv entry.related}" in

let tier_entries = [
  tier "policy-wiring" "Local Cairn policy wiring" [],
  tier "baseline-lifecycle" "Accepted specs, dogfood lifecycle, and focused verification rails" ["policy-wiring"],
  tier "roadmap-sequencing" "Roadmap classification, dependency, and overlap hygiene" ["baseline-lifecycle"],
  tier "runtime-loop" "Reusable runtime loop, lifecycle ownership, and facade inventory" ["roadmap-sequencing"],
  tier "injected-services" "Injected model/tool/provider/effect services" ["runtime-loop"],
  tier "receipt-provenance" "Safe receipts, artifacts, provenance, and boundary schemas" ["injected-services"],
  tier "session-ledger" "Neutral session trees, replay, compaction, and summaries" ["receipt-provenance"],
  tier "event-hooks" "Runtime events, hooks, middleware, and trust decisions" ["session-ledger"],
  tier "policy-guidance" "Capability policy, permissions, guidance, skills, memory, and retrieval budgets" ["event-hooks"],
  tier "concrete-adapters" "Std shell, providers, queue, sandbox, workspace, and dataspace adapters" ["policy-guidance"],
  tier "ui-headless" "TUI, headless, SDK, review, subagent, workflow, and MCP surfaces" ["concrete-adapters"],
  tier "export-surfaces" "Dataset, evidence, and external lifecycle export surfaces" ["ui-headless"],
  tier "optional-references" "Bounded optional reference adapters and non-default dependency gates" ["roadmap-sequencing"],
  tier "evaluation" "Evaluation-only studies and adoption matrices" ["roadmap-sequencing"],
]
in

let change_entries = [
  change "adapt-branchfs-speculative-workspaces" host_role "concrete-adapters" ["adapt-pi-tool-contracts", "adapt-sandlock-sandbox-enforcement", "adapt-clankers-provenance-ledger"] [],
  change "adapt-clankers-agent-definition-trust" core_role "policy-guidance" ["adapt-hermes-skill-manifests", "adapt-claude-code-permission-modes", "align-clash-patterns-with-ucan"] ["adapt-claude-code-agent-teams"],
  change "adapt-clankers-batch-trajectory-runner" product_role "export-surfaces" ["adapt-pi-headless-rpc-protocol", "adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout", "agent_harness_runtime.clankers_focused_rails.closeout"] ["add-animus-hf-dataset-export"],
  change "adapt-clankers-checkpoint-rollback-ux" host_role "concrete-adapters" ["adapt-git-ai-vcs-neutral-provenance", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout", "adapt-clankers-confirmation-broker-kit"] ["adapt-branchfs-speculative-workspaces", "adapt-openai-codex-worktree-automations"],
  change "adapt-clankers-confirmation-broker-kit" core_role "injected-services" ["agent_harness_runtime.clankers_injected_services.closeout", "align-clash-patterns-with-ucan"] ["adapt-clankers-checkpoint-rollback-ux", "adapt-clankers-tool-gateway-delivery", "adapt-clankers-remote-admission-receipts"],
  change "adapt-clankers-context-references" core_role "policy-guidance" ["adapt-hermes-retrieval-budgets", "adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-clankers-prompt-assembly-kit", "adapt-openai-codex-guidance-skill-budget", "add-multimodal-image-payloads", "add-animus-hf-dataset-export"],
  change "adapt-clankers-controller-continuation-policy" core_role "event-hooks" ["adapt-pi-runtime-event-lifecycle", "adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-openai-codex-jsonl-exec-sdk", "adapt-claude-code-workflow-commands"],
  change "adapt-clankers-cost-model-routing" core_role "policy-guidance" ["adapt-hermes-policy-profiles", "agent_harness_runtime.clankers_injected_services.closeout", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-pi-provider-registry-shell"],
  change "adapt-clankers-observability-audit-receipts" core_role "receipt-provenance" ["agent_harness_runtime.clankers_safe_receipts.closeout", "adapt-pi-runtime-event-lifecycle"] [],
  change "adapt-clankers-provenance-ledger" core_role "receipt-provenance" ["agent_harness_runtime.clankers_safe_receipts.closeout", "agent_harness_runtime.clankers_neutral_ledger.closeout"] ["adapt-git-ai-vcs-neutral-provenance"],
  change "adapt-clankers-prompt-assembly-kit" core_role "policy-guidance" ["adapt-hermes-policy-profiles", "adapt-openai-codex-guidance-skill-budget", "adapt-clankers-context-references", "adapt-clankers-agent-definition-trust"] ["add-multimodal-image-payloads", "adapt-claude-code-layered-memory", "adapt-openai-codex-opt-in-memories"],
  change "adapt-clankers-remote-admission-receipts" core_role "policy-guidance" ["align-clash-patterns-with-ucan", "adapt-clankers-confirmation-broker-kit", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-pi-headless-rpc-protocol", "adapt-claude-code-mcp-bridge"],
  change "adapt-clankers-runtime-patterns" core_role "runtime-loop" ["agent_harness_runtime.validation", "agent_harness_runtime.clankers_injected_services.closeout", "agent_harness_runtime.clankers_safe_receipts.closeout", "agent_harness_runtime.clankers_neutral_ledger.closeout", "adapt-clankers-steel-runtime-policy"] [],
  change "adapt-clankers-steel-runtime-policy" core_role "policy-guidance" ["agent_harness_runtime.validation", "agent_harness_runtime.cairn_policy.closeout"] [],
  change "adapt-clankers-tool-gateway-delivery" host_role "concrete-adapters" ["adapt-pi-tool-contracts", "adapt-hermes-plugin-tool-manifests", "agent_harness_runtime.clankers_safe_receipts.closeout", "adapt-pi-runtime-event-lifecycle"] ["adapt-hermes-background-work-queues", "add-ratatui-tool-host-surface"],
  change "adapt-claude-code-agent-teams" product_role "ui-headless" ["adapt-openai-codex-subagent-threads", "adapt-claude-code-permission-modes", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-claude-code-headless-surfaces" product_role "ui-headless" ["adapt-pi-headless-rpc-protocol", "adapt-openai-codex-jsonl-exec-sdk", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-claude-code-layered-memory" core_role "policy-guidance" ["adapt-hermes-policy-profiles", "agent_harness_runtime.clankers_neutral_ledger.closeout"] [],
  change "adapt-claude-code-lifecycle-hooks" core_role "event-hooks" ["adapt-pi-runtime-event-lifecycle", "adapt-openai-codex-hook-trust-lifecycle", "adapt-claude-code-permission-modes"] [],
  change "adapt-claude-code-mcp-bridge" product_role "ui-headless" ["adapt-pi-headless-rpc-protocol", "adapt-pi-tool-contracts", "adapt-hermes-plugin-tool-manifests", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-claude-code-permission-modes" core_role "policy-guidance" ["align-clash-patterns-with-ucan", "agent_harness_runtime.clankers_injected_services.closeout"] [],
  change "adapt-claude-code-workflow-commands" product_role "ui-headless" ["adapt-claude-code-permission-modes", "adapt-pi-skills-resource-discovery", "adapt-pi-headless-rpc-protocol", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-git-ai-vcs-neutral-provenance" core_role "receipt-provenance" ["adapt-clankers-provenance-ledger", "adapt-pi-tool-contracts"] [],
  change "adapt-host-control-protocol" product_role "ui-headless" ["adapt-pi-headless-rpc-protocol", "add-ratatui-tool-host-surface", "agent_harness_runtime.clankers_focused_rails.closeout"] ["adapt-openai-codex-jsonl-exec-sdk", "adapt-claude-code-headless-surfaces"],
  change "adapt-plugin-action-manifests" core_role "policy-guidance" ["adapt-hermes-plugin-tool-manifests", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-host-control-protocol"],
  change "adapt-session-restore-contract" core_role "session-ledger" ["adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-host-control-protocol", "adapt-pi-headless-rpc-protocol"],
  change "adapt-hermes-background-work-queues" host_role "concrete-adapters" ["agent_harness_runtime.clankers_injected_services.closeout", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-clankers-process-job-profiles" host_role "concrete-adapters" ["adapt-hermes-background-work-queues", "adapt-hermes-policy-profiles", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-hermes-kanban-orchestration" product_role "ui-headless" ["adapt-hermes-background-work-queues", "adapt-hermes-review-report-contracts", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-hermes-plugin-tool-manifests" core_role "policy-guidance" ["adapt-pi-tool-contracts", "align-clash-patterns-with-ucan"] [],
  change "adapt-hermes-policy-profiles" core_role "policy-guidance" ["align-clash-patterns-with-ucan"] [],
  change "adapt-hermes-researchspec-lifecycle" product_role "export-surfaces" ["agent_harness_runtime.cairn_dogfood.closeout", "adapt-hermes-review-report-contracts", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-hermes-retrieval-budgets" core_role "policy-guidance" ["adapt-hermes-policy-profiles"] [],
  change "adapt-hermes-review-report-contracts" core_role "receipt-provenance" ["adapt-pi-runtime-event-lifecycle", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-hermes-skill-manifests" core_role "policy-guidance" ["adapt-pi-skills-resource-discovery", "adapt-hermes-policy-profiles"] [],
  change "adapt-openai-codex-diff-review-mode" product_role "ui-headless" ["adapt-openai-codex-jsonl-exec-sdk", "adapt-hermes-review-report-contracts", "adapt-git-ai-vcs-neutral-provenance", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-openai-codex-execpolicy-rules" core_role "policy-guidance" ["align-clash-patterns-with-ucan", "adapt-pi-tool-contracts"] [],
  change "adapt-openai-codex-guidance-skill-budget" core_role "policy-guidance" ["adapt-hermes-skill-manifests", "adapt-hermes-retrieval-budgets"] [],
  change "adapt-openai-codex-hook-trust-lifecycle" core_role "event-hooks" ["adapt-pi-runtime-event-lifecycle", "align-clash-patterns-with-ucan"] [],
  change "adapt-openai-codex-jsonl-exec-sdk" product_role "ui-headless" ["agent_harness_runtime.validation", "adapt-pi-runtime-event-lifecycle", "adapt-pi-session-tree-ledger", "adapt-openai-codex-execpolicy-rules", "adapt-clankers-controller-continuation-policy", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-openai-codex-opt-in-memories" core_role "policy-guidance" ["adapt-claude-code-layered-memory", "adapt-pi-session-tree-ledger"] [],
  change "adapt-openai-codex-subagent-threads" product_role "ui-headless" ["adapt-openai-codex-jsonl-exec-sdk", "adapt-pi-session-tree-ledger", "adapt-hermes-background-work-queues", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-openai-codex-worktree-automations" product_role "ui-headless" ["adapt-branchfs-speculative-workspaces", "adapt-openai-codex-jsonl-exec-sdk", "adapt-sandlock-sandbox-enforcement", "adapt-git-ai-vcs-neutral-provenance", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-ornith-eval-feedback-loop" evaluation_role "evaluation" ["adapt-ornith-scaffold-profiles", "adapt-ornith-trust-boundary-monitor", "adapt-clankers-batch-trajectory-runner", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-ornith-scaffold-profiles" core_role "policy-guidance" ["adapt-clankers-steel-runtime-policy", "adapt-hermes-policy-profiles", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-ornith-eval-feedback-loop"],
  change "adapt-ornith-trust-boundary-monitor" core_role "policy-guidance" ["align-clash-patterns-with-ucan", "adapt-openai-codex-hook-trust-lifecycle", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-ornith-eval-feedback-loop"],
  change "adapt-pi-compaction-branch-summaries" core_role "session-ledger" ["adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-pi-headless-rpc-protocol" product_role "ui-headless" ["adapt-pi-runtime-event-lifecycle", "adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout", "adapt-clankers-remote-admission-receipts", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "adapt-pi-provider-registry-shell" host_role "concrete-adapters" ["agent_harness_runtime.clankers_injected_services.closeout", "adapt-hermes-policy-profiles"] ["add-multimodal-image-payloads"],
  change "adapt-pi-runtime-event-lifecycle" core_role "event-hooks" ["agent_harness_runtime.validation", "agent_harness_runtime.clankers_injected_services.closeout"] [],
  change "adapt-pi-session-tree-ledger" core_role "session-ledger" ["agent_harness_runtime.clankers_neutral_ledger.closeout"] [],
  change "adapt-pi-skills-resource-discovery" core_role "policy-guidance" ["adapt-hermes-policy-profiles", "align-clash-patterns-with-ucan"] [],
  change "adapt-pi-tool-contracts" core_role "injected-services" ["agent_harness_runtime.clankers_injected_services.closeout", "align-clash-patterns-with-ucan"] [],
  change "adapt-qualifier-quality-records" core_role "receipt-provenance" ["adapt-clankers-provenance-ledger", "adapt-hermes-review-report-contracts"] [],
  change "adapt-sandlock-sandbox-enforcement" host_role "concrete-adapters" ["align-clash-patterns-with-ucan", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "adapt-syndicate-dataspace-adapters" optional_role "optional-references" ["adapt-pi-runtime-event-lifecycle", "agent_harness_runtime.clankers_injected_services.closeout"] [],
  change "adapt-trellis-verified-core-boundaries" optional_role "optional-references" ["agent_harness_runtime.validation", "agent_harness_runtime.clankers_runtime_facade_inventory.closeout"] [],
  change "adapt-zerostack-context-file-layering" core_role "policy-guidance" ["adapt-clankers-context-references", "adapt-clankers-prompt-assembly-kit", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-pi-skills-resource-discovery", "adapt-zerostack-prompt-mode-profiles"],
  change "adapt-zerostack-fresh-context-tasks" product_role "ui-headless" ["adapt-openai-codex-subagent-threads", "adapt-pi-tool-contracts", "agent_harness_runtime.clankers_focused_rails.closeout"] ["adapt-claude-code-agent-teams"],
  change "adapt-zerostack-hashed-edit-protocol" core_role "injected-services" ["adapt-pi-tool-contracts", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["add-concrete-std-runtime-adapters"],
  change "adapt-zerostack-midturn-compaction" core_role "session-ledger" ["adapt-pi-compaction-branch-summaries", "adapt-pi-session-tree-ledger", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-pi-headless-rpc-protocol"],
  change "adapt-zerostack-permission-kernel" core_role "policy-guidance" ["adapt-claude-code-permission-modes", "adapt-openai-codex-execpolicy-rules", "align-clash-patterns-with-ucan"] ["adapt-hermes-policy-profiles"],
  change "adapt-zerostack-prompt-mode-profiles" core_role "policy-guidance" ["adapt-clankers-prompt-assembly-kit", "adapt-hermes-policy-profiles", "adapt-zerostack-permission-kernel"] ["adapt-openai-codex-guidance-skill-budget"],
  change "adapt-zerostack-provider-registry-hardening" host_role "concrete-adapters" ["adapt-pi-provider-registry-shell", "agent_harness_runtime.clankers_injected_services.closeout", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["add-concrete-std-runtime-adapters"],
  change "add-animus-hf-dataset-export" product_role "export-surfaces" ["adapt-pi-session-tree-ledger", "adapt-openai-codex-opt-in-memories", "agent_harness_runtime.clankers_safe_receipts.closeout", "adapt-hermes-review-report-contracts", "adapt-clankers-batch-trajectory-runner", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "add-concrete-std-runtime-adapters" host_role "concrete-adapters" ["agent_harness_runtime.clankers_injected_services.closeout", "adapt-pi-tool-contracts", "adapt-pi-provider-registry-shell", "adapt-openai-codex-execpolicy-rules", "agent_harness_runtime.clankers_safe_receipts.closeout"] [],
  change "add-multimodal-image-payloads" host_role "concrete-adapters" ["adapt-clankers-context-references", "adapt-clankers-prompt-assembly-kit", "adapt-pi-provider-registry-shell", "agent_harness_runtime.clankers_safe_receipts.closeout"] ["adapt-pi-headless-rpc-protocol", "adapt-openai-codex-jsonl-exec-sdk", "add-animus-hf-dataset-export"],
  change "add-ratatui-tool-host-surface" product_role "ui-headless" ["add-concrete-std-runtime-adapters", "adapt-pi-runtime-event-lifecycle", "agent_harness_runtime.clankers_safe_receipts.closeout", "agent_harness_runtime.clankers_focused_rails.closeout"] [],
  change "align-clash-patterns-with-ucan" core_role "policy-guidance" ["agent_harness_runtime.cairn_policy.closeout", "agent_harness_runtime.clankers_injected_services.closeout"] [],
  change "ensure-model-token-thought-streaming" core_role "concrete-adapters" ["adapt-pi-runtime-event-lifecycle", "adapt-pi-provider-registry-shell", "evaluate-clankers-provider-contract-parity"] ["adapt-pi-headless-rpc-protocol", "adapt-openai-codex-jsonl-exec-sdk", "add-ratatui-tool-host-surface"],
  change "evaluate-clankers-provider-contract-parity" evaluation_role "evaluation" ["adapt-pi-provider-registry-shell"] [],
  change "evaluate-clankers-tool-catalog-packs" evaluation_role "evaluation" ["adapt-hermes-plugin-tool-manifests", "adapt-pi-tool-contracts"] [],
  change "harden-preserves-boundary-schemas" core_role "receipt-provenance" ["agent_harness_runtime.validation", "agent_harness_runtime.clankers_neutral_ledger.closeout", "adapt-pi-runtime-event-lifecycle"] [],
]
in

let overlap_entries = [
  overlap "ledger" "adapt-pi-session-tree-ledger" ["adapt-clankers-provenance-ledger", "adapt-pi-compaction-branch-summaries", "adapt-zerostack-midturn-compaction", "adapt-clankers-context-references", "add-animus-hf-dataset-export"],
  overlap "receipt-redaction" "adapt-clankers-provenance-ledger" ["adapt-clankers-observability-audit-receipts", "adapt-clankers-remote-admission-receipts", "adapt-git-ai-vcs-neutral-provenance", "adapt-hermes-review-report-contracts", "adapt-clankers-checkpoint-rollback-ux", "adapt-clankers-tool-gateway-delivery", "adapt-clankers-batch-trajectory-runner", "adapt-ornith-trust-boundary-monitor", "adapt-ornith-eval-feedback-loop", "add-animus-hf-dataset-export"],
  overlap "audit-observability" "adapt-clankers-observability-audit-receipts" ["adapt-pi-runtime-event-lifecycle"],
  overlap "model-streaming" "ensure-model-token-thought-streaming" ["adapt-pi-runtime-event-lifecycle", "adapt-pi-headless-rpc-protocol", "adapt-openai-codex-jsonl-exec-sdk", "add-ratatui-tool-host-surface", "adapt-pi-provider-registry-shell", "adapt-clankers-cost-model-routing", "add-multimodal-image-payloads", "evaluate-clankers-provider-contract-parity"],
  overlap "provider-dispatch" "adapt-pi-provider-registry-shell" ["adapt-zerostack-provider-registry-hardening", "add-concrete-std-runtime-adapters", "adapt-clankers-cost-model-routing", "add-multimodal-image-payloads", "evaluate-clankers-provider-contract-parity"],
  overlap "hooks" "adapt-pi-runtime-event-lifecycle" ["adapt-openai-codex-hook-trust-lifecycle", "adapt-claude-code-lifecycle-hooks", "adapt-openai-codex-jsonl-exec-sdk"],
  overlap "manifest-catalog" "adapt-hermes-plugin-tool-manifests" ["adapt-clankers-tool-gateway-delivery", "adapt-plugin-action-manifests", "evaluate-clankers-tool-catalog-packs", "adapt-claude-code-mcp-bridge"],
  overlap "tool-contracts" "adapt-pi-tool-contracts" ["adapt-zerostack-hashed-edit-protocol", "adapt-hermes-background-work-queues", "adapt-clankers-tool-gateway-delivery", "add-ratatui-tool-host-surface", "add-concrete-std-runtime-adapters"],
  overlap "policy-guidance" "adapt-hermes-policy-profiles" ["adapt-clankers-prompt-assembly-kit", "adapt-zerostack-permission-kernel", "adapt-zerostack-prompt-mode-profiles", "adapt-clankers-cost-model-routing", "adapt-clankers-agent-definition-trust", "adapt-clankers-process-job-profiles", "adapt-hermes-retrieval-budgets", "adapt-openai-codex-guidance-skill-budget", "adapt-pi-skills-resource-discovery", "adapt-claude-code-layered-memory", "adapt-ornith-scaffold-profiles", "adapt-ornith-trust-boundary-monitor"],
  overlap "prompt-assembly" "adapt-clankers-prompt-assembly-kit" ["adapt-hermes-policy-profiles", "adapt-openai-codex-guidance-skill-budget", "adapt-zerostack-prompt-mode-profiles", "adapt-clankers-context-references", "adapt-zerostack-context-file-layering", "add-multimodal-image-payloads", "adapt-clankers-agent-definition-trust", "adapt-claude-code-layered-memory", "adapt-openai-codex-opt-in-memories", "adapt-ornith-scaffold-profiles"],
  overlap "context-references" "adapt-clankers-context-references" ["adapt-hermes-retrieval-budgets", "adapt-pi-skills-resource-discovery", "adapt-clankers-prompt-assembly-kit", "adapt-zerostack-context-file-layering", "adapt-zerostack-fresh-context-tasks", "add-multimodal-image-payloads", "adapt-clankers-batch-trajectory-runner", "add-animus-hf-dataset-export"],
  overlap "batch-export" "adapt-clankers-batch-trajectory-runner" ["add-animus-hf-dataset-export", "adapt-hermes-researchspec-lifecycle", "adapt-ornith-eval-feedback-loop"],
  overlap "scaffold-feedback" "adapt-ornith-scaffold-profiles" ["adapt-ornith-eval-feedback-loop", "adapt-ornith-trust-boundary-monitor", "adapt-clankers-batch-trajectory-runner", "adapt-clankers-prompt-assembly-kit"],
  overlap "workspace-checkpoints" "adapt-git-ai-vcs-neutral-provenance" ["adapt-clankers-checkpoint-rollback-ux", "adapt-branchfs-speculative-workspaces", "adapt-openai-codex-worktree-automations"],
  overlap "artifact-delivery" "adapt-clankers-tool-gateway-delivery" ["adapt-pi-tool-contracts", "adapt-hermes-background-work-queues", "add-ratatui-tool-host-surface"],
  overlap "multimodal-input" "add-multimodal-image-payloads" ["adapt-clankers-context-references", "adapt-clankers-prompt-assembly-kit", "adapt-pi-provider-registry-shell", "evaluate-clankers-provider-contract-parity", "add-animus-hf-dataset-export"],
  overlap "background-work" "adapt-hermes-background-work-queues" ["adapt-clankers-process-job-profiles", "adapt-clankers-tool-gateway-delivery", "adapt-hermes-kanban-orchestration"],
  overlap "agent-definitions" "adapt-clankers-agent-definition-trust" ["adapt-hermes-skill-manifests", "adapt-clankers-prompt-assembly-kit", "adapt-claude-code-agent-teams", "adapt-openai-codex-subagent-threads", "adapt-zerostack-fresh-context-tasks"],
  overlap "confirmation" "adapt-clankers-confirmation-broker-kit" ["adapt-clankers-checkpoint-rollback-ux", "adapt-clankers-tool-gateway-delivery", "adapt-clankers-remote-admission-receipts"],
  overlap "continuation-policy" "adapt-clankers-controller-continuation-policy" ["adapt-pi-runtime-event-lifecycle", "adapt-pi-session-tree-ledger", "adapt-openai-codex-jsonl-exec-sdk", "adapt-claude-code-workflow-commands"],
  overlap "session-restore" "adapt-session-restore-contract" ["adapt-pi-session-tree-ledger", "adapt-pi-headless-rpc-protocol", "adapt-host-control-protocol"],
  overlap "remote-admission" "adapt-clankers-remote-admission-receipts" ["align-clash-patterns-with-ucan", "adapt-pi-headless-rpc-protocol", "adapt-claude-code-mcp-bridge"],
  overlap "host-control" "adapt-host-control-protocol" ["adapt-pi-headless-rpc-protocol", "add-ratatui-tool-host-surface", "adapt-openai-codex-jsonl-exec-sdk", "adapt-claude-code-headless-surfaces", "adapt-session-restore-contract"],
  overlap "ui-headless" "adapt-pi-runtime-event-lifecycle" ["adapt-pi-headless-rpc-protocol", "adapt-openai-codex-jsonl-exec-sdk", "adapt-claude-code-headless-surfaces", "add-ratatui-tool-host-surface", "adapt-host-control-protocol"],
]
in

{
  tiers | Array Tier = tier_entries,
  changes | Array Change = change_entries,
  overlaps | Array Overlap = overlap_entries,
  roadmap_lines =
    std.string.join
      "\n"
      (
        (std.array.map tier_line tier_entries)
        @ (std.array.map change_line change_entries)
        @ (std.array.map overlap_line overlap_entries)
      ),
}
