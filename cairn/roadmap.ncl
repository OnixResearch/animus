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
  tier "router-crate" "Router crate barrier, pure decision cores, and receipt assembly" ["injected-services"],
]
in

let change_entries = [
  change "add-canonical-model-message-contracts" core_role "injected-services" [
    "agent_harness_runtime.model_dispatch.plan",
    "agent_harness_runtime.multimodal_image_payloads.typed_parts",
    "agent_harness_runtime.model_streaming.adapter_contract",
    "agent_harness_runtime.clankers_provider_parity.request_fields",
  ] ["negotiate-model-capability-matrix", "add-provider-projection-conformance-fixtures"],
  change "negotiate-model-capability-matrix" host_role "concrete-adapters" [
    "add-canonical-model-message-contracts",
    "agent_harness_runtime.pi_provider_registry.metadata",
    "agent_harness_runtime.model_dispatch.plan",
  ] ["add-canonical-model-message-contracts", "add-provider-projection-conformance-fixtures"],
  change "add-provider-projection-conformance-fixtures" evaluation_role "evaluation" [
    "add-canonical-model-message-contracts",
    "negotiate-model-capability-matrix",
    "agent_harness_runtime.clankers_provider_parity.message_conversion",
    "agent_harness_runtime.model_streaming.parser_entrypoint",
  ] ["add-canonical-model-message-contracts", "negotiate-model-capability-matrix"],
  change "add-tui-conversation-render-model" product_role "ui-headless" [
    "agent_harness_runtime.ratatui_tool_host.boundary",
    "agent_harness_runtime.cairn_policy.closeout",
    "agent_harness_runtime.clankers_injected_services.closeout",
  ] ["add-tui-streaming-markdown-view", "add-tui-prompt-input-and-keymap", "add-tui-tool-call-and-approval-panels", "add-tui-session-context-status-rail", "add-tui-agent-control-and-queue-dashboard", "add-tui-app-shell-and-event-loop"],
  change "add-tui-streaming-markdown-view" product_role "ui-headless" [
    "add-tui-conversation-render-model",
  ] ["add-tui-conversation-render-model"],
  change "add-tui-prompt-input-and-keymap" product_role "ui-headless" [
    "add-tui-conversation-render-model",
  ] ["add-tui-conversation-render-model"],
  change "add-tui-tool-call-and-approval-panels" product_role "ui-headless" [
    "add-tui-conversation-render-model",
    "agent_harness_runtime.ratatui_tool_host.bridge",
  ] ["add-tui-conversation-render-model"],
  change "add-tui-session-context-status-rail" product_role "ui-headless" [
    "add-tui-conversation-render-model",
  ] ["add-tui-conversation-render-model"],
  change "add-tui-agent-control-and-queue-dashboard" product_role "ui-headless" [
    "add-tui-conversation-render-model",
  ] ["add-tui-conversation-render-model"],
  change "add-tui-app-shell-and-event-loop" product_role "ui-headless" [
    "add-tui-conversation-render-model",
    "add-tui-streaming-markdown-view",
    "add-tui-prompt-input-and-keymap",
    "add-tui-tool-call-and-approval-panels",
    "add-tui-session-context-status-rail",
    "add-tui-agent-control-and-queue-dashboard",
  ] ["add-tui-conversation-render-model"],
  change "introduce-router-crate-barrier" foundation_role "router-crate" [
    "agent_harness_runtime.vllm_semantic_router_recipes.selection",
  ] ["define-router-execution-plan", "project-router-request-signals", "negotiate-router-capability-eligibility", "add-router-policy-contracts", "preserve-router-boundary-dtos"],
  change "define-router-execution-plan" core_role "router-crate" [
    "introduce-router-crate-barrier",
    "agent_harness_runtime.vllm_semantic_router_recipes.bounds",
  ] ["aggregate-router-route-results", "gate-router-budget-and-fallback", "wire-router-into-runtime-loop"],
  change "project-router-request-signals" core_role "router-crate" [
    "introduce-router-crate-barrier",
    "agent_harness_runtime.vllm_semantic_router_recipes.signals",
  ] ["add-supra-router-signal-adapter", "wire-router-into-runtime-loop"],
  change "aggregate-router-route-results" core_role "router-crate" [
    "define-router-execution-plan",
    "agent_harness_runtime.vllm_semantic_router_recipes.output_contract",
  ] ["emit-router-safe-receipts", "wire-router-into-runtime-loop", "evaluate-router-recipe-coverage"],
  change "emit-router-safe-receipts" core_role "router-crate" [
    "aggregate-router-route-results",
    "agent_harness_runtime.vllm_semantic_router_recipes.receipts",
  ] ["wire-router-into-runtime-loop", "evaluate-router-recipe-coverage"],
  change "wire-router-into-runtime-loop" core_role "runtime-loop" [
    "define-router-execution-plan",
    "agent_harness_runtime.model_dispatch.runtime_gate",
  ] ["aggregate-router-route-results", "emit-router-safe-receipts", "evaluate-router-recipe-coverage"],
  change "negotiate-router-capability-eligibility" core_role "policy-guidance" [
    "introduce-router-crate-barrier",
    "agent_harness_runtime.model_capability_matrix.negotiation",
  ] ["wire-router-into-runtime-loop"],
  change "gate-router-budget-and-fallback" core_role "policy-guidance" [
    "define-router-execution-plan",
    "agent_harness_runtime.vllm_semantic_router_recipes.bounds",
  ] ["wire-router-into-runtime-loop"],
  change "add-supra-router-signal-adapter" host_role "concrete-adapters" [
    "project-router-request-signals",
  ] [],
  change "add-router-policy-contracts" core_role "policy-guidance" [
    "introduce-router-crate-barrier",
  ] [],
  change "preserve-router-boundary-dtos" optional_role "optional-references" [
    "introduce-router-crate-barrier",
  ] [],
  change "evaluate-router-recipe-coverage" evaluation_role "evaluation" [
    "wire-router-into-runtime-loop",
    "aggregate-router-route-results",
    "emit-router-safe-receipts",
  ] [],
]
in

let overlap_entries = [
  overlap "canonical-request-provider-projection" "add-canonical-model-message-contracts" ["add-provider-projection-conformance-fixtures"],
  overlap "capability-negotiation-provider-claims" "negotiate-model-capability-matrix" ["add-provider-projection-conformance-fixtures"],
  overlap "tui-render-model" "add-tui-conversation-render-model" ["add-tui-streaming-markdown-view", "add-tui-prompt-input-and-keymap", "add-tui-tool-call-and-approval-panels", "add-tui-session-context-status-rail", "add-tui-agent-control-and-queue-dashboard", "add-tui-app-shell-and-event-loop"],
  overlap "router-execution-plan-materialization" "define-router-execution-plan" ["wire-router-into-runtime-loop", "gate-router-budget-and-fallback"],
  overlap "router-signal-projection" "project-router-request-signals" ["add-supra-router-signal-adapter"],
  overlap "router-receipt-assembly" "emit-router-safe-receipts" ["wire-router-into-runtime-loop", "evaluate-router-recipe-coverage"],
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
