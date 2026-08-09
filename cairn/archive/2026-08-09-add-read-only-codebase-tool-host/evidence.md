## Baseline

The production executor kit admitted a tool name and argument-name set but did not bind the complete request values. Animus also had no non-default read-only codebase tool host through that kit. The broad Nix rail used the stale `host_control_conformance` test target instead of the available `hostcontrol` target.

## Implementation

- The execution plan now uses a domain-separated BLAKE3 identity over the complete canonical tool request.
- Execution rejects request substitution before the injected service can run.
- Successful execution returns one bounded `ToolResponse` and one metadata-only `ToolExecutionReceipt`.
- The non-default `codebase-tool-host` feature exposes the exact `codebase_graph_query` tool over an explicit capability root and explicit Rust file list.
- The packaged smoke requires explicit root, path, symbol, kind, and result-limit arguments.
- The root compatibility facade was split into responsibility-based files to keep each source file within the Tiger Style line bound.
- The Harbor ATIF integration target now uses a Tiger Style-compatible hyphenated filename.

## Verification

- `cargo test --workspace --all-targets --locked` passed with 607 `animus-runtime` tests, 79 `animus-std` tests, and all other workspace targets passing or retaining their declared ignored live tests.
- `cargo clippy --workspace --all-targets --locked -- -D warnings` passed.
- `cargo fmt --all -- --check` passed.
- `scripts/cairn/check-codebase-tool-host --root .` passed its positive and negative Rust tests, feature-isolation checks, catalog checks, and packaged smoke checks.
- `scripts/cairn/check-codebase-graph --root .` passed.
- Component catalog, runtime facade inventory, dogfood-readiness, request-lifecycle, and Nickel export checks passed.
- `nix build .#checks.x86_64-linux.codebase-tool-host --no-link -L` passed.
- `nix build .#checks.x86_64-linux.host-control-extraction --no-link -L` passed with the corrected `hostcontrol` target.
- The focused executor-kit tests and default-feature Clippy checks passed after all core changes.
- The broad `nix flake check -L` now passes the repaired host-control target and reaches Octet. Octet reports 254 existing findings only in `active_turn_recovery`, `applicationbinding`, and `rejected_attempt`; it reports no finding in the changed executor kit, codebase tool host, facade split, or Harbor target. This existing debt blocks a complete flake verdict.
- Final tasks gate receipt: `ee4ade0f7b9d27cc637f2271a305384eac23e392c50ef88fd250d8104b0c7ce3`.
- Accepted-spec dry-run plan and receipt: `ea0b1a774858e6ee5d2effed8da5bd3249768c19de91895bdecdb124cb6d8568` and `c155f7167264cd844a647555367717ea8737446e1b0579d1b3fa575084654836`.
- Accepted-spec execution plan and receipt: `748cf36d0322aa03104ec6b2d9b3dcc703a2e0a410b8510b2c23c80fa4b2817a` and `c0986aaad935d39ca9bd9814b4f2fc98a93f9ec639dd31b01b4f3ddb8cb8073a`.
- Accepted specification hash after sync: `1eb66993b40a2b2fd904158f928a1bdb12e42e5a7246ca3e92eba67443206c67`.
- Provider-directed tool use and the live OpenAI smoke were not run.

## Non-claims

Passing deterministic and focused checks does not prove semantic code search, live provider-directed tool use, model quality, production readiness, host sandboxing, credential availability, network availability, or external service reliability. The codebase host does not gain mutation, process, network, provider, plugin, environment-discovery, or current-directory-discovery authority.
