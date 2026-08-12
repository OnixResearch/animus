# Evidence: adapt-pi-headless-rpc-protocol

## Implemented surfaces

- `crates/animus-runtime/src/headless/protocol.rs` defines pure headless command, response, queueing, framing, event-projection, and safe-error DTOs/helpers.
- `crates/animus-runtime/src/headless/tests/protocol.rs` covers positive and negative protocol fixtures.
- `crates/animus-runtime/src/headless/mod.rs` and `crates/animus-runtime/src/lib.rs` reexport the protocol facade.
- `docs/pi-headless-rpc-protocol.md` documents scope, text fixture shape, non-claims, and verification.
- `cairn/runtime-facade-inventory.ncl` classifies the new headless protocol exports.

## Passed checks

- `cargo fmt --all` — PASS during focused test runs.
- `cargo test -p animus-runtime --locked` — PASS: 56 tests, including 5 headless protocol fixtures and the batch fixtures that depend on this prerequisite.

## Blocked broader checks

- `cargo clippy -p animus-runtime --all-targets --offline -- -D warnings` remains blocked before reaching the headless protocol module by pre-existing `animus-adapters` lint debt. First issue class observed: `clippy::vec_init_then_push` in `crates/animus-adapters/src/actor/admit/core.rs`, followed by existing `struct_field_names`, `too_many_arguments`, `assertions_on_constants`, wildcard import, and missing docs/panics diagnostics. Next best check run: locked `animus-runtime` unit/doc tests.
- `scripts/cairn/check-runtime-facade-inventory --root .` remains blocked by pre-existing inventory drift across existing exports. First issue class observed: `RUNTIME_FACADE_UNCLASSIFIED_EXPORT` for existing adapter exports. Next best check: new headless protocol exports were added to `cairn/runtime-facade-inventory.ncl` and the runtime facade doc was updated.

## Non-claims

No network daemon, web server, MCP server, TUI, Pi subprocess compatibility mode, generic shell command execution, provider call, session persistence, or ledger storage was added or claimed.
