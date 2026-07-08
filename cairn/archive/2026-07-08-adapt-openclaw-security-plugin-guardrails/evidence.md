# Evidence: adapt-openclaw-security-plugin-guardrails

## Implementation

- Added `animus_adapters::openclaw_guardrails` as a pure no-std classifier and validator over host-supplied security snapshots, extension manifest data, trust/path summaries, and safe receipts.
- Added positive and negative fixtures covering hardened baseline classification, remote exposure, open messaging with broad tools, sandbox drift, model/tool risk mismatch, manifest schema-before-execution, unknown channel rejection, disabled-extension warnings, path-escape summaries, lifecycle-script risk, ambient host inspection denial, and receipt leak rejection.
- Added docs in `docs/openclaw-security-plugin-guardrails.md`, README discovery text, and the Nickel-owned runtime facade inventory entry.

## Checks

- Baseline before edits: `cargo test -p animus-adapters manifest` — passed.
- Baseline before edits: `cargo test -p animus-adapters security` — passed, 9 selected tests.
- Baseline before edits: `nix run path:$PWD#cairn -- validate --root .` — passed.
- `cargo fmt` — passed.
- `cargo test -p animus-adapters openclaw_guardrails` — passed, 3 selected tests.
- `cargo test -p animus-adapters` — passed, 486 tests.
- `cargo check --workspace --all-targets` — passed.
- `scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix run path:$PWD#cairn -- validate --root .` — passed.

## Non-claims

No CLI audit command, Doctor UI, config loader, filesystem scanner, network probe, package installer, extension runtime, gateway restart, auto-fix executor, symlink resolver, ownership checker, permission stat call, or live host inspection is implemented or claimed.
