# animus

`animus` is a modular Rust agent-harness framework scaffolded with explicit crate barriers, swappable adapters, and a functional-core / imperative-shell split.

## Architecture

- `crates/animus-core` is the `#![no_std]` functional core. It uses `alloc` only and owns deterministic business logic and typed core state.
- `crates/animus-adapters` is the `#![no_std]` component boundary crate. It defines implementation-free adapter contracts and DTOs for swappable harness components such as models and tools.
- `crates/animus-preserves` is the optional Preserves serialization bridge. It keeps cross-language text/binary boundary encoding out of the core contracts while making adapter messages portable.
- `crates/animus-steel` is the canonical Steel Scheme loop fabric. It lets embedders script the agent turn loop while Rust revalidates every selected model/tool/response action at adapter boundaries.
- `crates/animus-runtime` is the reusable agent-loop runtime. It drives typed Steel/fabric decisions through injected model and tool adapters, emits ordered runtime events/reports, and keeps provider, filesystem, process, network, UI, and persistence authority outside the runtime core.
- `crates/animus-std` is the standard-library host shell. It owns environment/config parsing, process-facing orchestration, and default std-backed adapters.
- `crates/animus-tui` is the optional terminal UI host shell. It keeps Rats/Ratatui-facing dependencies outside reusable core, adapter, Steel, and std runtime contracts.
- The root `animus` package is a thin compatibility layer that re-exports the workspace crates.

Embedders should depend on the smallest crate set they need: `animus-core` for pure state, `animus-adapters` for port contracts, `animus-preserves` for Preserves text/binary boundary payloads, `animus-steel` for the main scriptable agent-loop fabric, `animus-runtime` for reusable loop orchestration over injected adapters, `animus-std` for the default host shell, `animus-tui` for an optional Rats-backed terminal host shell, or root `animus` for the combined compatibility surface. The root `animus` crate enables the `std` feature by default for compatibility; disable default features to select the no-std facade over `animus-core` and `animus-adapters`. Runtime configuration is parsed once at the shell boundary and stored in typed structs before being passed into the core. Component calls cross typed adapter contracts so model, tool, memory, and host integrations can be swapped without changing core logic.

## Development safety rails

The workspace denies missing docs, `clippy::all`, `clippy::pedantic`, and undocumented unsafe blocks at crate roots. Unsafe code is currently forbidden by workspace lint policy. `config/repo.ncl` is the Nickel source of truth for repo-owned tool config, and the generated `dylint.toml` sets every registered Octet lint to `deny` for the pinned Octet input.

Every test suite should include positive and negative tests. Run baseline tests before core changes and repeat them after the change. Avoid process-global state in tests unless explicit isolation is in place.

See [docs/safety-contract.md](docs/safety-contract.md) for the durable repo-owned safety contract, [docs/claude-code-permission-modes.md](docs/claude-code-permission-modes.md) for bounded local permission-mode caveats, [docs/request-lifecycle.md](docs/request-lifecycle.md) for the maintained request-lifecycle ownership map, [docs/status-authority.md](docs/status-authority.md) for semantic agent status precedence, [docs/branchfs-speculative-workspaces.md](docs/branchfs-speculative-workspaces.md) for the bounded speculative-workspace reference boundary, [docs/agent-definition-trust.md](docs/agent-definition-trust.md) for typed agent-definition trust boundaries, [docs/hermes-skill-manifests.md](docs/hermes-skill-manifests.md) for host-provided skill manifest descriptors, [docs/goose-recipe-workflows.md](docs/goose-recipe-workflows.md) for reusable recipe-workflow contracts, [docs/goose-recipe-backed-subagents.md](docs/goose-recipe-backed-subagents.md) for recipe-backed child-run boundaries, [docs/goose-acp-mcp-interop.md](docs/goose-acp-mcp-interop.md) for ACP/MCP bridge descriptors, [docs/goose-scheduled-recipes.md](docs/goose-scheduled-recipes.md) for scheduled workflow jobs, [docs/runtime-facade-inventory.md](docs/runtime-facade-inventory.md) for public export classes, [docs/focused-verification-rails.md](docs/focused-verification-rails.md) for focused rail conventions, and [docs/release-evidence-index.md](docs/release-evidence-index.md) for the bounded future release-evidence index evaluation.

## Development shell

Enter the pinned Nix development shell:

```sh
nix develop
```

The shell provides Rust, Clippy, rustfmt, rust-analyzer, Nickel, pre-commit, and Octet/Tigerstyle tooling.

## Configuration

Repo-owned tool config lives in `config/repo.ncl` with typed Nickel contracts. The tool-native files `.cargo/config.toml`, `.pre-commit-config.yaml`, `clippy.toml`, and `dylint.toml` are generated artifacts for consumers that require TOML or YAML.

