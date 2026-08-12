expected_issue=OK

Typed runtime input animus-std::RuntimeConfig Runtime::from_env_pairs.
Deterministic core state animus-core::Intent animus-core::AnimusState.
Steel loop decision animus-steel::LoopTurn animus-steel::LoopDecision current-shape `CallTool`; legacy name-plus-raw-input tool decisions are rejected.
Adapter dispatch DTOs animus-adapters::ModelRequest current-shape `animus-adapters::ToolRequest` bounded `animus-adapters::ToolResponse` ToolDispatchPlan ModelAdapter ToolAdapter.
Std-shell adapter dispatch animus-std::run_with_model.
Runtime event/report projection animus-std::RunReport.
Safe receipt construction. Host ledger persistence seam LedgerService.
Forbidden dependencies and side effects: environment variables, filesystem paths, credentials, provider config, network resources, clocks, concrete host persistence.
scripts/cairn/check-tool-catalog --root .
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-evidence-safety --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cairn -- validate --root .
