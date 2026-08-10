# Request-context and skill-turn lifecycle evidence

## Result

Animus now has one public, authority-free request-context contribution boundary. It validates owner-admitted contributions, applies deterministic reduction, binds the exact model invocation, enforces finite lifetimes, and emits metadata-only receipts.

The boundary keeps source loading, policy, trust, provider dispatch, and persistence authority with existing owners. The injected skill-body service is the only host effect added by this change.

## Baseline

Before implementation, the complete `animus-runtime` suite passed with 606 tests. Existing model-profile, skill-source, request-lifecycle, standard-host, and Nickel configuration rails also passed.

The producer and authority inventory is in `docs/request-context-contributions.md`. It covers prompt, skill, memory, project-context, MCP, and runtime producers. It also records conflict, budget, lifetime, and persistence ownership.

## Reference boundary

The reviewed reference is Autolith commit `8755dc516c9f6c1126b148d3fc426312095de871`, under the ISC license. Animus adapted deterministic priority, supersession, conflict-group, edge-triggered context, and next-provider-response skill scenarios.

Animus rejected Autolith's prompt store, S-expression format, Common Lisp runtime, saved images, Bash supervisor, global mutable process state, and storage behavior. This change adds no Autolith runtime dependency.

## Implementation evidence

- `animus_runtime::request_context` is the public boundary.
- The pure core validates contributions and reduces supersession, deduplication, conflicts, mandatory budget, advisory priority, and exact bindings.
- Existing `ModelInvocationPlan` composition binds target, profile, prompt plan, policy, authority, and provider request.
- Fallback or profile drift requires complete recomposition.
- `CurrentRequest`, `UntilNextProviderResponse`, and `CurrentLogicalTurn` expire only on exact terminal observations.
- Skill completion waits for the exact completed, failed, or cancelled provider response. An explicit pre-dispatch failure can expire an unconsumed selection.
- Retention admission rejects conversation, compaction, memory, quality, and receipt payload storage.
- Receipts reject prompt bodies, skill bodies, MCP instructions, memory records, paths, credentials, provider data, and hidden directives.
- The focused receipt is `target/cairn/focused-rails/request-context-contributions.md`.

## Verification

The final validation set includes these passing checks:

- `cargo fmt --all -- --check`
- `cargo test -p animus-runtime --locked`: 617 passed
- `cargo clippy -p animus-runtime --all-targets --locked -- -D warnings`
- `cargo check -p animus-runtime --target wasm32-unknown-unknown --no-default-features --locked`
- `scripts/cairn/check-request-context-contributions --root .`: 11 focused request-context tests and one model-composition test passed
- `scripts/cairn/check-model-runtime-profiles --root .`: 18 runtime tests and three standard-host tests passed
- `scripts/cairn/check-skill-source-lifecycle --root .`
- `scripts/cairn/check-request-lifecycle --root .`
- `scripts/cairn/check-evidence-safety --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-wasm-surface --root .`
- `nix build .#checks.x86_64-linux.request-context-contributions --no-link -L`
- `nix run .#cargo-tigerstyle -- check`: zero findings
- `nix run .#cairn -- validate --root .`

The configured remote Nix builder was unavailable over SSH. Nix used the local builder and completed the relevant derivations.

## Lifecycle closeout

- Sync dry-run was unblocked.
- Sync execution applied all seven requirements with receipt `14db4a3d49e60d6539a1cc6f03fab01774a8ffeec00cf5af27ef3ed94be37b22`.
- Accepted-spec after-hash: `543a5817233a7fea6bc81165172f59823caf40533e0b52ec73af8319ce0bd079`.
- Archive dry-run was unblocked with plan `bc7a1b24bca99324f4e8410a635e4059a1da18ad189f4f671ad0aebfe9da9a5d`.
- Archive execution completed with receipt `aef6fbd8f5f40e264ec6ad1e6c29ef0cbd648da73992d1a0cb6f062351376d8f`.
- Archive mutation manifest: `a472b732817b6672b740f1d52ac76407171fb1ba24e3313e7d77907d7aeb1b0a`.

## Non-claims

This evidence does not prove prompt correctness, hidden-context confidentiality, memory truth, MCP server trust, provider delivery, model compliance, skill quality, task completion, production readiness, or release eligibility.