After editing `config/repo.ncl`, regenerate native config files:

```sh
scripts/config/export-nickel-configs
```

Check for generated-config drift:

```sh
check-nickel-configs
# or
nix run .#check-nickel-configs
```

## Wasm compile surface

The supported Wasm compile boundary targets `wasm32-unknown-unknown` for deterministic reusable contracts. The initial supported surface is:

```sh
cargo check -p animus-core --target wasm32-unknown-unknown --locked
cargo check -p animus-adapters --target wasm32-unknown-unknown --locked
cargo check -p animus --no-default-features --target wasm32-unknown-unknown --locked
```

The root `animus --no-default-features` profile intentionally excludes `animus-std`, `animus-steel`, `animus-tui`, `animus-preserves`, terminal UI, provider clients, native persistence, process execution, filesystem authority, credentials, browser APIs, WASI host behavior, plugin loading, and host shell orchestration. Optional bridges such as Preserves are not part of the initial Wasm rail; add them only with separate compatibility evidence and adapter DTO revalidation tests.

Run the focused rail:

```sh
scripts/cairn/check-wasm-surface --root .
nix build .#checks.x86_64-linux.wasm-surface --no-link -L
```

## Verification

```sh
cargo test -p animus-core
cargo test -p animus-adapters
cargo test -p animus-preserves
cargo test -p animus-steel
cargo test -p animus-runtime
cargo test -p animus-std
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p animus --features preserves
cargo check -p animus --features steel
cargo check -p animus --features runtime
cargo check -p animus --features tui
cargo tigerstyle check
check-nickel-configs
scripts/cairn/check-clankers-product-surfaces --root .
scripts/cairn/check-clash-ucan-authority --root .
scripts/cairn/check-plugin-manifest-runtime-neutral --root .
scripts/cairn/check-request-lifecycle --root .
scripts/cairn/check-steel-runtime-policy --root .
scripts/cairn/check-neutral-ledger --root .
scripts/cairn/check-injected-services --root .
scripts/cairn/check-release-evidence-index --root .
scripts/cairn/check-runtime-facade-inventory --root .
scripts/cairn/check-safe-receipts-artifacts --root .
scripts/cairn/check-tool-catalog --root .
scripts/cairn/check-wasm-surface --root .
nix run .#cargo-tigerstyle -- check
nix run .#check-nickel-configs
nix build .#checks.x86_64-linux.clash-ucan-authority --no-link -L
nix build .#checks.x86_64-linux.neutral-ledger --no-link -L
nix build .#checks.x86_64-linux.safe-receipts-artifacts --no-link -L
nix flake check
```

## Cairn drain helper

Generate a Steel Scheme drain plan for selected active changes:

```sh
steel scripts/ralph-drain-cairns.scm --change introduce-steel-runtime-loop
```

The helper defaults to the repo-pinned Cairn command (`nix run .#cairn --`) and emits Animus-specific guardrails for roadmap blockers, accepted-spec sync verification, evidence safety, and non-claims. Use `--self-test` to run its positive and negative checks.

## Pre-commit hooks

Install the repo hooks once per checkout:

```sh
pre-commit install
```

The `nickel-configs` hook runs `scripts/config/check-nickel-configs` before each commit to reject generated-config drift. The `octet-lints` hook runs `scripts/hooks/octet-lints`; that wrapper invokes the pinned flake app with `nix run .#cargo-tigerstyle -- check` when Nix is available, then falls back to `cargo tigerstyle check`.

## References

