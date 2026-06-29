let InventoryRecord = {
  export_path | String,
  owning_crate | String,
  source | String,
  class | String,
  side_effect | String,
  stability | String,
  review_note | String,
}
in
let entry = fun export_path_value owning_crate_value source_value class_value side_effect_value stability_value review_note_value =>
  {
    export_path = export_path_value,
    owning_crate = owning_crate_value,
    source = source_value,
    class = class_value,
    side_effect = side_effect_value,
    stability = stability_value,
    review_note = review_note_value,
  }
in
let line = fun record => "entry|%{record.export_path}|%{record.owning_crate}|%{record.source}|%{record.class}|%{record.side_effect}|%{record.stability}|%{record.review_note}"
in
let reusable = "reusable-contract" in
let injection = "host-injection-surface" in
let shell = "shell-adapter" in
let none = "none" in
let injected = "injected-only" in
let shell_owned = "shell-owned" in
let stable = "stable" in
let optional_stability = "optional" in
let root = "src/lib.rs" in
let core = "crates/animus-core/src/lib.rs" in
let adapters = "crates/animus-adapters/src/lib.rs" in
let preserves = "crates/animus-preserves/src/lib.rs" in
let runtime = "crates/animus-runtime/src/lib.rs" in
let std_shell = "crates/animus-std/src/lib.rs" in
let steel = "crates/animus-steel/src/lib.rs" in
let tui = "crates/animus-tui/src/lib.rs" in

