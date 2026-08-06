## Scope

This change adds pure active-turn recovery contracts and a non-default deterministic std fixture host.

The source reference is camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2` under the MIT license. Animus uses only the durable-before-dispatch ordering concept.

Animus rejects the upstream storage model, runtime dependency, timestamp identity, implicit atomicity, and ambient process state.

## Baseline

Before core edits, task `10877` ran focused interrupted-tool, provider-retry, and coding-session Cargo filters. Compilation passed. The selected filter names matched no direct tests, so this evidence is compilation-only.

Before implementation, the proposal, design, tasks, and repository validation gates passed.

## Ownership inventory

- `animus-runtime::session_persistence` owns host-injected ledger replay and restore planning.
- `animus-runtime::durable_session_store` owns pure private journal and commit validation.
- `animus-runtime::session_authority_profile` owns durable authority binding and drift decisions.
- `animus-runtime::interrupted_tool_repair` owns unmatched tool-call repair and no-rerun behavior.
- `animus-runtime::provider_retry` owns bounded same-plan provider retry after a failure.
- `animus-runtime::coding_session_host` owns the current pure session reducer.
- `animus-std` owns explicit policy reads and host transaction implementations.
- Consumers retain provider, tool, storage, clock, cancellation, cleanup, and completion authority.

## Implementation evidence

- Typed Nickel policy and positive and negative fixtures cover bounds, corruption posture, atomic replacement, transaction evidence, receipts, and non-claims.
- Rust revalidation derives a domain-separated BLAKE3 policy identity.
- Pure plans cover durable acceptance, checkpoint replacement, steering append, recovery, terminal observation, and cleanup.
- Provider dispatch and steering delivery require exact durable transaction evidence.
- Recovery delegates unmatched calls to interrupted-tool repair and has no tool-request type.
- The std fixture host preserves state on stale, partial, and store-failure outcomes.
- Metadata-only receipts reject raw message, reasoning, tool, provider, credential, environment, path, store-error, and unsupported-claim data.

## Initial focused result

`cargo test -p animus-runtime --locked active_turn_recovery` passed 15 positive and negative tests after the pure core was added.

## Closeout checks

- `scripts/cairn/check-active-turn-recovery --root .` passed 15 runtime tests, 3 std-host tests, 16 interrupted-tool tests, and 17 provider-retry tests.
- The focused rail passed Clippy for runtime and std surfaces.
- The focused rail passed runtime, std, and root no-default checks.
- `scripts/cairn/check-component-catalog --root .` passed positive and negative fixtures.
- `scripts/cairn/check-runtime-facade-inventory --root .` passed with all public exports classified.
- `scripts/config/check-nickel-configs` passed after generated policy staging.
- `cargo fmt --all -- --check` passed.
- Cairn repository validation and proposal, design, and tasks gates passed.
- `nix build .#checks.x86_64-linux.active-turn-recovery --no-link -L` passed on the local builder after the configured remote builder was unavailable.
- Wasm is not applicable because the accepted generic Wasm boundary excludes `animus-runtime` and `animus-std`.

## Non-claims

This evidence does not prove store correctness, power-loss durability, provider acceptance, tool non-execution, exactly-once effects, transcript completeness, task completion, production readiness, or release eligibility.
