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
  change "define-dogfoodable-host-readiness-roadmap" foundation_role "roadmap-sequencing" [
    "agent_harness_runtime.cairn_policy.closeout",
    "agent_harness_runtime.clankers_focused_rails.closeout",
    "agent_harness_runtime.validation",
  ] ["adapt-claude-code-headless-surfaces", "adapt-pi-headless-rpc-protocol", "add-ratatui-tool-host-surface", "adapt-claude-code-mcp-bridge", "add-concrete-std-runtime-adapters", "adapt-clankers-batch-trajectory-runner"],
  change "adapt-prime-agent-schedule-reliability" core_role "runtime-loop" [
    "agent_harness_runtime.goose_scheduled_recipes.data_only",
    "agent_harness_runtime.messaging_receipts.modes",
  ] ["add-bounded-terminal-video-playback"],
  change "add-sequenced-session-event-replay" core_role "session-ledger" [
    "agent_harness_runtime.conversation_view_projection.boundary",
    "agent_harness_runtime.conversation_view_projection.identity",
  ] ["add-tui-conversation-render-model", "adapt-pi-headless-rpc-protocol"],
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
  change "adopt-latest-rats-image-protocol-adapters" product_role "ui-headless" [
    "add-tui-streaming-markdown-view",
    "add-tui-app-shell-and-event-loop",
    "agent_harness_runtime.ratatui_tool_host.boundary",
    "agent_harness_runtime.multimodal_image_payloads.typed_parts",
  ] ["add-provider-video-payload-contracts", "add-bounded-terminal-video-playback"],
  change "add-provider-video-payload-contracts" host_role "concrete-adapters" [
    "add-canonical-model-message-contracts",
    "negotiate-model-capability-matrix",
    "agent_harness_runtime.multimodal_image_payloads.typed_parts",
  ] ["adopt-latest-rats-image-protocol-adapters", "add-bounded-terminal-video-playback"],
  change "add-bounded-terminal-video-playback" product_role "ui-headless" [
    "adopt-latest-rats-image-protocol-adapters",
    "add-tui-app-shell-and-event-loop",
  ] ["add-provider-video-payload-contracts"],
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
  change "define-model-router-execution-service" host_role "injected-services" [
    "define-router-execution-plan",
    "wire-router-into-runtime-loop",
    "agent_harness_runtime.clankers_injected_services.contracts",
  ] ["evaluate-router-recipe-coverage", "add-concrete-provider-streaming-adapters"],
  change "add-concrete-provider-streaming-adapters" host_role "concrete-adapters" [
    "agent_harness_runtime.model_streaming.adapter_contract",
    "agent_harness_runtime.model_streaming.parser_entrypoint",
    "agent_harness_runtime.model_streaming.thought_visibility",
    "negotiate-model-capability-matrix",
    "add-provider-projection-conformance-fixtures",
  ] ["define-model-router-execution-service", "add-provider-projection-conformance-fixtures"],
  change "adapt-openclaw-gateway-control-plane" host_role "ui-headless" [
    "agent_harness_runtime.pi_headless_protocol.command_response",
    "agent_harness_runtime.pi_headless_protocol.event_stream",
    "agent_harness_runtime.clankers_remote_admission.before_effects",
  ] ["adapt-openclaw-node-capability-hosts", "adapt-openclaw-session-isolation-tools", "adapt-openclaw-security-plugin-guardrails"],
  change "extract-agent-comms-transport-contract" core_role "ui-headless" [
    "agent_harness_runtime.pi_headless_protocol.event_stream",
    "agent_harness_runtime.clankers_remote_admission.before_effects",
    "agent_harness_runtime.subagent_facade.boundary",
  ] ["adapt-iroh-agent-comms-host", "adapt-openclaw-gateway-control-plane", "adapt-goose-remote-server-admission"],
  change "adapt-iroh-agent-comms-host" host_role "concrete-adapters" [
    "extract-agent-comms-transport-contract",
    "agent_harness_runtime.clankers_remote_admission.before_effects",
    "agent_harness_runtime.pi_headless_protocol.event_stream",
  ] ["extract-agent-comms-transport-contract"],
  change "adapt-openclaw-node-capability-hosts" host_role "concrete-adapters" [
    "adapt-openclaw-gateway-control-plane",
    "agent_harness_runtime.host_control.transport_neutral",
    "agent_harness_runtime.clankers_confirmation_broker.requests",
    "agent_harness_runtime.zerostack_permission_kernel.receipts",
  ] ["adapt-openclaw-security-plugin-guardrails"],
  change "adapt-openclaw-session-isolation-tools" core_role "session-ledger" [
    "agent_harness_runtime.provenance_ledger.session_ledger",
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.subagent_facade.boundary",
    "agent_harness_runtime.subagent_coordinator.lifecycle",
  ] ["adapt-openclaw-gateway-control-plane", "adapt-openclaw-security-plugin-guardrails"],
  change "add-nested-subagent-delegation-graphs" core_role "ui-headless" [
    "agent_harness_runtime.subagent_facade.boundary",
    "agent_harness_runtime.subagent_context_packets.identity",
    "agent_harness_runtime.subagent_coordinator.service",
    "agent_harness_runtime.subagent_coordinator.lifecycle",
    "agent_harness_runtime.subagent_fanout_fanin.plan",
    "agent_harness_runtime.subagent_fanout_fanin.aggregation",
  ] ["adapt-openclaw-session-isolation-tools", "adapt-hermes-multi-lens-focus-decks", "adapt-hermes-plan-loop-mode-fsm", "add-runtime-dag-contract-kit"],
  change "add-runtime-dag-contract-kit" core_role "runtime-loop" [
    "agent_harness_runtime.validation",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
    "agent_harness_runtime.clankers_runtime_facade_inventory.schema",
  ] ["extend-recipe-workflows-with-dags", "add-researchspec-promotion-dags", "add-roadmap-dag-explanations", "add-nested-subagent-delegation-graphs", "adapt-goose-code-mode-tool-planning"],
  change "extend-recipe-workflows-with-dags" core_role "policy-guidance" [
    "add-runtime-dag-contract-kit",
    "agent_harness_runtime.goose_recipe_workflows.manifest",
    "agent_harness_runtime.goose_recipe_workflows.trust",
    "agent_harness_runtime.goose_recipe_workflows.retry_policy",
  ] ["adapt-goose-code-mode-tool-planning"],
  change "add-researchspec-promotion-dags" core_role "export-surfaces" [
    "add-runtime-dag-contract-kit",
    "agent_harness_runtime.hermes_researchspec.keep_promotion",
    "agent_harness_runtime.hermes_researchspec.keep_promotion.overlap",
    "agent_harness_runtime.hermes_researchspec.runs",
  ] ["adapt-hermes-scoped-validation-planner", "adapt-hermes-vcs-snapshot-ledger"],
  change "add-roadmap-dag-explanations" foundation_role "roadmap-sequencing" [
    "agent_harness_runtime.cairn_policy.closeout",
    "agent_harness_runtime.validation",
  ] ["add-runtime-dag-contract-kit", "define-dogfoodable-host-readiness-roadmap"],
  change "adapt-openclaw-security-plugin-guardrails" host_role "policy-guidance" [
    "agent_harness_runtime.clash_ucan.capability_request",
    "agent_harness_runtime.zerostack_permission_kernel.receipts",
    "agent_harness_runtime.hermes_plugin_manifests.validation",
    "agent_harness_runtime.plugin_action_manifests.diagnostics",
  ] ["adapt-openclaw-gateway-control-plane", "adapt-openclaw-node-capability-hosts", "adapt-openclaw-session-isolation-tools"],
  change "surface-memory-family-contract-index" core_role "policy-guidance" [
    "agent_harness_runtime.memory_adapters.family",
    "agent_harness_runtime.memory_adapters.verification",
    "agent_harness_runtime.memory_conformance.matrix",
  ] ["add-cognitive-memory-taxonomy", "add-memory-routing-contracts", "enrich-memory-retrieval-quality-policy", "add-production-memory-policy-descriptors", "add-memory-evaluation-rail"],
  change "add-cognitive-memory-taxonomy" core_role "policy-guidance" [
    "agent_harness_runtime.memory_adapters.record_envelope",
    "agent_harness_runtime.memory_adapters.retrieval_plan",
    "agent_harness_runtime.memory_adapters.receipts",
  ] ["surface-memory-family-contract-index", "add-memory-routing-contracts", "enrich-memory-retrieval-quality-policy"],
  change "add-memory-routing-contracts" core_role "policy-guidance" [
    "agent_harness_runtime.memory_adapters.family",
    "agent_harness_runtime.memory_adapters.retrieval_plan",
    "agent_harness_runtime.memory_controls.validation",
  ] ["add-cognitive-memory-taxonomy", "enrich-memory-retrieval-quality-policy", "add-production-memory-policy-descriptors"],
  change "enrich-memory-retrieval-quality-policy" core_role "policy-guidance" [
    "agent_harness_runtime.memory_adapters.retrieval_plan",
    "agent_harness_runtime.memory_adapters.record_envelope",
    "agent_harness_runtime.memory_controls.influence_explain",
  ] ["add-cognitive-memory-taxonomy", "add-memory-routing-contracts", "add-memory-evaluation-rail"],
  change "add-production-memory-policy-descriptors" core_role "policy-guidance" [
    "agent_harness_runtime.memory_controls.forget_update",
    "agent_harness_runtime.memory_conformance.failure_taxonomy",
    "agent_harness_runtime.memory_adapters.receipts",
  ] ["surface-memory-family-contract-index", "add-memory-routing-contracts", "add-memory-evaluation-rail"],
  change "add-memory-evaluation-rail" evaluation_role "evaluation" [
    "agent_harness_runtime.memory_adapters.verification",
    "agent_harness_runtime.memory_conformance.verification",
    "agent_harness_runtime.memory_controls.verification",
  ] ["surface-memory-family-contract-index", "enrich-memory-retrieval-quality-policy", "add-production-memory-policy-descriptors"],
  change "add-router-round-robin-selection" core_role "router-crate" [
    "agent_harness_runtime.model_router.crate_barrier",
    "agent_harness_runtime.model_router.execution_plan",
    "agent_harness_runtime.model_router.budget_fallback.chain",
    "agent_harness_runtime.model_router.policy_contracts",
  ] ["define-router-execution-plan", "gate-router-budget-and-fallback", "wire-router-into-runtime-loop"],
  change "adapt-goose-session-branching-controls" core_role "session-ledger" [
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.provenance_ledger.branch_queries",
    "agent_harness_runtime.provenance_ledger.receipts",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["adapt-goose-message-queue-interrupts", "adapt-goose-remote-server-admission"],
  change "adapt-goose-message-queue-interrupts" core_role "event-hooks" [
    "agent_harness_runtime.clankers_continuation_policy.state",
    "agent_harness_runtime.clankers_continuation_policy.receipts",
    "agent_harness_runtime.pi_headless_protocol.queueing",
    "agent_harness_runtime.host_control.events_waits",
  ] ["adapt-goose-session-branching-controls", "adapt-goose-remote-server-admission"],
  change "add-agent-control-pubsub" core_role "event-hooks" [
    "adapt-goose-message-queue-interrupts",
    "agent_harness_runtime.host_control.events_waits",
  ] ["add-coding-session-host-facade", "add-tui-agent-control-and-queue-dashboard", "extract-host-control-protocol"],
  change "adapt-goose-code-mode-tool-planning" core_role "policy-guidance" [
    "agent_harness_runtime.pi_tool_contracts.definition_metadata",
    "agent_harness_runtime.pi_tool_contracts.argument_validation",
    "agent_harness_runtime.pi_tool_contracts.output_bounds",
    "agent_harness_runtime.clankers_steel_policy.enforced",
    "agent_harness_runtime.hermes_plugin_manifests.executor_boundary",
  ] ["adapt-goose-message-queue-interrupts"],
  change "adapt-goose-remote-server-admission" host_role "ui-headless" [
    "agent_harness_runtime.clankers_remote_admission.requests",
    "agent_harness_runtime.clankers_remote_admission.host_owned",
    "agent_harness_runtime.clankers_remote_admission.receipts",
    "agent_harness_runtime.pi_headless_protocol.command_response",
    "agent_harness_runtime.host_control.transport_neutral",
  ] ["adapt-goose-session-branching-controls", "adapt-goose-message-queue-interrupts", "adapt-openclaw-gateway-control-plane"],
  change "adapt-hermes-done-review-lifecycle" core_role "ui-headless" [
    "agent_harness_runtime.hermes_review_reports.report",
    "agent_harness_runtime.hermes_review_reports.metrics_boundary",
    "agent_harness_runtime.claude_code_headless.completion",
    "agent_harness_runtime.clankers_injected_services.provider_boundary",
    "agent_harness_runtime.clankers_safe_receipts.rejects_leaks",
  ] ["adapt-hermes-corrective-memory-metrics", "adapt-hermes-scoped-validation-planner", "adapt-hermes-roi-recommendation-contracts"],
  change "adapt-hermes-corrective-memory-metrics" core_role "policy-guidance" [
    "agent_harness_runtime.hermes_review_reports.promotions",
    "agent_harness_runtime.qualifier_quality.envelope",
    "agent_harness_runtime.qualifier_quality.lifecycle",
    "agent_harness_runtime.memory_controls.validation",
  ] ["adapt-hermes-done-review-lifecycle", "adapt-hermes-roi-recommendation-contracts"],
  change "adapt-hermes-multi-lens-focus-decks" core_role "policy-guidance" [
    "agent_harness_runtime.claude_code_agents.task_contract",
    "agent_harness_runtime.claude_code_agents.context_packet",
    "agent_harness_runtime.hermes_skill_manifest.descriptor",
    "agent_harness_runtime.hermes_retrieval_budgets.descriptor",
  ] ["adapt-hermes-roi-recommendation-contracts"],
  change "adapt-hermes-scoped-validation-planner" core_role "concrete-adapters" [
    "agent_harness_runtime.clankers_process_job_profiles.resolution",
    "agent_harness_runtime.clankers_continuation_policy.state",
    "agent_harness_runtime.claude_code_headless.completion",
    "agent_harness_runtime.pi_tool_contracts.failure_classes",
  ] ["adapt-hermes-done-review-lifecycle", "adapt-hermes-plan-loop-mode-fsm"],
  change "adapt-hermes-plan-loop-mode-fsm" core_role "policy-guidance" [
    "agent_harness_runtime.zerostack_prompt_profiles.descriptors",
    "agent_harness_runtime.zerostack_prompt_profiles.directives",
    "agent_harness_runtime.clankers_continuation_policy.state",
    "agent_harness_runtime.hermes_policy_profiles.enforcement_boundary",
  ] ["adapt-hermes-scoped-validation-planner", "adapt-hermes-roi-recommendation-contracts"],
  change "adapt-hermes-roi-recommendation-contracts" core_role "ui-headless" [
    "agent_harness_runtime.claude_code_commands.outputs",
    "agent_harness_runtime.hermes_policy_profiles.prompt_hygiene",
    "agent_harness_runtime.hermes_review_reports.report",
    "agent_harness_runtime.cairn_policy.default_validation",
  ] ["adapt-hermes-done-review-lifecycle", "adapt-hermes-corrective-memory-metrics", "adapt-hermes-multi-lens-focus-decks"],
  change "adapt-hermes-host-asset-reconciliation" host_role "concrete-adapters" [
    "agent_harness_runtime.hermes_skill_manifest.layout_boundary",
    "agent_harness_runtime.hermes_plugin_manifests.executor_boundary",
    "agent_harness_runtime.pi_resources.host_discovery",
    "agent_harness_runtime.pi_resources.project_trust",
  ] ["adapt-hermes-multi-lens-focus-decks"],
  change "adapt-hermes-vcs-snapshot-ledger" core_role "session-ledger" [
    "agent_harness_runtime.clankers_checkpoint_rollback.dto",
    "agent_harness_runtime.clankers_checkpoint_rollback.service",
    "agent_harness_runtime.provenance_ledger.branch_queries",
    "agent_harness_runtime.provenance_ledger.receipts",
  ] ["adapt-hermes-scoped-validation-planner"],
  change "refactor-adapter-crate-maintainability" core_role "concrete-adapters" [
    "agent_harness_runtime.validation",
    "agent_harness_runtime.clankers_focused_rails.closeout",
  ] ["harden-provider-stream-parser-rails", "add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-fake-backed-headless-dogfood-shell"],
  change "harden-provider-stream-parser-rails" host_role "concrete-adapters" [
    "refactor-adapter-crate-maintainability",
    "agent_harness_runtime.model_streaming.parser_entrypoint",
    "agent_harness_runtime.clankers_safe_receipts.rejects_leaks",
  ] ["add-concrete-provider-streaming-adapters", "add-provider-projection-conformance-fixtures", "add-concrete-provider-dispatch-adapter"],
  change "add-concrete-provider-dispatch-adapter" host_role "concrete-adapters" [
    "add-canonical-model-message-contracts",
    "negotiate-model-capability-matrix",
    "add-provider-projection-conformance-fixtures",
    "harden-provider-stream-parser-rails",
    "agent_harness_runtime.pi_provider_registry.metadata",
    "agent_harness_runtime.clankers_injected_services.provider_boundary",
  ] ["define-model-router-execution-service", "add-concrete-provider-streaming-adapters", "add-fake-backed-headless-dogfood-shell"],
  change "add-host-tool-executor-capability-packs" host_role "concrete-adapters" [
    "adapt-goose-code-mode-tool-planning",
    "agent_harness_runtime.pi_tool_contracts.definition_metadata",
    "agent_harness_runtime.pi_tool_contracts.argument_validation",
    "agent_harness_runtime.pi_tool_contracts.output_bounds",
    "agent_harness_runtime.hermes_plugin_manifests.executor_boundary",
  ] ["adapt-clankers-tool-gateway-delivery", "adapt-hermes-host-asset-reconciliation", "add-fake-backed-headless-dogfood-shell"],
  change "add-host-session-persistence-adapter" host_role "concrete-adapters" [
    "adapt-goose-session-branching-controls",
    "adapt-hermes-vcs-snapshot-ledger",
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.provenance_ledger.receipts",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["adapt-pi-session-tree-ledger", "adapt-session-restore-contract", "add-fake-backed-headless-dogfood-shell"],
  change "add-fake-backed-headless-dogfood-shell" product_role "ui-headless" [
    "define-dogfoodable-host-readiness-roadmap",
    "agent_harness_runtime.pi_headless_protocol.command_response",
    "agent_harness_runtime.pi_headless_protocol.event_stream",
    "agent_harness_runtime.clankers_injected_services.provider_boundary",
    "agent_harness_runtime.clankers_safe_receipts.rejects_leaks",
  ] ["add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-packaged-headless-dogfood-smoke"],
  change "add-packaged-headless-dogfood-smoke" product_role "ui-headless" [
    "add-fake-backed-headless-dogfood-shell",
    "agent_harness_runtime.dogfood_headless_shell.entrypoint",
    "agent_harness_runtime.dogfood_headless_shell.receipts",
    "agent_harness_runtime.dogfood_readiness.evidence_ladder",
  ] ["add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-current-head-release-evidence-index"],
  change "add-current-head-release-evidence-index" core_role "export-surfaces" [
    "agent_harness_runtime.clankers_evidence_index.current_head",
    "agent_harness_runtime.clankers_evidence_index.non_claims",
    "agent_harness_runtime.clankers_evidence_index.closeout",
    "agent_harness_runtime.packaged_headless_dogfood_smoke.verification",
  ] ["add-packaged-headless-dogfood-smoke"],
  change "adapt-turnstone-intent-validation" core_role "policy-guidance" [
    "agent_harness_runtime.pi_tool_contracts.definition_metadata",
    "agent_harness_runtime.pi_tool_contracts.argument_validation",
    "agent_harness_runtime.zerostack_permission_kernel.receipts",
    "agent_harness_runtime.clankers_confirmation_broker.requests",
  ] ["adapt-turnstone-output-guard", "add-turnstone-headless-eval-scorer", "add-host-tool-executor-capability-packs"],
  change "adapt-turnstone-output-guard" core_role "policy-guidance" [
    "agent_harness_runtime.pi_tool_contracts.output_bounds",
    "agent_harness_runtime.clankers_safe_receipts.rejects_leaks",
    "agent_harness_runtime.pi_event_lifecycle.mode_neutral",
  ] ["adapt-turnstone-intent-validation", "add-turnstone-headless-eval-scorer", "add-host-tool-executor-capability-packs"],
  change "add-turnstone-headless-eval-scorer" evaluation_role "evaluation" [
    "agent_harness_runtime.claude_code_headless.events",
    "agent_harness_runtime.pi_tool_contracts.failure_classes",
    "agent_harness_runtime.ornith_eval_feedback.plan",
    "agent_harness_runtime.ornith_eval_feedback.receipts",
  ] ["adapt-turnstone-intent-validation", "adapt-turnstone-output-guard", "add-fake-backed-headless-dogfood-shell"],
  change "adopt-cap-std-context-boundary" host_role "concrete-adapters" [
    "agent_harness_runtime.clankers_context_references.resolvers",
    "agent_harness_runtime.clankers_context_references.bounds",
    "agent_harness_runtime.zerostack_context_layering.discovery",
    "agent_harness_runtime.zerostack_context_layering.admission",
  ] ["adapt-hermes-host-asset-reconciliation", "add-host-session-persistence-adapter"],
  change "document-ingest-adapter-boundary" host_role "concrete-adapters" [
    "agent_harness_runtime.clankers_context_references.resolvers",
    "agent_harness_runtime.clankers_context_references.bounds",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["adopt-cap-std-context-boundary"],
  change "ml-evidence-adapter-boundary" host_role "concrete-adapters" [
    "agent_harness_runtime.model_router.execution_plan",
    "agent_harness_runtime.model_router_execution.service_contract",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["define-model-router-execution-service", "add-concrete-provider-dispatch-adapter"],
  change "add-component-catalog" foundation_role "roadmap-sequencing" [
    "refactor-adapter-crate-maintainability",
    "agent_harness_runtime.clankers_runtime_facade_inventory.schema",
    "agent_harness_runtime.clankers_focused_rails.closeout",
  ] ["split-adapter-family-crates", "split-runtime-kernel-feature-packs", "split-std-host-adapter-crates", "split-preserves-boundary-bridges", "add-tool-executor-kit", "add-ledger-session-adapter-seam"],
  change "split-adapter-family-crates" core_role "concrete-adapters" [
    "add-component-catalog",
    "refactor-adapter-crate-maintainability",
    "introduce-router-crate-barrier",
  ] ["split-preserves-boundary-bridges", "add-tool-executor-kit"],
  change "split-runtime-kernel-feature-packs" core_role "runtime-loop" [
    "add-component-catalog",
    "add-runtime-dag-contract-kit",
    "wire-router-into-runtime-loop",
  ] ["split-std-host-adapter-crates", "add-ledger-session-adapter-seam", "add-tool-executor-kit"],
  change "split-std-host-adapter-crates" host_role "concrete-adapters" [
    "add-component-catalog",
    "split-runtime-kernel-feature-packs",
    "agent_harness_runtime.std_runtime_adapters.adapter_families",
    "adopt-cap-std-context-boundary",
  ] ["add-tool-executor-kit", "add-ledger-session-adapter-seam"],
  change "split-preserves-boundary-bridges" core_role "receipt-provenance" [
    "add-component-catalog",
    "preserve-router-boundary-dtos",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["split-adapter-family-crates", "add-ledger-session-adapter-seam"],
  change "add-tool-executor-kit" host_role "concrete-adapters" [
    "add-component-catalog",
    "split-adapter-family-crates",
    "split-runtime-kernel-feature-packs",
    "adapt-goose-code-mode-tool-planning",
  ] ["add-host-tool-executor-capability-packs", "add-fake-backed-headless-dogfood-shell"],
  change "add-per-model-runtime-profiles" core_role "policy-guidance" [
    "agent_harness_runtime.model_capability_matrix.negotiation",
    "agent_harness_runtime.hermes_skill_manifest.progressive_disclosure",
    "agent_harness_runtime.openai_codex_opt_in_memories.policy_controls",
    "agent_harness_runtime.clankers_prompt_assembly.sources",
    "agent_harness_runtime.tool_executor_kit.contracts",
    "agent_harness_runtime.model_router_execution.service_contract",
  ] ["negotiate-model-capability-matrix", "define-model-router-execution-service", "add-tool-executor-kit", "add-host-tool-executor-capability-packs"],
  change "add-capability-bound-agent-roles" core_role "policy-guidance" [
    "add-per-model-runtime-profiles",
    "agent_harness_runtime.clankers_agent_definitions.capability_ceilings",
    "agent_harness_runtime.openai_codex_subagent_threads.role_profiles",
    "agent_harness_runtime.clash_ucan.capability_request",
    "agent_harness_runtime.tool_executor_kit.admission",
    "agent_harness_runtime.ornith_trust_boundary.judge_veto",
  ] ["add-per-model-runtime-profiles", "define-model-router-execution-service", "add-tool-executor-kit"],
  change "receipt-authoritative-agent-completion" core_role "receipt-provenance" [
    "agent_harness_runtime.clankers_safe_receipts.closeout",
    "agent_harness_runtime.clankers_continuation_policy.priority",
    "agent_harness_runtime.clankers_steel_policy.nickel",
  ] ["adapt-hermes-scoped-validation-planner", "adapt-clankers-controller-continuation-policy", "adapt-clankers-steel-runtime-policy", "add-current-head-release-evidence-index"],
  change "extract-host-control-protocol" optional_role "optional-references" [
    "agent_harness_runtime.host_control.transport_neutral",
    "agent_harness_runtime.host_control.verification",
  ] ["adapt-host-control-protocol", "split-adapter-family-crates", "add-ratatui-tool-host-surface"],
  change "add-sandlock-linux-host-adapter" host_role "concrete-adapters" [
    "agent_harness_runtime.sandlock_sandbox.platform",
    "agent_harness_runtime.sandlock_sandbox.fail_closed",
    "agent_harness_runtime.clash_ucan.capability_request",
  ] ["adapt-sandlock-sandbox-enforcement", "add-concrete-std-runtime-adapters", "split-std-host-adapter-crates"],
  change "add-bubblewrap-agent-guardrail-host" host_role "concrete-adapters" [
    "add-sandlock-linux-host-adapter",
    "split-std-host-adapter-crates",
    "agent_harness_runtime.sandlock_sandbox.platform",
    "agent_harness_runtime.clash_ucan.capability_request",
  ] ["bind-session-runtime-authority-profile"],
  change "add-branchfs-speculative-host-adapter" host_role "concrete-adapters" [
    "agent_harness_runtime.branchfs_workspace.adapter",
    "agent_harness_runtime.branchfs_workspace.fail_closed",
    "agent_harness_runtime.branchfs_workspace.receipts",
  ] ["adapt-branchfs-speculative-workspaces", "adapt-git-ai-vcs-neutral-provenance", "split-std-host-adapter-crates"],
  change "add-ledger-session-adapter-seam" core_role "session-ledger" [
    "add-component-catalog",
    "split-runtime-kernel-feature-packs",
    "adapt-goose-session-branching-controls",
    "adapt-hermes-vcs-snapshot-ledger",
  ] ["add-host-session-persistence-adapter", "add-fake-backed-headless-dogfood-shell"],
  change "define-spacewasm-embedded-compatibility-profile" host_role "concrete-adapters" [
    "agent_harness_runtime.wasm_compile.surface",
    "agent_harness_runtime.wasm_compile.shell_isolation",
    "agent_harness_runtime.component_catalog.source",
  ] ["add-component-catalog", "split-runtime-kernel-feature-packs"],
  change "adapt-tau-interrupted-tool-call-repair" core_role "session-ledger" [
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.pi_tool_contracts.failure_classes",
    "agent_harness_runtime.session_persistence_adapter.failures",
  ] ["add-durable-session-store-host", "add-coding-session-host-facade", "add-ledger-session-adapter-seam"],
  change "add-durable-session-store-host" host_role "concrete-adapters" [
    "adapt-tau-interrupted-tool-call-repair",
    "add-ledger-session-adapter-seam",
    "agent_harness_runtime.session_persistence_adapter.service",
    "agent_harness_runtime.session_persistence_adapter.admission",
  ] ["add-host-session-persistence-adapter", "add-coding-session-host-facade", "add-redacted-session-tree-export"],
  change "add-coding-session-host-facade" product_role "ui-headless" [
    "adapt-tau-interrupted-tool-call-repair",
    "add-durable-session-store-host",
    "agent_harness_runtime.pi_headless_protocol.command_response",
    "agent_harness_runtime.pi_headless_protocol.queueing",
    "agent_harness_runtime.pi_event_lifecycle.mode_neutral",
  ] ["add-fake-backed-headless-dogfood-shell", "add-tui-app-shell-and-event-loop", "add-redacted-session-tree-export", "bind-session-runtime-authority-profile", "add-acp-stdio-host-adapter"],
  change "bind-session-runtime-authority-profile" core_role "ui-headless" [
    "add-coding-session-host-facade",
    "add-durable-session-store-host",
    "add-per-model-runtime-profiles",
    "add-sandlock-linux-host-adapter",
  ] ["add-acp-stdio-host-adapter", "add-codebase-graph-host-adapter", "add-redacted-session-tree-export"],
  change "add-codebase-graph-host-adapter" host_role "ui-headless" [
    "add-host-tool-executor-capability-packs",
    "adopt-cap-std-context-boundary",
    "bind-session-runtime-authority-profile",
  ] ["add-coding-session-host-facade", "document-ingest-adapter-boundary"],
  change "add-acp-stdio-host-adapter" product_role "ui-headless" [
    "add-coding-session-host-facade",
    "bind-session-runtime-authority-profile",
    "agent_harness_runtime.goose_acp_mcp_interop.verification",
  ] ["extract-host-control-protocol", "add-tui-app-shell-and-event-loop"],
  change "adapt-tau-provider-retry-execution" host_role "concrete-adapters" [
    "add-concrete-provider-dispatch-adapter",
    "agent_harness_runtime.provider_dispatch_adapter.plan",
    "agent_harness_runtime.model_streaming.adapter_contract",
    "agent_harness_runtime.model_runtime_profiles.dispatch_binding.retry",
  ] ["add-concrete-provider-streaming-adapters", "add-coding-session-host-facade"],
  change "add-redacted-session-tree-export" product_role "export-surfaces" [
    "add-durable-session-store-host",
    "add-coding-session-host-facade",
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.hf_dataset_export.policy",
  ] ["add-current-head-release-evidence-index", "add-host-session-persistence-adapter"],
  change "add-aeneas-proof-campaign-recipe" product_role "ui-headless" [
    "add-capability-bound-agent-roles",
    "add-per-model-runtime-profiles",
    "add-tool-executor-kit",
    "receipt-authoritative-agent-completion",
  ] ["adapt-hermes-scoped-validation-planner", "receipt-authoritative-agent-completion"],
  change "add-lean4-agent-loop-fabric" optional_role "optional-references" [
    "agent_harness_runtime.composability.loop_fabric_trait",
    "agent_harness_runtime.loop.contract",
    "agent_harness_runtime.clankers_safe_receipts.closeout",
  ] ["add-aeneas-proof-campaign-recipe", "add-acp-stdio-host-adapter", "split-std-host-adapter-crates"],
  change "add-mesh-llm-provider-adapter" host_role "concrete-adapters" [
    "add-canonical-model-message-contracts",
    "define-model-router-execution-service",
    "add-concrete-provider-streaming-adapters",
    "add-per-model-runtime-profiles",
  ] ["adapt-tau-provider-retry-execution", "negotiate-model-capability-matrix"],
  change "adopt-standalone-nickel-export-shell" optional_role "optional-references" [
    "agent_harness_runtime.nickel_policy_contracts.projection_freshness",
    "agent_harness_runtime.nickel_policy_contracts.authority_boundary",
  ] ["add-per-model-runtime-profiles", "add-capability-bound-agent-roles"],
  change "add-trajectory-transcript-adapter" host_role "concrete-adapters" [
    "agent_harness_runtime.clankers_process_job_profiles.resolution",
    "agent_harness_runtime.clankers_batch_runner.output",
    "agent_harness_runtime.provenance_ledger.session_ledger",
    "agent_harness_runtime.hf_dataset_export.session_source",
  ] ["adapt-clankers-batch-trajectory-runner", "add-redacted-session-tree-export"],
  change "add-canonical-conversation-view-projection" core_role "session-ledger" [
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.pi_event_lifecycle.mode_neutral",
  ] ["add-tui-conversation-render-model", "add-durable-active-turn-recovery", "add-redacted-session-tree-export"],
  change "add-durable-active-turn-recovery" host_role "concrete-adapters" [
    "add-durable-session-store-host",
    "adapt-tau-interrupted-tool-call-repair",
    "adapt-tau-provider-retry-execution",
  ] ["add-agent-control-pubsub", "add-coding-session-host-facade", "add-canonical-conversation-view-projection"],
  change "add-secretspec-sops-host-resolution" host_role "concrete-adapters" [
    "evaluate-bounded-execution-contract",
    "split-std-host-adapter-crates",
  ] ["bind-session-runtime-authority-profile", "add-sandlock-linux-host-adapter"],
  change "evaluate-bounded-execution-contract" evaluation_role "evaluation" [
    "agent_harness_runtime.clankers_process_job_profiles.resolution",
    "agent_harness_runtime.sandlock_sandbox.platform",
    "agent_harness_runtime.nickel_policy_contracts.authority_boundary",
  ] ["add-secretspec-sops-host-resolution", "add-sandlock-linux-host-adapter"],
  change "add-harbor-atif-json-adapter" host_role "export-surfaces" [
    "add-trajectory-transcript-adapter",
    "agent_harness_runtime.clankers_batch_runner.output",
    "add-redacted-session-tree-export",
  ] ["add-ledger-session-adapter-seam", "add-current-head-release-evidence-index"],
  change "add-skill-source-lifecycle-contracts" core_role "policy-guidance" [
    "agent_harness_runtime.hermes_skill_manifest.descriptor",
    "agent_harness_runtime.hermes_skill_manifest.layout_boundary",
  ] ["adapt-hermes-host-asset-reconciliation", "add-per-model-runtime-profiles"],
  change "pilot-application-host-binding" product_role "ui-headless" [
    "bind-session-runtime-authority-profile",
    "add-fake-backed-headless-dogfood-shell",
  ] ["add-acp-stdio-host-adapter", "add-codebase-graph-host-adapter"],
  change "type-acp-session-identities" product_role "ui-headless" [
    "add-acp-stdio-host-adapter",
    "bind-session-runtime-authority-profile",
  ] ["extract-host-control-protocol", "pilot-application-host-binding"],
  change "add-native-evaluation-core" core_role "runtime-loop" [
    "agent_harness_runtime.harbor_eval.planning",
    "agent_harness_runtime.clankers_batch_runner.resume",
    "agent_harness_runtime.pi_session_tree.ledger_contract",
    "agent_harness_runtime.agent_role_profiles.separation",
  ] ["add-evaluation-adapter-family", "add-std-evaluation-host", "add-evaluation-composition-conformance", "add-harbor-atif-json-adapter"],
  change "add-evaluation-adapter-family" host_role "injected-services" [
    "add-native-evaluation-core",
    "agent_harness_runtime.harbor_eval.execution_ports",
    "agent_harness_runtime.model_streaming.adapter_contract",
  ] ["add-std-evaluation-host", "add-evaluation-composition-conformance", "add-harbor-atif-json-adapter"],
  change "add-std-evaluation-host" host_role "concrete-adapters" [
    "add-native-evaluation-core",
    "add-evaluation-adapter-family",
    "add-durable-active-turn-recovery",
    "add-coding-session-host-facade",
  ] ["add-evaluation-composition-conformance", "add-concrete-provider-streaming-adapters", "add-harbor-atif-json-adapter"],
  change "add-evaluation-composition-conformance" evaluation_role "evaluation" [
    "add-native-evaluation-core",
    "add-evaluation-adapter-family",
    "add-std-evaluation-host",
    "agent_harness_runtime.composability.conformance_matrix",
  ] ["add-provider-projection-conformance-fixtures", "add-harbor-atif-json-adapter", "add-durable-active-turn-recovery"],
]
in

let overlap_entries = [
  overlap "canonical-request-provider-projection" "add-canonical-model-message-contracts" ["add-provider-projection-conformance-fixtures"],
  overlap "capability-negotiation-provider-claims" "negotiate-model-capability-matrix" ["add-provider-projection-conformance-fixtures"],
  overlap "tui-render-model" "add-tui-conversation-render-model" ["add-tui-streaming-markdown-view", "add-tui-prompt-input-and-keymap", "add-tui-tool-call-and-approval-panels", "add-tui-session-context-status-rail", "add-tui-agent-control-and-queue-dashboard", "add-tui-app-shell-and-event-loop", "adopt-latest-rats-image-protocol-adapters", "add-bounded-terminal-video-playback"],
  overlap "terminal-image-protocols" "adopt-latest-rats-image-protocol-adapters" ["add-tui-streaming-markdown-view", "add-tui-app-shell-and-event-loop", "add-bounded-terminal-video-playback"],
  overlap "provider-video-payloads" "add-provider-video-payload-contracts" ["add-canonical-model-message-contracts", "negotiate-model-capability-matrix", "adopt-latest-rats-image-protocol-adapters", "add-bounded-terminal-video-playback"],
  overlap "terminal-video-playback" "add-bounded-terminal-video-playback" ["adopt-latest-rats-image-protocol-adapters", "add-provider-video-payload-contracts", "add-tui-app-shell-and-event-loop"],
  overlap "router-execution-plan-materialization" "define-router-execution-plan" ["wire-router-into-runtime-loop", "gate-router-budget-and-fallback"],
  overlap "router-signal-projection" "project-router-request-signals" ["add-supra-router-signal-adapter"],
  overlap "router-receipt-assembly" "emit-router-safe-receipts" ["wire-router-into-runtime-loop", "evaluate-router-recipe-coverage"],
  overlap "router-execution-service" "define-model-router-execution-service" ["wire-router-into-runtime-loop", "add-concrete-provider-streaming-adapters"],
  overlap "provider-streaming-claims" "add-concrete-provider-streaming-adapters" ["add-provider-projection-conformance-fixtures", "negotiate-model-capability-matrix"],
  overlap "openclaw-gateway-control-plane" "adapt-openclaw-gateway-control-plane" ["adapt-openclaw-node-capability-hosts", "adapt-openclaw-session-isolation-tools", "adapt-openclaw-security-plugin-guardrails"],
  overlap "openclaw-node-invocation" "adapt-openclaw-node-capability-hosts" ["adapt-openclaw-gateway-control-plane", "adapt-openclaw-security-plugin-guardrails"],
  overlap "openclaw-session-tool-visibility" "adapt-openclaw-session-isolation-tools" ["adapt-openclaw-security-plugin-guardrails"],
  overlap "nested-subagent-delegation-graphs" "add-nested-subagent-delegation-graphs" ["adapt-openclaw-session-isolation-tools", "adapt-hermes-multi-lens-focus-decks", "adapt-hermes-plan-loop-mode-fsm", "add-runtime-dag-contract-kit"],
  overlap "runtime-dag-contract-kit" "add-runtime-dag-contract-kit" ["extend-recipe-workflows-with-dags", "add-researchspec-promotion-dags", "add-roadmap-dag-explanations", "add-nested-subagent-delegation-graphs", "adapt-goose-code-mode-tool-planning"],
  overlap "recipe-workflow-dags" "extend-recipe-workflows-with-dags" ["add-runtime-dag-contract-kit", "adapt-goose-code-mode-tool-planning"],
  overlap "researchspec-promotion-dags" "add-researchspec-promotion-dags" ["add-runtime-dag-contract-kit", "adapt-hermes-scoped-validation-planner", "adapt-hermes-vcs-snapshot-ledger"],
  overlap "roadmap-dag-explanations" "add-roadmap-dag-explanations" ["add-runtime-dag-contract-kit", "define-dogfoodable-host-readiness-roadmap"],
  overlap "openclaw-security-plugin-guardrails" "adapt-openclaw-security-plugin-guardrails" ["adapt-openclaw-gateway-control-plane", "adapt-openclaw-node-capability-hosts", "adapt-openclaw-session-isolation-tools"],
  overlap "memory-family-contracts" "surface-memory-family-contract-index" ["add-cognitive-memory-taxonomy", "add-memory-routing-contracts", "enrich-memory-retrieval-quality-policy", "add-production-memory-policy-descriptors", "add-memory-evaluation-rail"],
  overlap "memory-routing-quality" "add-memory-routing-contracts" ["add-cognitive-memory-taxonomy", "enrich-memory-retrieval-quality-policy", "add-production-memory-policy-descriptors", "add-memory-evaluation-rail"],
  overlap "router-round-robin-selection" "add-router-round-robin-selection" ["define-router-execution-plan", "gate-router-budget-and-fallback", "wire-router-into-runtime-loop"],
  overlap "goose-session-branching-controls" "adapt-goose-session-branching-controls" ["adapt-goose-message-queue-interrupts", "adapt-goose-remote-server-admission"],
  overlap "goose-queued-control-events" "adapt-goose-message-queue-interrupts" ["adapt-goose-session-branching-controls", "adapt-goose-remote-server-admission", "add-agent-control-pubsub"],
  overlap "live-agent-control-pause-resume" "add-agent-control-pubsub" ["adapt-goose-message-queue-interrupts", "add-coding-session-host-facade", "add-tui-agent-control-and-queue-dashboard", "extract-host-control-protocol"],
  overlap "goose-typed-code-mode-tool-planning" "adapt-goose-code-mode-tool-planning" ["adapt-goose-message-queue-interrupts"],
  overlap "goose-remote-backend-admission" "adapt-goose-remote-server-admission" ["adapt-goose-session-branching-controls", "adapt-goose-message-queue-interrupts", "adapt-openclaw-gateway-control-plane"],
  overlap "hermes-review-quality-loop" "adapt-hermes-done-review-lifecycle" ["adapt-hermes-corrective-memory-metrics", "adapt-hermes-scoped-validation-planner", "adapt-hermes-roi-recommendation-contracts"],
  overlap "hermes-corrective-memory-promotion" "adapt-hermes-corrective-memory-metrics" ["adapt-hermes-done-review-lifecycle", "adapt-hermes-roi-recommendation-contracts"],
  overlap "hermes-analysis-focus-decks" "adapt-hermes-multi-lens-focus-decks" ["adapt-hermes-roi-recommendation-contracts"],
  overlap "hermes-scoped-validation-followups" "adapt-hermes-scoped-validation-planner" ["adapt-hermes-done-review-lifecycle", "adapt-hermes-plan-loop-mode-fsm"],
  overlap "hermes-plan-loop-continuations" "adapt-hermes-plan-loop-mode-fsm" ["adapt-hermes-scoped-validation-planner", "adapt-hermes-roi-recommendation-contracts"],
  overlap "hermes-roi-action-routing" "adapt-hermes-roi-recommendation-contracts" ["adapt-hermes-done-review-lifecycle", "adapt-hermes-corrective-memory-metrics", "adapt-hermes-multi-lens-focus-decks"],
  overlap "hermes-host-asset-reconciliation" "adapt-hermes-host-asset-reconciliation" ["adapt-hermes-multi-lens-focus-decks"],
  overlap "hermes-vcs-snapshot-ledger" "adapt-hermes-vcs-snapshot-ledger" ["adapt-hermes-scoped-validation-planner"],
  overlap "adapter-maintainability-hotspots" "refactor-adapter-crate-maintainability" ["harden-provider-stream-parser-rails", "add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-fake-backed-headless-dogfood-shell"],
  overlap "provider-stream-parser-rails" "harden-provider-stream-parser-rails" ["add-concrete-provider-streaming-adapters", "add-provider-projection-conformance-fixtures", "add-concrete-provider-dispatch-adapter"],
  overlap "concrete-provider-dispatch" "add-concrete-provider-dispatch-adapter" ["define-model-router-execution-service", "add-concrete-provider-streaming-adapters", "add-fake-backed-headless-dogfood-shell"],
  overlap "tool-executor-capability-packs" "add-host-tool-executor-capability-packs" ["adapt-goose-code-mode-tool-planning", "adapt-hermes-host-asset-reconciliation", "add-fake-backed-headless-dogfood-shell"],
  overlap "host-session-persistence-adapter" "add-host-session-persistence-adapter" ["adapt-goose-session-branching-controls", "adapt-hermes-vcs-snapshot-ledger", "add-fake-backed-headless-dogfood-shell"],
  overlap "fake-backed-headless-dogfood-shell" "add-fake-backed-headless-dogfood-shell" ["add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-packaged-headless-dogfood-smoke"],
  overlap "packaged-headless-dogfood-smoke" "add-packaged-headless-dogfood-smoke" ["add-fake-backed-headless-dogfood-shell", "add-concrete-provider-dispatch-adapter", "add-host-tool-executor-capability-packs", "add-host-session-persistence-adapter", "add-current-head-release-evidence-index"],
  overlap "current-head-release-evidence-index" "add-current-head-release-evidence-index" ["add-packaged-headless-dogfood-smoke"],
  overlap "turnstone-intent-validation" "adapt-turnstone-intent-validation" ["adapt-turnstone-output-guard", "add-turnstone-headless-eval-scorer", "add-host-tool-executor-capability-packs"],
  overlap "turnstone-output-guard" "adapt-turnstone-output-guard" ["adapt-turnstone-intent-validation", "add-turnstone-headless-eval-scorer", "add-host-tool-executor-capability-packs"],
  overlap "turnstone-headless-eval-scorer" "add-turnstone-headless-eval-scorer" ["adapt-turnstone-intent-validation", "adapt-turnstone-output-guard", "add-fake-backed-headless-dogfood-shell"],
  overlap "cap-std-context-boundary" "adopt-cap-std-context-boundary" ["adapt-hermes-host-asset-reconciliation", "add-host-session-persistence-adapter", "document-ingest-adapter-boundary"],
  overlap "document-ingest-adapter-boundary" "document-ingest-adapter-boundary" ["adopt-cap-std-context-boundary"],
  overlap "ml-evidence-adapter-boundary" "ml-evidence-adapter-boundary" ["define-model-router-execution-service", "add-concrete-provider-dispatch-adapter"],
  overlap "per-model-runtime-context-composition" "add-per-model-runtime-profiles" ["negotiate-model-capability-matrix", "define-model-router-execution-service", "add-tool-executor-kit", "add-host-tool-executor-capability-packs"],
  overlap "capability-bound-semantic-agent-roles" "add-capability-bound-agent-roles" ["add-per-model-runtime-profiles", "define-model-router-execution-service", "add-tool-executor-kit"],
  overlap "receipt-authoritative-completion" "receipt-authoritative-agent-completion" ["adapt-hermes-scoped-validation-planner", "add-current-head-release-evidence-index"],
  overlap "standalone-host-control-protocol" "extract-host-control-protocol" ["split-adapter-family-crates"],
  overlap "sandlock-linux-host-enforcement" "add-sandlock-linux-host-adapter" ["split-std-host-adapter-crates"],
  overlap "bubblewrap-agent-guardrail-host" "add-bubblewrap-agent-guardrail-host" ["add-sandlock-linux-host-adapter", "split-std-host-adapter-crates", "bind-session-runtime-authority-profile"],
  overlap "branchfs-speculative-host-workspaces" "add-branchfs-speculative-host-adapter" ["split-std-host-adapter-crates"],
  overlap "component-catalog-lego-map" "add-component-catalog" ["split-adapter-family-crates", "split-runtime-kernel-feature-packs", "split-std-host-adapter-crates", "split-preserves-boundary-bridges", "add-tool-executor-kit", "add-ledger-session-adapter-seam"],
  overlap "adapter-family-crate-split" "split-adapter-family-crates" ["add-component-catalog", "split-preserves-boundary-bridges", "add-tool-executor-kit"],
  overlap "runtime-kernel-feature-packs" "split-runtime-kernel-feature-packs" ["add-component-catalog", "split-std-host-adapter-crates", "add-ledger-session-adapter-seam", "add-tool-executor-kit"],
  overlap "std-host-adapter-authority-split" "split-std-host-adapter-crates" ["add-component-catalog", "split-runtime-kernel-feature-packs", "add-tool-executor-kit", "add-ledger-session-adapter-seam"],
  overlap "preserves-bridge-family-split" "split-preserves-boundary-bridges" ["add-component-catalog", "split-adapter-family-crates", "add-ledger-session-adapter-seam"],
  overlap "tool-executor-kit" "add-tool-executor-kit" ["split-adapter-family-crates", "split-runtime-kernel-feature-packs", "add-host-tool-executor-capability-packs"],
  overlap "ledger-session-adapter-seam" "add-ledger-session-adapter-seam" ["split-runtime-kernel-feature-packs", "split-std-host-adapter-crates", "add-host-session-persistence-adapter"],
  overlap "spacewasm-diagnostic-host-and-wasm-surface" "define-spacewasm-embedded-compatibility-profile" ["add-component-catalog", "split-runtime-kernel-feature-packs"],
  overlap "interrupted-tool-call-transcript-repair" "adapt-tau-interrupted-tool-call-repair" ["add-durable-session-store-host", "add-coding-session-host-facade", "add-ledger-session-adapter-seam"],
  overlap "durable-session-store-host" "add-durable-session-store-host" ["add-host-session-persistence-adapter", "add-ledger-session-adapter-seam", "add-coding-session-host-facade", "add-redacted-session-tree-export"],
  overlap "coding-session-host-facade" "add-coding-session-host-facade" ["add-fake-backed-headless-dogfood-shell", "add-tui-app-shell-and-event-loop", "add-durable-session-store-host", "bind-session-runtime-authority-profile", "add-acp-stdio-host-adapter"],
  overlap "session-runtime-authority-profile" "bind-session-runtime-authority-profile" ["add-durable-session-store-host", "add-coding-session-host-facade", "add-per-model-runtime-profiles", "add-sandlock-linux-host-adapter", "add-codebase-graph-host-adapter", "add-acp-stdio-host-adapter"],
  overlap "codebase-graph-retrieval" "add-codebase-graph-host-adapter" ["adopt-cap-std-context-boundary", "add-host-tool-executor-capability-packs", "bind-session-runtime-authority-profile"],
  overlap "acp-stdio-host" "add-acp-stdio-host-adapter" ["add-coding-session-host-facade", "bind-session-runtime-authority-profile", "extract-host-control-protocol", "add-tui-app-shell-and-event-loop"],
  overlap "provider-retry-execution" "adapt-tau-provider-retry-execution" ["add-concrete-provider-dispatch-adapter", "add-concrete-provider-streaming-adapters"],
  overlap "redacted-session-tree-export" "add-redacted-session-tree-export" ["add-durable-session-store-host", "add-coding-session-host-facade", "add-current-head-release-evidence-index"],
  overlap "aeneas-proof-campaign-recipe" "add-aeneas-proof-campaign-recipe" ["add-capability-bound-agent-roles", "add-per-model-runtime-profiles", "add-tool-executor-kit", "receipt-authoritative-agent-completion"],
  overlap "lean4-agent-loop-fabric" "add-lean4-agent-loop-fabric" ["add-aeneas-proof-campaign-recipe", "add-acp-stdio-host-adapter", "split-std-host-adapter-crates"],
  overlap "mesh-llm-provider-adapter" "add-mesh-llm-provider-adapter" ["add-canonical-model-message-contracts", "define-model-router-execution-service", "add-concrete-provider-streaming-adapters", "adapt-tau-provider-retry-execution"],
  overlap "standalone-nickel-export-shell" "adopt-standalone-nickel-export-shell" ["add-per-model-runtime-profiles", "add-capability-bound-agent-roles"],
  overlap "trajectory-transcript-normalization" "add-trajectory-transcript-adapter" ["add-ledger-session-adapter-seam", "add-redacted-session-tree-export"],
  overlap "canonical-conversation-view-projection" "add-canonical-conversation-view-projection" ["add-tui-conversation-render-model", "add-durable-active-turn-recovery", "add-redacted-session-tree-export"],
  overlap "durable-active-turn-recovery" "add-durable-active-turn-recovery" ["add-durable-session-store-host", "adapt-tau-interrupted-tool-call-repair", "add-agent-control-pubsub"],
  overlap "secretspec-sops-host-resolution" "add-secretspec-sops-host-resolution" ["evaluate-bounded-execution-contract", "add-sandlock-linux-host-adapter", "bind-session-runtime-authority-profile"],
  overlap "bounded-execution-contract-evaluation" "evaluate-bounded-execution-contract" ["add-sandlock-linux-host-adapter", "add-secretspec-sops-host-resolution"],
  overlap "harbor-atif-json-interchange" "add-harbor-atif-json-adapter" ["add-trajectory-transcript-adapter", "add-redacted-session-tree-export"],
  overlap "skill-source-lifecycle" "add-skill-source-lifecycle-contracts" ["adapt-hermes-host-asset-reconciliation", "add-per-model-runtime-profiles"],
  overlap "application-host-binding-pilot" "pilot-application-host-binding" ["bind-session-runtime-authority-profile", "add-acp-stdio-host-adapter", "add-codebase-graph-host-adapter"],
  overlap "typed-acp-session-identities" "type-acp-session-identities" ["add-acp-stdio-host-adapter", "extract-host-control-protocol", "pilot-application-host-binding"],
  overlap "native-evaluation-core" "add-native-evaluation-core" ["add-evaluation-adapter-family", "add-std-evaluation-host", "add-evaluation-composition-conformance", "add-harbor-atif-json-adapter"],
  overlap "native-evaluation-service-ports" "add-evaluation-adapter-family" ["add-native-evaluation-core", "add-std-evaluation-host", "add-harbor-atif-json-adapter"],
  overlap "native-evaluation-host" "add-std-evaluation-host" ["add-evaluation-adapter-family", "add-durable-active-turn-recovery", "add-concrete-provider-streaming-adapters", "add-harbor-atif-json-adapter"],
  overlap "native-evaluation-conformance" "add-evaluation-composition-conformance" ["add-native-evaluation-core", "add-evaluation-adapter-family", "add-std-evaluation-host", "add-provider-projection-conformance-fixtures", "add-harbor-atif-json-adapter"],
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
