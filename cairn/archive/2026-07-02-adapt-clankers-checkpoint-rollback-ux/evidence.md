# Evidence: adapt-clankers-checkpoint-rollback-ux

## Implementation

- Added `animus-adapters` checkpoint contracts in `crates/animus-adapters/src/checkpoint.rs`.
- Exported checkpoint DTOs, pure policy/rollback validation functions, injected service boundary traits, safe receipt DTOs, and receipt leak validation from `crates/animus-adapters/src/lib.rs`.
- Added `docs/checkpoint-rollback.md` and linked it from `README.md`.

## Requirement coverage

- r[agent_harness_runtime.clankers_checkpoint_rollback.dto]: `CheckpointRecord`, `CheckpointDraft`, `CheckpointHash`, backend/status/failure enums.
- r[agent_harness_runtime.clankers_checkpoint_rollback.dto.backend_neutral]: positive fixtures cover `jj`, `git`, `in_memory`, and `workspace_isolation` backends.
- r[agent_harness_runtime.clankers_checkpoint_rollback.service]: `CheckpointService`, `CheckpointServiceRequest`, `CheckpointServiceResponse`, and service-state failure classes.
- r[agent_harness_runtime.clankers_checkpoint_rollback.service.missing_strict]: strict protected mutation test rejects missing service before dispatch.
- r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation]: `validate_rollback` requires explicit approval.
- r[agent_harness_runtime.clankers_checkpoint_rollback.confirmation.required]: missing confirmation test returns blocked rollback and no restore permission.
- r[agent_harness_runtime.clankers_checkpoint_rollback.freshness]: rollback validation compares expected and observed workspace hashes.
- r[agent_harness_runtime.clankers_checkpoint_rollback.freshness.stale]: stale workspace test rejects restore before backend mutation.
- r[agent_harness_runtime.clankers_checkpoint_rollback.receipts]: `CheckpointReceipt` computes a BLAKE3 safe receipt hash over bounded metadata.
- r[agent_harness_runtime.clankers_checkpoint_rollback.receipts.rejects_leaks]: receipt leak fixtures reject raw diff, raw file body, credential, environment dump, and full path list sentinels.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters` passed with 278 tests.
- Post-change: `nix develop -c cargo fmt` passed.
- Post-change: `nix develop -c cargo test -p animus-adapters` passed with 287 tests.
- Post-change: `nix develop -c cargo check -p animus-adapters` passed.
- Post-change clippy attempt: `nix develop -c cargo clippy -p animus-adapters --all-targets -- -D warnings` failed on existing non-checkpoint diagnostics; captured log contained no `checkpoint.rs` diagnostics after local fixes. Next best step for full clippy is to retire existing lint debt outside this change.