- [Basalt](https://github.com/OnixResearch/basalt) — hardened UCAN contract boundaries for Nickel policy artifacts and Steel DTO evaluation backends. Added as the optional Cargo feature `basalt-reference` through the SSH-pinned `basalt` package and as a flake source input.
- [Cairn](https://github.com/OnixResearch/cairn) — Rust lifecycle engine for native repository lifecycle work. Added as the optional Cargo feature `cairn-reference` through the SSH-pinned `cairn-core` package and as a flake source input.
- [Trellis](https://github.com/OnixResearch/trellis) — composable verified-logic building blocks. Added as the optional Cargo feature `trellis-reference` through the SSH-pinned `verified-logic` package and as a flake source input.
- [`../clankers`](../clankers) — terminal coding agent in Rust, inspired by pi and built to be hacked on.
- [Clash](https://github.com/empathic/clash) — command-line agent safety harness used as a bounded reference for capability taxonomy, explain traces, sandbox descriptors, and scenario tests; not adopted wholesale.
- [Pi](https://github.com/earendil-works/pi) — self-extensible coding agent harness with tool calling, terminal UI, and multi-provider LLM runtime; used as a reference codebase.
- [Git AI](https://github.com/git-ai-project/git-ai) — explicit line-level AI provenance, checkpointing, attribution stats, and Git notes interoperability used as a bounded reference for VCS-neutral Animus provenance; Git-specific behavior remains optional adapter scope.
- [Toolpath](https://github.com/empathic/toolpath) — tool-agnostic artifact transformation provenance format used as a bounded reference for optional redacted Graph/Path/Step interchange; Animus-owned provenance remains internal.
- [Qualifier](https://github.com/empathic/qualifier) — continuous annotation and Metabox-style record substrate used as a bounded reference for safe quality records, span freshness, append-only sidecars, and AGENTS-CLI guidance; not adopted as the Animus session ledger or core runtime dependency.
- [Preserves](https://preserves.dev/) — expressive data model and text/binary serialization formats for portable adapter boundary payloads. Added through the optional `animus-preserves` crate and root `preserves` feature.
- [Syndicate](https://syndicate-lang.org/) / [Syndicate/rs](https://git.syndicate-lang.org/syndicate-lang/syndicate-rs) — syndicated actors, dataspaces, assertion-based state replication, facets, and conversational concurrency; useful as an architectural reference for future runtime/dataspace adapters, not a core dependency yet.
- [Steel Scheme](https://github.com/mattwparas/steel) — embeddable Scheme implementation used by `animus-steel` as the main scriptable agent-loop fabric.
- [Rats](https://github.com/brittonr/rats) — Rust terminal UI widget workspace used by the optional `animus-tui` host shell through an SSH-pinned Cargo dependency.
- [`../hermes`](../hermes) — Hermes Agent workspace used as a bounded reference for skill manifests, plugin/tool manifests, review report contracts, Kanban orchestration, policy profiles, retrieval budgets, background work queues, and ResearchSpec lifecycle patterns.
- [Herdr](https://github.com/ogulcancelik/herdr) — terminal-native agent multiplexer reference for semantic agent status authority, session restore semantics, host-control protocol shape, and plugin/action manifests; adapted only as bounded patterns behind Animus-owned contracts.
- [Claude Code](https://docs.anthropic.com/en/docs/claude-code/overview) — coding-agent product reference for layered project memory, permission modes, workflow commands, lifecycle hooks, agent teams, headless automation, and MCP bridge ergonomics; adapted only as bounded patterns behind Animus-owned contracts.
- [Goose](https://goose-docs.ai/) — general-purpose agent reference for reusable recipes, structured automation output, ACP/MCP interoperability, recipe-backed subagents, and layered security review; adapted only as bounded patterns behind Animus-owned contracts.
- [vLLM Semantic Router micro-agent recipes](https://vllm.ai/blog/2026-06-29-micro-agent-frontier-models) — router-side Confidence, Ratings, ReMoM, Fusion, and workflow recipe patterns used as a bounded reference for Animus-owned model-router contracts; no vLLM serving dependency or benchmark claims are adopted.
- [Claude Converter](https://github.com/FredyRivera-dev/claude_converter) — compact session-to-Hugging-Face `messages` conversion and inspection surface used as a bounded reference for Animus-owned dataset export; Animus sessions remain the source format.
- [OpenAI Codex](https://github.com/openai/codex) — coding-agent reference for exec policy rules, hook trust lifecycles, non-interactive JSONL execution and SDK control, worktree automations, subagent threads, guidance/skill budgeting, opt-in memories, and read-only review mode; adapted only as bounded patterns behind Animus-owned contracts.
- [Sandlock](https://github.com/multikernel/sandlock) — lightweight Linux process sandbox using Landlock, seccomp-bpf, seccomp user notification, COW filesystem handling, and network/HTTP ACLs; useful as a bounded reference for std-shell sandbox enforcement and descriptor translation, not a core dependency.
- [BranchFS](https://github.com/multikernel/branchfs) — FUSE-based speculative branching filesystem with copy-on-write workspaces, nested branches, atomic commit/abort, and `@branch` paths; useful as a bounded reference for optional workspace-isolation adapters, not a required dependency.
- [`../mc`](../mc) — sibling Cairn-heavy workspace whose `scripts/ralph-drain-cairns.scm` Steel loop was adapted as an Animus-specific drain planner; no Minecraft runtime behavior is adopted.
- [Zerostack](https://github.com/gi-dellav/zerostack) — minimal Rust coding-agent reference for permission modes, prompt profiles, provider registry hardening, hashed edits, mid-turn compaction, fresh-context tasks, and context-file layering; adapted only as clean-room bounded patterns due to GPL licensing.
