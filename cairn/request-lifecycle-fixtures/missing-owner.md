expected_issue=REQUEST_LIFECYCLE_MARKER_MISSING

Typed runtime input animus-std::RuntimeConfig Runtime::from_env_pairs.
Deterministic core state animus-core::Intent animus-core::AnimusState.
Steel loop decision animus-steel::LoopTurn.
Adapter dispatch DTOs animus-adapters::ModelRequest animus-adapters::ToolRequest ModelAdapter ToolAdapter.
Std-shell adapter dispatch animus-std::run_with_model.
Runtime event/report projection animus-std::RunReport.
Safe receipt construction. Host ledger persistence seam LedgerService.
Forbidden dependencies and side effects: environment variables, filesystem paths, credentials, provider config, network resources, clocks, concrete host persistence.
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-evidence-safety --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cairn -- validate --root .