let entry_values = [
  entry "animus::AdapterContractError" "animus" root reusable none stable "root adapter validation error reexport",
  entry "animus::ComponentAdapter" "animus" root injection injected stable "root host-supplied component port reexport",
  entry "animus::MAX_ADAPTER_FIELD_CHARS" "animus" root reusable none stable "root adapter limit reexport",
  entry "animus::MIN_ADAPTER_FIELD_CHARS" "animus" root reusable none stable "root adapter limit reexport",
  entry "animus::ModelAdapter" "animus" root injection injected stable "root model port reexport",
  entry "animus::ModelRequest" "animus" root reusable none stable "root model request DTO reexport",
  entry "animus::ModelResponse" "animus" root reusable none stable "root model response DTO reexport",
  entry "animus::ToolAdapter" "animus" root injection injected stable "root tool port reexport",
  entry "animus::ToolRequest" "animus" root reusable none stable "root tool request DTO reexport",
  entry "animus::ToolResponse" "animus" root reusable none stable "root tool response DTO reexport",
  entry "animus::AnimusState" "animus" root reusable none stable "root core state reexport",
  entry "animus::CoreError" "animus" root reusable none stable "root core error reexport",
  entry "animus::Intent" "animus" root reusable none stable "root validated intent reexport",
  entry "animus::MAX_INTENT_CHARS" "animus" root reusable none stable "root intent limit reexport",
  entry "animus::MIN_INTENT_CHARS" "animus" root reusable none stable "root intent limit reexport",
  entry "animus::ConfigError" "animus" root shell shell_owned stable "root std shell config error reexport",
  entry "animus::INTENT_ENV_KEY" "animus" root shell shell_owned stable "root std shell setting key reexport",
  entry "animus::RunError" "animus" root shell shell_owned stable "root std shell run error reexport",
  entry "animus::RunReport" "animus" root shell shell_owned stable "root std shell report reexport",
  entry "animus::RuntimeConfig" "animus" root shell shell_owned stable "root std shell config reexport",
  entry "animus::SummaryModelAdapter" "animus" root shell shell_owned stable "root std shell default adapter reexport",
  entry "animus::load_runtime" "animus" root shell shell_owned stable "root std shell loader reexport",
  entry "animus::run" "animus" root shell shell_owned stable "root std shell runner reexport",
  entry "animus::run_with_model" "animus" root shell shell_owned stable "root std shell injected runner reexport",
  entry "animus::basalt" "animus" root reusable none optional_stability "optional Basalt reference reexport",
  entry "animus::cairn_core" "animus" root reusable none optional_stability "optional Cairn reference reexport",
  entry "animus::trellis" "animus" root reusable none optional_stability "optional Trellis reference reexport",
  entry "animus::animus_preserves" "animus" root reusable none optional_stability "optional Preserves bridge reexport",
  entry "animus::animus_runtime" "animus" root reusable none optional_stability "optional runtime crate reexport",
  entry "animus::animus_steel" "animus" root reusable none optional_stability "optional Steel fabric reexport",
  entry "animus::animus_tui" "animus" root shell shell_owned optional_stability "optional terminal shell reexport",

  entry "animus-core::MIN_INTENT_CHARS" "animus-core" core reusable none stable "intent lower bound",
  entry "animus-core::MAX_INTENT_CHARS" "animus-core" core reusable none stable "intent upper bound",
  entry "animus-core::Intent" "animus-core" core reusable none stable "validated intent DTO",
  entry "animus-core::AnimusState" "animus-core" core reusable none stable "deterministic state DTO",
  entry "animus-core::CoreError" "animus-core" core reusable none stable "core validation error",

  entry "animus-adapters::AdapterContractError" "animus-adapters" adapters reusable none stable "adapter boundary validation error",
  entry "animus-adapters::MAX_ADAPTER_FIELD_CHARS" "animus-adapters" adapters reusable none stable "adapter field upper bound",
  entry "animus-adapters::MIN_ADAPTER_FIELD_CHARS" "animus-adapters" adapters reusable none stable "adapter field lower bound",
  entry "animus-adapters::ModelRequest" "animus-adapters" adapters reusable none stable "model request DTO",
  entry "animus-adapters::ModelResponse" "animus-adapters" adapters reusable none stable "model response DTO",
  entry "animus-adapters::ToolRequest" "animus-adapters" adapters reusable none stable "tool request DTO",
  entry "animus-adapters::ToolResponse" "animus-adapters" adapters reusable none stable "tool response DTO",
  entry "animus-adapters::ModelAdapter" "animus-adapters" adapters injection injected stable "model port trait",
  entry "animus-adapters::ToolAdapter" "animus-adapters" adapters injection injected stable "tool port trait",
  entry "animus-adapters::ComponentAdapter" "animus-adapters" adapters injection injected stable "generic component port trait",
  entry "animus-adapters::LaunchPolicy" "animus-adapters" adapters reusable none stable "manifest launch policy DTO",
  entry "animus-adapters::LaunchPolicyDraft" "animus-adapters" adapters reusable none stable "manifest launch policy draft DTO",
  entry "animus-adapters::NetworkPolicy" "animus-adapters" adapters reusable none stable "manifest connectivity policy DTO",
  entry "animus-adapters::PROHIBITED_PLUGIN_MANIFEST_EFFECTS" "animus-adapters" adapters reusable none stable "manifest prohibited effects constant",
  entry "animus-adapters::PluginDescriptor" "animus-adapters" adapters reusable none stable "manifest descriptor DTO",
  entry "animus-adapters::PluginDescriptorDraft" "animus-adapters" adapters reusable none stable "manifest descriptor draft DTO",
  entry "animus-adapters::PluginDescriptorError" "animus-adapters" adapters reusable none stable "manifest descriptor validation error",
  entry "animus-adapters::RedactionPolicy" "animus-adapters" adapters reusable none stable "manifest redaction policy DTO",
  entry "animus-adapters::RedactionPolicyDraft" "animus-adapters" adapters reusable none stable "manifest redaction policy draft DTO",
  entry "animus-adapters::RuntimeKind" "animus-adapters" adapters reusable none stable "manifest runtime kind DTO",
  entry "animus-adapters::SandboxMode" "animus-adapters" adapters reusable none stable "manifest isolation mode DTO",
  entry "animus-adapters::SandboxRequirements" "animus-adapters" adapters reusable none stable "manifest isolation requirements DTO",
  entry "animus-adapters::SandboxRequirementsDraft" "animus-adapters" adapters reusable none stable "manifest isolation requirements draft DTO",
  entry "animus-adapters::ToolDescriptor" "animus-adapters" adapters reusable none stable "manifest tool descriptor DTO",
  entry "animus-adapters::ToolDescriptorDraft" "animus-adapters" adapters reusable none stable "manifest tool descriptor draft DTO",
  entry "animus-adapters::WorkingDirectoryMode" "animus-adapters" adapters reusable none stable "manifest working-directory DTO",

  entry "animus-preserves::from_bytes" "animus-preserves" preserves reusable none optional_stability "Preserves byte decoder",
  entry "animus-preserves::from_text" "animus-preserves" preserves reusable none optional_stability "Preserves text decoder",
  entry "animus-preserves::to_bytes" "animus-preserves" preserves reusable none optional_stability "Preserves byte encoder",
  entry "animus-preserves::to_text" "animus-preserves" preserves reusable none optional_stability "Preserves text encoder",
  entry "animus-preserves::CodecError" "animus-preserves" preserves reusable none optional_stability "Preserves codec error",
  entry "animus-preserves::BoundaryMessage" "animus-preserves" preserves reusable none optional_stability "Preserves boundary envelope",
  entry "animus-preserves::BoundaryKind" "animus-preserves" preserves reusable none optional_stability "Preserves boundary kind",
  entry "animus-preserves::RuntimeFailureEnvelope" "animus-preserves" preserves reusable none optional_stability "runtime failure boundary envelope",
  entry "animus-preserves::RuntimeReportStatus" "animus-preserves" preserves reusable none optional_stability "runtime report status envelope",

  entry "animus-runtime::DEFAULT_MAX_TURNS" "animus-runtime" runtime reusable none stable "runtime max-turn default",
  entry "animus-runtime::MIN_MAX_TURNS" "animus-runtime" runtime reusable none stable "runtime max-turn lower bound",
  entry "animus-runtime::MaxTurns" "animus-runtime" runtime reusable none stable "runtime max-turn budget DTO",
  entry "animus-runtime::RuntimeConfig" "animus-runtime" runtime reusable none stable "runtime execution config DTO",
  entry "animus-runtime::RuntimeError" "animus-runtime" runtime reusable none stable "runtime validation error",
  entry "animus-runtime::RuntimeFailure" "animus-runtime" runtime reusable none stable "runtime failure status DTO",
  entry "animus-runtime::AdapterComponent" "animus-runtime" runtime reusable none stable "runtime adapter component enum",
  entry "animus-runtime::RuntimeEvent" "animus-runtime" runtime reusable none stable "runtime event DTO",
  entry "animus-runtime::LoopFabric" "animus-runtime" runtime injection injected stable "caller-supplied loop fabric port",
  entry "animus-runtime::RuntimeInput" "animus-runtime" runtime reusable none stable "runtime input DTO",
  entry "animus-runtime::RunReport" "animus-runtime" runtime reusable none stable "runtime report DTO",
  entry "animus-runtime::RuntimeStatus" "animus-runtime" runtime reusable none stable "runtime status DTO",
  entry "animus-runtime::RuntimeAdapters" "animus-runtime" runtime injection injected stable "borrowed model tool adapter bundle",
  entry "animus-runtime::run_agent_loop" "animus-runtime" runtime injection injected stable "runtime loop over caller-supplied ports",
  entry "animus-runtime::run_with_steel_program" "animus-runtime" runtime injection injected stable "Steel runtime loop over caller-supplied ports",
  entry "animus-runtime::AdapterOutcome" "animus-runtime" runtime reusable none stable "adapter outcome DTO",
  entry "animus-runtime::AdapterRequest" "animus-runtime" runtime reusable none stable "adapter request DTO",
  entry "animus-runtime::AdapterStep" "animus-runtime" runtime reusable none stable "adapter transition DTO",
  entry "animus-runtime::DecisionStep" "animus-runtime" runtime reusable none stable "decision transition DTO",
  entry "animus-runtime::TurnPlan" "animus-runtime" runtime reusable none stable "turn planning DTO",
  entry "animus-runtime::TurnState" "animus-runtime" runtime reusable none stable "turn state DTO",
  entry "animus-runtime::apply_adapter_outcome" "animus-runtime" runtime reusable none stable "pure adapter transition helper",
  entry "animus-runtime::apply_decision" "animus-runtime" runtime reusable none stable "pure decision transition helper",
  entry "animus-runtime::complete_with_fabric_error" "animus-runtime" runtime reusable none stable "pure failure report helper",
  entry "animus-runtime::plan_next_turn" "animus-runtime" runtime reusable none stable "pure turn planning helper",

  entry "animus-std::INTENT_ENV_KEY" "animus-std" std_shell shell shell_owned stable "std shell setting key",
  entry "animus-std::load_runtime" "animus-std" std_shell shell shell_owned stable "std shell runtime loader",
  entry "animus-std::RunError" "animus-std" std_shell shell shell_owned stable "std shell run error",
  entry "animus-std::RunReport" "animus-std" std_shell shell shell_owned stable "std shell report",
  entry "animus-std::SummaryModelAdapter" "animus-std" std_shell shell shell_owned stable "std shell default adapter",
  entry "animus-std::run" "animus-std" std_shell shell shell_owned stable "std shell runner",
  entry "animus-std::run_with_model" "animus-std" std_shell shell shell_owned stable "std shell injected runner",
  entry "animus-std::RuntimeConfig" "animus-std" std_shell shell shell_owned stable "std shell runtime config",
  entry "animus-std::ConfigError" "animus-std" std_shell shell shell_owned stable "std shell config error",

  entry "animus-steel::LoopDecision" "animus-steel" steel reusable none optional_stability "Steel loop decision DTO",
  entry "animus-steel::LoopError" "animus-steel" steel reusable none optional_stability "Steel loop error",
  entry "animus-steel::SteelLoopFabric" "animus-steel" steel reusable none optional_stability "Steel loop fabric handle",
  entry "animus-steel::LoopTurn" "animus-steel" steel reusable none optional_stability "Steel turn input DTO",

  entry "animus-tui::rats" "animus-tui" tui shell shell_owned optional_stability "terminal widget shell reexport",
]
in
{
  entries | Array InventoryRecord = entry_values,
  inventory_lines = std.string.join "\n" (std.array.map line entry_values),
}
