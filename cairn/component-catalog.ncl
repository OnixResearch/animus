let ComponentRecord = {
  component_id | String,
  owner | String,
  component_class | String,
  feature_gate | String,
  required_authority | String,
  depends_on | Array String,
  composes_with | Array String,
  capability_family | String,
  focused_checks | Array String,
  evidence_labels | Array String,
  public_import | String,
}
in
let class_reusable = "reusable-contract" in
let class_host_injection = "host-injection-surface" in
let class_shell = "shell-adapter" in
let class_optional_bridge = "optional-bridge" in
let class_runtime_prelude = "runtime-prelude" in
let class_crate_feature = "crate-feature" in
let authority_none = "none" in
let authority_injected = "injected-only" in
let authority_shell = "shell-owned" in
let feature_always = "always" in
let comma = "," in
let doc_break = "<br>" in
let component = fun component_id_value owner_value class_value feature_value authority_value family_value focused_checks_value public_import_value =>
  {
    component_id = component_id_value,
    owner = owner_value,
    component_class = class_value,
    feature_gate = feature_value,
    required_authority = authority_value,
    depends_on = [],
    composes_with = [],
    capability_family = family_value,
    focused_checks = focused_checks_value,
    evidence_labels = ["safe-labels", "counts", "blake3-digests"],
    public_import = public_import_value,
  }
in
let rebuild = fun deps links evidence record => {
  component_id = record.component_id,
  owner = record.owner,
  component_class = record.component_class,
  feature_gate = record.feature_gate,
  required_authority = record.required_authority,
  depends_on = deps,
  composes_with = links,
  capability_family = record.capability_family,
  focused_checks = record.focused_checks,
  evidence_labels = evidence,
  public_import = record.public_import,
} in
let with_deps = fun deps record => rebuild deps record.composes_with record.evidence_labels record in
let with_composition = fun links record => rebuild record.depends_on links record.evidence_labels record in
let with_evidence = fun labels record => rebuild record.depends_on record.composes_with labels record in
let line = fun record =>
  "component|%{record.component_id}|%{record.owner}|%{record.component_class}|%{record.feature_gate}|%{record.required_authority}|%{std.string.join comma record.depends_on}|%{std.string.join comma record.composes_with}|%{record.capability_family}|%{std.string.join comma record.focused_checks}|%{std.string.join comma record.evidence_labels}|%{record.public_import}"
in
let docs_header = "# Component catalog\n\nThis projection is generated from `cairn/component-catalog.ncl`. Do not edit catalog rows by hand; update the Nickel source and run `scripts/cairn/check-component-catalog --root .`.\n\nThe catalog names Lego-style Animus components, the smallest public import surface, authority posture, dependency links, capability family, and focused validation rail. It is metadata evidence only: it does not claim provider, filesystem, process, network, terminal, credential, persistence, production, or live-service readiness unless a row explicitly classifies that authority as shell-owned or injected.\n"
in
let docs_row = fun record =>
  "| `%{record.component_id}` | `%{record.component_class}` | `%{record.owner}` | `%{record.feature_gate}` | `%{record.required_authority}` | `%{record.capability_family}` | `%{std.string.join doc_break record.focused_checks}` | `%{record.public_import}` |"
in
let render_docs = fun records =>
  docs_header
  ++ "\n| Component | Class | Owner | Feature gate | Authority | Family | Focused checks | Smallest import |\n"
  ++ "| --- | --- | --- | --- | --- | --- | --- | --- |\n"
  ++ std.string.join "\n" (std.array.map docs_row records)
  ++ "\n\n## Validation\n\nRun `scripts/cairn/check-component-catalog --root .` to validate labels, dependency links, cycles, focused checks, safe evidence fields, fixtures, and this projection. The Nix check is `nix build .#checks.x86_64-linux.component-catalog --no-link -L`.\n"
