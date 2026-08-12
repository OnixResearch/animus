# Evidence: adapt-agent-status-authority

## Baseline

- `nix develop -c cargo test -p animus-adapters -p animus-runtime` — passed before implementation.
- `nix run path:$PWD#cairn -- gate proposal adapt-agent-status-authority --root .`
- `nix run path:$PWD#cairn -- gate design adapt-agent-status-authority --root .`
- `nix run path:$PWD#cairn -- gate tasks adapt-agent-status-authority --root .`

## Implementation checks

- `nix develop -c cargo test -p animus-adapters status` — passed.
- `nix develop -c cargo test -p animus-runtime status` — passed.
- `nix develop -c cargo fmt --all` — passed.
- `nix develop -c cargo test -p animus-adapters -p animus-runtime` — passed after implementation.
- `nix develop -c cargo clippy -p animus-adapters -p animus-runtime --all-targets -- -D warnings` — passed.
- `nix develop -c cargo fmt --all -- --check` — passed.
- `nix develop -c cargo test --workspace` — passed.
- `nix develop -c cargo check --workspace --all-targets` — passed.
- `nix develop -c cargo clippy --workspace --all-targets -- -D warnings` — passed.
- `nix develop -c scripts/cairn/check-request-lifecycle --root .` — passed.
- `nix develop -c scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix develop -c scripts/cairn/check-evidence-safety --root .` — passed.

## Coverage

Positive coverage includes valid trusted lifecycle status observations, host status adapter batches, trusted lifecycle precedence over heuristic fallback, stale lifecycle fallback, deterministic pure selection, and presentation metadata retained separately from semantic state.

Negative coverage rejects malformed status state, malformed source identifiers, missing targets, unsupported session references, invalid evidence hashes, untrusted lifecycle authority, conflicting lifecycle authorities, custom label semantic escalation, raw prompt summaries, terminal snapshots, provider payload summaries, credential-looking summaries, and unbounded detector evidence summaries.
