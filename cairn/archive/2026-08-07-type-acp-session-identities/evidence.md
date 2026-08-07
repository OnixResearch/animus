# Evidence: Type ACP session identities

## Verdict

PASS for the bounded nominal-identity migration.

Typed identities establish local role separation and checked syntax. They do not establish client authorization, sandbox enforcement, provider correctness, replay equivalence, arbitrary ACP compatibility, production readiness, or release eligibility.

## Baseline

Before implementation, `scripts/cairn/check-acp-stdio-host --root .` passed all 32 ACP host tests and its linked facade, permission, event, receipt, Clippy, and dependency-isolation checks.

The baseline focused source identity was `af369f96a99d89b12e3cf0020727fdcd9d580b9211f0d96d54d796d35b4bf707`.

## Implementation evidence

- Added 12 private-by-default nominal roles for ACP external identities and Animus BLAKE3 identities.
- Added deterministic admission for empty, oversized, malformed, whitespace-bearing, and unsupported-digest input.
- Kept `AcpDecodedRequest` as the bounded raw DTO and added `AcpAdmittedRequest` before reducer or facade use.
- Migrated connection state, bindings, correlations, run state, facade commands, observations, updates, permission inputs, and receipt drafts.
- Kept protocol and receipt projection as explicit string boundaries.
- Declared reviewed domains in `config/acp-nominal-identities.ncl` for current static checks and future Octet policy use.
- Added one same-role compile-pass doctest and six cross-role compile-fail doctests.

## Focused verification

`scripts/cairn/check-acp-nominal-identities --root .` passed.

- ACP unit tests: 37 passed.
- Same-role compile-pass doctests: 1 passed.
- Cross-role compile-fail doctests: 6 passed.
- Coding-session host tests: 13 passed.
- TUI event projection tests: 2 passed.
- Root ACP feature check, default check, and no-default-feature check: passed.
- Focused ACP Clippy with warnings denied: passed.
- Existing ACP wire, event, permission, cancellation, receipt, and fixture-client rail: passed.

Focused source identity: `df046bc816c3aa0ad20f48bbe5b63a05ab7cd3d0a88722fd335690daa32abb47`.

Focused receipt: `target/cairn/focused-rails/acp-nominal-identities.md`.

## Broad verification

The following checks passed:

- `nix run .#check-nickel-configs`
- `cargo check --workspace --all-targets --locked`
- `cargo clippy --workspace --all-targets --locked -- -D warnings`
- `cargo fmt --all -- --check`
- `scripts/cairn/check-component-catalog --root .`
- `scripts/cairn/check-runtime-facade-inventory --root .`
- `scripts/cairn/check-evidence-safety --root .`
- `nix run path:$PWD#cairn -- validate --root .`
- proposal, design, and tasks Cairn gates
- `nix build .#checks.x86_64-linux.acp-nominal-identities -L --option builders ''`

The configured remote builder was unavailable during Nickel validation. Nix continued on the local builder, and the selected checks passed.

## Compatibility evidence

The reviewed protocol cohort remains `agent-client-protocol 0.10.4` with `agent-client-protocol-schema 0.11.4` under Apache-2.0.

Existing positive and negative codec, fixture-client, event, permission, cancellation, and receipt tests passed after migration. JSON-RPC and metadata-only receipt projection still use the original string values at their boundaries. No protocol or receipt schema version changed.
