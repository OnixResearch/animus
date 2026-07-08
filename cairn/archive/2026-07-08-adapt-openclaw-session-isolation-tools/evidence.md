# Evidence: adapt-openclaw-session-isolation-tools

## Implementation

- Added `animus_adapters::openclaw_session` as a pure no-std planning module for host-store-neutral session isolation and session-tool behavior.
- Added positive and negative fixtures covering per-channel-peer session key separation, identity-link normalization, shared-DM privacy diagnostics, ambient store discovery denial, ambiguous metadata rejection, maintenance previews, sandbox visibility clamps, cross-agent denial, bounded history previews, inter-session provenance, missing lineage rejection, and raw transcript leak rejection.
- Added docs in `docs/openclaw-session-isolation-tools.md`, README discovery text, and the Nickel-owned runtime facade inventory entry.

## Checks

- Baseline before edits: `cargo test -p animus-adapters session` — passed, 12 selected tests.
- Baseline before edits: `nix run path:$PWD#cairn -- validate --root .` — passed.
- `cargo fmt` — passed.
- `cargo test -p animus-adapters openclaw_session` — passed, 3 selected tests.
- `cargo test -p animus-adapters` — passed, 483 tests.
- `cargo check --workspace --all-targets` — passed.
- `scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix run path:$PWD#cairn -- validate --root .` — passed.

## Non-claims

No concrete session store, JSONL transcript format, global state directory, cleanup scheduler, clock use, channel connector, gateway-owned persistence, transcript filesystem traversal, child-run executor, or live cross-session delivery is implemented or claimed.