in
let component_values = [
  component "core.intent-state" "animus-core" class_reusable feature_always authority_none "core" ["cargo test -p animus-core --locked", "scripts/cairn/check-wasm-surface --root ."] "animus_core::{Intent, AnimusState, CoreError}",
  component "adapters.dto-boundary" "animus-adapters" class_reusable feature_always authority_none "adapter-contracts" ["cargo test -p animus-adapters --locked", "scripts/cairn/check-runtime-facade-inventory --root ."] "animus_adapters::{ModelRequest, ToolRequest, ToolResponse}",
  with_deps ["adapters.dto-boundary"] (component "adapters.model-port" "animus-adapters" class_host_injection feature_always authority_injected "model" ["cargo test -p animus-adapters model --locked", "scripts/cairn/check-adapter-conformance-index --root ."] "animus_adapters::ModelAdapter"),
  with_deps ["adapters.dto-boundary"] (component "adapters.tool-port" "animus-adapters" class_host_injection feature_always authority_injected "tool" ["cargo test -p animus-adapters tool::tests --locked", "scripts/cairn/check-tool-catalog --root ."] "animus_adapters::ToolAdapter"),
  with_deps ["adapters.dto-boundary"] (component "adapters.status-observations" "animus-adapters" class_host_injection "status" authority_injected "status" ["cargo test -p animus-adapters status::tests --locked", "scripts/cairn/check-runtime-facade-inventory --root ."] "animus_adapters::HostStatusAdapter"),
  with_deps ["adapters.dto-boundary"] (component "adapters.safe-receipts" "animus-adapter-safe" class_reusable feature_always authority_none "safe-receipt" ["cargo test -p animus-adapter-safe --locked", "scripts/cairn/check-adapter-family-crates --root ."] "animus_adapter_safe::{Record, Draft, ArtifactHash}"),
  with_deps ["adapters.safe-receipts"] (component "adapters.observability" "animus-adapter-observability" class_reusable feature_always authority_none "observability" ["cargo test -p animus-adapter-observability --locked", "scripts/cairn/check-adapter-family-crates --root ."] "animus_adapter_observability::{MetricSample, LogRecord, MetricsAdapter}"),
  with_deps ["adapters.dto-boundary", "adapters.safe-receipts"] (component "adapters.context-references" "animus-adapters" class_reusable feature_always authority_none "context" ["cargo test -p animus-adapters context --locked", "scripts/cairn/check-request-lifecycle --root ."] "animus_adapters::{ContextReference, ContextResolverSet}"),
  with_deps ["adapters.context-references"] (component "adapters.document-ingest" "animus-adapters" class_reusable feature_always authority_none "document-ingest" ["cargo test -p animus-adapters document --locked", "scripts/cairn/check-adapter-conformance-index --root ."] "animus_adapters::{DocumentIngestReport, validate_document_ingest_report}"),
  with_deps ["adapters.dto-boundary"] (component "adapters.model-evidence" "animus-adapters" class_reusable feature_always authority_none "model-evidence" ["cargo test -p animus-adapters modelevidence --locked", "scripts/cairn/check-adapter-conformance-index --root ."] "animus_adapters::{ModelEvidenceReport, validate_model_evidence_report}"),
  with_deps ["adapters.dto-boundary", "adapters.safe-receipts"] (component "adapters.orchestration" "animus-adapters" class_host_injection "orchestration" authority_injected "orchestration" ["cargo test -p animus-adapters orchestration --locked", "scripts/cairn/check-composition-conformance --root ."] "animus_adapters::orchestration::OrchestrationService"),
  component "router.recipe-selection" "animus-router" class_reusable "router" authority_none "router" ["cargo test -p animus-router --locked", "scripts/cairn/check-router-policy --root ."] "animus_router::{Recipe, Signals, select_recipe}",
  with_deps ["core.intent-state", "adapters.dto-boundary", "adapters.model-port", "adapters.tool-port"] (component "runtime.loop-kernel" "animus-runtime" class_runtime_prelude "loop" authority_injected "runtime" ["cargo check -p animus-runtime --no-default-features --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::kernel"),
  with_deps ["runtime.loop-kernel", "adapters.tool-port"] (component "runtime.tool-dispatch-planning" "animus-runtime" class_reusable "runtime" authority_none "tool" ["cargo test -p animus-runtime shell::tests --locked", "scripts/cairn/check-tool-catalog --root ."] "animus_runtime::ToolDispatchPlan"),
  with_deps ["runtime.loop-kernel", "adapters.safe-receipts"] (component "runtime.ledger-session-seam" "animus-runtime" class_host_injection "runtime" authority_injected "runtime" ["cargo test -p animus-runtime session_persistence --locked", "scripts/cairn/check-request-lifecycle --root ."] "animus_runtime::session_persistence"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.headless" "animus-runtime" class_crate_feature "headless" authority_none "headless" ["cargo check -p animus-runtime --no-default-features --features headless --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::headless_prelude"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.provenance" "animus-runtime" class_crate_feature "provenance" authority_none "provenance" ["cargo check -p animus-runtime --no-default-features --features provenance --locked", "scripts/cairn/check-provenance-ledger --root ."] "animus_runtime::provenance_prelude"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.worker-team" "animus-runtime" class_crate_feature "worker-team" authority_injected "worker-team" ["cargo test -p animus-runtime worker_team --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::worker_prelude"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.policy" "animus-runtime" class_crate_feature "policy" authority_injected "policy" ["cargo test -p animus-runtime hooks --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::policy_prelude"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.batch" "animus-runtime" class_crate_feature "batch" authority_injected "batch" ["cargo test -p animus-runtime batch --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::batch"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.compaction" "animus-runtime" class_crate_feature "compaction" authority_none "compaction" ["cargo test -p animus-runtime pi_compaction --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::PiCompactionPlan"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.dataset-export" "animus-runtime" class_crate_feature "dataset-export" authority_none "dataset-export" ["cargo test -p animus-runtime dataset_export --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::DatasetExportPolicy"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.quality" "animus-runtime" class_crate_feature "quality" authority_none "quality" ["cargo test -p animus-runtime quality_records --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::quality_records"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.research" "animus-runtime" class_crate_feature "research" authority_none "research" ["cargo test -p animus-runtime researchspec --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::researchspec_promotion_dag"),
  with_deps ["runtime.loop-kernel"] (component "runtime.pack.scaffold" "animus-runtime" class_crate_feature "scaffold" authority_none "scaffold" ["cargo test -p animus-runtime scaffold --locked", "scripts/cairn/check-runtime-kernel-split --root ."] "animus_runtime::scaffold"),
  with_deps ["runtime.loop-kernel", "adapters.orchestration"] (component "runtime.pack.orchestration" "animus-runtime" class_crate_feature "orchestration" authority_injected "orchestration" ["cargo test -p animus-runtime adapter_family_bridge --locked", "scripts/cairn/check-composition-conformance --root ."] "animus_runtime::OrchestrationAdapterBoardService"),
  with_deps ["runtime.loop-kernel", "adapters.status-observations"] (component "runtime.status-prelude" "animus-runtime" class_runtime_prelude "status" authority_none "status" ["cargo test -p animus-runtime status::tests --locked", "scripts/cairn/check-runtime-facade-inventory --root ."] "animus_runtime::status_prelude"),
  with_deps ["runtime.loop-kernel", "adapters.observability"] (component "runtime.observability-projection" "animus-runtime" class_reusable "runtime" authority_none "observability" ["cargo test -p animus-runtime observability --locked", "scripts/cairn/check-composition-conformance --root ."] "animus_runtime::{project_runtime_event_observability, project_run_report_observability}"),
  with_deps ["adapters.dto-boundary", "runtime.loop-kernel"] (component "steel.loop-fabric" "animus-steel" class_runtime_prelude "steel" authority_none "steel" ["cargo test -p animus-steel --locked", "scripts/cairn/check-steel-runtime-policy --root ."] "animus_steel::{LoopTurn, LoopDecision, SteelLoopFabric}"),
  with_deps ["adapters.dto-boundary"] (component "preserves.adapter-bridge" "animus-preserves" class_optional_bridge "preserves" authority_none "preserves" ["cargo test -p animus-preserves --locked", "cargo check -p animus --features preserves --locked"] "animus_preserves"),
  with_deps ["runtime.loop-kernel", "adapters.context-references"] (component "std.context-shell" "animus-std" class_shell "std" authority_shell "context" ["cargo test -p animus-std context --locked", "scripts/cairn/check-request-lifecycle --root ."] "animus_std::{LocalFileContextResolver, LocalDirectoryContextResolver}"),
  with_deps ["runtime.loop-kernel", "adapters.observability"] (component "std.observability-sinks" "animus-std" class_shell "std" authority_shell "observability" ["cargo test -p animus-std observability --locked", "scripts/cairn/check-composition-conformance --root ."] "animus_std::{NoopObservabilitySink, InMemoryObservabilitySink}"),
  with_deps ["runtime.loop-kernel", "adapters.model-port", "adapters.tool-port"] (component "std.runtime-shell" "animus-std" class_shell "std" authority_shell "std-shell" ["cargo test -p animus-std --locked", "scripts/cairn/check-request-lifecycle --root ."] "animus_std::{RuntimeConfig, run_with_model}"),
  with_deps ["runtime.loop-kernel"] (component "tui.host-shell" "animus-tui" class_shell "tui" authority_shell "tui" ["cargo check -p animus --features tui --locked", "scripts/cairn/check-runtime-facade-inventory --root ."] "animus_tui"),
  with_deps ["core.intent-state", "adapters.dto-boundary", "runtime.loop-kernel", "std.runtime-shell"] (component "root.compat-facade" "animus" class_crate_feature "default" authority_shell "compatibility" ["cargo check -p animus --locked", "scripts/cairn/check-runtime-facade-inventory --root ."] "animus"),
]
in
{
  component_contract = ComponentRecord,
  classes = {
    reusable = class_reusable,
    host_injection = class_host_injection,
    shell = class_shell,
    optional_bridge = class_optional_bridge,
    runtime_prelude = class_runtime_prelude,
    crate_feature = class_crate_feature,
  },
  authorities = {
    none = authority_none,
    injected = authority_injected,
    shell = authority_shell,
  },
  features = { always = feature_always },
  make_component = component,
  add_dependencies = with_deps,
  add_composition = with_composition,
  set_evidence = with_evidence,
  render_line = line,
  render_docs_fn = render_docs,
  components | Array ComponentRecord = component_values,
  component_lines = std.string.join "\n" (std.array.map line component_values),
  docs_markdown = render_docs component_values,
}
