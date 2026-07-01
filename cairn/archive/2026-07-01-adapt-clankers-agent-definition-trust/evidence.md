# Evidence: adapt-clankers-agent-definition-trust

## Implementation evidence

- Added typed agent-definition manifest, discovery, admission, activation, capability-ceiling, and receipt DTO/core surfaces under `crates/animus-adapters/src/actor/` with front-door aliases in `crates/animus-adapters/src/front/actor.rs`.
- Added positive and negative actor tests covering valid user manifests, deterministic precedence, admitted project activation, safe receipts, malformed manifests, duplicate same-precedence definitions, unadmitted project definitions, capability escalation, bounded collection/prompt budgets, and receipt leak rejection.
- Added `docs/agent-definition-trust.md` documenting typed definitions, default-untrusted project definitions, ceiling-only authority, and safe receipt metadata.
- Promoted `r[agent_harness_runtime.clankers_agent_definitions.*]` requirements into `cairn/specs/agent-harness-runtime/spec.md` after Cairn sync reported an unchanged accepted spec for the full-spec-shaped delta.

## Verification evidence

- Baseline lifecycle check before edits: `nix run path:/home/brittonr/git/animus#cairn -- validate --root /home/brittonr/git/animus` passed with `valid: true`.
- Cairn gates: `gate proposal`, `gate design`, and `gate tasks` for `adapt-clankers-agent-definition-trust` passed.
- Focused tests before sync/archive: `nix develop -c cargo test -p animus-adapters actor --locked` passed: 10 passed, 0 failed.
- Sync dry-run: `nix run path:/home/brittonr/git/animus#cairn -- sync adapt-clankers-agent-definition-trust --root /home/brittonr/git/animus` was unblocked with one `sync_delta_spec` action.
- Sync execute: `nix run path:/home/brittonr/git/animus#cairn -- sync adapt-clankers-agent-definition-trust --root /home/brittonr/git/animus --execute` completed; accepted spec content required manual promotion because before/after hashes were unchanged.
- Validation after spec promotion: `nix run path:/home/brittonr/git/animus#cairn -- validate --root /home/brittonr/git/animus` passed with `valid: true`.
- Focused tests after spec promotion: `nix develop -c cargo test -p animus-adapters actor --locked` passed: 10 passed, 0 failed.
- Workspace check: `nix develop -c cargo check --workspace --all-targets --locked` passed.

## Non-claims

This evidence does not claim full agent-team orchestration, Clankers Markdown/YAML compatibility, runtime prompt assembly adoption, live provider behavior, TUI behavior, daemon behavior, or production readiness.
