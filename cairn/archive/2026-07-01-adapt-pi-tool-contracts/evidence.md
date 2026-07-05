# Evidence: adapt Pi tool contracts

## Lifecycle gates

- Baseline `nix run .#cairn -- gate proposal adapt-pi-tool-contracts --root .`: PASS.
- Baseline `nix run .#cairn -- gate design adapt-pi-tool-contracts --root .`: PASS.
- Baseline `nix run .#cairn -- gate tasks adapt-pi-tool-contracts --root .`: PASS.
- Closeout `gate proposal`, `gate design`, and `gate tasks`: PASS; task gate receipt `4f0d3e5fbb9d743281b5531e428d54b654176443542a83a295b4be0cc8d96141`.
- `nix run .#cairn -- validate --root .`: PASS after implementation and accepted-spec merge.
- `scripts/cairn/check-roadmap --root .`: PASS after stale archived-roadmap references were removed.

## Focused implementation checks

- Baseline before tool-contract edits: `nix develop -c cargo test -p animus-adapters tool --locked`: PASS, 11 focused tests.
- After implementation: `nix develop -c cargo test -p animus-adapters tool --locked`: PASS, 19 focused tests.
- Broader adapter rail: `nix develop -c cargo test -p animus-adapters --locked`: PASS, 195 tests plus doc-tests.
- Formatting: `nix develop -c cargo fmt --all`: PASS.
- Clippy: `nix develop -c cargo clippy -p animus-adapters --all-targets --locked -- -D warnings`: BLOCKED by pre-existing non-tool lint debt in already-staged/untracked modules. The recaptured log `.pi/drain/animus-adapters-clippy-tool-contracts-after-tool-fix.log` contains no `crates/animus-adapters/src/tool.rs` diagnostics.

## Implemented coverage

- Validated tool definition metadata includes name, label, description, schema identity, prompt snippet, prompt guidelines, source/provenance labels, capability labels, and result metadata shape.
- Prompt context construction includes only active tool metadata and keeps snippets/guidelines attributable to the tool name.
- Argument preparation applies pure compatibility mappings before strict current-field validation and rejects unknown, malformed, and duplicate prepared fields.
- Bounded output records visible text, truncation status, byte/character/line counts, safe references, and BLAKE3 content hashes without embedding full oversized bodies.
- Mutation planning serializes same-target mutation windows while leaving distinct targets independent.
- Dispatch preconditions return typed missing-executor and denied-capability failures without fallback process, shell, or provider execution.

## Accepted spec verification

- `agent_harness_runtime.pi_tool_contracts.*` requirement IDs are present in `cairn/specs/agent-harness-runtime/spec.md` after manual merge because the sync dry-run/execute plan did not insert the delta text automatically.
