# Evidence: adapt-hermes-kanban-orchestration

## Baseline

- `nix run .#cairn -- validate --root .` passed before implementation.
- `scripts/cairn/check-roadmap --root .` passed before implementation.
- `nix run .#cairn -- gate proposal adapt-hermes-kanban-orchestration --root .` passed before implementation.
- `nix run .#cairn -- gate design adapt-hermes-kanban-orchestration --root .` passed before implementation.
- `nix run .#cairn -- gate tasks adapt-hermes-kanban-orchestration --root .` passed before implementation.
- `cargo test -p animus-runtime worker_team --locked` passed as the nearest pre-existing runtime orchestration baseline.

## Implementation evidence

- Added pure runtime Kanban DTO validation, dependency promotion, workspace mutation admission, tenant-safe context assembly, and injected board service failure behavior in `crates/animus-runtime/src/kanban.rs`.
- Added positive and negative unit tests covering valid graph acceptance, fan-out/fan-in promotion, explicit workspace admission, tenant-label preservation, injected-service absence, missing workspace, duplicate IDs, dependency cycles, ambiguous blocked parents, stale completion revisions, cross-tenant dependency/context rejection, quarantine without body admission, unsafe handoff body rejection, and out-of-scope mutation rejection.
- Documented the bounded adaptation in `docs/hermes-kanban-orchestration.md` and linked it from `README.md`.
- Updated the runtime facade inventory documentation and Nickel inventory entries for the new public Kanban exports.

## Passing checks after implementation

- `cargo test -p animus-runtime kanban --locked` passed: 8 focused Kanban tests.
- `cargo test -p animus-runtime --lib --locked` passed: 162 runtime unit tests.
- `cargo check -p animus-runtime --all-targets --locked` passed with the pre-existing `excessive_file_length` unknown-lint warning.
- `nix run .#cairn -- validate --root .` passed.
- `scripts/cairn/check-roadmap --root .` passed.
- `nix run .#cairn -- gate proposal adapt-hermes-kanban-orchestration --root .` passed.
- `nix run .#cairn -- gate design adapt-hermes-kanban-orchestration --root .` passed.
- `nix run .#cairn -- gate tasks adapt-hermes-kanban-orchestration --root .` passed.

## Broader rail blockers

- `cargo clippy -p animus-runtime --all-targets -- -D warnings` is blocked before this change's code by pre-existing `animus-adapters` Clippy errors, including `clippy::needless_pass_by_value` in `crates/animus-adapters/src/workspace/receipt/mod.rs`.
- `cargo clippy -p animus-runtime --all-targets --no-deps -- -D warnings` is blocked by pre-existing runtime missing `# Errors` docs in older public Result-returning helpers, including `crates/animus-runtime/src/worker_team/core.rs`.
- `cargo check --workspace --all-targets --locked` is blocked by a pre-existing `animus-preserves` non-exhaustive match over `RuntimeEvent::LifecycleHookOutcome`.
- `scripts/cairn/check-runtime-facade-inventory --root .` is blocked by pre-existing inventory drift outside Kanban exports; a Kanban-specific grep of the rail output found no Kanban issues.

## Requirement coverage

- `r[agent_harness_runtime.hermes_kanban.task_graph]`
- `r[agent_harness_runtime.hermes_kanban.task_graph.valid]`
- `r[agent_harness_runtime.hermes_kanban.dependencies]`
- `r[agent_harness_runtime.hermes_kanban.dependencies.promote]`
- `r[agent_harness_runtime.hermes_kanban.dependencies.cycle]`
- `r[agent_harness_runtime.hermes_kanban.workspace_scope]`
- `r[agent_harness_runtime.hermes_kanban.workspace_scope.constrained]`
- `r[agent_harness_runtime.hermes_kanban.tenant_scope]`
- `r[agent_harness_runtime.hermes_kanban.tenant_scope.reject_cross_tenant]`
- `r[agent_harness_runtime.hermes_kanban.handoffs]`
- `r[agent_harness_runtime.hermes_kanban.handoffs.block]`
- `r[agent_harness_runtime.hermes_kanban.verification]`
- `r[agent_harness_runtime.hermes_kanban.verification.closeout]`
