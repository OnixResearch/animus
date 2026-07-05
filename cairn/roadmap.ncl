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
]
in

let overlap_entries = [
  overlap "canonical-request-provider-projection" "add-canonical-model-message-contracts" ["add-provider-projection-conformance-fixtures"],
  overlap "capability-negotiation-provider-claims" "negotiate-model-capability-matrix" ["add-provider-projection-conformance-fixtures"],
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
